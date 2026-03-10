//! 创建笔记链接表
//!
//! 用于实现笔记间的双向链接功能

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NoteLink::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NoteLink::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(NoteLink::SourceNoteId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(NoteLink::TargetNoteId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(NoteLink::CreateTime)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建索引
        manager
            .create_index(
                Index::create()
                    .name("idx_note_link_source")
                    .table(NoteLink::Table)
                    .col(NoteLink::SourceNoteId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_note_link_target")
                    .table(NoteLink::Table)
                    .col(NoteLink::TargetNoteId)
                    .to_owned(),
            )
            .await?;

        // 唯一约束：同一对链接不能重复
        manager
            .create_index(
                Index::create()
                    .name("idx_note_link_unique")
                    .table(NoteLink::Table)
                    .col(NoteLink::SourceNoteId)
                    .col(NoteLink::TargetNoteId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NoteLink::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum NoteLink {
    Table,
    Id,
    SourceNoteId,
    TargetNoteId,
    CreateTime,
}
