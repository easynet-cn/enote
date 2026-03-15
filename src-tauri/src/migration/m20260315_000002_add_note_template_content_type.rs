//! 为 note_template 表添加 content_type 列
//!
//! 支持模板的内容类型：0 = HTML（富文本），1 = Markdown

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(NoteTemplate::Table)
                    .add_column(
                        ColumnDef::new(NoteTemplate::ContentType)
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
                    .table(NoteTemplate::Table)
                    .drop_column(NoteTemplate::ContentType)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum NoteTemplate {
    Table,
    ContentType,
}
