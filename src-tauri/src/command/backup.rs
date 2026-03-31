use super::*;

/// 导出数据库备份
#[tauri::command]
pub async fn export_backup(
    app_state: tauri::State<'_, Arc<AppState>>,
    format: String,
    path: String,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    match format.as_str() {
        "sql" => service::backup::export_sql(&db, &path)
            .await
            .map_err(AppError::from)?,
        "excel" => service::backup::export_excel(&db, &path)
            .await
            .map_err(AppError::from)?,
        "csv" => service::backup::export_csv(&db, &path)
            .await
            .map_err(AppError::from)?,
        _ => return Err(AppError::code("UNSUPPORTED_EXPORT_FORMAT")),
    }
    let _ = service::app_log::log_action(
        &db, "backup", "export",
        None, None,
        &format!("Exported backup: format={}, path={}", format, path), None,
    ).await;
    Ok(())
}

/// 导入数据库备份
#[tauri::command]
pub async fn import_backup(
    app_state: tauri::State<'_, Arc<AppState>>,
    format: String,
    path: String,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    match format.as_str() {
        "sql" => service::backup::import_sql(&db, &path)
            .await
            .map_err(AppError::from)?,
        "excel" => service::backup::import_excel(&db, &path)
            .await
            .map_err(AppError::from)?,
        "csv" => service::backup::import_csv(&db, &path)
            .await
            .map_err(AppError::from)?,
        _ => return Err(AppError::code("UNSUPPORTED_IMPORT_FORMAT")),
    }
    let _ = service::app_log::log_action(
        &db, "backup", "import",
        None, None,
        &format!("Imported backup: format={}, path={}", format, path), None,
    ).await;
    Ok(())
}

/// 执行一次自动备份
#[tauri::command]
pub async fn auto_backup(app_state: tauri::State<'_, Arc<AppState>>) -> Result<String, AppError> {
    let db = require_db(&app_state).await?;
    let filename = service::backup::auto_backup(&db, &app_state.app_data_dir)
        .await
        .map_err(AppError::from)?;
    Ok(filename)
}

/// 清理旧备份文件
#[tauri::command]
pub async fn cleanup_old_backups(
    app_state: tauri::State<'_, Arc<AppState>>,
    max_count: usize,
) -> Result<u32, AppError> {
    service::backup::cleanup_old_backups(&app_state.app_data_dir, max_count).map_err(AppError::from)
}

/// 列出所有自动备份
#[tauri::command]
pub async fn list_auto_backups(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<(String, u64)>, AppError> {
    service::backup::list_backups(&app_state.app_data_dir).map_err(AppError::from)
}
