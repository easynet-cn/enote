//! 笔记附件服务
//!
//! 提供附件的保存、查询、删除功能。
//! 附件文件存储在 `app_data_dir/attachments/` 目录中。
//! 支持 SHA256 去重：相同内容的文件只存储一份物理副本。

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Local;
use sea_orm::*;
use sha2::{Digest, Sha256};
use tracing::info;

use crate::entity;
use crate::model::{AttachmentStats, NoteAttachment};

/// 最大附件大小：50 MB
const MAX_FILE_SIZE: usize = 50 * 1024 * 1024;

/// 获取附件存储目录
fn attachments_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("attachments")
}

/// 确保附件存储目录存在
fn ensure_attachments_dir(app_data_dir: &Path) -> Result<PathBuf> {
    let dir = attachments_dir(app_data_dir);
    if !dir.exists() {
        std::fs::create_dir_all(&dir).context("Failed to create attachments directory")?;
    }
    Ok(dir)
}

/// 计算文件的 SHA256 哈希值
fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// 保存附件文件并创建数据库记录
///
/// 支持 SHA256 去重：如果已有相同哈希的文件，复用物理文件并增加引用计数。
pub async fn save_attachment(
    db: &DatabaseConnection,
    app_data_dir: &Path,
    note_id: i64,
    file_name: &str,
    file_data: &[u8],
    mime_type: &str,
) -> Result<NoteAttachment> {
    // 校验文件大小
    if file_data.len() > MAX_FILE_SIZE {
        anyhow::bail!("File size exceeds maximum limit of 50 MB");
    }

    // 安全校验：防止通过文件名进行路径穿越
    if file_name.contains("..") || file_name.contains('/') || file_name.contains('\\') {
        anyhow::bail!("Invalid file name");
    }

    let dir = ensure_attachments_dir(app_data_dir)?;
    let file_hash = compute_sha256(file_data);

    // 检查是否已存在相同哈希的文件（去重）
    let existing = entity::note_attachment::Entity::find()
        .filter(entity::note_attachment::Column::FileHash.eq(&file_hash))
        .one(db)
        .await?;

    let (stored_name, should_write) = if let Some(ref existing_model) = existing {
        // 复用已有物理文件，增加引用计数
        let mut am: entity::note_attachment::ActiveModel = existing_model.clone().into_active_model();
        am.ref_count = ActiveValue::Set(existing_model.ref_count + 1);
        am.update(db).await?;

        info!(
            "Attachment dedup: reusing file {} (hash={})",
            existing_model.file_path, file_hash
        );
        (existing_model.file_path.clone(), false)
    } else {
        // 生成唯一文件名以避免冲突
        let timestamp = Local::now().format("%Y%m%d_%H%M%S_%3f");
        let ext = Path::new(file_name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let stem = Path::new(file_name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("file");
        let name = if ext.is_empty() {
            format!("{}_{}", timestamp, stem)
        } else {
            format!("{}_{}.{}", timestamp, stem, ext)
        };
        (name, true)
    };

    // 仅在没有重复文件时写入磁盘
    if should_write {
        let file_path = dir.join(&stored_name);
        std::fs::write(&file_path, file_data).context("Failed to write attachment file")?;
    }

    let now = Local::now().naive_local();
    let model = entity::note_attachment::ActiveModel {
        id: ActiveValue::NotSet,
        note_id: ActiveValue::Set(note_id),
        file_name: ActiveValue::Set(file_name.to_string()),
        file_path: ActiveValue::Set(stored_name.clone()),
        file_size: ActiveValue::Set(file_data.len() as i64),
        mime_type: ActiveValue::Set(mime_type.to_string()),
        file_hash: ActiveValue::Set(file_hash),
        ref_count: ActiveValue::Set(1),
        create_time: ActiveValue::Set(now),
    };

    let entity = model.insert(db).await?;
    info!(
        "Attachment saved: note_id={}, file={}",
        note_id, stored_name
    );
    Ok(NoteAttachment::from(entity))
}

/// 查询笔记的所有附件
pub async fn find_by_note_id(
    db: &DatabaseConnection,
    note_id: i64,
) -> Result<Vec<NoteAttachment>> {
    let models = entity::note_attachment::Entity::find()
        .filter(entity::note_attachment::Column::NoteId.eq(note_id))
        .order_by_desc(entity::note_attachment::Column::CreateTime)
        .all(db)
        .await?;
    Ok(models.into_iter().map(NoteAttachment::from).collect())
}

/// 删除附件（文件 + 数据库记录）
///
/// 如果同一物理文件被多个附件引用，仅减少引用计数；
/// 当引用计数降为 0 时才删除物理文件。
pub async fn delete_by_id(
    db: &DatabaseConnection,
    app_data_dir: &Path,
    id: i64,
) -> Result<()> {
    if let Some(model) = entity::note_attachment::Entity::find_by_id(id)
        .one(db)
        .await?
    {
        // 安全校验：防止路径穿越
        if model.file_path.contains("..") || model.file_path.contains('/') || model.file_path.contains('\\') {
            anyhow::bail!("Invalid attachment file path detected");
        }

        // 检查是否有其他附件引用同一物理文件
        let same_file_count = entity::note_attachment::Entity::find()
            .filter(entity::note_attachment::Column::FilePath.eq(&model.file_path))
            .count(db)
            .await?;

        // 删除数据库记录
        entity::note_attachment::Entity::delete_by_id(id)
            .exec(db)
            .await?;

        // 仅当没有其他引用时删除物理文件
        if same_file_count <= 1 {
            let file_path = attachments_dir(app_data_dir).join(&model.file_path);
            if file_path.exists() {
                std::fs::remove_file(&file_path).context("Failed to delete attachment file")?;
            }
        } else {
            // 更新剩余记录的引用计数
            let remaining = entity::note_attachment::Entity::find()
                .filter(entity::note_attachment::Column::FilePath.eq(&model.file_path))
                .all(db)
                .await?;
            let new_count = remaining.len() as i32;
            for r in remaining {
                let mut am: entity::note_attachment::ActiveModel = r.into_active_model();
                am.ref_count = ActiveValue::Set(new_count);
                am.update(db).await?;
            }
        }

        info!("Attachment deleted: id={}, file={}", id, model.file_path);
    }
    Ok(())
}

/// 获取附件统计信息
pub async fn get_stats(
    db: &DatabaseConnection,
    app_data_dir: &Path,
) -> Result<AttachmentStats> {
    let total_count = entity::note_attachment::Entity::find()
        .count(db)
        .await?;

    // 按 file_path 去重计数 → 物理文件数
    let all_attachments = entity::note_attachment::Entity::find()
        .all(db)
        .await?;

    let unique_paths: std::collections::HashSet<&str> = all_attachments
        .iter()
        .map(|a| a.file_path.as_str())
        .collect();
    let unique_file_count = unique_paths.len() as u64;

    let total_size: i64 = all_attachments.iter().map(|a| a.file_size).sum();

    // 计算孤立文件数
    let dir = attachments_dir(app_data_dir);
    let mut orphan_count = 0u64;
    if dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if !unique_paths.contains(name.as_str()) {
                    orphan_count += 1;
                }
            }
        }
    }

    Ok(AttachmentStats {
        total_count,
        unique_file_count,
        total_size,
        orphan_count,
    })
}

/// 清理孤立附件文件（磁盘上存在但数据库中无记录的文件）
pub async fn cleanup_orphans(
    db: &DatabaseConnection,
    app_data_dir: &Path,
) -> Result<u32> {
    let dir = attachments_dir(app_data_dir);
    if !dir.exists() {
        return Ok(0);
    }

    // 获取数据库中所有已知的文件路径
    let all_attachments = entity::note_attachment::Entity::find()
        .all(db)
        .await?;
    let known_paths: std::collections::HashSet<String> = all_attachments
        .iter()
        .map(|a| a.file_path.clone())
        .collect();

    let mut deleted = 0u32;
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !known_paths.contains(&name) {
                if std::fs::remove_file(entry.path()).is_ok() {
                    deleted += 1;
                    info!("Orphan attachment file deleted: {}", name);
                }
            }
        }
    }

    if deleted > 0 {
        info!("Cleaned up {} orphan attachment files", deleted);
    }
    Ok(deleted)
}
