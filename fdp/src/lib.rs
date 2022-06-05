use serde_json::Value;

pub mod action;
pub mod state;
pub mod types;

pub fn read_source_path(path: &str) -> Result<types::Source, String> {
    types::Source::new(path).ok_or("Directory does not exist".into())
}

pub fn save_root_metadata(path: &str, metadata: Value) -> Result<(), String> {
    let mut source = read_source_path(&path)?;
    source.metadata = types::Metadata::Object(metadata);
    if let Err(err) = source.metadata.write(&source.metadata_path()) {
        log::error!("{}", err);
        Err("Cannot save the metadata".to_string())
    } else {
        Ok(())
    }
}

pub fn save_dataset_metadata(path: &str, name: &str, metadata: Value) -> Result<(), String> {
    match read_source_path(&path)?
        .datasets
        .into_iter()
        .find(|d| d.name == name)
    {
        None => Err("Cannot locate the dataset".to_string()),
        Some(mut dataset) => {
            dataset.metadata = types::Metadata::Object(metadata);
            if let Err(err) = dataset.metadata.write(&dataset.metadata_path()) {
                log::error!("{}", err);
                Err("Cannot save the metadata".to_string())
            } else {
                Ok(())
            }
        }
    }
}

pub fn save_resource_metadata(
    path: &str,
    dataset: &str,
    name: &str,
    metadata: Value,
) -> Result<(), String> {
    match read_source_path(&path)?
        .datasets
        .into_iter()
        .find(|d| d.name == dataset)
        .and_then(|d| d.resources.into_iter().find(|r| r.name == name))
    {
        None => Err("Cannot locate the resource".to_string()),
        Some(mut resource) => {
            resource.metadata = types::Metadata::Object(metadata);
            if let Err(err) = resource.metadata.write(&resource.metadata_path()) {
                log::error!("{}", err);
                Err("Cannot save the metadata".to_string())
            } else {
                Ok(())
            }
        }
    }
}
