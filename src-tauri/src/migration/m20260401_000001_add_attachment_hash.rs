use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 添加 file_hash 列（SHA256 哈希，用于去重）
        manager
            .alter_table(
                Table::alter()
                    .table(NoteAttachment::Table)
                    .add_column(
                        ColumnDef::new(NoteAttachment::FileHash)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        // 添加 ref_count 列（引用计数，去重时多个记录可能指向同一文件）
        manager
            .alter_table(
                Table::alter()
                    .table(NoteAttachment::Table)
                    .add_column(
                        ColumnDef::new(NoteAttachment::RefCount)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .to_owned(),
            )
            .await?;

        // 为 file_hash 创建索引以加速去重查询
        manager
            .create_index(
                Index::create()
                    .name("idx_note_attachment_file_hash")
                    .table(NoteAttachment::Table)
                    .col(NoteAttachment::FileHash)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_note_attachment_file_hash")
                    .table(NoteAttachment::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(NoteAttachment::Table)
                    .drop_column(NoteAttachment::RefCount)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(NoteAttachment::Table)
                    .drop_column(NoteAttachment::FileHash)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum NoteAttachment {
    Table,
    FileHash,
    RefCount,
}
