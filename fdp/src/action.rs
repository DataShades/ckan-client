use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Seek};

use crate::read_source_path;
use crate::types::{
    AvailableProjects, Metadata, ProgressedUpload, Project, RegisteredUpload, User,
    ValidationResult,
};
use ckanapi::{Params, CKAN};
use serde_json::{json, Value};

const CHUNK_SIZE: usize = 5 * 1024 * 1024;

pub trait FdpClient {
    fn user_info(&self) -> crate::Result<User>;
    fn available_projects(&self, name: &str) -> Vec<Project>;
    fn project_set(&self, id: Option<&str>) -> crate::Result<Value>;

    fn show_submission(&self) -> Option<Vec<Value>>;

    fn validate_dataset<P: AsRef<OsStr>>(
        &self,
        path: P,
        name: &str,
    ) -> crate::Result<ValidationResult>;
    fn validate_resource<P: AsRef<OsStr>>(
        &self,
        path: P,
        dataset: &str,
        name: &str,
    ) -> crate::Result<ValidationResult>;

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
    fn user_info(&self) -> crate::Result<User> {
        let resp = self.build("nswflood_me").send::<User>()?;

        Ok(resp.extract()?)
    }

    fn available_projects(&self, name: &str) -> Vec<Project> {
        let payload = Params::Json(json!({"name": name, "rows": 10, "fl": "id,name,title"}));

        match self
            .build("nswflood_available_project_list")
            .params(payload)
            .send()
            .map_or(None, |r| r.extract().ok())
        {
            Some::<AvailableProjects>(projects) => projects.results,
            None => Vec::new(),
        }
    }

    fn project_set(&self, id: Option<&str>) -> crate::Result<Value> {
        let payload = Params::Json(json!({"id": id}));
        let resp = self.build("nswflood_submission_project_set").params(payload).send()?;
        Ok(resp.extract()?)
    }

    fn show_submission(&self) -> Option<Vec<Value>> {
        self.build("nswflood_submission_details")
            .params(Params::Empty)
            .send()
            .ok()?
            .extract()
            .ok()
    }

    fn show_upload(&self, dataset: &str, name: &str) -> Option<ProgressedUpload> {
        let payload = Params::Json(json!({"name": name, "dataset": dataset}));

        self.build("nswflood_upload_show")
            .params(payload)
            .send()
            .ok()?
            .extract()
            .ok()?
    }

    fn validate_dataset<P: AsRef<OsStr>>(
        &self,
        path: P,
        name: &str,
    ) -> crate::Result<ValidationResult> {
        let source = read_source_path(&path)?;
        let root_metadata = match source.metadata {
            Metadata::Empty => json!({}),
            Metadata::Object(ref v) => v.clone(),
        };
        let dataset = source.get_dataset(name).ok_or("Dataset not found")?;
        match &dataset.metadata {
            Metadata::Empty => Err("Dataset has no metadata".into()),
            Metadata::Object(metadata) => {
                let req = self
                    .build("nswflood_submission_validate_dataset")
                    .params(Params::Json(
                        json!({"data": metadata.clone(), "name": &dataset.name, "root": root_metadata}),
                    ));
                Ok(req.send()?.extract()?)
            }
        }
    }

    fn validate_resource<P: AsRef<OsStr>>(
        &self,
        path: P,
        dataset: &str,
        name: &str,
    ) -> crate::Result<ValidationResult> {
        let source = read_source_path(&path)?;
        let dataset = source.get_dataset(dataset).ok_or("Dataset not found")?;
        let res = dataset.get_resoure(name).ok_or("Resource not found")?;
        match &res.metadata {
            Metadata::Empty => Err("Resource has no metadata".into()),
            Metadata::Object(metadata) => {
                let req = self
                    .build("nswflood_submission_validate_resource")
                    .params(Params::Json(
                        json!({"data": metadata.clone(), "dataset": &dataset.name, "name": &res.name}),
                    ));
                Ok(req.send()?.extract()?)
            }
        }
    }

    fn register_upload(&self, path: &str, dataset: &str, name: &str) -> Option<RegisteredUpload> {
        let source = read_source_path(&path).ok()?;
        let res = source.get_dataset(dataset)?.get_resoure(name)?;
        let payload = Params::Json(json!({"name": name, "dataset": dataset, "size": res.size()}));

        self.build("nswflood_upload_register")
            .params(payload)
            .send()
            .ok()?
            .extract()
            .ok()
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

        match self
            .build("nswflood_upload_progress")
            .params(payload)
            .send()
            .ok()?
            .extract()
            .ok()
        {
            None => None,
            Some::<ProgressedUpload>(flake) => {
                if flake.data.bytes_uploaded == res.size() {
                    self.build("nswflood_upload_complete")
                        .params(Params::Json(json!({"dataset": dataset, "name": name})))
                        .send()
                        .ok()?
                        .extract()
                        .ok()
                } else {
                    Some(flake)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::types::Source;

    use super::*;
    fn ckan() -> CKAN {
        let mut ckan = CKAN::from("http://localhost:5000");

        if let Some(token) = std::env::var_os("CKAN_TOKEN") {
            ckan.login(token.into_string().unwrap());
        }

        ckan
    }

    #[test]
    fn test_user_info_return_data() {
        let ckan = ckan();
        let result = ckan.user_info();
        assert!(result.is_ok(), "Cannot get user info: {:?}", result);
    }

    #[test]
    fn test_project_set() {
        let ckan = ckan();
        let result = ckan.project_set("<id>");
        assert!(result.is_ok(), "Cannot set project: {:?}", result);
    }

    #[test]
    fn test_validate_dataset_and_resource() {
        let dir = tempdir().unwrap();
        let mut source = Source::new(dir.path()).unwrap();
        source.metadata = Metadata::default();
        source.metadata.write(&source.metadata_path()).unwrap();

        source.add_dataset("dataset").unwrap();

        let ckan = ckan();
        assert!(ckan.validate_dataset(&source.path, "dataset").is_err());

        let dataset = source.get_dataset_mut("dataset").unwrap();

        dataset.add_resource("resource").unwrap();

        let resource = dataset.get_resoure_mut("resource").unwrap();
        resource.metadata = Metadata::Object(json!({"url": "https://demo.ckan.org"}));
        resource.metadata.write(&resource.metadata_path()).unwrap();

        dataset.metadata = Metadata::Object(json!({}));
        dataset.metadata.write(&dataset.metadata_path()).unwrap();

        let result: ValidationResult = ckan.validate_dataset(&source.path, "dataset").unwrap();
        assert_eq!(result.data, json!({}));
        assert!(result.errors.as_object().unwrap().len() > 0);

        let result: ValidationResult = ckan.validate_dataset(&source.path, "dataset").unwrap();
        assert_eq!(result.data, json!({}));
        assert!(result.errors.as_object().unwrap().len() > 0);

        let result: ValidationResult = ckan
            .validate_resource(&source.path, "dataset", "resource")
            .unwrap();
        assert_eq!(
            result.data,
            json!({"url": "https://demo.ckan.org", "format": "org"})
        );
        assert_eq!(result.errors, json!({}));
    }
}
