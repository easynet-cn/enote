use super::*;

/// 获取所有设置（带内存缓存）
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

    // 缓存未命中，从数据库读取
    let db = require_db(&app_state).await?;
    let settings = service::settings::get_all(&db)
        .await
        .map_err(AppError::from)?;

    // 写入缓存
    {
        let mut cache = app_state.settings_cache.write().await;
        *cache = Some(settings.clone());
    }

    Ok(settings)
}

/// 保存设置（写穿缓存）
#[tauri::command]
pub async fn save_settings(
    app_state: tauri::State<'_, Arc<AppState>>,
    settings: HashMap<String, String>,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    let keys: Vec<String> = settings.keys().cloned().collect();
    service::settings::save(&db, settings.clone())
        .await
        .map_err(AppError::from)?;

    // 写穿缓存：将保存的 key-value 合并到已有缓存中，避免下次读取时重查 DB
    {
        let mut cache = app_state.settings_cache.write().await;
        if let Some(ref mut cached) = *cache {
            for (k, v) in settings {
                cached.insert(k, v);
            }
        }
    }

    let _ = service::app_log::log_action(
        &db, "settings", "update",
        None, None,
        &format!("Updated settings: {}", keys.join(", ")), None,
    ).await;

    Ok(())
}
