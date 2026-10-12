use super::*;

/// 获取所有笔记本
#[tauri::command]
pub async fn find_all_notebooks(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<Notebook>, AppError> {
    let store = require_note_store(&app_state).await?;
    store.find_all_notebooks().await
}

/// 创建笔记本
#[tauri::command]
pub async fn create_notebook(
    app_state: tauri::State<'_, Arc<AppState>>,
    notebook: Notebook,
) -> Result<Option<Notebook>, AppError> {
    notebook.validate().map_err(AppError::from)?;
    let store = require_note_store(&app_state).await?;
    store.create_notebook(&notebook).await
}

/// 根据 ID 删除笔记本
#[tauri::command]
pub async fn delete_notebook_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let store = require_note_store(&app_state).await?;
    store.delete_notebook_by_id(id).await
}

/// 更新笔记本
#[tauri::command]
pub async fn update_notebook(
    app_state: tauri::State<'_, Arc<AppState>>,
    notebook: Notebook,
) -> Result<Option<Notebook>, AppError> {
    notebook.validate().map_err(AppError::from)?;
    let store = require_note_store(&app_state).await?;
    store.update_notebook(&notebook).await
}
