//! 创建 tag 表迁移
//!
//! 标签表用于对笔记进行分类标记。

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(pk_auto(Tag::Id))
                    .col(string(Tag::Name).not_null())
                    .col(string(Tag::Icon).not_null().default(""))
                    .col(string(Tag::Cls).not_null().default(""))
                    .col(integer(Tag::SortOrder).not_null().default(0))
                    .col(date_time(Tag::CreateTime).not_null())
                    .col(date_time(Tag::UpdateTime).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await
    }
}

/// 标签表定义
#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Name,
    Icon,
    Cls,
    SortOrder,
    CreateTime,
    UpdateTime,
}
