use csv;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use std::collections::HashMap;
use std::ops::Index;
use std::path::PathBuf;
use std::{
    ffi::OsStr,
    fs::{self, DirEntry},
};

const METADATA_EXT: &str = "csv";
const METADATA_FILENAME: &str = "metadata.csv";

#[derive(Debug)]
pub enum MetadataContent<'a> {
    Master,
    Dataset(&'a str),
    Resource(&'a OsStr, &'a str),
}
pub fn dataset_comments() -> HashMap<String, Vec<String>> {
    HashMap::from([
        ("first".into(), vec!["First field".into(), "---".into()]),
        (
            "dataset_type".into(),
            vec![
                "A number between 1 and 18(inclusive)".into(),
                "Allowed values and their corresponding types:".into(),
                "1: Dataset handover checklist and description".into(),
                "2: Report PDFs - all volumes".into(),
                "3: Completed NSW Flood Database Template".into(),
                "4: Spatial flood layers (post processed layers - not direct model outputs)".into(),
                "5: Collected Data".into(),
                "6: Hydrological, Hydraulic and flood damage model input files".into(),
                "7: Hydrological, Hydraulic and flood damage pre-processed model output files".into(),
                "8: Hydraulic modelling post processed files for AVIs".into(),
                "9: Base information on buildings".into(),
                "10: Survey Information – all required".into(),
                "11: Lidar".into(),
                "12: Aerial Imagery".into(),
                "13: Emergency Response Planning".into(),
                "14: Land use planning".into(),
                "15: Management options".into(),
                "16: Plans for works".into(),
                "17: All Other Required Data".into(),
                "18: Community Consultation".into(),
                ],
        ),
        ("title".into(), vec!["Provide a clear and unambiguous title for your Flood Dataset".into()]),
        (
            "name".into(),
            vec!["Unique name(URL) of the dataset".into(),
            "Only the following characters are allowed:".into(),
            "  * lowercased letters".into(),
            "  * digits".into(),
            "  * hyphens(-) and underscores(_)".into(),
            "In addition, name must be no longer than 100 characters(and at least 2 characters)".into(),
            ],
        ),
        (
            "notes".into(),
            vec![
                "Desctiption. For long descriptions use three quotation marks:".into(),
                "notes = \"\"\"Roses are red".into(),
                "Violets are blue\"\"\"".into(),
            ],
        ),
        (
            "publication_date".into(),
            vec!["Date in YYYY-MM-DD format: 2022-11-24".into()],
        ),
        ("tag_string".into(), vec!["Comma-separated tags".into()]),
        ("spatial_data".into(), vec!["Either 'yes' or 'no'".into()]),
        ("capture_method".into(), vec![
            "The method by which the spatial data was created; digitised and/or exported from a model".into(),
            "This field is required if `spatial_data` is set to `yes`. Otherwise this field must be empty".into(),
        ]),
        ("data_status".into(), vec![
            "Currency status of the data and other artefacts produced by the Flood Project. One of:".into(),
            "  * completed".into(),
            "    - Completed - production of the data has been completed".into(),
            "  * retired".into(),
            "    - Retired - item is no longer recommended for use. It has not been superseded by another item".into(),
            "  * superseded".into(),
            "    - Superseded - data for the whole geographic extent has been replaced by new data".into(),
            "  * partiallySuperseded".into(),
            "    - Partially Superseded - data for part of the geographic extent has been replaced by new data".into(),
            "  * deprecated".into(),
            "    - Deprecated - resource superseded and will become obsolete, use only for historical purposes".into(),
            "  * draft".into(),
            "    - Draft - this flood project is in the draft stage of development. It is NOT final.".into(),
            ]),
        ("license_id".into(), vec![
            "ID of the license. One of:".into(),
            " cc-by".into(),
            "    - Creative Commons Attribution 4.0".into(),
            " oeh".into(),
            "    - DPIE Licence".into(),
            " 3rd".into(),
            "    - 3rd party licence".into(),
            " internal".into(),
            "    - Internal use only".into(),
            ]),
        (
            "dataset_status".into(),
            vec!["One of: final, draft or updated".into()],
        ),
        (
            "update_freq".into(),
            vec!["One of: daily, weekly, monthly, quarterly, yearly, as_required".into()],
        ),
        ("author".into(), vec!["The name of the author".into()]),
        ("url".into(), vec!["Source URL of the dataset".into()]),
        ("data_comment".into(), vec!["Comment to the dataset".into()]),
        (
            "access_level".into(),
            vec!["One of: open, registered, internal, restricted".into()],
        ),
    ])
}

