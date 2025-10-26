use std::sync::Arc;

use crate::{
    config::AppState,
    model::{Note, NoteSearchPageParam, NotebookResult, PageResult},
    service,
};

#[tauri::command]
pub async fn find_all_notebooks(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<NotebookResult, String> {
    let db = &app_state.database_connection;

    let total_count = service::note::total_count(db)
        .await
        .map_err(|e| e.to_string())?;
    let notebooks = service::notebook::find_all(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(NotebookResult::new(total_count, notebooks))
}

#[tauri::command]
pub async fn create_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    note: Note,
) -> Result<Option<Note>, String> {
    let db = &app_state.database_connection;

    service::note::create(db, &note)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    note: Note,
) -> Result<Option<Note>, String> {
    let db = &app_state.database_connection;

    service::note::update(db, &note)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_note_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), String> {
    let db = &app_state.database_connection;

    service::note::delete_by_id(db, id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_page_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
    search_param: NoteSearchPageParam,
) -> Result<PageResult<Note>, String> {
    let db = &app_state.database_connection;

    service::note::search_page(db, &search_param)
        .await
        .map_err(|e| e.to_string())
}
