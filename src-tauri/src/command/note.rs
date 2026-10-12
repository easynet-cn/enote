use super::*;

/// 创建笔记
#[tauri::command]
pub async fn create_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    note: Note,
) -> Result<Option<Note>, AppError> {
    note.validate().map_err(AppError::from)?;
    let store = require_note_store(&app_state).await?;
    store.create_note(&note).await
}

/// 更新笔记
#[tauri::command]
pub async fn update_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    note: Note,
) -> Result<Option<Note>, AppError> {
    note.validate().map_err(AppError::from)?;
    let store = require_note_store(&app_state).await?;
    store.update_note(&note).await
}

/// 根据 ID 删除笔记（软删除，移入回收站）
#[tauri::command]
pub async fn delete_note_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let store = require_note_store(&app_state).await?;
    store.delete_note_by_id(id).await
}

/// 分页搜索笔记
#[tauri::command]
pub async fn search_page_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut search_param: NoteSearchPageParam,
) -> Result<PageResult<Note>, AppError> {
    search_param.normalize();
    let store = require_note_store(&app_state).await?;
    store.search_page_notes(&search_param).await
}

#[tauri::command]
pub async fn note_stats(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut search_param: NoteSearchPageParam,
) -> Result<NoteStatsResult, AppError> {
    search_param.normalize();
    let store = require_note_store(&app_state).await?;
    store.note_stats(&search_param).await
}

/// 批量移动笔记到指定笔记本
#[tauri::command]
pub async fn batch_move_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
    note_ids: Vec<i64>,
    notebook_id: i64,
) -> Result<(), AppError> {
    let store = require_note_store(&app_state).await?;
    store.batch_move_notes(&note_ids, notebook_id).await
}

/// 批量软删除笔记
#[tauri::command]
pub async fn batch_delete_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
    note_ids: Vec<i64>,
) -> Result<(), AppError> {
    let store = require_note_store(&app_state).await?;
    store.batch_delete_notes(&note_ids).await
}

/// 分页搜索笔记历史记录
#[tauri::command]
pub async fn search_page_note_histories(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut search_param: NoteHistorySearchPageParam,
) -> Result<PageResult<NoteHistory>, AppError> {
    search_param.page_param.normalize();
    let store = require_note_store(&app_state).await?;
    store.search_page_note_histories(&search_param).await
}

/// 切换笔记收藏/星标状态
#[tauri::command]
pub async fn toggle_note_star(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<Option<Note>, AppError> {
    let store = require_note_store(&app_state).await?;
    store.toggle_note_star(id).await
}

/// 切换笔记置顶状态
#[tauri::command]
pub async fn toggle_note_pin(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<Option<Note>, AppError> {
    let store = require_note_store(&app_state).await?;
    store.toggle_note_pin(id).await
}

/// 从回收站恢复笔记
#[tauri::command]
pub async fn restore_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let store = require_note_store(&app_state).await?;
    store.restore_note(id).await
}

/// 永久删除笔记
#[tauri::command]
pub async fn permanent_delete_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let store = require_note_store(&app_state).await?;
    store.permanent_delete_note(id).await
}

/// 清空回收站
#[tauri::command]
pub async fn empty_trash(app_state: tauri::State<'_, Arc<AppState>>) -> Result<(), AppError> {
    let store = require_note_store(&app_state).await?;
    store.empty_trash().await
}

/// 获取回收站笔记列表（分页）
#[tauri::command]
pub async fn find_deleted_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
    page_param: PageParam,
) -> Result<PageResult<Note>, AppError> {
    let store = require_note_store(&app_state).await?;
    store.find_deleted_notes(&page_param).await
}

/// 批量更新笔记本排序
#[tauri::command]
pub async fn reorder_notebooks(
    app_state: tauri::State<'_, Arc<AppState>>,
    orders: Vec<(i64, i32)>,
) -> Result<(), AppError> {
    let store = require_note_store(&app_state).await?;
    store.reorder_notebooks(orders).await
}

/// 批量更新标签排序
#[tauri::command]
pub async fn reorder_tags(
    app_state: tauri::State<'_, Arc<AppState>>,
    orders: Vec<(i64, i32)>,
) -> Result<(), AppError> {
    let store = require_note_store(&app_state).await?;
    store.reorder_tags(orders).await
}
