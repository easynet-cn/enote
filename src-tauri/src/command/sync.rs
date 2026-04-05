use super::*;

/// 获取同步预览信息
#[tauri::command]
pub async fn get_sync_preview(
    app_state: tauri::State<'_, Arc<AppState>>,
    target_profile_id: String,
    target_db_password: Option<String>,
) -> Result<SyncPreview, AppError> {
    if is_server_backend(&app_state).await {
        return Err(AppError::code("SERVER_FEATURE_NOT_SUPPORTED"));
    }
    let db = require_db(&app_state).await?;
    let profile_id = app_state.active_profile_id.read().await;
    service::sync::get_preview(
        &db,
        &app_state.app_data_dir,
        &profile_id,
        &target_profile_id,
        target_db_password.as_deref(),
    )
    .await
    .map_err(AppError::from)
}

/// 开始同步
#[tauri::command]
pub async fn start_sync(
    app_state: tauri::State<'_, Arc<AppState>>,
    app_handle: tauri::AppHandle,
    options: SyncOptions,
    target_db_password: Option<String>,
    target_encryption_key: Option<String>,
) -> Result<SyncLog, AppError> {
    if is_server_backend(&app_state).await {
        return Err(AppError::code("SERVER_FEATURE_NOT_SUPPORTED"));
    }
    let db = require_db(&app_state).await?;
    let profile_id = app_state.active_profile_id.read().await;
    let enc_key = app_state.encryption_key.read().await;

    // 获取目标端加密密钥：优先使用前端传入的，否则从 Keychain 获取
    let target_key = if let Some(key) = target_encryption_key {
        if key.is_empty() { None } else { Some(key) }
    } else {
        service::keychain::get_encryption_key(&options.target_profile_id).unwrap_or(None)
    };

    service::sync::sync_to_profile(
        &db,
        &app_state.app_data_dir,
        &profile_id,
        &options,
        target_db_password.as_deref(),
        enc_key.as_deref(),
        target_key.as_deref(),
        &app_handle,
    )
    .await
    .map_err(AppError::from)
}

/// 查询同步日志列表（分页）
#[tauri::command]
pub async fn find_sync_logs(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut page: PageParam,
) -> Result<PageResult<SyncLog>, AppError> {
    if is_server_backend(&app_state).await {
        return Err(AppError::code("SERVER_FEATURE_NOT_SUPPORTED"));
    }
    page.normalize();
    let db = require_db(&app_state).await?;
    service::sync_log::search_page(&db, &page)
        .await
        .map_err(AppError::from)
}

/// 查询同步明细（分页，支持过滤）
#[tauri::command]
pub async fn find_sync_log_details(
    app_state: tauri::State<'_, Arc<AppState>>,
    sync_log_id: i64,
    table_name: Option<String>,
    status: Option<String>,
    mut page: PageParam,
) -> Result<PageResult<SyncLogDetail>, AppError> {
    if is_server_backend(&app_state).await {
        return Err(AppError::code("SERVER_FEATURE_NOT_SUPPORTED"));
    }
    page.normalize();
    let db = require_db(&app_state).await?;
    service::sync_log::search_detail_page(
        &db,
        sync_log_id,
        table_name.as_deref(),
        status.as_deref(),
        &page,
    )
    .await
    .map_err(AppError::from)
}

/// 删除同步日志
#[tauri::command]
pub async fn delete_sync_log(
    app_state: tauri::State<'_, Arc<AppState>>,
    sync_log_id: i64,
) -> Result<(), AppError> {
    if is_server_backend(&app_state).await {
        return Err(AppError::code("SERVER_FEATURE_NOT_SUPPORTED"));
    }
    let db = require_db(&app_state).await?;
    service::sync_log::delete_by_id(&db, sync_log_id)
        .await
        .map_err(AppError::from)
}

/// 清空所有同步日志
#[tauri::command]
pub async fn clear_sync_logs(app_state: tauri::State<'_, Arc<AppState>>) -> Result<(), AppError> {
    if is_server_backend(&app_state).await {
        return Err(AppError::code("SERVER_FEATURE_NOT_SUPPORTED"));
    }
    let db = require_db(&app_state).await?;
    service::sync_log::clear_all(&db)
        .await
        .map_err(AppError::from)
}

/// 导出同步日志为 JSON
#[tauri::command]
pub async fn export_sync_log(
    app_state: tauri::State<'_, Arc<AppState>>,
    sync_log_id: i64,
    path: String,
) -> Result<(), AppError> {
    if is_server_backend(&app_state).await {
        return Err(AppError::code("SERVER_FEATURE_NOT_SUPPORTED"));
    }
    let db = require_db(&app_state).await?;

    // 获取日志
    let log = service::sync_log::find_by_id(&db, sync_log_id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::code("SYNC_LOG_NOT_FOUND"))?;

    // 获取所有明细
    let page = PageParam {
        page_index: 1,
        page_size: 10000,
    };
    let details = service::sync_log::search_detail_page(&db, sync_log_id, None, None, &page)
        .await
        .map_err(AppError::from)?;

    // 构造导出数据
    let export_data = serde_json::json!({
        "syncLog": log,
        "details": details.data,
    });

    // 写入文件
    let json_str = serde_json::to_string_pretty(&export_data)
        .map_err(|e| AppError::code_with_args("JSON_SERIALIZE_FAILED", vec![e.to_string()]))?;
    std::fs::write(&path, json_str)
        .map_err(|e| AppError::code_with_args("FILE_WRITE_FAILED", vec![e.to_string()]))?;

    Ok(())
}
