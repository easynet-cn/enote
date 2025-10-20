use std::sync::Arc;

use tauri::Manager;

use crate::config::AppState;

mod config;
mod entity;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
