#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

fn main() {
    tauri::Builder::default()
        .manage(fdp::state::PortalState::default())
        .invoke_handler(tauri::generate_handler![
            commands::login,
            commands::list_projects,
            commands::read_source_path,
            commands::save_root_metadata,
            commands::save_dataset_metadata,
            commands::save_resource_metadata,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
