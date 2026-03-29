//! 添加复合搜索索引
//!
//! - note(deleted_at, notebook_id, update_time DESC): 覆盖主搜索查询模式
//! - note(deleted_at, update_time DESC): 覆盖不按笔记本筛选的查询

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
                    "CREATE INDEX IF NOT EXISTS idx_note_search_composite ON note (deleted_at, notebook_id, update_time DESC)"
                ).await?;
                db.execute_unprepared(
                    "CREATE INDEX IF NOT EXISTS idx_note_deleted_update ON note (deleted_at, update_time DESC)"
                ).await?;
            }
            sea_orm::DatabaseBackend::MySql => {
                let has_idx1: Vec<sea_orm::QueryResult> = db.query_all(sea_orm::Statement::from_string(
                    backend,
                    "SELECT 1 FROM information_schema.statistics WHERE table_schema = DATABASE() AND table_name = 'note' AND index_name = 'idx_note_search_composite' LIMIT 1".to_owned(),
                )).await?;
                if has_idx1.is_empty() {
                    db.execute_unprepared(
                        "CREATE INDEX idx_note_search_composite ON note (deleted_at, notebook_id, update_time DESC)",
                    ).await?;
                }

                let has_idx2: Vec<sea_orm::QueryResult> = db.query_all(sea_orm::Statement::from_string(
                    backend,
                    "SELECT 1 FROM information_schema.statistics WHERE table_schema = DATABASE() AND table_name = 'note' AND index_name = 'idx_note_deleted_update' LIMIT 1".to_owned(),
                )).await?;
                if has_idx2.is_empty() {
                    db.execute_unprepared(
                        "CREATE INDEX idx_note_deleted_update ON note (deleted_at, update_time DESC)",
                    ).await?;
                }
            }
            sea_orm::DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    "CREATE INDEX IF NOT EXISTS idx_note_search_composite ON note (deleted_at, notebook_id, update_time DESC)"
                ).await?;
                db.execute_unprepared(
                    "CREATE INDEX IF NOT EXISTS idx_note_deleted_update ON note (deleted_at, update_time DESC)"
                ).await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS idx_note_search_composite")
            .await
            .ok();
        db.execute_unprepared("DROP INDEX IF EXISTS idx_note_deleted_update")
            .await
            .ok();
        Ok(())
    }
}
