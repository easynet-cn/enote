//! 创建 notebook 表迁移
//!
//! 笔记本表用于组织和分类笔记，支持层级结构。

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notebook::Table)
                    .if_not_exists()
                    .col(pk_auto(Notebook::Id))
                    .col(big_integer(Notebook::ParentId).not_null().default(0))
                    .col(string(Notebook::Name).not_null())
                    .col(string(Notebook::Description).not_null().default(""))
                    .col(string(Notebook::Icon).not_null().default(""))
                    .col(string(Notebook::Cls).not_null().default(""))
                    .col(integer(Notebook::SortOrder).not_null().default(0))
                    .col(date_time(Notebook::CreateTime).not_null())
                    .col(date_time(Notebook::UpdateTime).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Notebook::Table).to_owned())
            .await
    }
}

/// 笔记本表定义
#[derive(DeriveIden)]
enum Notebook {
    Table,
    Id,
    ParentId,
    Name,
    Description,
    Icon,
    Cls,
    SortOrder,
    CreateTime,
    UpdateTime,
}
