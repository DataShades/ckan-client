#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
use fdp::state::PortalState;

fn main() {
    env_logger::init();
    tauri::Builder::default()
        .manage(PortalState::default())
        .invoke_handler(tauri::generate_handler![
            commands::submission_finalize,
            commands::login,
            commands::list_projects,
            commands::project_set,

            commands::read_source_path,
            commands::save_root_metadata,
            commands::save_dataset_metadata,
            commands::save_resource_metadata,
            commands::add_dataset,
            commands::validate_dataset,
            commands::validate_resource,
            commands::add_resource,

            commands::show_submission,
            commands::show_upload,
            commands::register_upload,
            commands::progress_upload,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
