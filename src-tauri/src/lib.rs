use std::sync::Arc;

use tauri::Manager;

use crate::config::AppState;

mod command;
mod config;
mod entity;
mod model;
mod service;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let handle = app.handle();

                let configuration = config::Configuration::new(handle).unwrap();
                let database_connection = configuration.database_connection().await.unwrap();
                let app_state = Arc::new(AppState {
                    configuration,
                    database_connection,
                });

                app.manage(app_state);

                Ok(())
            })
        })
        .invoke_handler(tauri::generate_handler![
            command::find_all_notebooks,
            command::create_note,
            command::update_note,
            command::delete_note_by_id,
            command::search_page_notes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
