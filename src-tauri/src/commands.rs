
use fdp::action::FdpClient;
use fdp::state::PortalState;

use fdp::types::{User, Project, Portal, Source};

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
pub fn read_source_path(path: &str) ->  Result<Source, String>{
    fdp::read_source_path(&path)
}
