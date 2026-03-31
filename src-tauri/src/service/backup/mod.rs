//! 数据库备份与恢复服务
//!
//! 支持 SQL、Excel、CSV 三种格式的导出和导入

mod csv_format;
mod excel;
mod sql;

pub use csv_format::*;
pub use excel::*;
pub use sql::*;

use std::path::Path;

use chrono::NaiveDateTime;
use sea_orm::*;
use tracing::info;

use crate::entity::{note, note_history, note_tags, notebook, tag};

/// 分页批次大小
pub(super) const BATCH_SIZE: u64 = 500;

pub(super) const DT_FMT: &str = "%Y-%m-%d %H:%M:%S";

// ============================================================================
// 数据结构
// ============================================================================

pub(super) struct BackupData {
    pub notebooks: Vec<notebook::Model>,
    pub tags: Vec<tag::Model>,
    pub notes: Vec<note::Model>,
    pub note_tags: Vec<note_tags::Model>,
    pub note_histories: Vec<note_history::Model>,
}

// ============================================================================
// 通用工具函数
// ============================================================================

pub(super) fn format_dt(dt: &NaiveDateTime) -> String {
    dt.format(DT_FMT).to_string()
}

pub(super) fn parse_dt(s: &str) -> anyhow::Result<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s.trim(), DT_FMT)
        .map_err(|e| anyhow::anyhow!("Failed to parse date '{}': {}", s, e))
}

