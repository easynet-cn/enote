use super::*;

/// 获取所有标签
#[tauri::command]
pub async fn find_all_tags(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<Tag>, AppError> {
    let store = require_note_store(&app_state).await?;
    store.find_all_tags().await
}

/// 创建标签
#[tauri::command]
pub async fn create_tag(
    app_state: tauri::State<'_, Arc<AppState>>,
    tag: Tag,
) -> Result<Option<Tag>, AppError> {
    tag.validate().map_err(AppError::from)?;
    let store = require_note_store(&app_state).await?;
    store.create_tag(&tag).await
}

/// 根据 ID 删除标签
#[tauri::command]
pub async fn delete_tag_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let store = require_note_store(&app_state).await?;
    store.delete_tag_by_id(id).await
}

/// 更新标签
#[tauri::command]
pub async fn update_tag(
    app_state: tauri::State<'_, Arc<AppState>>,
    tag: Tag,
) -> Result<Option<Tag>, AppError> {
    tag.validate().map_err(AppError::from)?;
    let store = require_note_store(&app_state).await?;
    store.update_tag(&tag).await
}
