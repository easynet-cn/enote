//! 数据库迁移模块
//!
//! 本模块包含所有数据库表的迁移定义。
//! 使用 sea-orm-migration 在应用启动时自动创建和更新数据库表结构。
//!
//! # 迁移列表
//! - m20251221_000001: 创建 notebook 表
//! - m20251221_000002: 创建 note 表
//! - m20251221_000003: 创建 tag 表
//! - m20251221_000004: 创建 note_tags 表
//! - m20251221_000005: 创建 note_history 表
//! - m20251230_000001: 创建 note_fts 全文搜索表（仅 SQLite）

pub use sea_orm_migration::prelude::*;

mod m20251221_000001_create_notebook_table;
mod m20251221_000002_create_note_table;
mod m20251221_000003_create_tag_table;
mod m20251221_000004_create_note_tags_table;
mod m20251221_000005_create_note_history_table;
mod m20251230_000001_create_note_fts_table;
mod m20260306_000001_add_note_history_index;
mod m20260310_000001_create_settings_table;
mod m20260310_000002_add_note_is_pinned;
mod m20260310_000003_add_note_deleted_at;
mod m20260311_000001_upgrade_note_fts;
mod m20260311_000002_create_note_template_table;
mod m20260311_000003_create_note_link_table;
mod m20260315_000001_add_note_history_source;
mod m20260315_000002_add_note_template_content_type;
mod m20260315_000003_add_mcp_access;
mod m20260317_000001_upgrade_note_fts_tags;
mod m20260318_000001_add_performance_indexes;
mod m20260320_000001_create_sync_log_tables;
mod m20260320_000002_add_query_indexes;
mod m20260327_000001_create_app_log_table;
mod m20260328_000001_add_search_composite_indexes;
mod m20260329_000001_fix_app_log_id_type;

/// 数据库迁移器
///
/// 包含所有迁移的集合，按顺序执行
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251221_000001_create_notebook_table::Migration),
            Box::new(m20251221_000002_create_note_table::Migration),
            Box::new(m20251221_000003_create_tag_table::Migration),
            Box::new(m20251221_000004_create_note_tags_table::Migration),
            Box::new(m20251221_000005_create_note_history_table::Migration),
            Box::new(m20251230_000001_create_note_fts_table::Migration),
            Box::new(m20260306_000001_add_note_history_index::Migration),
            Box::new(m20260310_000001_create_settings_table::Migration),
            Box::new(m20260310_000002_add_note_is_pinned::Migration),
            Box::new(m20260310_000003_add_note_deleted_at::Migration),
            Box::new(m20260311_000001_upgrade_note_fts::Migration),
            Box::new(m20260311_000002_create_note_template_table::Migration),
            Box::new(m20260311_000003_create_note_link_table::Migration),
            Box::new(m20260315_000001_add_note_history_source::Migration),
            Box::new(m20260315_000002_add_note_template_content_type::Migration),
            Box::new(m20260315_000003_add_mcp_access::Migration),
            Box::new(m20260317_000001_upgrade_note_fts_tags::Migration),
            Box::new(m20260318_000001_add_performance_indexes::Migration),
            Box::new(m20260320_000001_create_sync_log_tables::Migration),
            Box::new(m20260320_000002_add_query_indexes::Migration),
            Box::new(m20260327_000001_create_app_log_table::Migration),
            Box::new(m20260328_000001_add_search_composite_indexes::Migration),
            Box::new(m20260329_000001_fix_app_log_id_type::Migration),
        ]
    }
}
