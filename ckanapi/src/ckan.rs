use reqwest::{
    multipart::{Form, Part},
    Client,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

/// Client for the CKAN API.
///
/// It can be created from the string with a URL of the CKAN
/// application. Optionally, API Token can be used to switch the client into an
/// authenticated mode. Without a token, all requrest are made on behalf of an
/// anonymous user.
///
/// The result of an API call deserialized into the specified type.
///
/// ```no_run
/// # use ckanapi::{CKAN, Params};
/// # use serde::Deserialize;
/// #[derive(Deserialize, Debug)]
/// struct StatusShow {
///     site_title: String,
/// }
///
/// # fn main() {
/// let mut client = CKAN::from("https://demo.ckan.org");
///
/// match client.invoke("status_show", Params::Empty).extract() {
///     Some(StatusShow { site_title }) => assert_eq!("CKAN Demo", site_title),
///     None => panic!("CKAN Demo portal is not available")
/// }
/// # }
/// ```
///
/// If the application mounted under non-root path, this must be reflected in
/// the URL.
///
#[derive(Debug)]
pub struct CKAN {
    url: String,
    token: Option<String>,
    client: Client,
}

impl CKAN {
    /// Check if the client is anonymous(without an API Token).
    ///
    /// # Examples
    /// ```
    /// # let mut client = ckanapi::CKAN::from("http://demo.ckan.org");
    /// assert!(client.is_anon());
    ///
    /// client.login("my-secret-token");
    /// assert!(!client.is_anon());
    /// ```
    pub fn is_anon(&self) -> bool {
        self.token.is_none()
    }

    /// Set the API Token that will be used for authorization.
    ///
    /// # Examples
    /// ```
    /// # let mut client = ckanapi::CKAN::from("http://demo.ckan.org");
    /// client.login("my-secret-token");
    /// assert!(!client.is_anon());
    /// ```
    pub fn login<T: Into<String>>(&mut self, token: T) {
        self.token.replace(token.into());
    }

    /// Remove and return current API Token.
    ///
    /// # Examples
    /// ```
    /// # let mut client = ckanapi::CKAN::from("http://demo.ckan.org");
    /// client.login("token");
    /// let token = client.logout();
    ///
    /// assert!(client.is_anon());
    /// assert_eq!(Some("token".to_string()), token);
    /// ```
    pub fn logout(&mut self) -> Option<String> {
        self.token.take()
    }

    pub fn build<A: Into<Action>>(&self, action: A) -> RequestBuilder {
        let url = format!("{}{}", self.url, action.into().to_path());
        let mut req = self.client.post(url);
        if let Some(token) = &self.token {
            log::debug!("Set token: {}", token);
            req = req.header(reqwest::header::AUTHORIZATION, token);
        }
        RequestBuilder { request: req }
    }
}

pub struct RequestBuilder {
    request: reqwest::RequestBuilder,
}

impl RequestBuilder {
    pub fn params(mut self, params: Params) -> Self {
        self.request = match params {
            Params::Empty => self.request,
            Params::Multipart(fields) => {
                let mut form = Form::new();

                for (k, v) in fields {
                    form = match v {
                        MultipartField::Literal(v) => form.text(k, v),
                        // MultipartField::Filepath(_v) => {
                        //     panic!("Files are not supported at the moment")
                        // }
                        MultipartField::Blob(v) => form.part(
                            k,
                            Part::bytes(v)
                                .file_name("upload")
                                .mime_str("application/octet-stream")
                                .expect("Unexpected content type"),
                        ),
                    };
                }

                self.request.multipart(form)
            }
            Params::Json(ref data) => self.request.json(data),
        };
        self
    }
    pub async fn send<T>(self) -> Result<Response<T>, CKANError>
    where
        T: for<'de> Deserialize<'de>,
    {
        Ok(self.request.send().await?.json::<Response<T>>().await?)
    }
}

impl<T> From<T> for CKAN
where
    T: Into<String>,
{
    fn from(url: T) -> CKAN {
        let mut url = url.into();
        if !url.ends_with('/') {
            url.push('/');
        }

        CKAN {
            url,
            client: Client::new(),
            token: None,
        }
    }
}

#[derive(Error, Debug, Serialize)]
pub enum CKANError {
    #[error("{0}")]
    Request(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Authorization(String),

    #[error("{0}")]
    Validation(serde_json::Value),

    #[error("{0}")]
    Complex(Value),

    #[error("some error")]
    Plain,
}

impl From<reqwest::Error> for CKANError {
    fn from(source: reqwest::Error) -> Self {
        Self::Request(source.to_string())
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Result(Success<T>),
    Error(Fail),
    Exception(String),
}

impl<T: std::fmt::Debug> Response<T> {
    pub fn extract(self) -> Result<T, CKANError> {
        match self {
            Response::Exception(msg) => Err(CKANError::Request(msg)),
            Response::Result(Success { result, .. }) => Ok(result),
            Response::Error(Fail { mut error, .. }) => {
                match error["__type"] {
                    Value::String(ref t) if t == "Not Found Error" => {
                        if let Some(msg) = error["message"].as_str() {
                            Err(CKANError::NotFound(msg.to_string()))
                        } else {
                            Err(CKANError::Complex(error))
                        }
                    }
                    Value::String(ref t) if t == "Authorization Error" => {
                        if let Some(msg) = error["message"].as_str() {
                            Err(CKANError::Authorization(msg.to_string()))
                        } else {
                            Err(CKANError::Complex(error))
                        }
                    }
                    Value::String(ref t) if t == "Validation Error" => {
                        if let Some(map) = error.as_object_mut() {
                            map.remove("__type");
                        } else {
                        }
                        Err(CKANError::Validation(error))
                    }
                    _ => {
                        // dbg!("nothing", &error);
                        Err(CKANError::Complex(error))
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Success<T> {
    pub help: String,
    pub result: T,
}

#[derive(Deserialize, Debug)]
pub struct Fail {
    pub help: String,
    pub error: Value,
}

#[derive(Debug)]
pub struct Action {
    pub name: String,
    pub version: u8,
}

impl Action {
    fn to_path(&self) -> String {
        format!("api/{}/action/{}", self.version, &self.name)
    }
}

impl<T> From<T> for Action
where
    T: Into<String>,
{
    fn from(name: T) -> Self {
        Self {
            name: name.into(),
            version: 3,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Params {
    Empty,
    Multipart(Vec<(String, MultipartField)>),
    Json(Value),
}

impl Params {
    /// Create an empty multipart payload, suitable for the file-uploads.
    pub fn multipart() -> Self {
        Params::Multipart(Vec::new())
    }

    /// Create an empty JSON payload. Should be prefered most of the time.
    pub fn json() -> Self {
        Params::Json(serde_json::json!({}))
    }

    /// Add a plain field to the JSON or multipart payload.
    ///
    /// # Examples
    /// ```
    /// # use ckanapi::{Params, MultipartField};
    /// # use serde_json::json;
    /// let mut payload = Params::json();
    /// payload.add_field("name", "test");
    /// assert_eq!(Params::Json(json!({"name": "test"})), payload);
    /// ```
    ///
    /// ```
    /// # use ckanapi::{Params, MultipartField};
    /// # use serde_json::json;
    /// let mut payload = Params::multipart();
    /// payload.add_field("name", "test");
    /// assert_eq!(Params::Multipart(
    ///     vec![("name".into(), MultipartField::Literal("test".into()))]),
    ///     payload
    /// );
    /// ```
    pub fn add_field<N: Into<String>, V: Into<String>>(&mut self, name: N, value: V) -> &mut Self {
        match self {
            Params::Multipart(fields) => {
                fields.push((name.into(), MultipartField::Literal(value.into())))
            }
            Params::Json(data) => data[name.into()] = Value::from(value.into()),
            _ => {}
        };
        self
    }

    // pub fn add_file<N: Into<String>, V: Into<String>>(&mut self, name: N, value: V) -> &mut Self {
    //     if let Params::Multipart(fields) = self {
    //         fields.push((name.into(), MultipartField::Filepath(value.into())));
    //     }
    //     self
    // }

    /// Add a file to the multipart payload using its binary content.
    ///
    /// # Examples
    /// ```
    /// # use ckanapi::{Params, MultipartField};
    /// # use serde_json::json;
    /// let mut payload = Params::multipart();
    /// payload.add_blob("file", vec![53, 64, 55]);
    ///
    /// assert_eq!(Params::Multipart(
    ///     vec![("file".into(), MultipartField::Blob(vec![53, 64, 55]))]),
    ///     payload
    /// );
    /// ```
    pub fn add_blob<N: Into<String>, V: IntoIterator<Item = u8>>(
        &mut self,
        name: N,
        value: V,
    ) -> &mut Self {
        if let Params::Multipart(fields) = self {
            fields.push((
                name.into(),
                MultipartField::Blob(value.into_iter().collect()),
            ));
        }
        self
    }
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum MultipartField {
    Literal(String),
    // Filepath(String),
    Blob(Vec<u8>),
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use super::*;

    fn ckan() -> CKAN {
        let mut ckan = CKAN::from("http://localhost:5000");

        if let Some(token) = std::env::var_os("CKAN_TOKEN") {
            ckan.login(token.into_string().unwrap());
        }

        ckan
    }

    #[tokio::test]
    async fn test_status_show() {
        let resp: Value = ckan()
            .build("status_show")
            .send()
            .await.unwrap()
            .extract()
            .unwrap();
        assert_eq!(resp["site_title"], "CKAN Demo");
    }

    #[tokio::test]
    async fn test_params_json() {
        let mut payload = Params::json();
        payload.add_field("rows", "0");

        let resp: Value = ckan()
            .build("package_search")
            .params(payload)
            .send()
            .await.unwrap()
            .extract()
            .unwrap();
        assert!(resp["count"].as_i64().unwrap() > 0);
        assert!(resp["results"].as_array().unwrap().len() == 0);
    }

    #[tokio::test]
    async fn test_params_multipart() {
        let mut payload = Params::multipart();
        payload.add_field("rows", "0");

        let resp: Value = ckan()
            .build("package_search")
            .params(payload)
            .send()
            .await.unwrap()
            .extract()
            .unwrap();
        assert!(resp["count"].as_i64().unwrap() > 0);
        assert!(resp["results"].as_array().unwrap().len() == 0);
    }

    #[tokio::test]
    async fn test_auth_error() {
        let mut ckan = ckan();
        ckan.logout();
        let err = ckan
            .build("package_create")
            .send::<Value>()
            .await.unwrap()
            .extract()
            .err()
            .unwrap();
        match err {
            CKANError::Authorization(msg) => {
                // pass
            }
            _ => panic!("Unexpected error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_not_found_error() {
        let client = CKAN::from("http://localhost:5000");
        let err = client
            .build("package_show")
            .params(Params::Json(
                serde_json::json!({"id": "|not-a-read-dataset|"}),
            ))
            .send::<Value>()
            .await.unwrap()
            .extract()
            .err()
            .unwrap();
        match err {
            CKANError::NotFound(msg) => {
                assert_eq!("Not found", msg);
            }
            _ => panic!("Unexpected error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_validation_error() {
        let client = CKAN::from("http://localhost:5000");
        let err = client
            .build("package_show")
            .send::<Value>()
            .await.unwrap()
            .extract()
            .err()
            .unwrap();
        match err {
            CKANError::Validation(data) => {
                assert_eq!(serde_json::json!({"name_or_id": ["Missing value"]}), data);
            }
            _ => panic!("Unexpected error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_async() {}
}
