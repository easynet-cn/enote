use super::*;

use crate::model::{CloudBackupEntry, CloudStorageConfig};

// ============================================================================
// 云备份配置文件存储（所有后端可用，不依赖数据库）
// ============================================================================

/// 云备份非敏感配置（存本地 YAML 文件，按 profile 隔离）
///
/// 敏感信息（secret_access_key/password）仍存 Keychain。
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
struct CloudBackupFileConfig {
    provider: String,
    endpoint: String,
    bucket: String,
    region: String,
    access_key_id: String,
    prefix: String,
    username: String,
}

/// 云备份配置目录：`{app_data_dir}/cloud_backup/`
fn cloud_config_dir(app_data_dir: &std::path::Path) -> std::path::PathBuf {
    app_data_dir.join("cloud_backup")
}

/// 单个 profile 的云备份配置文件路径
fn cloud_config_file(app_data_dir: &std::path::Path, profile_id: &str) -> std::path::PathBuf {
    cloud_config_dir(app_data_dir).join(format!("{}.yml", profile_id))
}

/// 读取本地云备份配置文件
fn read_cloud_config_file(
    app_data_dir: &std::path::Path,
    profile_id: &str,
) -> Option<CloudBackupFileConfig> {
    let path = cloud_config_file(app_data_dir, profile_id);
    let content = std::fs::read_to_string(&path).ok()?;
    yaml_serde::from_str(&content).ok()
}

/// 写入本地云备份配置文件
fn write_cloud_config_file(
    app_data_dir: &std::path::Path,
    profile_id: &str,
    config: &CloudBackupFileConfig,
) -> Result<(), AppError> {
    let dir = cloud_config_dir(app_data_dir);
    std::fs::create_dir_all(&dir)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("创建云备份配置目录失败: {}", e)))?;
    let content =
        yaml_serde::to_string(config).map_err(|e| AppError::Internal(anyhow::anyhow!("{}", e)))?;
    std::fs::write(cloud_config_file(app_data_dir, profile_id), content)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("写入云备份配置文件失败: {}", e)))?;
    Ok(())
}

/// 从数据库 settings 表迁移云备份配置到本地文件（仅 Database 后端可迁移）
///
/// 兼容旧版本：配置原本存于 settings 表，首次读取时自动迁移到文件。
async fn try_migrate_cloud_config_from_settings(
    app_state: &AppState,
    profile_id: &str,
) -> Option<CloudBackupFileConfig> {
    let db = app_state.database_connection.read().await.clone()?;
    let settings = service::settings::get_all(&db).await.ok()?;
    let provider = settings.get("cloudBackupProvider")?.clone();
    if provider.is_empty() {
        return None;
    }
    let config = CloudBackupFileConfig {
        provider,
        endpoint: settings
            .get("cloudBackupEndpoint")
            .cloned()
            .unwrap_or_default(),
        bucket: settings
            .get("cloudBackupBucket")
            .cloned()
            .unwrap_or_default(),
        region: settings
            .get("cloudBackupRegion")
            .cloned()
            .unwrap_or_default(),
        access_key_id: settings
            .get("cloudBackupAccessKeyId")
            .cloned()
            .unwrap_or_default(),
        prefix: settings
            .get("cloudBackupPrefix")
            .cloned()
            .unwrap_or_default(),
        username: settings
            .get("cloudBackupUsername")
            .cloned()
            .unwrap_or_default(),
    };
    // 迁移结果静默落盘，失败则下次重试
    let _ = write_cloud_config_file(&app_state.app_data_dir, profile_id, &config);
    Some(config)
}

/// 从 settings 和 keychain 构建 CloudStorageConfig
///
/// 非敏感配置优先读本地文件（所有后端可用）；
/// 文件不存在且数据库可用时，自动从 settings 表迁移。
async fn load_cloud_config(app_state: &AppState) -> Result<CloudStorageConfig, AppError> {
    let profile_id = app_state.active_profile_id.read().await.clone();

    let file_config = match read_cloud_config_file(&app_state.app_data_dir, &profile_id) {
        Some(c) => Some(c),
        None => try_migrate_cloud_config_from_settings(app_state, &profile_id).await,
    };

    let file_config = file_config.ok_or_else(|| AppError::code("CLOUD_BACKUP_NOT_CONFIGURED"))?;
    if file_config.provider.is_empty() {
        return Err(AppError::code("CLOUD_BACKUP_NOT_CONFIGURED"));
    }

    // 从 keychain 获取敏感信息
    let secret_access_key = service::keychain::get_cloud_backup_secret(&profile_id)
        .unwrap_or(None)
        .unwrap_or_default();
    let password = service::keychain::get_cloud_backup_password(&profile_id)
        .unwrap_or(None)
        .unwrap_or_default();

    Ok(CloudStorageConfig {
        provider: file_config.provider,
        endpoint: file_config.endpoint,
        bucket: file_config.bucket,
        region: file_config.region,
        access_key_id: file_config.access_key_id,
        secret_access_key,
        prefix: file_config.prefix,
        username: file_config.username,
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

/// 保存云备份配置（非敏感项存本地文件，敏感项存 keychain）
///
/// 数据库可用时同步双写 settings 表，保证回退旧版本仍可读取。
#[tauri::command]
pub async fn save_cloud_backup_config(
    app_state: tauri::State<'_, Arc<AppState>>,
    config: CloudStorageConfig,
) -> Result<(), AppError> {
    let profile_id = app_state.active_profile_id.read().await.clone();

    // 非敏感信息存本地文件（所有后端通用）
    let file_config = CloudBackupFileConfig {
        provider: config.provider.clone(),
        endpoint: config.endpoint.clone(),
        bucket: config.bucket.clone(),
        region: config.region.clone(),
        access_key_id: config.access_key_id.clone(),
        prefix: config.prefix.clone(),
        username: config.username.clone(),
    };
    write_cloud_config_file(&app_state.app_data_dir, &profile_id, &file_config)?;

    // 数据库可用时双写 settings 表（兼容旧版本回退）
    if let Some(db) = app_state.database_connection.read().await.clone() {
        let mut settings = HashMap::new();
        settings.insert("cloudBackupProvider".to_string(), config.provider.clone());
        settings.insert("cloudBackupEndpoint".to_string(), config.endpoint.clone());
        settings.insert("cloudBackupBucket".to_string(), config.bucket.clone());
        settings.insert("cloudBackupRegion".to_string(), config.region.clone());
        settings.insert(
            "cloudBackupAccessKeyId".to_string(),
            config.access_key_id.clone(),
        );
        settings.insert("cloudBackupPrefix".to_string(), config.prefix.clone());
        settings.insert("cloudBackupUsername".to_string(), config.username.clone());
        // 尽力而为，失败不影响文件存储结果
        let _ = service::settings::save(&db, settings).await;
    }

    // 敏感信息存 keychain
    if !config.secret_access_key.is_empty() {
        service::keychain::set_cloud_backup_secret(&profile_id, &config.secret_access_key)
            .map_err(AppError::from)?;
    }
    if !config.password.is_empty() {
        service::keychain::set_cloud_backup_password(&profile_id, &config.password)
            .map_err(AppError::from)?;
    }

    info!(
        "Cloud backup config saved for provider: {}",
        config.provider
    );
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
///
/// Database 后端生成 SQL 备份；Git 后端打包整个仓库为 zip；
/// Server 后端数据在远程服务器，本地无可备份内容。
#[tauri::command]
pub async fn cloud_backup_now(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<String, AppError> {
    // 1. 本地备份（按后端分发）
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
