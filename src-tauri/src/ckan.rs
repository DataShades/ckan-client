#![allow(unused_imports, dead_code)]

use std::sync::Mutex;

use ckanapi::{Action, Params, CKAN};

use serde::{Deserialize, Serialize};


pub struct CKANState(pub Mutex<CKAN>);

impl CKANState {

    pub fn replace(&self, ckan: CKAN) {
        *self.0.lock().unwrap() = ckan;
    }
}

pub fn ckan_state() -> CKANState {
    CKANState(Mutex::new(CKAN::new("https://demo.ckan.org")))
}

#[derive(Debug, Deserialize)]
pub struct AvailableProjects {
    count: i32,
    results: Vec<Project>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    id: String,
    name: String,
    title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    display_name: String,
    id: String,
}

pub fn user_info(ckan: &CKAN) -> Option<User> {
    let action = Action::new("nswflood_me", Params::Empty);
    ckan.invoke(action).extract()
}

pub fn available_projects(ckan: &CKAN, name: String) -> Vec<Project> {
    let action = Action::new(
        "nswflood_available_project_list",
        Params::Json(serde_json::json!({"name": name, "rows": 10, "fl": "id,name,title"})),
    );
    match ckan.invoke(action).extract() {
        Some(projects) => projects,
        None => AvailableProjects {
            count: 0,
            results: vec![],
        },
    }
    .results
}

#[cfg(test)]
mod tests {
    use super::*;
    const TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJqdGkiOiJ0VUJSdWptc1hrNGFxX0JnOVVsUW4xQlBwR1ZYeTBIRk8tb2Y2TFR3VW1qaGxvcFYzbk1yZ0VIRTJiQjZpMm83NU5DRU5HWTV4TnFmaV9VaCIsImlhdCI6MTY1NDA5NTEyNX0.Nl1IM48b-dQhnh6COyZjkgmwnfgjZDCCGP5S-OtwRSc";

    #[test]
    fn test_name() {
        env_logger::builder().is_test(true).init();

        let ckan = CKAN::new("http://localhost:5000").with_token(TOKEN);

        if let Some(user) = user_info(&ckan) {
            log::debug!("{:?}", user);
        }

        let projects = available_projects(&ckan);
        log::debug!("{:#?}", projects);
    }
}
