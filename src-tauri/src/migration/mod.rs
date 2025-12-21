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

pub use sea_orm_migration::prelude::*;

mod m20251221_000001_create_notebook_table;
mod m20251221_000002_create_note_table;
mod m20251221_000003_create_tag_table;
mod m20251221_000004_create_note_tags_table;
mod m20251221_000005_create_note_history_table;

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
        ]
    }
}
