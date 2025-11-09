use std::sync::Arc;

use crate::{
    config::AppState,
    model::{Note, NotePageResult, NoteSearchPageParam, Notebook, Tag},
    service,
};

#[tauri::command]
pub async fn find_all_notebooks(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<Notebook>, String> {
    let db = &app_state.database_connection;

    let notebooks = service::notebook::find_all(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(notebooks)
}

#[tauri::command]
pub async fn create_notebook(
    app_state: tauri::State<'_, Arc<AppState>>,
    notebook: Notebook,
) -> Result<Option<Notebook>, String> {
    let db = &app_state.database_connection;

    service::notebook::create(db, &notebook)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_tag(
    app_state: tauri::State<'_, Arc<AppState>>,
    tag: Tag,
) -> Result<Option<Tag>, String> {
    let db = &app_state.database_connection;

    service::tag::create(db, &tag)
        .await
        .map_err(|e| e.to_string())
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
) -> Result<NotePageResult, String> {
    let db = &app_state.database_connection;

    service::note::search_page(db, &search_param)
        .await
        .map_err(|e| e.to_string())
}