pub(super) fn escape_sql(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

pub(super) async fn clear_tables(txn: &impl ConnectionTrait) -> anyhow::Result<()> {
    note_tags::Entity::delete_many().exec(txn).await?;
    note_history::Entity::delete_many().exec(txn).await?;
    note::Entity::delete_many().exec(txn).await?;
    tag::Entity::delete_many().exec(txn).await?;
    notebook::Entity::delete_many().exec(txn).await?;
    Ok(())
}

pub(super) async fn restore_data(
    txn: &impl ConnectionTrait,
    data: &BackupData,
) -> anyhow::Result<()> {
    use sea_orm::ActiveValue::Set;

    // 批量插入 notebooks
    if !data.notebooks.is_empty() {
        let models: Vec<notebook::ActiveModel> = data
            .notebooks
            .iter()
            .map(|m| notebook::ActiveModel {
                id: Set(m.id),
                parent_id: Set(m.parent_id),
                name: Set(m.name.clone()),
                description: Set(m.description.clone()),
                icon: Set(m.icon.clone()),
                cls: Set(m.cls.clone()),
                sort_order: Set(m.sort_order),
                mcp_access: Set(m.mcp_access),
                create_time: Set(m.create_time),
                update_time: Set(m.update_time),
            })
            .collect();
        notebook::Entity::insert_many(models).exec(txn).await?;
    }

    // 批量插入 tags
    if !data.tags.is_empty() {
        let models: Vec<tag::ActiveModel> = data
            .tags
            .iter()
            .map(|m| tag::ActiveModel {
                id: Set(m.id),
                name: Set(m.name.clone()),
                icon: Set(m.icon.clone()),
                cls: Set(m.cls.clone()),
                sort_order: Set(m.sort_order),
                mcp_access: Set(m.mcp_access),
                create_time: Set(m.create_time),
                update_time: Set(m.update_time),
            })
            .collect();
        tag::Entity::insert_many(models).exec(txn).await?;
    }

    // 批量插入 notes（分批 500 条，避免大数据集内存压力）
    for chunk in data.notes.chunks(500) {
        let models: Vec<note::ActiveModel> = chunk
            .iter()
            .map(|m| note::ActiveModel {
                id: Set(m.id),
                notebook_id: Set(m.notebook_id),
                title: Set(m.title.clone()),
                content: Set(m.content.clone()),
                content_type: Set(m.content_type),
                is_pinned: Set(m.is_pinned),
                is_starred: Set(m.is_starred),
                mcp_access: Set(m.mcp_access),
                create_time: Set(m.create_time),
                update_time: Set(m.update_time),
                deleted_at: Set(m.deleted_at),
            })
            .collect();
        note::Entity::insert_many(models).exec(txn).await?;
    }

    // 批量插入 note_tags
    if !data.note_tags.is_empty() {
        for chunk in data.note_tags.chunks(500) {
            let models: Vec<note_tags::ActiveModel> = chunk
                .iter()
                .map(|m| note_tags::ActiveModel {
                    id: Set(m.id),
                    note_id: Set(m.note_id),
                    tag_id: Set(m.tag_id),
                    sort_order: Set(m.sort_order),
                    create_time: Set(m.create_time),
                    update_time: Set(m.update_time),
                })
                .collect();
            note_tags::Entity::insert_many(models).exec(txn).await?;
        }
    }

    // 批量插入 note_histories
    for chunk in data.note_histories.chunks(500) {
        let models: Vec<note_history::ActiveModel> = chunk
            .iter()
            .map(|m| note_history::ActiveModel {
                id: Set(m.id),
                note_id: Set(m.note_id),
                old_content: Set(m.old_content.clone()),
                new_content: Set(m.new_content.clone()),
                extra: Set(m.extra.clone()),
                operate_type: Set(m.operate_type),
                operate_source: Set(m.operate_source),
                operate_time: Set(m.operate_time),
                create_time: Set(m.create_time),
            })
            .collect();
        note_history::Entity::insert_many(models).exec(txn).await?;
    }

    Ok(())
}

// ============================================================================
// 自动备份
// ============================================================================

/// 执行自动备份，将 SQL 备份保存到 `{app_data_dir}/backups/` 目录
pub async fn auto_backup(db: &DatabaseConnection, app_data_dir: &Path) -> anyhow::Result<String> {
    let backup_dir = app_data_dir.join("backups");
    if !backup_dir.exists() {
        std::fs::create_dir_all(&backup_dir)?;
    }

    let filename = format!(
        "enote_backup_{}.sql",
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    );
    let path = backup_dir.join(&filename);
    let path_str = path.to_string_lossy().to_string();

    export_sql(db, &path_str).await?;
    info!("Auto backup completed: {}", path_str);

    Ok(filename)
}

/// 清理旧备份，保留最近 max_count 个文件
pub fn cleanup_old_backups(app_data_dir: &Path, max_count: usize) -> anyhow::Result<u32> {
    let backup_dir = app_data_dir.join("backups");
    if !backup_dir.exists() {
        return Ok(0);
    }

    let mut files: Vec<_> = std::fs::read_dir(&backup_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("enote_backup_") && n.ends_with(".sql"))
                .unwrap_or(false)
        })
        .collect();

    // 按修改时间排序（最新在前）
    files.sort_by(|a, b| {
        let ta = a.metadata().and_then(|m| m.modified()).ok();
        let tb = b.metadata().and_then(|m| m.modified()).ok();
        tb.cmp(&ta)
    });

    let mut deleted = 0u32;
    if files.len() > max_count {
        for entry in &files[max_count..] {
            if std::fs::remove_file(entry.path()).is_ok() {
                deleted += 1;
            }
        }
    }

    if deleted > 0 {
        info!("Cleaned up {} old backup files", deleted);
    }
    Ok(deleted)
}

/// 列出所有自动备份文件
pub fn list_backups(app_data_dir: &Path) -> anyhow::Result<Vec<(String, u64)>> {
    let backup_dir = app_data_dir.join("backups");
    if !backup_dir.exists() {
        return Ok(Vec::new());
    }

    let mut files: Vec<(String, u64)> = std::fs::read_dir(&backup_dir)?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with("enote_backup_") && name.ends_with(".sql") {
                let size = e.metadata().ok().map(|m| m.len()).unwrap_or(0);
                Some((name, size))
            } else {
                None
            }
        })
        .collect();

    // 按文件名倒序（最新在前）
    files.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(files)
}
