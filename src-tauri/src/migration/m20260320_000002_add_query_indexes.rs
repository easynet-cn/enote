//! 添加查询优化索引
//!
//! - note_tags(tag_id): 按标签筛选笔记
//! - note_link(source_note_id), note_link(target_note_id): 双向链接查询
//! - note_history(note_id, operate_time DESC): 历史记录分页
//! - note_template(sort_order DESC, update_time DESC): 模板列表排序

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const INDEXES: &[(&str, &str, &str)] = &[
    ("idx_note_tags_tag_id", "note_tags", "(tag_id)"),
    ("idx_note_link_source", "note_link", "(source_note_id)"),
    ("idx_note_link_target", "note_link", "(target_note_id)"),
    (
        "idx_note_history_note_operate",
        "note_history",
        "(note_id, operate_time DESC)",
    ),
    (
        "idx_note_template_sort",
        "note_template",
        "(sort_order DESC, update_time DESC)",
    ),
];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = db.get_database_backend();

        for (idx_name, table, columns) in INDEXES {
            match backend {
                sea_orm::DatabaseBackend::Sqlite | sea_orm::DatabaseBackend::Postgres => {
                    db.execute_unprepared(&format!(
                        "CREATE INDEX IF NOT EXISTS {idx_name} ON {table} {columns}"
                    ))
                    .await?;
                }
                sea_orm::DatabaseBackend::MySql => {
                    let check = format!(
                        "SELECT 1 FROM information_schema.statistics WHERE table_schema = DATABASE() AND table_name = '{table}' AND index_name = '{idx_name}' LIMIT 1"
                    );
                    let has: Vec<sea_orm::QueryResult> = db
                        .query_all(sea_orm::Statement::from_string(backend, check))
                        .await?;
                    if has.is_empty() {
                        db.execute_unprepared(&format!(
                            "CREATE INDEX {idx_name} ON {table} {columns}"
                        ))
                        .await?;
                    }
                }
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        for (idx_name, _, _) in INDEXES {
            db.execute_unprepared(&format!("DROP INDEX IF EXISTS {idx_name}"))
                .await
                .ok();
        }
        Ok(())
    }
}
