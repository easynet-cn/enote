//! 创建应用日志表迁移
//!
//! app_log: 记录应用操作审计日志

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 app_log 表
        manager
            .create_table(
                Table::create()
                    .table(AppLog::Table)
                    .if_not_exists()
                    .col(pk_auto(AppLog::Id))
                    .col(string(AppLog::Level).not_null())
                    .col(string(AppLog::Module).not_null())
                    .col(string(AppLog::Action).not_null())
                    .col(string_null(AppLog::TargetId))
                    .col(string_null(AppLog::TargetName))
                    .col(text(AppLog::Message).not_null())
                    .col(text_null(AppLog::Detail))
                    .col(date_time(AppLog::CreateTime).not_null())
                    .to_owned(),
            )
            .await?;

        // 创建索引
        manager
            .create_index(
                Index::create()
                    .name("idx_app_log_level")
                    .table(AppLog::Table)
                    .col(AppLog::Level)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_app_log_module")
                    .table(AppLog::Table)
                    .col(AppLog::Module)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_app_log_create_time")
                    .table(AppLog::Table)
                    .col(AppLog::CreateTime)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AppLog::Table).to_owned())
            .await?;
        Ok(())
    }
}

/// 应用日志表定义
#[derive(DeriveIden)]
enum AppLog {
    Table,
    Id,
    Level,
    Module,
    Action,
    TargetId,
    TargetName,
    Message,
    Detail,
    CreateTime,
}
