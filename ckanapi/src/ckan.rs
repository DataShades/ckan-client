use std::collections::HashMap;

use reqwest::blocking::{
    multipart::{Form, Part},
    Client,
};

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug)]
pub struct CKAN {
    url: String,
    token: Option<String>,
    client: Client,
}

impl CKAN {
    pub fn new(url: &str) -> Self {
        CKAN {
            url: url.trim_matches('/').to_owned(),
            client: Client::new(),
            token: None,
        }
    }

    pub fn with_token(self, token: &str) -> Self {
        CKAN {
            token: Some(token.into()),
            ..self
        }
    }
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn invoke<T: for<'de> Deserialize<'de>>(&self, action: Action) -> Response<T> {
        let url = format!("{}/api/action/{}", self.url, &action.name);
        let mut req = self.client.post(url);
        if let Some(token) = &self.token {
            req = req.header(reqwest::header::AUTHORIZATION, token);
        }

        req = match action.params {
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
    pub error: Value,
}

#[derive(Debug)]
pub struct Action {
    pub name: String,
    pub params: Params,
}

impl Action {
    pub fn new(name: &str, params: Params) -> Self {
        Action {
            name: name.into(),
            params,
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
    Json(Value),
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    // use super::*;

    //     #[test]
    //     fn test_name() {
    //         let ckan = CKAN::new("http://localhost:5000");

    //         let mut plain: HashMap<String, String> = HashMap::new();
    //         plain.insert("rows".into(), "0".into());
    //         plain.insert("fl".into(), "id".into());

    //         let action = Action::new(
    //             "package_search",
    //             // Params::Json(serde_json::json!({"rows": 1, "fl": "id"})),
    //             Params::Multipart(plain, HashMap::new()),
    //         );

    //         let result: Response<Value> = ckan.invoke(action);
    //         // dbg!(result);
    //     }
}
