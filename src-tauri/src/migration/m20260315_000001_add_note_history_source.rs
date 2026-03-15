//! 为 note_history 表添加 operate_source 列
//!
//! 记录操作来源：0 = 用户操作（Tauri UI），1 = MCP 操作（AI 工具）

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(NoteHistory::Table)
                    .add_column(
                        ColumnDef::new(NoteHistory::OperateSource)
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
                    .table(NoteHistory::Table)
                    .drop_column(NoteHistory::OperateSource)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum NoteHistory {
    Table,
    OperateSource,
}
