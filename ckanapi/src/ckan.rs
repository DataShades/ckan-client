use std::collections::HashMap;

use reqwest::blocking::{
    multipart::{Form, Part},
    Client,
};

use serde::Deserialize;

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
/// # use ckanapi::{CKAN, Action, Params};
/// # use serde::Deserialize;
/// #[derive(Deserialize, Debug)]
/// struct StatusShow {
///     site_title: String,
/// }
///
/// # fn main() {
/// let mut client = CKAN::from("https://demo.ckan.org");
/// let action = Action::new("status_show");
///
/// match client.invoke(action, Params::Empty).extract() {
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

    pub fn invoke<T: for<'de> Deserialize<'de>>(
        &self,
        action: Action,
        params: Params,
    ) -> Response<T> {
        let url = format!("{}{}", self.url, action.to_path());
        let mut req = self.client.post(url);
        if let Some(token) = &self.token {
            req = req.header(reqwest::header::AUTHORIZATION, token);
        }

        req = match params {
            Params::Empty => req,
            Params::Multipart(plain, files, blobs) => {
                let mut form = Form::new();
                for (k, v) in plain.into_iter() {
                    form = form.text(k, v);
                }

                for (k, v) in files.into_iter() {
                    form = match form.file(k, v) {
                        Ok(form) => form,
                        Err(err) => return Response::FileError(err.to_string()),
                    };
                }

                for (k, v) in blobs.into_iter() {
                    form = form.part(
                        k,
                        Part::bytes(v)
                            .file_name("upload")
                            .mime_str("application/octet-stream")
                            .expect("Unexpected content type"),
                    );
                }

                req.multipart(form)
            }
            Params::Json(ref data) => req.json(data),
        };

        match req.send() {
            Ok(resp) => match resp.json::<Response<T>>() {
                Ok(result) => result,
                Err(err) => Response::DecodeError(err.to_string()),
            },
            Err(err) => Response::ReqwestError(err.to_string()),
        }
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

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Result(Success<T>),
    Error(Fail),
    StringError(String),
    ReqwestError(String),
    DecodeError(String),
    FileError(String),
}

impl<T: std::fmt::Debug> Response<T> {
    pub fn extract(self) -> Option<T> {
        match self {
            Response::Result(Success { result, .. }) => Some(result),
            err => {
                log::error!("Error during API request: {:?}", err);
                None
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
    pub error: serde_json::Value,
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

#[derive(Debug)]
pub enum Params {
    Empty,
    Multipart(
        HashMap<String, String>,
        HashMap<String, String>,
        HashMap<String, Vec<u8>>,
    ),
    Json(serde_json::Value),
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_with_any_string() {
        _ = CKAN::from("http://localhost:5000");
        _ = CKAN::from("http://localhost:5000".to_string());
    }

    #[test]
    fn test_status_show() {
        let client = CKAN::from("http://localhost:5000");
        let result = client.invoke(Action::from("status_show"), Params::Empty).extract();
        dbg!(result);
    }
    #[tokio::test]
    async fn test_async() {}
}
