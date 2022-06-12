use std::fs::File;
use std::io::{Read, Seek};

use crate::read_source_path;
use crate::types::{AvailableProjects, ProgressedUpload, Project, RegisteredUpload, User};
use ckanapi::{Params, CKAN};

const CHUNK_SIZE: usize = 5 * 1024 * 1024;

pub trait FdpClient {
    fn user_info(&self) -> Option<User>;
    fn available_projects(&self, name: &str) -> Vec<Project>;

    fn show_submission(&self) -> Option<Vec<serde_json::Value>>;
    fn show_upload(&self, dataset: &str, name: &str) -> Option<ProgressedUpload>;
    fn register_upload(&self, path: &str, dataset: &str, name: &str) -> Option<RegisteredUpload>;
    fn progress_upload(
        &self,
        path: &str,
        dataset: &str,
        name: &str,
        part: u64,
    ) -> Option<ProgressedUpload>;
}

impl FdpClient for CKAN {
    fn user_info(&self) -> Option<User> {
        self.invoke("nswflood_me", Params::Empty).extract()
    }

    fn available_projects(&self, name: &str) -> Vec<Project> {
        let payload =
            Params::Json(serde_json::json!({"name": name, "rows": 10, "fl": "id,name,title"}));

        match self
            .invoke("nswflood_available_project_list", payload)
            .extract()
        {
            Some::<AvailableProjects>(projects) => projects.results,
            None => Vec::new(),
        }
    }

    fn show_submission(&self) -> Option<Vec<serde_json::Value>> {
        self.invoke("nswflood_submission_details", Params::Empty)
            .extract()
    }

    fn show_upload(&self, dataset: &str, name: &str) -> Option<ProgressedUpload> {
        let payload = Params::Json(serde_json::json!({"name": name, "dataset": dataset}));

        self.invoke("nswflood_upload_show", payload).extract()
    }

    fn register_upload(&self, path: &str, dataset: &str, name: &str) -> Option<RegisteredUpload> {
        let source = read_source_path(&path).ok()?;
        let res = source.get_dataset(dataset)?.get_resoure(name)?;
        let payload =
            Params::Json(serde_json::json!({"name": name, "dataset": dataset, "size": res.size()}));

        self.invoke("nswflood_upload_register", payload).extract()
    }

    fn progress_upload(
        &self,
        path: &str,
        dataset: &str,
        name: &str,
        part: u64,
    ) -> Option<ProgressedUpload> {
        let source = read_source_path(&path).ok()?;
        let res = source.get_dataset(dataset)?.get_resoure(name)?;
        let mut path = res.path.clone();
        path.push(&res.name);

        let mut file = File::open(path).ok()?;
        let offset = (part - 1) * CHUNK_SIZE as u64;
        file.seek(std::io::SeekFrom::Start(offset)).ok()?;
        let mut buf = vec![];

        let reader = std::io::BufReader::new(file);
        reader
            .take(CHUNK_SIZE as u64)
            .read_to_end(&mut buf)
            .expect("Cannot read a chunk from file");
        let size = buf.len();

        let mut payload = Params::multipart();
        payload
            .add_field("dataset".to_string(), dataset.to_string())
            .add_field("name".to_string(), name.to_string())
            .add_field("part_number".to_string(), part.to_string())
            .add_field("size".to_string(), size.to_string())
            .add_blob("content".to_string(), buf);

        match self.invoke("nswflood_upload_progress", payload).extract() {
            None => None,
            Some::<ProgressedUpload>(flake) => {
                if flake.data.bytes_uploaded == res.size() {
                    self.invoke(
                        "nswflood_upload_complete",
                        Params::Json(serde_json::json!({"dataset": dataset, "name": name})),
                    )
                    .extract()
                } else {
                    Some(flake)
                }
            }
        }
    }
}
