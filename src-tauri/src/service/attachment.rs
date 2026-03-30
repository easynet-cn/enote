//! 笔记附件服务
//!
//! 提供附件的保存、查询、删除功能。
//! 附件文件存储在 `app_data_dir/attachments/` 目录中。

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Local;
use sea_orm::*;
use tracing::info;

use crate::entity;
use crate::model::NoteAttachment;

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

/// 保存附件文件并创建数据库记录
///
/// # 参数
/// - `db`: 数据库连接
/// - `app_data_dir`: 应用数据目录
/// - `note_id`: 所属笔记 ID
/// - `file_name`: 原始文件名
/// - `file_data`: 文件二进制数据
/// - `mime_type`: MIME 类型
///
/// # 返回
/// 创建的附件记录
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

    let dir = ensure_attachments_dir(app_data_dir)?;

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
    let stored_name = if ext.is_empty() {
        format!("{}_{}", timestamp, stem)
    } else {
        format!("{}_{}.{}", timestamp, stem, ext)
    };

    let file_path = dir.join(&stored_name);
    std::fs::write(&file_path, file_data).context("Failed to write attachment file")?;

    let now = Local::now().naive_local();
    let model = entity::note_attachment::ActiveModel {
        id: ActiveValue::NotSet,
        note_id: ActiveValue::Set(note_id),
        file_name: ActiveValue::Set(file_name.to_string()),
        file_path: ActiveValue::Set(stored_name.clone()),
        file_size: ActiveValue::Set(file_data.len() as i64),
        mime_type: ActiveValue::Set(mime_type.to_string()),
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
pub async fn delete_by_id(
    db: &DatabaseConnection,
    app_data_dir: &Path,
    id: i64,
) -> Result<()> {
    if let Some(model) = entity::note_attachment::Entity::find_by_id(id)
        .one(db)
        .await?
    {
        let file_path = attachments_dir(app_data_dir).join(&model.file_path);
        if file_path.exists() {
            std::fs::remove_file(&file_path).context("Failed to delete attachment file")?;
        }
        entity::note_attachment::Entity::delete_by_id(id)
            .exec(db)
            .await?;
        info!("Attachment deleted: id={}, file={}", id, model.file_path);
    }
    Ok(())
}
