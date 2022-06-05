use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{self, DirEntry};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Source {
    pub path: PathBuf,
    pub metadata: Metadata,
    pub datasets: Vec<Dataset>,
}

impl Source {
    pub fn new(path: &str) -> Option<Self> {
        let path = PathBuf::from(path);
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
        path.push("metadata.json");
        path
    }

    fn refresh_metadata(&mut self) {
        self.metadata = Metadata::from(&self.metadata_path());
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
}

fn entry_tuples(e: DirEntry, resource: bool) -> Option<(String, String)> {
    let path = e.path();
    let name = path.file_name()?;
    let stem = path.file_stem()?;
    let source = path.parent()?;
    let base = source.join(stem);
    let is_json = e.path().extension().map(|e| e.to_ascii_lowercase()) == Some("json".into());

    if e.file_type().ok()?.is_dir() {
        if resource {
            None
        } else {
            Some((
                source.to_string_lossy().into(),
                e.file_name().to_string_lossy().into(),
            ))
        }
    } else if is_json && base.exists() {
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
        let mut path = self.path.clone();
        path.push(format!("{}.json", &self.name));
        path
    }

    fn refresh_metadata(&mut self) {
        self.metadata = Metadata::from(&self.metadata_path());
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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Resource {
    pub path: PathBuf,
    pub name: String,
    pub metadata: Metadata,
}
impl Resource {
    pub fn new(path: &str, name: &str) -> Option<Self> {
        let path = PathBuf::from(path);

        if path.is_dir() {
            let mut resource = Resource {
                name: name.to_owned(),
                path,
                metadata: Metadata::Empty,
            };
            resource.refresh_metadata();
            Some(resource)
        } else {
            None
        }
    }

    pub fn metadata_path(&self) -> PathBuf {
        let mut path = self.path.clone();
        path.push(format!("{}.json", &self.name));
        path
    }

    fn refresh_metadata(&mut self) {
        self.metadata = Metadata::from(&self.metadata_path());
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Metadata {
    Empty,
    Object(Value),
}

impl Metadata {
    pub fn write(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        match self {
            Metadata::Empty => {
                _ = fs::remove_file(path);
                Ok(())
            }
            Metadata::Object(v) => {
                let contents = serde_json::to_string_pretty(v).unwrap();
                fs::write(path, contents)
            }
        }
    }
    pub fn from(path: &PathBuf) -> Self {
        if path.is_file() {
            let content = fs::read_to_string(path.as_path()).unwrap();
            match serde_json::from_str(&content) {
                Ok(data) => Self::Object(data),
                _ => Self::Empty,
            }
        } else {
            Self::Empty
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
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
    fn test_dataset_with_file_and_meta_resource() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_dataset_with_file_and_meta_resource".into());
        let mut dataset_path = PathBuf::from(&dir.path);
        dataset_path.push("test");
        fs::create_dir(&dataset_path).unwrap();

        let mut resource_path = dataset_path.clone();
        resource_path.push("test.txt");
        fs::write(resource_path, "hello").unwrap();

        let mut metadata_path = dataset_path.clone();
        metadata_path.push("test.txt.json");
        fs::write(metadata_path, "hello").unwrap();

        let dataset = Dataset::new(&dir.path, "test").unwrap();
        assert_eq!(1, dataset.resources.len());
    }

    #[test]
    fn test_source_with_dir_and_meta_dataset() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_source_with_dir_and_meta_dataset".into());
        let mut dataset_path = PathBuf::from(&dir.path);
        dataset_path.push("test");
        fs::create_dir(&dataset_path).unwrap();

        let mut metadata_path = PathBuf::from(&dir.path);
        metadata_path.push("test.json");
        fs::write(metadata_path, "{}").unwrap();

        let source = Source::new(&dir.path).unwrap();
        assert_eq!(1, source.datasets.len());
        assert_eq!("test", source.datasets[0].name);
    }

    #[test]
    fn test_source_with_metadata_only() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_source_with_metadata_only".into());
        let mut dataset_path = PathBuf::from(&dir.path);
        dataset_path.push("test");
        fs::create_dir(&dataset_path).unwrap();

        let source = Source::new(&dir.path).unwrap();
        assert_eq!(1, source.datasets.len());
        assert_eq!("test", source.datasets[0].name);
    }

    #[test]
    fn test_source_with_no_initial_metadata() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_source_with_no_initial_metadata".into());
        let source = Source::new(&dir.path).unwrap();
        assert_eq!(Metadata::Empty, source.metadata);
    }

    #[test]
    fn test_metadata_changes() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_metadata_changes".into());
        let mut source = Source::new(&dir.path).unwrap();
        let path = source.metadata_path();

        let data = json!({"title": "Test"});
        source.metadata = Metadata::Object(data.clone());
        assert_eq!(Metadata::Empty, Metadata::from(&path));

        source.metadata.write(&path).unwrap();
        assert_eq!(Metadata::Object(data), Metadata::from(&path));
    }
    #[test]
    fn test_source_with_existing_metadata() {
        let dir = Dir::new("/tmp/fdp/rust/test/test_source_with_existing_metadata".into());
        let data = json!({"title": "Test"});
        let mut path = PathBuf::from(&dir.path);
        path.push("metadata.json");

        fs::write(&path, serde_json::to_string_pretty(&data).unwrap()).unwrap();
        let mut source = Source::new(&dir.path).unwrap();
        assert_eq!(source.metadata, Metadata::Object(data));

        fs::write(&path, "test").unwrap();
        source.refresh_metadata();
        assert_eq!(source.metadata, Metadata::Empty);

        fs::write(&path, r#""test""#).unwrap();
        source.refresh_metadata();
        assert_eq!(source.metadata, Metadata::Object(json!("test")));
    }
}
