use super::*;

/// 获取当前 Git 后端的仓库路径（从 profile 配置文件读取）
async fn current_git_repo_path(app_state: &AppState) -> Result<std::path::PathBuf, AppError> {
    let profile_id = app_state.active_profile_id.read().await.clone();
    let config = service::profile::read_profile(&app_state.app_data_dir, &profile_id)
        .map_err(AppError::from)?;
    config
        .git
        .map(|g| std::path::PathBuf::from(g.repo_path))
        .filter(|p| !p.as_os_str().is_empty())
        .ok_or_else(|| AppError::code("GIT_CONFIG_MISSING"))
}

/// 将当前 Git 仓库打包为 zip 备份，保存到 `{app_data_dir}/backups/`
pub(crate) async fn backup_git_repo_to_backups_dir(
    app_state: &AppState,
) -> Result<String, AppError> {
    let repo_path = current_git_repo_path(app_state).await?;
    let ts = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("enote_backup_{}.git.zip", ts);
    let backup_dir = app_state.app_data_dir.join("backups");
    tokio::fs::create_dir_all(&backup_dir)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("创建备份目录失败: {}", e)))?;
    let zip_path = backup_dir.join(&filename).to_string_lossy().to_string();
    tokio::task::spawn_blocking(move || {
        service::backup::export_git_repo_zip(&repo_path, &zip_path)
    })
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("备份任务执行失败: {}", e)))?
    .map_err(AppError::from)?;
    info!("Git repo backup completed: {}", filename);
    Ok(filename)
}

/// 检查导出/导入类功能的后端支持情况
///
/// 三格式（SQL/Excel/CSV）依赖数据库实体，仅 Database 后端可用。
async fn require_database_backend(app_state: &AppState) -> Result<(), AppError> {
    match &*app_state.backend.read().await {
        ProfileBackend::Database => Ok(()),
        ProfileBackend::Server(_) => Err(AppError::code("SERVER_FEATURE_NOT_SUPPORTED")),
        ProfileBackend::Git => Err(AppError::code("GIT_FEATURE_NOT_SUPPORTED")),
    }
}

/// 导出数据库备份
#[tauri::command]
pub async fn export_backup(
    app_state: tauri::State<'_, Arc<AppState>>,
    format: String,
    path: String,
) -> Result<(), AppError> {
    require_database_backend(&app_state).await?;
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
        &db,
        "backup",
        "export",
        None,
        None,
        &format!("Exported backup: format={}, path={}", format, path),
        None,
    )
    .await;
    Ok(())
}

/// 导入数据库备份
#[tauri::command]
pub async fn import_backup(
    app_state: tauri::State<'_, Arc<AppState>>,
    format: String,
    path: String,
) -> Result<(), AppError> {
    require_database_backend(&app_state).await?;
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
        &db,
        "backup",
        "import",
        None,
        None,
        &format!("Imported backup: format={}, path={}", format, path),
        None,
    )
    .await;
    Ok(())
}

/// 执行一次自动备份
///
/// Database 后端生成 SQL 备份；Git 后端打包仓库为 zip；
/// Server 后端数据在远程服务器，本地无可备份内容。
#[tauri::command]
pub async fn auto_backup(app_state: tauri::State<'_, Arc<AppState>>) -> Result<String, AppError> {
    let filename = match &*app_state.backend.read().await {
        ProfileBackend::Database => {
            let db = require_db(&app_state).await?;
            service::backup::auto_backup(&db, &app_state.app_data_dir)
                .await
                .map_err(AppError::from)?
        }
        ProfileBackend::Git => backup_git_repo_to_backups_dir(&app_state).await?,
        ProfileBackend::Server(_) => {
            return Err(AppError::code("SERVER_FEATURE_NOT_SUPPORTED"));
        }
    };
    Ok(filename)
}

/// 恢复 Git 仓库备份（将 backups 目录下的 zip 解压到目标仓库路径）
///
/// 目标目录必须为空或不存在，避免覆盖现有数据。
/// 建议恢复到新路径后通过 Profile 切换使用，不要直接覆盖正在使用的仓库。
#[tauri::command]
pub async fn restore_git_backup(
    app_state: tauri::State<'_, Arc<AppState>>,
    filename: String,
    target_dir: String,
) -> Result<(), AppError> {
    // 防路径逃逸：filename 只允许是 backups 目录下的文件名
    let file_name = std::path::Path::new(&filename)
        .file_name()
        .ok_or_else(|| AppError::code("BACKUP_FILE_NOT_FOUND"))?;
    let zip_path = app_state.app_data_dir.join("backups").join(file_name);
    if !zip_path.is_file() {
        return Err(AppError::code("BACKUP_FILE_NOT_FOUND"));
    }

    let target = std::path::PathBuf::from(&target_dir);
    tokio::task::spawn_blocking(move || {
        service::backup::restore_git_repo_zip(&zip_path.to_string_lossy(), &target)
    })
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("恢复任务执行失败: {}", e)))?
    .map_err(AppError::from)?;
    info!("Git repo backup restored to: {}", target_dir);
    Ok(())
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
