use super::*;

/// 创建笔记
#[tauri::command]
pub async fn create_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    note: Note,
) -> Result<Option<Note>, AppError> {
    note.validate().map_err(AppError::from)?;
    let db = require_db(&app_state).await?;
    let enc_key = app_state.encryption_key.read().await;
    let is_encrypted = service::app_log::should_skip_content(&note.content);
    let result = service::note::create_with_key(
        &db,
        &note,
        OperateSource::User,
        enc_key.as_deref(),
    )
    .await
    .map_err(AppError::from)?;
    if let Some(ref n) = result {
        let msg = if is_encrypted {
            format!("Created note: {} [encrypted content]", n.title)
        } else {
            format!("Created note: {}", n.title)
        };
        let _ = service::app_log::log_action(
            &db, "note", "create",
            Some(&n.id.to_string()), Some(&n.title),
            &msg, None,
        ).await;
    }
    Ok(result)
}

/// 更新笔记
#[tauri::command]
pub async fn update_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    note: Note,
) -> Result<Option<Note>, AppError> {
    note.validate().map_err(AppError::from)?;
    let db = require_db(&app_state).await?;
    let enc_key = app_state.encryption_key.read().await;
    let is_encrypted = service::app_log::should_skip_content(&note.content);
    let result = service::note::update_with_key(
        &db,
        &note,
        OperateSource::User,
        enc_key.as_deref(),
    )
    .await
    .map_err(AppError::from)?;
    if result.is_some() {
        let msg = if is_encrypted {
            format!("Updated note: {} [encrypted content]", note.title)
        } else {
            format!("Updated note: {}", note.title)
        };
        let _ = service::app_log::log_action(
            &db, "note", "update",
            Some(&note.id.to_string()), Some(&note.title),
            &msg, None,
        ).await;
    }
    Ok(result)
}

/// 根据 ID 删除笔记（软删除，移入回收站）
#[tauri::command]
pub async fn delete_note_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::note::delete_by_id(&db, id)
        .await
        .map_err(AppError::from)?;
    let _ = service::app_log::log_action(
        &db, "note", "delete",
        Some(&id.to_string()), None,
        &format!("Moved note id={} to trash", id), None,
    ).await;
    Ok(())
}

/// 分页搜索笔记
#[tauri::command]
pub async fn search_page_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut search_param: NoteSearchPageParam,
) -> Result<PageResult<Note>, AppError> {
    search_param.normalize();
    let db = require_db(&app_state).await?;
    let enc_key = app_state.encryption_key.read().await;
    service::note::search_page_with_key(&db, &search_param, enc_key.as_deref())
        .await
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn note_stats(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut search_param: NoteSearchPageParam,
) -> Result<NoteStatsResult, AppError> {
    search_param.normalize();
    let db = require_db(&app_state).await?;
    service::note::stats(&db, &search_param)
        .await
        .map_err(AppError::from)
}

/// 批量移动笔记到指定笔记本
#[tauri::command]
pub async fn batch_move_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
    note_ids: Vec<i64>,
    notebook_id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::note::batch_move(&db, &note_ids, notebook_id)
        .await
        .map_err(AppError::from)
}

/// 批量软删除笔记
#[tauri::command]
pub async fn batch_delete_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
    note_ids: Vec<i64>,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::note::batch_delete(&db, &note_ids)
        .await
        .map_err(AppError::from)
}

/// 分页搜索笔记历史记录
#[tauri::command]
pub async fn search_page_note_histories(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut search_param: NoteHistorySearchPageParam,
) -> Result<PageResult<NoteHistory>, AppError> {
    search_param.page_param.normalize();
    let db = require_db(&app_state).await?;
    service::note_history::search_page(&db, &search_param)
        .await
        .map_err(AppError::from)
}

/// 切换笔记收藏/星标状态
#[tauri::command]
pub async fn toggle_note_star(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<Option<Note>, AppError> {
    let db = require_db(&app_state).await?;
    service::note::toggle_star(&db, id)
        .await
        .map_err(AppError::from)
}

/// 切换笔记置顶状态
#[tauri::command]
pub async fn toggle_note_pin(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<Option<Note>, AppError> {
    let db = require_db(&app_state).await?;
    service::note::toggle_pin(&db, id)
        .await
        .map_err(AppError::from)
}

/// 从回收站恢复笔记
#[tauri::command]
pub async fn restore_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::note::restore_by_id(&db, id)
        .await
        .map_err(AppError::from)?;
    let _ = service::app_log::log_action(
        &db, "note", "restore",
        Some(&id.to_string()), None,
        &format!("Restored note id={} from trash", id), None,
    ).await;
    Ok(())
}

/// 永久删除笔记
#[tauri::command]
pub async fn permanent_delete_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    let enc_key = app_state.encryption_key.read().await;
    service::note::permanent_delete_by_id(&db, id, OperateSource::User, enc_key.as_deref())
        .await
        .map_err(AppError::from)?;
    let _ = service::app_log::log_action(
        &db, "note", "permanent_delete",
        Some(&id.to_string()), None,
        &format!("Permanently deleted note id={}", id), None,
    ).await;
    Ok(())
}

/// 清空回收站
#[tauri::command]
pub async fn empty_trash(app_state: tauri::State<'_, Arc<AppState>>) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    let enc_key = app_state.encryption_key.read().await;
    service::note::empty_trash(&db, enc_key.as_deref())
        .await
        .map_err(AppError::from)?;
    let _ = service::app_log::log_action(
        &db, "note", "empty_trash",
        None, None,
        "Emptied trash", None,
    ).await;
    Ok(())
}

/// 获取回收站笔记列表（分页）
#[tauri::command]
pub async fn find_deleted_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
    page_param: PageParam,
) -> Result<PageResult<Note>, AppError> {
    let db = require_db(&app_state).await?;
    let enc_key = app_state.encryption_key.read().await;
    service::note::find_deleted_with_key(&db, &page_param, enc_key.as_deref())
        .await
        .map_err(AppError::from)
}

/// 批量更新笔记本排序
#[tauri::command]
pub async fn reorder_notebooks(
    app_state: tauri::State<'_, Arc<AppState>>,
    orders: Vec<(i64, i32)>,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::notebook::reorder(&db, orders)
        .await
        .map_err(AppError::from)
}

/// 批量更新标签排序
#[tauri::command]
pub async fn reorder_tags(
    app_state: tauri::State<'_, Arc<AppState>>,
    orders: Vec<(i64, i32)>,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::tag::reorder(&db, orders)
        .await
        .map_err(AppError::from)
}
