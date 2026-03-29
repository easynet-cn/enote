//! 修复所有表 id 列类型
//!
//! PostgreSQL 中 pk_auto 创建的是 SERIAL (int4)，
//! 但 entity 定义为 i64 (int8)，导致类型不匹配。
//! 此迁移将所有表的 id 列改为 BIGINT (int8) 以匹配 Rust 类型。

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// 需要修复 id 列类型的所有表
const TABLES: &[&str] = &[
    "notebook",
    "note",
    "tag",
    "note_tags",
    "note_history",
    "note_template",
    "note_link",
    "settings",
    "sync_log",
    "sync_log_detail",
    "app_log",
];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        // 仅 PostgreSQL 需要修复，SQLite 不区分整数大小，MySQL 的 pk_auto 已是 BIGINT
        if matches!(backend, sea_orm::DatabaseBackend::Postgres) {
            for table in TABLES {
                db.execute_unprepared(&format!(
                    "ALTER TABLE {table} ALTER COLUMN id SET DATA TYPE BIGINT"
                )).await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        if matches!(backend, sea_orm::DatabaseBackend::Postgres) {
            for table in TABLES {
                db.execute_unprepared(&format!(
                    "ALTER TABLE {table} ALTER COLUMN id SET DATA TYPE INTEGER"
                )).await?;
            }
        }

        Ok(())
    }
}
