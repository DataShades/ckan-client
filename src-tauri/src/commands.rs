use fdp::action::FdpClient;
use fdp::state::PortalState;

use fdp::types::{Portal, Project, Source, User, ProgressedUpload, RegisteredUpload};
use serde_json::Value;

#[tauri::command]
pub fn login(state: tauri::State<PortalState>, portal: Portal) -> Result<User, String> {
    state.replace(portal);
    let client = state.client()?;
    client.user_info().ok_or("User not found".into())
}

#[tauri::command]
pub fn list_projects(state: tauri::State<PortalState>, name: &str) -> Result<Vec<Project>, String> {
    Ok(state.client()?.available_projects(&name))
}

#[tauri::command]
pub fn read_source_path(path: &str) -> Result<Source, String> {
    fdp::read_source_path(&path)
}

#[tauri::command]
pub fn save_root_metadata(path: &str, metadata: Value) -> Result<(), String> {
    fdp::save_root_metadata(path, metadata)
}

#[tauri::command]
pub fn save_dataset_metadata(path: &str, name: &str, metadata: Value) -> Result<(), String> {
    fdp::save_dataset_metadata(path, name, metadata)
}

#[tauri::command]
pub fn save_resource_metadata(path: &str, dataset: &str, name: &str, metadata: Value) -> Result<(), String> {
    fdp::save_resource_metadata(path, dataset, name, metadata)
}

#[tauri::command]
pub fn add_dataset(path: &str, name: &str) -> Result<(), String> {
    fdp::add_dataset(path, name)
}

#[tauri::command]
pub fn add_resource(path: &str, dataset: &str, name: &str) -> Result<(), String> {
    fdp::add_resource(path, dataset, name)
}

#[tauri::command]
pub fn show_submission(state: tauri::State<PortalState>) -> Result<Vec<Value>, String> {
    state.client()?.show_submission().ok_or("Cannot get details of the current submission".into())
}

#[tauri::command]
pub fn show_upload(state: tauri::State<PortalState>, dataset: &str, name: &str) -> Result<ProgressedUpload, String> {
    state.client()?.show_upload(dataset, name).ok_or("Upload has not started yet".into())
}

#[tauri::command]
pub fn register_upload(state: tauri::State<PortalState>, path: &str, dataset: &str, name: &str) -> Result<RegisteredUpload, String> {
    state.client()?.register_upload(path, dataset, name).ok_or("Upload cannot be started".into())
}

#[tauri::command]
pub fn progress_upload(state: tauri::State<PortalState>, path: &str, dataset: &str, name: &str, part: u64) -> Result<ProgressedUpload, String> {
    state.client()?.progress_upload(path, dataset, name, part).ok_or("Cannot continue uploading the file".into())
}
