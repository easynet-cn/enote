//! 为 notebook、tag、note 表添加 mcp_access 列
//!
//! MCP 三层访问控制：
//! - 0 = Inherit (继承上层设置)
//! - 1 = ReadWrite (AI 可读可写)
//! - 2 = ReadOnly (AI 只能读取)
//! - 3 = Deny (AI 完全不可访问)

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Notebook::Table)
                    .add_column(
                        ColumnDef::new(Notebook::McpAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Tag::Table)
                    .add_column(
                        ColumnDef::new(Tag::McpAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Note::Table)
                    .add_column(
                        ColumnDef::new(Note::McpAccess)
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
                    .drop_column(Note::McpAccess)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Tag::Table)
                    .drop_column(Tag::McpAccess)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Notebook::Table)
                    .drop_column(Notebook::McpAccess)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Notebook {
    Table,
    McpAccess,
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    McpAccess,
}

#[derive(DeriveIden)]
enum Note {
    Table,
    McpAccess,
}
