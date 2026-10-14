use super::*;

/// 获取所有设置（带内存缓存）
///
/// Database 后端优先读本地文件（迁移后），文件不存在时从 settings 表迁移；
/// Git 后端读本地文件；Server 后端走远程 API。
#[tauri::command]
pub async fn get_all_settings(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<HashMap<String, String>, AppError> {
    // 尝试从缓存读取
    {
        let cache = app_state.settings_cache.read().await;
        if let Some(ref cached) = *cache {
            return Ok(cached.clone());
        }
    }

    let profile_id = app_state.active_profile_id.read().await.clone();

    let settings = match &*app_state.backend.read().await {
        ProfileBackend::Server(_) => {
            let client = require_server(&app_state).await?;
            client.get_all_settings().await?
        }
        ProfileBackend::Git => {
            let dir = app_state.app_data_dir.clone();
            let pid = profile_id.clone();
            tokio::task::spawn_blocking(move || service::settings_file::get_all(&dir, &pid))
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("读取设置失败: {}", e)))?
                .map_err(AppError::from)?
        }
        ProfileBackend::Database => {
            // 优先读文件（迁移后），不存在则从 db 迁移
            let dir = app_state.app_data_dir.clone();
            let file_settings = tokio::task::spawn_blocking(move || {
                service::settings_file::get_all(&dir, &profile_id)
            })
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("{}", e)))?
            .unwrap_or_default();

            if !file_settings.is_empty() {
                file_settings
            } else {
                let db = require_db(&app_state).await?;
                let pid = app_state.active_profile_id.read().await.clone();
                service::settings_file::migrate_from_db(&app_state.app_data_dir, &pid, &db)
                    .await
                    .map_err(AppError::from)?
            }
        }
    };

    // 写入缓存
    {
        let mut cache = app_state.settings_cache.write().await;
        *cache = Some(settings.clone());
    }

    Ok(settings)
}

/// 保存设置（写穿缓存）
///
/// 所有后端都写本地文件；Database 后端同时双写 settings 表（兼容旧版本回退）。
#[tauri::command]
pub async fn save_settings(
    app_state: tauri::State<'_, Arc<AppState>>,
    settings: HashMap<String, String>,
) -> Result<(), AppError> {
    let profile_id = app_state.active_profile_id.read().await.clone();
    let keys: Vec<String> = settings.keys().cloned().collect();

    match &*app_state.backend.read().await {
        ProfileBackend::Server(_) => {
            let client = require_server(&app_state).await?;
            client.save_settings(&settings).await?;
        }
        ProfileBackend::Git | ProfileBackend::Database => {
            // 所有后端写本地文件
            let dir = app_state.app_data_dir.clone();
            let settings_clone = settings.clone();
            let pid = profile_id.clone();
            tokio::task::spawn_blocking(move || {
                service::settings_file::save(&dir, &pid, &settings_clone)
            })
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("保存设置失败: {}", e)))?
            .map_err(AppError::from)?;

            // Database 后端双写 settings 表（兼容旧版本回退）
            if matches!(&*app_state.backend.read().await, ProfileBackend::Database) {
                if let Some(db) = app_state.database_connection.read().await.clone() {
                    let _ = service::settings::save(&db, settings.clone()).await;
                }
            }
        }
    }

    // 写穿缓存
    {
        let mut cache = app_state.settings_cache.write().await;
        if let Some(ref mut cached) = *cache {
            for (k, v) in settings {
                cached.insert(k, v);
            }
        }
    }

    // 日志（仅 Database 后端有 db 时记录）
    if let Some(db) = app_state.database_connection.read().await.clone() {
        let _ = service::app_log::log_action(
            &db,
            "settings",
            "update",
            None,
            None,
            &format!("Updated settings: {}", keys.join(", ")),
            None,
        )
        .await;
    }

    Ok(())
}