pub fn resource_comments() -> HashMap<String, Vec<String>> {
    HashMap::from([
        (
            "name".into(),
            vec!["Human-readable name of the resource".into()],
        ),
        (
            "description".into(),
            vec![
                "Desctiption. For long descriptions use three quotation marks:".into(),
                "description = \"\"\"Roses are red".into(),
                "Violets are blue\"\"\"".into(),
            ],
        ),
    ])
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Source {
    pub path: PathBuf,
    pub metadata: Metadata,
    pub datasets: Vec<Dataset>,
}

impl Source {
    pub fn new<T: AsRef<OsStr>>(path: T) -> Option<Self> {
        let path = PathBuf::from(path.as_ref());
        if path.is_dir() {
            let mut source = Source {
                path,
                metadata: Metadata::Empty,
                datasets: Vec::new(),
            };
            source.refresh_metadata();
            source.gather_datasets();
            Some(source)
        } else {
            None
        }
    }

    pub fn metadata_path(&self) -> PathBuf {
        let mut path = self.path.clone();
        path.push(METADATA_FILENAME);
        path
    }

    fn refresh_metadata(&mut self) {
        self.metadata = Metadata::for_source(&self.metadata_path());
    }

    fn gather_datasets(&mut self) {
        let path = &self.path;
        self.datasets = match fs::read_dir(path) {
            Err(err) => {
                log::error!("{:?}", err);
                Vec::new()
            }
            Ok(entries) => entries
                .filter_map(|r| r.ok())
                .filter_map(|e| entry_tuples(e, false))
                .filter_map(|(path, name)| Dataset::new(&path, &name))
                .collect(),
        };
    }

    pub fn add_dataset(&mut self, name: &str) -> std::io::Result<()> {
        let mut path = self.path.clone();
        path.push(name);
        fs::create_dir(path)?;
        self.gather_datasets();
        Ok(())
    }

    pub fn get_dataset_mut(&mut self, name: &str) -> Option<&mut Dataset> {
        self.datasets.iter_mut().find(|d| d.name == name)
    }

    pub fn get_dataset(&self, name: &str) -> Option<&Dataset> {
        self.datasets.iter().find(|d| d.name == name)
    }
}

fn entry_tuples(e: DirEntry, resource: bool) -> Option<(String, String)> {
    let path = e.path();
    let name = path.file_name()?;
    let stem = path.file_stem()?;
    let source = path.parent()?;
    let base = source.join(stem);
    let is_metadata =
        e.path().extension().map(|e| e.to_ascii_lowercase()) == Some(METADATA_EXT.into());

    if e.file_type().ok()?.is_dir() {
        if resource {
            None
        } else {
            Some((
                source.to_string_lossy().into(),
                e.file_name().to_string_lossy().into(),
            ))
        }
    } else if is_metadata && base.exists() {
        None
    } else if resource {
        Some((
            source.to_string_lossy().into(),
            name.to_string_lossy().into(),
        ))
    } else {
        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dataset {
    pub name: String,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub resources: Vec<Resource>,
}

impl Dataset {
    pub fn new(path: &str, name: &str) -> Option<Self> {
        let path = PathBuf::from(path);

        if path.is_dir() {
            let mut dataset = Dataset {
                name: name.to_owned(),
                path,
                metadata: Metadata::Empty,
                resources: Vec::new(),
            };
            dataset.refresh_metadata();
            dataset.gather_resources();
            Some(dataset)
        } else {
            None
        }
    }

    pub fn metadata_path(&self) -> PathBuf {
        self.path.join(METADATA_FILENAME)
    }

    fn refresh_metadata(&mut self) {
        self.metadata = Metadata::for_dataset(&self.metadata_path(), &self.name);
    }
    fn gather_resources(&mut self) {
        let mut path = self.path.clone();
        path.push(&self.name);

        self.resources = match fs::read_dir(path) {
            Err(err) => {
                log::error!("{:?}", err);
                Vec::new()
            }
            Ok(entries) => entries
                .filter_map(|r| r.ok())
                .filter_map(|e| entry_tuples(e, true))
                .filter_map(|(path, name)| Resource::new(&path, &name))
                .collect(),
        };
    }
    pub fn add_resource(&mut self, name: &str) -> std::io::Result<()> {
        let mut path = self.path.clone();
        path.push(&self.name);
        path.push(name);
        if path.exists() {
            Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "File already exists",
            ))
        } else {
            fs::write(path, "")?;
            self.gather_resources();
            Ok(())
        }
    }
    pub fn get_resoure(&self, name: &str) -> Option<&Resource> {
        self.resources.iter().find(|d| d.name == name)
    }
    pub fn get_resoure_mut(&mut self, name: &str) -> Option<&mut Resource> {
        self.resources.iter_mut().find(|d| d.name == name)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Resource {
    pub path: PathBuf,
    pub name: String,
    pub metadata: Metadata,
    pub size: u64,
}
impl Resource {
    pub fn new(path: &str, name: &str) -> Option<Self> {
        let path = PathBuf::from(path);

        if path.is_dir() {
            let mut resource = Resource {
                name: name.to_owned(),
                path,
                metadata: Metadata::Empty,
                size: 0,
            };
            resource.refresh_metadata();
            resource.size = resource.size();
            Some(resource)
        } else {
            None
        }
    }

    pub fn metadata_path(&self) -> PathBuf {
        let path = self.path.parent().unwrap();
        path.join(METADATA_FILENAME)
    }

    fn refresh_metadata(&mut self) {
        self.metadata = Metadata::for_resource(
            &self.metadata_path(),
            self.path.file_name().unwrap(),
            &self.name,
        );
    }

    pub fn size(&self) -> u64 {
        let mut path = self.path.clone();
        path.push(&self.name);
        fs::metadata(path).map(|m| m.len()).unwrap_or(0)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Metadata {
    Empty,
    Object(Value),
}

impl Metadata {
    pub fn write<P: AsRef<OsStr>>(
        &self,
        path: &P,
        _comments: HashMap<String, Vec<String>>,
    ) -> Result<(), std::io::Error> {
        match self {
            Metadata::Empty => fs::remove_file(path.as_ref()),
            Metadata::Object(v) => {
                let mut wtr = csv::Writer::from_path(path.as_ref())?;
                wtr.write_record(&[
                    "Dataset Field",
                    "Description",
                    "Master metadata",
                    "Dataset 1",
                    "Dataset 2",
                ])?;
                match v {
                    Value::Object(v) => {
                        for (k, v) in v {
                            wtr.write_record(&[
                                k,
                                &_comments.get(k).unwrap_or(&vec!["".to_string()]).join("\n"),
                                match v {
                                    Value::String(s) => s,
                                    _ => "",
                                },
                                "Value for Dataset 1",
                                "Value for Dataset 2",
                            ])?;
                        }
                    }
                    _ => (),
                }
                wtr.write_record(&[
                    "Resource Field",
                    "Description",
                    "",
                    "Resource 1 from Dataset 1.txt",
                    "Resource 1 from Dataset 2.json",
                ])?;
                wtr.write_record(&[
                    "name",
                    "Human-readable name of the resource",
                    "",
                    "Value for Resource 1",
                    "Value for Resource 1",
                ])?;
                wtr.write_record(&[
                    "description",
                    "Desctiption. For long descriptions use three quotation marks:\n\
                    description = \"\"\"Roses are red\n\
                    Violets are blue\"\"\"",
                    "",
                    "Value for Resource 1",
                    "Value for Resource 1",
                ])?;
                wtr.write_record(&[
                    "Resource Field",
                    "Description",
                    "",
                    "Resource 2 from Dataset 1.csv",
                    "",
                ])?;
                wtr.write_record(&[
                    "name",
                    "Human-readable name of the resource",
                    "",
                    "Value for Resource 2",
                    "",
                ])?;
                wtr.write_record(&[
                    "description",
                    "Desctiption. For long descriptions use three quotation marks:\n\
                    description = \"\"\"Roses are red\n\
                    Violets are blue\"\"\"",
                    "",
                    "Value for Resource 2",
                    "",
                ])?;

                Ok(())
            }
        }
    }
    pub fn patch(&mut self, patch: Value) {
        if let (Self::Object(Value::Object(obj)), Value::Object(ref mut patch)) = (self, patch) {
            obj.append(patch);
        }
    }

    fn for_source(path: &PathBuf) -> Self {
        Metadata::extract_key_from_csv(path, MetadataContent::Master)
    }

    fn for_dataset(path: &PathBuf, name: &str) -> Self {
        Metadata::extract_key_from_csv(path, MetadataContent::Dataset(name))
    }

    fn for_resource(path: &PathBuf, dataset: &OsStr, name: &str) -> Self {
        Metadata::extract_key_from_csv(path, MetadataContent::Resource(dataset, name))
    }

    fn extract_key_from_csv(path: &PathBuf, key: MetadataContent) -> Self {
        if path.is_file() {
            let mut reader = csv::Reader::from_path(path).unwrap();

            let header = match reader.headers() {
                Ok(header) => header.clone(),
                Err(_) => return Self::Empty,
            };
            let records = reader.records();

            let idx = match key {
                MetadataContent::Master => Some(2),
                MetadataContent::Dataset(name) => header.iter().position(|s| s == name),
                MetadataContent::Resource(name, _) => header.iter().position(|s| s == name),
            };

            match idx {
                None => Self::Empty,
                Some(idx) => {
                    let iter = records
                        .map(|record| record.unwrap())
                        .filter(|record| record.len() > idx)
                        .map(|record| (record.index(0).to_string(), record.index(idx).to_string()));

                    let pairs = match key {
                        MetadataContent::Master | MetadataContent::Dataset(..) => iter
                            .take_while(|(key, _)| *key != "Resource Field")
                            .collect::<Vec<(String, String)>>(),
                        MetadataContent::Resource(_, name) => iter
                            .skip_while(|(key, value)| *key != "Resource Field" || *value != name)
                            .skip(1)
                            .take_while(|(key, _)| *key != "Resource Field")
                            .collect::<Vec<(String, String)>>(),
                    };

                    if pairs.len() == 0 {
                        return Self::Empty;
                    }

                    let data = pairs
                        .iter()
                        .filter(|(_, v)| *v != "")
                        .map(|(k, v)| (k.to_string(), Value::from(v.to_string())))
                        .collect();

                    Self::Object(Value::Object(data))
                }
            }
        } else {
            Self::Empty
        }
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::Object(json!({
            "title": "Title",
            "name": "dataset",
            "access_level": "open",
            "dataset_status": "draft",
            "publication_date": "2022-01-01",
            "author": "Name",
            "theme": "Emergency Management",
            "language": "en",
            "license_id": "unspecified",
            "notes": "Description",
            "dataset_type": "1",
            "update_freq": "daily",
            "flood_studies": "<id>",
            "owner_org": "<id>"
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use pretty_assertions::assert_eq;
    use serde_json::json;

    struct Dir {
        path: String,
    }
    impl Dir {
        fn new(path: String) -> Self {
            if let Err(err) = fs::remove_dir_all(&path) {
                log::warn!("{:?}", err);
            }
            fs::create_dir_all(&path).unwrap();
            Dir { path }
        }
    }

    #[test]
    fn test_source_with_no_datasets() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_source_with_no_datasets".into());
        let source = Source::new(&dir.path).unwrap();
        assert_eq!(0, source.datasets.len());
    }

    #[test]
    fn test_source_with_dir_only_dataset() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_source_with_dir_only_dataset".into());
        let mut dataset_path = PathBuf::from(&dir.path);
        dataset_path.push("test");
        fs::create_dir(&dataset_path).unwrap();

        let source = Source::new(&dir.path).unwrap();
        assert_eq!(1, source.datasets.len());
        assert_eq!("test", source.datasets[0].name);
    }

    #[test]
    fn test_dataset_with_no_resources() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_dataset_with_no_resources".into());
        let mut dataset_path = PathBuf::from(&dir.path);
        dataset_path.push("test");
        fs::create_dir(&dataset_path).unwrap();

        let dataset = Dataset::new(&dir.path, "test").unwrap();
        assert_eq!(0, dataset.resources.len());
    }

    #[test]
    fn test_dataset_with_file_resource() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_dataset_with_file_resource".into());
        let mut dataset_path = PathBuf::from(&dir.path);
        dataset_path.push("test");
        fs::create_dir(&dataset_path).unwrap();

        let mut resource_path = dataset_path.clone();
        resource_path.push("test.txt");
        fs::write(resource_path, "hello").unwrap();

        let dataset = Dataset::new(&dir.path, "test").unwrap();
        assert_eq!(1, dataset.resources.len());
    }

    #[test]
    fn test_source_with_no_initial_metadata() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_source_with_no_initial_metadata".into());
        let source = Source::new(&dir.path).unwrap();
        assert_eq!(Metadata::Empty, source.metadata);
    }

    #[test]
    fn test_add_dataset_to_source() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_add_dataset_to_source".into());
        let mut source = Source::new(&dir.path).unwrap();
        assert_eq!(0, source.datasets.len());
        source.add_dataset("test").unwrap();
        assert_eq!(1, source.datasets.len());
    }
    #[test]
    fn test_add_resource_to_dataset() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_add_resource_to_dataset".into());
        let mut source = Source::new(&dir.path).unwrap();
        source.add_dataset("test").unwrap();
        let dataset = source.get_dataset_mut("test").unwrap();
        assert_eq!(0, dataset.resources.len());
        dataset.add_resource("test.txt").unwrap();
        assert_eq!(1, dataset.resources.len());
    }

    #[test]
    fn test_csv_parsing() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_csv_parsing".into());
        let mut path = PathBuf::from(&dir.path);
        path.push("metadata.csv");

        fs::write(
            &path,
            "\
Dataset Field,Description,Master metadata,Dataset 1,Dataset 2
first,,master,test-1,test-2
second,,slave,,
Resource Field,,,Resource 1 1,Resource 2 1
name,,,res-1-1,res-2-1
Resource Field,,,Resource 1 2,Resource 2 2
name,,,res-1-2,res-2-2
",
        )
        .unwrap();

        assert_eq!(
            Metadata::for_source(&path),
            Metadata::Object(json!({"first": "master", "second": "slave"}))
        );

        assert_eq!(
            Metadata::for_dataset(&path, "Dataset 1"),
            Metadata::Object(json!({"first": "test-1"}))
        );

        assert_eq!(
            Metadata::for_dataset(&path, "Dataset 2"),
            Metadata::Object(json!({"first": "test-2"}))
        );

        assert_eq!(Metadata::for_dataset(&path, "Dataset 3"), Metadata::Empty);

        assert_eq!(
            Metadata::for_resource(&path, OsStr::new("Dataset 1"), "Resource 1 1"),
            Metadata::Object(json!({"name": "res-1-1"}))
        );
        assert_eq!(
            Metadata::for_resource(&path, OsStr::new("Dataset 1"), "Resource 1 2"),
            Metadata::Object(json!({"name": "res-1-2"}))
        );
        assert_eq!(
            Metadata::for_resource(&path, OsStr::new("Dataset 2"), "Resource 2 1"),
            Metadata::Object(json!({"name": "res-2-1"}))
        );
        assert_eq!(
            Metadata::for_resource(&path, OsStr::new("Dataset 2"), "Resource 2 2"),
            Metadata::Object(json!({"name": "res-2-2"}))
        );
        assert_eq!(
            Metadata::for_resource(&path, OsStr::new("Dataset 1"), "Resource 1 3"),
            Metadata::Empty
        );
        assert_eq!(
            Metadata::for_resource(&path, OsStr::new("Dataset 3"), "Resource 1 1"),
            Metadata::Empty
        );

        let metadata = Metadata::for_source(&path);
        metadata.write(&path, crate::types::dataset_comments());

        dbg!(fs::read_to_string(&path));
        dbg!(path);
    }
}
