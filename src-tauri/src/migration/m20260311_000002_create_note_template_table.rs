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
                    // 注意：MySQL 下 TEXT/BLOB 列的 DEFAULT 只能是 NULL，不能是非 NULL 值
                    // （如空字符串会报 ERROR 1101）。本列为 NOT NULL，故无法设置 DEFAULT，
                    // 默认值改由业务层在插入时显式赋值
                    .col(text(NoteTemplate::Content).not_null())
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
