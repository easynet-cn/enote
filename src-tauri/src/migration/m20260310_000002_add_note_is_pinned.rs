//! 为 note 表添加 is_pinned 列
//!
//! 支持笔记置顶功能

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Note::Table)
                    .add_column(
                        ColumnDef::new(Note::IsPinned)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Note::Table)
                    .drop_column(Note::IsPinned)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Note {
    Table,
    IsPinned,
}
