mod source;

use serde::{Deserialize, Serialize};

pub use self::source::{Source, Dataset, Resource, Metadata};


#[derive(Debug,Deserialize)]
pub struct AvailableProjects {
    pub count: i32,
    pub results: Vec<Project>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub display_name: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Portal {
    pub url: Option<String>,
    pub token: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct RegisteredUpload {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProgressedUpload {
    pub id: String,
    pub data: UploadData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadData {
    pub key: String,
    pub size: u64,
    pub dataset: String,
    pub name: String,
    pub completed: bool,
    pub bytes_uploaded: u64,

}
