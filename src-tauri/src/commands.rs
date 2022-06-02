use ckanapi::CKAN;
use serde::Deserialize;
use crate::ckan;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Portal {
    url: Option<String>,
    token: Option<String>,
}


#[tauri::command]
pub fn login(state: tauri::State<'_, ckan::CKANState>, portal: Portal) -> Result<ckan::User, String> {
    match portal {
        Portal {token: Some(token), url: Some(url)} => {
            let ckan = CKAN::new(&url).with_token(&token);
            state.replace(ckan);
            ckan::user_info(&state.0.lock().unwrap()).ok_or("User not found".into())
        },
        _ => Err("URL and token must be defined".into()),

    }
}

#[tauri::command]
pub fn list_projects(state: tauri::State<ckan::CKANState>, name: String) -> Vec<ckan::Project> {
    ckan::available_projects(&state.0.lock().unwrap(), name)
}


#[tauri::command]
pub fn read_source_path(path: String) ->  Result<Vec<i32>, String>{
    match fs::read_dir(path) {
        Err(err) => Err(err.to_string()),
        _ => Ok(vec![1,2,3]),
    }

}
