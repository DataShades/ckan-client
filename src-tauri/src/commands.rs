use fdp::action::FdpClient;
use fdp::state::PortalState;

use fdp::types::{Portal, Project, Source, User};
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
