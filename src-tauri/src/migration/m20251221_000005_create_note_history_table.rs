//! 创建 note_history 表迁移
//!
//! 笔记历史记录表，用于追踪笔记的变更历史。

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NoteHistory::Table)
                    .if_not_exists()
                    .col(pk_auto(NoteHistory::Id))
                    .col(big_integer(NoteHistory::NoteId).not_null())
                    .col(text(NoteHistory::OldContent).not_null())
                    .col(text(NoteHistory::NewContent).not_null())
                    .col(text(NoteHistory::Extra).not_null().default(""))
                    .col(integer(NoteHistory::OperateType).not_null())
                    .col(date_time(NoteHistory::OperateTime).not_null())
                    .col(date_time(NoteHistory::CreateTime).not_null())
                    .to_owned(),
            )
            .await?;

        // 创建索引加速按笔记查询历史（如果不存在）
        if !manager.has_index("note_history", "idx_note_history_note_id").await? {
            manager
                .create_index(
                    Index::create()
                        .name("idx_note_history_note_id")
                        .table(NoteHistory::Table)
                        .col(NoteHistory::NoteId)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NoteHistory::Table).to_owned())
            .await
    }
}

/// 笔记历史表定义
#[derive(DeriveIden)]
enum NoteHistory {
    Table,
    Id,
    NoteId,
    OldContent,
    NewContent,
    Extra,
    OperateType,
    OperateTime,
    CreateTime,
}
