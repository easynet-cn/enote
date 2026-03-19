//! 添加性能优化索引
//!
//! - note(deleted_at, notebook_id): 优化回收站和笔记本筛选查询
//! - note(is_pinned, update_time): 优化置顶排序查询

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = db.get_database_backend();

        match backend {
            sea_orm::DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "CREATE INDEX IF NOT EXISTS idx_note_deleted_notebook ON note (deleted_at, notebook_id)"
                ).await?;
                db.execute_unprepared(
                    "CREATE INDEX IF NOT EXISTS idx_note_pinned_update ON note (is_pinned DESC, update_time DESC)"
                ).await?;
            }
            sea_orm::DatabaseBackend::MySql => {
                let has_idx1: Vec<sea_orm::QueryResult> = db.query_all(sea_orm::Statement::from_string(
                    backend,
                    "SELECT 1 FROM information_schema.statistics WHERE table_schema = DATABASE() AND table_name = 'note' AND index_name = 'idx_note_deleted_notebook' LIMIT 1".to_owned(),
                )).await?;
                if has_idx1.is_empty() {
                    db.execute_unprepared(
                        "CREATE INDEX idx_note_deleted_notebook ON note (deleted_at, notebook_id)"
                    ).await?;
                }

                let has_idx2: Vec<sea_orm::QueryResult> = db.query_all(sea_orm::Statement::from_string(
                    backend,
                    "SELECT 1 FROM information_schema.statistics WHERE table_schema = DATABASE() AND table_name = 'note' AND index_name = 'idx_note_pinned_update' LIMIT 1".to_owned(),
                )).await?;
                if has_idx2.is_empty() {
                    db.execute_unprepared(
                        "CREATE INDEX idx_note_pinned_update ON note (is_pinned DESC, update_time DESC)"
                    ).await?;
                }
            }
            sea_orm::DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    "CREATE INDEX IF NOT EXISTS idx_note_deleted_notebook ON note (deleted_at, notebook_id)"
                ).await?;
                db.execute_unprepared(
                    "CREATE INDEX IF NOT EXISTS idx_note_pinned_update ON note (is_pinned DESC, update_time DESC)"
                ).await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS idx_note_deleted_notebook").await.ok();
        db.execute_unprepared("DROP INDEX IF EXISTS idx_note_pinned_update").await.ok();
        Ok(())
    }
}
