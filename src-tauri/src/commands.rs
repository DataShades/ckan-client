
use fdp::action::FdpClient;
use fdp::state::PortalState;
use fdp::types::{Portal, ProgressedUpload, Project, RegisteredUpload, Source, User, ValidationResult};
use serde_json::Value;

#[tauri::command]
pub fn login(state: tauri::State<PortalState>, portal: Portal) -> fdp::Result<User> {
    state.replace(portal);
    state.client()?.user_info()
}

#[tauri::command]
pub fn list_projects(state: tauri::State<PortalState>, name: &str) -> fdp::Result<Vec<Project>> {
    Ok(state.client()?.available_projects(&name))
}


#[tauri::command]
pub fn project_set(state: tauri::State<PortalState>, id: Option<&str>) -> fdp::Result<Value> {
    state.client()?.project_set(id)
}

#[tauri::command]
pub fn read_source_path(path: &str) -> fdp::Result<Source> {
    fdp::read_source_path(&path)
}

#[tauri::command]
pub fn save_root_metadata(path: &str, metadata: Value) -> fdp::Result<()> {
    fdp::save_root_metadata(path, metadata)
}

#[tauri::command]
pub fn save_dataset_metadata(path: &str, name: &str, metadata: Value) -> fdp::Result<()> {
    fdp::save_dataset_metadata(path, name, metadata)
}

#[tauri::command]
pub fn save_resource_metadata(
    path: &str,
    dataset: &str,
    name: &str,
    metadata: Value,
) -> fdp::Result<()> {
    fdp::save_resource_metadata(path, dataset, name, metadata)
}

#[tauri::command]
pub fn add_dataset(path: &str, name: &str) -> fdp::Result<()> {
    fdp::add_dataset(path, name)
}

#[tauri::command]
pub fn validate_dataset(state: tauri::State<PortalState>, path: &str, name: &str) -> fdp::Result<ValidationResult> {
    state.client()?.validate_dataset(path, name)
}

#[tauri::command]
pub fn validate_resource(
    state: tauri::State<PortalState>,
    path: &str,
    dataset: &str,
    name: &str,
) -> fdp::Result<ValidationResult> {
    state.client()?.validate_resource(path, dataset, name)
}

#[tauri::command]
pub fn add_resource(path: &str, dataset: &str, name: &str) -> fdp::Result<()> {
    fdp::add_resource(path, dataset, name)
}

#[tauri::command]
pub fn show_submission(state: tauri::State<PortalState>) -> fdp::Result<Vec<Value>> {
    state
        .client()?
        .show_submission()
        .ok_or("Cannot get details of the current submission".into())
}

#[tauri::command]
pub fn show_upload(
    state: tauri::State<PortalState>,
    dataset: &str,
    name: &str,
) -> fdp::Result<ProgressedUpload> {
    state
        .client()?
        .show_upload(dataset, name)
        .ok_or("Upload has not started yet".into())
}


#[tauri::command]
pub fn register_upload(
    state: tauri::State<'_, PortalState>,
    path: &str,
    dataset: &str,
    name: &str,
) -> fdp::Result<RegisteredUpload> {
    state
        .client()?
        .register_upload(path, dataset, name)
        .ok_or("Upload cannot be started".into())
}

#[tauri::command]
pub fn progress_upload(
    state: tauri::State<PortalState>,
    path: &str,
    dataset: &str,
    name: &str,
    part: u64,
) -> fdp::Result<ProgressedUpload> {
    state
        .client()?
        .progress_upload(path, dataset, name, part)
        .ok_or("Cannot continue uploading the file".into())
}
