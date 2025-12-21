//! 创建 note 表迁移
//!
//! 笔记表存储笔记的标题和内容。

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Note::Table)
                    .if_not_exists()
                    .col(pk_auto(Note::Id))
                    .col(big_integer(Note::NotebookId).not_null().default(0))
                    .col(string(Note::Title).not_null())
                    .col(text(Note::Content).not_null())
                    // 内容类型：0 = HTML（默认），1 = Markdown
                    .col(integer(Note::ContentType).not_null().default(0))
                    .col(date_time(Note::CreateTime).not_null())
                    .col(date_time(Note::UpdateTime).not_null())
                    .to_owned(),
            )
            .await?;

        // 创建索引加速按笔记本查询（如果不存在）
        if !manager.has_index("note", "idx_note_notebook_id").await? {
            manager
                .create_index(
                    Index::create()
                        .name("idx_note_notebook_id")
                        .table(Note::Table)
                        .col(Note::NotebookId)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Note::Table).to_owned())
            .await
    }
}

/// 笔记表定义
#[derive(DeriveIden)]
enum Note {
    Table,
    Id,
    NotebookId,
    Title,
    Content,
    /// 内容类型：0 = HTML，1 = Markdown
    ContentType,
    CreateTime,
    UpdateTime,
}
