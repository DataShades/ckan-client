use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct AvailableProjects {
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
