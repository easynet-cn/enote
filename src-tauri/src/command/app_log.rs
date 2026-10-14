use super::*;

/// 前端日志文件操作（Git 后端）在阻塞线程池执行
async fn file_log_op<T>(
    app_state: &AppState,
    op: impl FnOnce(&std::path::Path) -> Result<T, anyhow::Error> + Send + 'static,
) -> Result<T, AppError>
where
    T: Send + 'static,
{
    let dir = app_state.app_data_dir.clone();
    tokio::task::spawn_blocking(move || op(&dir))
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("日志任务执行失败: {}", e)))?
        .map_err(AppError::from)
}

/// 分页搜索应用日志
#[tauri::command]
pub async fn search_page_app_logs(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut param: AppLogSearchParam,
) -> Result<PageResult<AppLog>, AppError> {
    param.page_param.normalize();
    match &*app_state.backend.read().await {
        ProfileBackend::Server(_) => {
            let client = require_server(&app_state).await?;
            client.search_page_app_logs(&param).await
        }
        ProfileBackend::Git => {
            file_log_op(&app_state, move |dir| {
                service::app_log_file::search_page(dir, &param)
            })
            .await
        }
        ProfileBackend::Database => {
            let db = require_db(&app_state).await?;
            service::app_log::search_page(&db, &param)
                .await
                .map_err(AppError::from)
        }
    }
}

/// 删除单条应用日志
#[tauri::command]
pub async fn delete_app_log(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    match &*app_state.backend.read().await {
        ProfileBackend::Server(_) => {
            let client = require_server(&app_state).await?;
            client.delete_app_log(id).await
        }
        ProfileBackend::Git => {
            file_log_op(&app_state, move |dir| {
                service::app_log_file::delete_by_id(dir, id).map(|_| ())
            })
            .await
        }
        ProfileBackend::Database => {
            let db = require_db(&app_state).await?;
            service::app_log::delete_by_id(&db, id)
                .await
                .map_err(AppError::from)
        }
    }
}

/// 批量删除应用日志
#[tauri::command]
pub async fn delete_batch_app_logs(
    app_state: tauri::State<'_, Arc<AppState>>,
    ids: Vec<i64>,
) -> Result<u64, AppError> {
    match &*app_state.backend.read().await {
        ProfileBackend::Server(_) => {
            let client = require_server(&app_state).await?;
            client.delete_batch_app_logs(&ids).await
        }
        ProfileBackend::Git => {
            file_log_op(&app_state, move |dir| {
                service::app_log_file::delete_batch(dir, &ids)
            })
            .await
        }
        ProfileBackend::Database => {
            let db = require_db(&app_state).await?;
            service::app_log::delete_batch(&db, ids)
                .await
                .map_err(AppError::from)
        }
    }
}

/// 清空所有应用日志
#[tauri::command]
pub async fn clear_app_logs(app_state: tauri::State<'_, Arc<AppState>>) -> Result<u64, AppError> {
    match &*app_state.backend.read().await {
        ProfileBackend::Server(_) => {
            let client = require_server(&app_state).await?;
            client.clear_app_logs().await
        }
        ProfileBackend::Git => {
            file_log_op(&app_state, move |dir| service::app_log_file::clear_all(dir)).await
        }
        ProfileBackend::Database => {
            let db = require_db(&app_state).await?;
            service::app_log::clear_all(&db)
                .await
                .map_err(AppError::from)
        }
    }
}

/// 按时间清理应用日志
#[tauri::command]
pub async fn cleanup_app_logs_before(
    app_state: tauri::State<'_, Arc<AppState>>,
    before: String,
) -> Result<u64, AppError> {
    let dt = chrono::NaiveDateTime::parse_from_str(&before, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| AppError::code_with_args("INVALID_DATE", vec![e.to_string()]))?;
    match &*app_state.backend.read().await {
        ProfileBackend::Server(_) => {
            let client = require_server(&app_state).await?;
            client.cleanup_app_logs_before(&before).await
        }
        ProfileBackend::Git => {
            file_log_op(&app_state, move |dir| {
                service::app_log_file::delete_before(dir, dt)
            })
            .await
        }
        ProfileBackend::Database => {
            let db = require_db(&app_state).await?;
            service::app_log::delete_before(&db, dt)
                .await
                .map_err(AppError::from)
        }
    }
}

/// 从前端写入日志
#[tauri::command]
pub async fn write_frontend_log(
    app_state: tauri::State<'_, Arc<AppState>>,
    log: AppLog,
) -> Result<(), AppError> {
    match &*app_state.backend.read().await {
        ProfileBackend::Server(_) => {
            let client = require_server(&app_state).await?;
            client.write_frontend_log(&log).await
        }
        ProfileBackend::Git => {
            file_log_op(&app_state, move |dir| {
                service::app_log_file::append_frontend_log(dir, &log)
            })
            .await
        }
        ProfileBackend::Database => {
            let db = require_db(&app_state).await?;
            service::app_log::log_from_frontend(&db, &log)
                .await
                .map_err(AppError::from)
        }
    }
}

/// 列出日志文件
#[tauri::command]
pub async fn list_log_files(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<LogFileInfo>, AppError> {
    let log_dir = app_state.app_data_dir.join("logs");
    service::app_log::list_log_files(&log_dir).map_err(AppError::from)
}

/// 读取日志文件内容
#[tauri::command]
pub async fn read_log_file(
    app_state: tauri::State<'_, Arc<AppState>>,
    filename: String,
) -> Result<String, AppError> {
    let log_dir = app_state.app_data_dir.join("logs");
    service::app_log::read_log_file(&log_dir, &filename).map_err(AppError::from)
}

/// 删除日志文件
#[tauri::command]
pub async fn delete_log_files(
    app_state: tauri::State<'_, Arc<AppState>>,
    filenames: Vec<String>,
) -> Result<u32, AppError> {
    let log_dir = app_state.app_data_dir.join("logs");
    service::app_log::delete_log_files(&log_dir, filenames).map_err(AppError::from)
}

/// 清理旧日志文件
#[tauri::command]
pub async fn cleanup_old_log_files(
    app_state: tauri::State<'_, Arc<AppState>>,
    retention_days: u32,
) -> Result<u32, AppError> {
    let log_dir = app_state.app_data_dir.join("logs");
    service::app_log::cleanup_old_log_files(&log_dir, retention_days).map_err(AppError::from)
}
