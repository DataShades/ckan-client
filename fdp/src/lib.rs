pub mod action;
pub mod state;
pub mod types;

use std::ffi::OsStr;

use ckanapi::CKANError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Result<T> = core::result::Result<T, FdpError>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FdpError {
    Auth(String),
    Request(String),
    Plain(String),
}

impl From<&str> for FdpError {
    fn from(source: &str) -> Self {
        Self::Plain(source.into())
    }
}
impl From<CKANError> for FdpError {
    fn from(source: CKANError) -> Self {
        match source {
            CKANError::Authorization(msg) => Self::Auth(msg),
            CKANError::Request(msg) => Self::Request(msg),
            _ => Self::Plain(format!("Unexpected error: {:?}", source)),
        }
    }
}

pub fn read_source_path<T: AsRef<OsStr>>(path: T) -> Result<types::Source> {
    types::Source::new(path).ok_or("Directory does not exist".into())
}

pub fn save_root_metadata<T: AsRef<OsStr>>(path: T, metadata: Value) -> Result<()> {
    let mut source = read_source_path(&path)?;
    source.metadata = types::Metadata::Object(metadata);
    if let Err(err) = source.metadata.write(&source.metadata_path()) {
        log::error!("{}", err);
        Err("Cannot save the metadata".into())
    } else {
        Ok(())
    }
}

pub fn add_dataset<T: AsRef<OsStr>>(path: T, name: &str) -> Result<()> {
    let mut source = read_source_path(&path)?;
    match source.add_dataset(name) {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!("{}", err);
            Err("Dataset cannot be created".into())
        }
    }
}

pub fn add_resource<T: AsRef<OsStr>>(path: T, dataset: &str, name: &str) -> Result<()> {
    let mut source = read_source_path(&path)?;
    match source.get_dataset_mut(dataset) {
        None => Ok(()),
        Some(dataset) => match dataset.add_resource(name) {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("{}", err);
                Err("Resource cannot be created".into())
            }
        },
    }
}

pub fn save_dataset_metadata<T: AsRef<OsStr>>(path: T, name: &str, metadata: Value) -> Result<()> {
    match read_source_path(&path)?.get_dataset_mut(name) {
        None => Err("Cannot locate the dataset".into()),
        Some(dataset) => {
            dataset.metadata = types::Metadata::Object(metadata);
            if let Err(err) = dataset.metadata.write(&dataset.metadata_path()) {
                log::error!("{}", err);
                Err("Cannot save the metadata".into())
            } else {
                Ok(())
            }
        }
    }
}

pub fn save_resource_metadata<T: AsRef<OsStr>>(
    path: T,
    dataset: &str,
    name: &str,
    metadata: Value,
) -> Result<()> {
    match read_source_path(&path)?
        .datasets
        .into_iter()
        .find(|d| d.name == dataset)
        .and_then(|d| d.resources.into_iter().find(|r| r.name == name))
    {
        None => Err("Cannot locate the resource".into()),
        Some(mut resource) => {
            resource.metadata = types::Metadata::Object(metadata);
            if let Err(err) = resource.metadata.write(&resource.metadata_path()) {
                log::error!("{}", err);
                Err("Cannot save the metadata".into())
            } else {
                Ok(())
            }
        }
    }
}
