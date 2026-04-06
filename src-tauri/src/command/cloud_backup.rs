use super::*;

use crate::model::{CloudBackupEntry, CloudStorageConfig};

/// 从 settings 和 keychain 构建 CloudStorageConfig
async fn load_cloud_config(app_state: &AppState) -> Result<CloudStorageConfig, AppError> {
    let db = require_db(app_state).await?;
    let settings = service::settings::get_all(&db).await.map_err(AppError::from)?;

    let provider = settings.get("cloudBackupProvider").cloned().unwrap_or_default();
    if provider.is_empty() {
        return Err(AppError::code("CLOUD_BACKUP_NOT_CONFIGURED"));
    }

    let profile_id = app_state.active_profile_id.read().await.clone();

    // 从 keychain 获取敏感信息
    let secret_access_key = service::keychain::get_cloud_backup_secret(&profile_id)
        .unwrap_or(None)
        .unwrap_or_default();
    let password = service::keychain::get_cloud_backup_password(&profile_id)
        .unwrap_or(None)
        .unwrap_or_default();

    Ok(CloudStorageConfig {
        provider,
        endpoint: settings.get("cloudBackupEndpoint").cloned().unwrap_or_default(),
        bucket: settings.get("cloudBackupBucket").cloned().unwrap_or_default(),
        region: settings.get("cloudBackupRegion").cloned().unwrap_or_default(),
        access_key_id: settings.get("cloudBackupAccessKeyId").cloned().unwrap_or_default(),
        secret_access_key,
        prefix: settings.get("cloudBackupPrefix").cloned().unwrap_or_default(),
        username: settings.get("cloudBackupUsername").cloned().unwrap_or_default(),
        password,
    })
}

/// 测试云存储连接
#[tauri::command]
pub async fn test_cloud_connection(
    _app_state: tauri::State<'_, Arc<AppState>>,
    config: CloudStorageConfig,
) -> Result<(), AppError> {
    service::cloud_backup::test_connection(&config)
        .await
        .map_err(AppError::from)
}

/// 保存云备份配置（敏感信息存 keychain）
#[tauri::command]
pub async fn save_cloud_backup_config(
    app_state: tauri::State<'_, Arc<AppState>>,
    config: CloudStorageConfig,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    let profile_id = app_state.active_profile_id.read().await.clone();

    // 非敏感信息存 settings
    let mut settings = HashMap::new();
    settings.insert("cloudBackupProvider".to_string(), config.provider.clone());
    settings.insert("cloudBackupEndpoint".to_string(), config.endpoint.clone());
    settings.insert("cloudBackupBucket".to_string(), config.bucket.clone());
    settings.insert("cloudBackupRegion".to_string(), config.region.clone());
    settings.insert("cloudBackupAccessKeyId".to_string(), config.access_key_id.clone());
    settings.insert("cloudBackupPrefix".to_string(), config.prefix.clone());
    settings.insert("cloudBackupUsername".to_string(), config.username.clone());

    service::settings::save(&db, settings).await.map_err(AppError::from)?;

    // 敏感信息存 keychain
    if !config.secret_access_key.is_empty() {
        service::keychain::set_cloud_backup_secret(&profile_id, &config.secret_access_key)
            .map_err(AppError::from)?;
    }
    if !config.password.is_empty() {
        service::keychain::set_cloud_backup_password(&profile_id, &config.password)
            .map_err(AppError::from)?;
    }

    info!("Cloud backup config saved for provider: {}", config.provider);
    Ok(())
}

/// 上传本地备份文件到云端
#[tauri::command]
pub async fn upload_backup_to_cloud(
    app_state: tauri::State<'_, Arc<AppState>>,
    filename: String,
) -> Result<(), AppError> {
    let config = load_cloud_config(&app_state).await?;
    service::cloud_backup::upload_backup(&config, &app_state.app_data_dir, &filename)
        .await
        .map_err(AppError::from)
}

/// 立即执行云备份（本地备份 + 上传）
#[tauri::command]
pub async fn cloud_backup_now(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<String, AppError> {
    let db = require_db(&app_state).await?;
    // 1. 本地备份
    let filename = service::backup::auto_backup(&db, &app_state.app_data_dir)
        .await
        .map_err(AppError::from)?;
    // 2. 上传云端
    let config = load_cloud_config(&app_state).await?;
    service::cloud_backup::upload_backup(&config, &app_state.app_data_dir, &filename)
        .await
        .map_err(AppError::from)?;
    Ok(filename)
}

/// 列出云端备份文件
#[tauri::command]
pub async fn list_cloud_backups(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<CloudBackupEntry>, AppError> {
    let config = load_cloud_config(&app_state).await?;
    service::cloud_backup::list_cloud_backups(&config)
        .await
        .map_err(AppError::from)
}

/// 从云端下载备份文件到本地
#[tauri::command]
pub async fn download_cloud_backup(
    app_state: tauri::State<'_, Arc<AppState>>,
    filename: String,
) -> Result<String, AppError> {
    let config = load_cloud_config(&app_state).await?;
    service::cloud_backup::download_cloud_backup(&config, &app_state.app_data_dir, &filename)
        .await
        .map_err(AppError::from)
}

/// 清理云端旧备份
#[tauri::command]
pub async fn cleanup_cloud_backups(
    app_state: tauri::State<'_, Arc<AppState>>,
    max_count: usize,
) -> Result<u32, AppError> {
    let config = load_cloud_config(&app_state).await?;
    service::cloud_backup::cleanup_cloud_backups(&config, max_count)
        .await
        .map_err(AppError::from)
}
