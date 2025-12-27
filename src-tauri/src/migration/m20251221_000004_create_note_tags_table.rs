//! 创建 note_tags 表迁移
//!
//! 笔记标签关联表，实现笔记与标签的多对多关系。

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NoteTags::Table)
                    .if_not_exists()
                    .col(pk_auto(NoteTags::Id))
                    .col(big_integer(NoteTags::NoteId).not_null())
                    .col(big_integer(NoteTags::TagId).not_null())
                    .col(integer(NoteTags::SortOrder).not_null().default(0))
                    .col(date_time(NoteTags::CreateTime).not_null())
                    // 注意：保持与现有实体一致的拼写 "upate_time"
                    .col(date_time(NoteTags::UpateTime).not_null())
                    .to_owned(),
            )
            .await?;

        // 创建索引加速查询（如果不存在）
        if !manager.has_index("note_tags", "idx_note_tags_note_id").await? {
            manager
                .create_index(
                    Index::create()
                        .name("idx_note_tags_note_id")
                        .table(NoteTags::Table)
                        .col(NoteTags::NoteId)
                        .to_owned(),
                )
                .await?;
        }

        if !manager.has_index("note_tags", "idx_note_tags_tag_id").await? {
            manager
                .create_index(
                    Index::create()
                        .name("idx_note_tags_tag_id")
                        .table(NoteTags::Table)
                        .col(NoteTags::TagId)
                        .to_owned(),
                )
                .await?;
        }

        // 创建复合唯一索引（note_id + tag_id，防止重复关联）
        if !manager
            .has_index("note_tags", "idx_note_tags_unique")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .name("idx_note_tags_unique")
                        .table(NoteTags::Table)
                        .col(NoteTags::NoteId)
                        .col(NoteTags::TagId)
                        .unique()
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NoteTags::Table).to_owned())
            .await
    }
}

/// 笔记标签关联表定义
#[derive(DeriveIden)]
enum NoteTags {
    Table,
    Id,
    NoteId,
    TagId,
    SortOrder,
    CreateTime,
    #[sea_orm(iden = "upate_time")]
    UpateTime,
}
