//! 创建 note_template 表迁移
//!
//! 笔记模板表用于存储可复用的笔记模板。

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NoteTemplate::Table)
                    .if_not_exists()
                    .col(pk_auto(NoteTemplate::Id))
                    .col(string(NoteTemplate::Name).not_null())
                    .col(text(NoteTemplate::Content).not_null().default(""))
                    .col(integer(NoteTemplate::SortOrder).not_null().default(0))
                    .col(date_time(NoteTemplate::CreateTime).not_null())
                    .col(date_time(NoteTemplate::UpdateTime).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NoteTemplate::Table).to_owned())
            .await
    }
}

/// 笔记模板表定义
#[derive(DeriveIden)]
enum NoteTemplate {
    Table,
    Id,
    Name,
    Content,
    SortOrder,
    CreateTime,
    UpdateTime,
}
