#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod ckan;
mod commands;
mod context;

fn main() {
    tauri::Builder::default()
        .manage(ckan::ckan_state())
        .invoke_handler(tauri::generate_handler![
            commands::login,
            commands::list_projects,
            commands::read_source_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
