use crate::types::{AvailableProjects, Project, User};
use ckanapi::{Action, Params, CKAN};

pub trait FdpClient {
    fn user_info(&self) -> Option<User>;

    fn available_projects(&self, name: &str) -> Vec<Project>;
}

impl FdpClient for CKAN {
    fn user_info(&self) -> Option<User> {
        let action = Action::new("nswflood_me", Params::Empty);
        self.invoke(action).extract()
    }

    fn available_projects(&self, name: &str) -> Vec<Project> {
        let action = Action::new(
            "nswflood_available_project_list",
            Params::Json(serde_json::json!({"name": name, "rows": 10, "fl": "id,name,title"})),
        );
        match self.invoke::<AvailableProjects>(action).extract() {
            Some(projects) => projects.results,
            None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJqdGkiOiJ0VUJSdWptc1hrNGFxX0JnOVVsUW4xQlBwR1ZYeTBIRk8tb2Y2TFR3VW1qaGxvcFYzbk1yZ0VIRTJiQjZpMm83NU5DRU5HWTV4TnFmaV9VaCIsImlhdCI6MTY1NDA5NTEyNX0.Nl1IM48b-dQhnh6COyZjkgmwnfgjZDCCGP5S-OtwRSc";

    #[test]
    fn test_name() {
        env_logger::builder().is_test(true).init();

        let ckan = CKAN::new("http://localhost:5000").with_token(TOKEN);

        if let Some(user) = ckan.user_info() {
            log::info!("{:?}", user);
        }

        let projects = ckan.available_projects("");
        log::info!("{:#?}", projects);
    }
}
