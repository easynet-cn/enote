//! 创建 note_attachment 表迁移
//!
//! 笔记附件表用于存储笔记关联的文件附件元数据。

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NoteAttachment::Table)
                    .if_not_exists()
                    .col(pk_auto(NoteAttachment::Id))
                    .col(big_integer(NoteAttachment::NoteId).not_null())
                    .col(string(NoteAttachment::FileName).not_null())
                    .col(string(NoteAttachment::FilePath).not_null())
                    .col(big_integer(NoteAttachment::FileSize).not_null().default(0))
                    .col(string(NoteAttachment::MimeType).not_null().default(""))
                    .col(date_time(NoteAttachment::CreateTime).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_note_attachment_note_id")
                    .table(NoteAttachment::Table)
                    .col(NoteAttachment::NoteId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NoteAttachment::Table).to_owned())
            .await
    }
}

/// 笔记附件表定义
#[derive(DeriveIden)]
enum NoteAttachment {
    Table,
    Id,
    NoteId,
    FileName,
    FilePath,
    FileSize,
    MimeType,
    CreateTime,
}
