//! 云备份服务模块
//!
//! 通过 OpenDAL 统一抽象层支持多种云存储后端（S3/OSS/COS/WebDAV/MinIO）

use std::path::Path;

use anyhow::{bail, Result};
use opendal::services;
use opendal::Operator;
use tracing::info;

use crate::model::{CloudBackupEntry, CloudStorageConfig};

/// 根据配置构建 OpenDAL Operator
fn build_operator(config: &CloudStorageConfig) -> Result<Operator> {
    match config.provider.as_str() {
        "s3" => {
            let builder = services::S3::default()
                .bucket(&config.bucket)
                .region(&config.region)
                .endpoint(&config.endpoint)
                .access_key_id(&config.access_key_id)
                .secret_access_key(&config.secret_access_key);
            Ok(Operator::new(builder)?.finish())
        }
        "oss" => {
            let builder = services::Oss::default()
                .bucket(&config.bucket)
                .endpoint(&config.endpoint)
                .access_key_id(&config.access_key_id)
                .access_key_secret(&config.secret_access_key);
            Ok(Operator::new(builder)?.finish())
        }
        "cos" => {
            let builder = services::Cos::default()
                .bucket(&config.bucket)
                .endpoint(&config.endpoint)
                .secret_id(&config.access_key_id)
                .secret_key(&config.secret_access_key);
            Ok(Operator::new(builder)?.finish())
        }
        "minio" => {
            // MinIO 兼容 S3 协议
            let builder = services::S3::default()
                .bucket(&config.bucket)
                .region(&config.region)
                .endpoint(&config.endpoint)
                .access_key_id(&config.access_key_id)
                .secret_access_key(&config.secret_access_key);
            Ok(Operator::new(builder)?.finish())
        }
        "webdav" => {
            let builder = services::Webdav::default()
                .endpoint(&config.endpoint)
                .username(&config.username)
                .password(&config.password);
            Ok(Operator::new(builder)?.finish())
        }
        other => bail!("Unsupported cloud provider: {}", other),
    }
}

/// 拼接云端路径（prefix + filename）
fn cloud_path(config: &CloudStorageConfig, filename: &str) -> String {
    let prefix = config.prefix.trim_matches('/');
    if prefix.is_empty() {
        filename.to_string()
    } else {
        format!("{}/{}", prefix, filename)
    }
}

/// 测试云存储连接
pub async fn test_connection(config: &CloudStorageConfig) -> Result<()> {
    let op = build_operator(config)?;
    let prefix = config.prefix.trim_matches('/');
    let list_path = if prefix.is_empty() {
        "/".to_string()
    } else {
        format!("{}/", prefix)
    };
    // 尝试列出目录验证连接
    let _ = op.list(&list_path).await?;
    info!("Cloud storage connection test successful: {}", config.provider);
    Ok(())
}

/// 上传本地备份文件到云端
pub async fn upload_backup(
    config: &CloudStorageConfig,
    app_data_dir: &Path,
    filename: &str,
) -> Result<()> {
    let op = build_operator(config)?;
    let local_path = app_data_dir.join("backups").join(filename);
    let data = tokio::fs::read(&local_path).await?;
    let path = cloud_path(config, filename);
    op.write(&path, data).await?;
    info!("Uploaded backup to cloud: {}", path);
    Ok(())
}

/// 列出云端备份文件
pub async fn list_cloud_backups(config: &CloudStorageConfig) -> Result<Vec<CloudBackupEntry>> {
    let op = build_operator(config)?;
    let prefix = config.prefix.trim_matches('/');
    let list_path = if prefix.is_empty() {
        "/".to_string()
    } else {
        format!("{}/", prefix)
    };

    let entries = op.list(&list_path).await?;
    let mut backups = Vec::new();

    for entry in entries {
        let name = entry.name().to_string();
        if name.starts_with("enote_backup_") && name.ends_with(".sql") {
            let meta = entry.metadata();
            backups.push(CloudBackupEntry {
                name,
                size: meta.content_length(),
                last_modified: meta
                    .last_modified()
                    .map(|t| format!("{}", t))
                    .unwrap_or_default(),
            });
        }
    }

    // 按文件名倒序（最新在前）
    backups.sort_by(|a, b| b.name.cmp(&a.name));
    Ok(backups)
}

/// 从云端下载备份文件到本地
pub async fn download_cloud_backup(
    config: &CloudStorageConfig,
    app_data_dir: &Path,
    filename: &str,
) -> Result<String> {
    let op = build_operator(config)?;
    let path = cloud_path(config, filename);
    let data = op.read(&path).await?;

    let backup_dir = app_data_dir.join("backups");
    if !backup_dir.exists() {
        tokio::fs::create_dir_all(&backup_dir).await?;
    }
    let local_path = backup_dir.join(filename);
    tokio::fs::write(&local_path, data.to_vec()).await?;

    info!("Downloaded cloud backup: {} -> {}", path, local_path.display());
    Ok(local_path.to_string_lossy().to_string())
}

/// 清理云端旧备份，保留最近 max_count 个
pub async fn cleanup_cloud_backups(
    config: &CloudStorageConfig,
    max_count: usize,
) -> Result<u32> {
    let backups = list_cloud_backups(config).await?;
    if backups.len() <= max_count {
        return Ok(0);
    }

    let op = build_operator(config)?;
    let mut deleted = 0u32;
    for entry in &backups[max_count..] {
        let path = cloud_path(config, &entry.name);
        if op.delete(&path).await.is_ok() {
            deleted += 1;
        }
    }

    if deleted > 0 {
        info!("Cleaned up {} old cloud backup files", deleted);
    }
    Ok(deleted)
}
