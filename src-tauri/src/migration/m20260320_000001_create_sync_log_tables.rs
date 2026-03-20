//! 创建同步日志表迁移
//!
//! sync_log: 记录每次同步操作的汇总信息
//! sync_log_detail: 记录每条数据的同步明细

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 sync_log 表
        manager
            .create_table(
                Table::create()
                    .table(SyncLog::Table)
                    .if_not_exists()
                    .col(pk_auto(SyncLog::Id))
                    .col(string(SyncLog::SourceProfile).not_null())
                    .col(string(SyncLog::TargetProfile).not_null())
                    .col(string(SyncLog::SourceDbType).not_null())
                    .col(string(SyncLog::TargetDbType).not_null())
                    .col(string(SyncLog::SyncMode).not_null())
                    .col(text(SyncLog::SyncScope).not_null())
                    .col(string_null(SyncLog::BackupFormat))
                    .col(string_null(SyncLog::SourceBackup))
                    .col(string_null(SyncLog::TargetBackup))
                    .col(string(SyncLog::Status).not_null())
                    .col(integer(SyncLog::TotalCount).not_null().default(0))
                    .col(integer(SyncLog::SuccessCount).not_null().default(0))
                    .col(integer(SyncLog::FailedCount).not_null().default(0))
                    .col(text_null(SyncLog::ErrorMessage))
                    .col(date_time(SyncLog::StartedAt).not_null())
                    .col(date_time_null(SyncLog::FinishedAt))
                    .col(date_time(SyncLog::CreateTime).not_null())
                    .to_owned(),
            )
            .await?;

        // 创建 sync_log_detail 表
        manager
            .create_table(
                Table::create()
                    .table(SyncLogDetail::Table)
                    .if_not_exists()
                    .col(pk_auto(SyncLogDetail::Id))
                    .col(big_integer(SyncLogDetail::SyncLogId).not_null())
                    .col(string(SyncLogDetail::TableName).not_null())
                    .col(big_integer(SyncLogDetail::SourceId).not_null())
                    .col(big_integer_null(SyncLogDetail::TargetId))
                    .col(string(SyncLogDetail::RecordName).not_null())
                    .col(string(SyncLogDetail::Status).not_null())
                    .col(text_null(SyncLogDetail::ErrorMessage))
                    .col(date_time(SyncLogDetail::SyncedAt).not_null())
                    .col(date_time(SyncLogDetail::CreateTime).not_null())
                    .to_owned(),
            )
            .await?;

        // 创建索引
        manager
            .create_index(
                Index::create()
                    .name("idx_sync_log_detail_log_id")
                    .table(SyncLogDetail::Table)
                    .col(SyncLogDetail::SyncLogId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SyncLogDetail::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SyncLog::Table).to_owned())
            .await?;
        Ok(())
    }
}

/// 同步日志表定义
#[derive(DeriveIden)]
enum SyncLog {
    Table,
    Id,
    SourceProfile,
    TargetProfile,
    SourceDbType,
    TargetDbType,
    SyncMode,
    SyncScope,
    BackupFormat,
    SourceBackup,
    TargetBackup,
    Status,
    TotalCount,
    SuccessCount,
    FailedCount,
    ErrorMessage,
    StartedAt,
    FinishedAt,
    CreateTime,
}

/// 同步日志明细表定义
#[derive(DeriveIden)]
enum SyncLogDetail {
    Table,
    Id,
    SyncLogId,
    TableName,
    SourceId,
    TargetId,
    RecordName,
    Status,
    ErrorMessage,
    SyncedAt,
    CreateTime,
}
