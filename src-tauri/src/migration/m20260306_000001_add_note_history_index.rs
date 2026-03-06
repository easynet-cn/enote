//! 为 note_history 表添加复合索引
//!
//! 优化按笔记 ID 和时间倒序查询历史记录的性能

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 添加 (note_id, create_time) 复合索引，加速历史记录分页查询
        if !manager
            .has_index("note_history", "idx_note_history_note_id_create_time")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .name("idx_note_history_note_id_create_time")
                        .table(NoteHistory::Table)
                        .col(NoteHistory::NoteId)
                        .col(NoteHistory::CreateTime)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_note_history_note_id_create_time")
                    .table(NoteHistory::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum NoteHistory {
    Table,
    NoteId,
    CreateTime,
}
