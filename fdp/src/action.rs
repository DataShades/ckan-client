use std::collections::HashMap;
use std::fs::File;
use std::io::{Seek, Read};

use crate::types::{AvailableProjects, Project, User, RegisteredUpload ,ProgressedUpload};
use crate::read_source_path;
use ckanapi::{Action, Params, CKAN};

const CHUNK_SIZE: usize = 5 * 1024 * 1024;

pub trait FdpClient {
    fn user_info(&self) -> Option<User>;
    fn available_projects(&self, name: &str) -> Vec<Project>;

    fn show_submission(&self) -> Option<Vec<serde_json::Value>>;
    fn show_upload(&self, dataset: &str, name: &str) -> Option<ProgressedUpload>;
    fn register_upload(&self, path: &str, dataset: &str, name: &str) -> Option<RegisteredUpload>;
    fn progress_upload(&self, path: &str, dataset: &str, name: &str, part: u64) -> Option<ProgressedUpload>;
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

    fn show_submission(&self) -> Option<Vec<serde_json::Value>> {
        let action = Action::new(
            "nswflood_submission_details",
            Params::Empty,
        );

        self.invoke(action).extract()

    }

    fn show_upload(&self, dataset: &str, name: &str) -> Option<ProgressedUpload> {
        let action = Action::new(
            "nswflood_upload_show",
            Params::Json(serde_json::json!({"name": name, "dataset": dataset})),
        );

        self.invoke(action).extract()

    }


    fn register_upload(&self, path: &str, dataset: &str, name: &str) -> Option<RegisteredUpload> {
        let source = read_source_path(&path).ok()?;
        let res = source.get_dataset(dataset)?.get_resoure(name)?;
        let action = Action::new(
            "nswflood_upload_register",
            Params::Json(serde_json::json!({"name": name, "dataset": dataset, "size": res.size()})),
        );

        self.invoke(action).extract()
    }

    fn progress_upload(&self, path: &str, dataset: &str, name: &str, part: u64) -> Option<ProgressedUpload> {
        let source = read_source_path(&path).ok()?;
        let res = source.get_dataset(dataset)?.get_resoure(name)?;
        let mut path = res.path.clone();
        path.push(&res.name);

        let mut file = File::open(path).ok()?;
        let offset = (part - 1) * CHUNK_SIZE as u64;
        file.seek(std::io::SeekFrom::Start(offset)).ok()?;
        let mut buf = vec![];


        let reader = std::io::BufReader::new(file);
        reader.take(CHUNK_SIZE as u64).read_to_end(&mut buf).expect("Cannot read a chunk from file");
        let size = buf.len();

        let mut blobs = HashMap::new();
        blobs.insert("content".to_string(), buf);

        let mut fields = HashMap::new();
        fields.insert("dataset".to_string(), dataset.to_string());
        fields.insert("name".to_string(), name.to_string());
        fields.insert("part_number".to_string(), part.to_string());
        fields.insert("size".to_string(), size.to_string());

        let files = HashMap::new();

        let action = Action::new(
            "nswflood_upload_progress",
            Params::Multipart(fields, files, blobs),
        );


        match self.invoke::<ProgressedUpload>(action).extract() {
            None => None,
            Some(flake) => {

                if flake.data.bytes_uploaded == res.size() {
                    let action = Action::new(
                        "nswflood_upload_complete",
                        Params::Json(serde_json::json!({"dataset": dataset, "name": name})),
                    );

                    self.invoke(action).extract()
                } else {
                    Some(flake)
                }
            }
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

        // if let Some(user) = ckan.user_info() {
        //     log::error!("{:?}", user);
        // }

        // let projects = ckan.available_projects("");
        // log::error!("{:#?}", projects);
        // let reg = ckan.show_submission();
        // dbg!(reg);
        // let reg = ckan.show_upload("src", "lib.rs");
        // dbg!(reg);

        // let reg = ckan.register_upload(".", "src", "lib.rs");
        // dbg!(reg);
        // let reg = ckan.progress_upload(".", "src", "lib.rs", 1);
        // dbg!(reg);

    }
}
