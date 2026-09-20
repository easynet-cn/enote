//! 创建待办相关表迁移
//!
//! 创建 `todo_list`（待办清单）和 `todo`（待办）两张表。
//!
//! # 跨数据库兼容说明
//!
//! 本迁移需同时支持 SQLite / MySQL / PostgreSQL，遵循以下约束：
//!
//! - **不使用 `pk_auto()`**：PostgreSQL 下会生成 `SERIAL(int4)`，与 entity 的 `i64` 不匹配
//!   （参见 `m20260329_000001_fix_app_log_id_type`）。统一使用
//!   `big_integer().primary_key().auto_increment()`。
//! - **`text` 字段不设置 DEFAULT**：MySQL 下 TEXT/BLOB 列的 DEFAULT 只能是 NULL，
//!   不能是非 NULL 值（如空字符串会报 ERROR 1101）。本表 text 列均为 NOT NULL，
//!   故不设 DEFAULT，默认值一律在业务层处理。
//! - **布尔语义统一用 `integer`（0/1）**：与 `note.is_pinned` / `is_starred` 保持一致。
//! - **索引创建区分后端**：MySQL 不支持 `CREATE INDEX IF NOT EXISTS`，需先查
//!   `information_schema.statistics` 判重。

use sea_orm_migration::{prelude::*, schema::*};

/// 需要创建的索引：(索引名, 表名, 列定义)
const INDEXES: &[(&str, &str, &str)] = &[
    ("idx_todo_deleted", "todo", "(deleted_at)"),
    ("idx_todo_completed", "todo", "(is_completed)"),
    ("idx_todo_due", "todo", "(due_date)"),
    ("idx_todo_list", "todo", "(list_id)"),
    ("idx_todo_parent", "todo", "(parent_id)"),
    ("idx_todo_note", "todo", "(note_id)"),
    ("idx_todo_remind", "todo", "(remind_at, is_reminded)"),
    ("idx_todo_tags_todo_id", "todo_tags", "(todo_id)"),
    ("idx_todo_tags_tag_id", "todo_tags", "(tag_id)"),
];

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建待办清单表
        manager
            .create_table(
                Table::create()
                    .table(TodoList::Table)
                    .if_not_exists()
                    .col(big_integer(TodoList::Id).primary_key().auto_increment())
                    .col(string(TodoList::Name).not_null())
                    .col(string(TodoList::Icon).not_null().default(""))
                    .col(string(TodoList::Color).not_null().default(""))
                    .col(integer(TodoList::SortOrder).not_null().default(0))
                    .col(integer(TodoList::McpAccess).not_null().default(0))
                    .col(date_time(TodoList::CreateTime).not_null())
                    .col(date_time(TodoList::UpdateTime).not_null())
                    .to_owned(),
            )
            .await?;

        // 创建待办表
        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(big_integer(Todo::Id).primary_key().auto_increment())
                    .col(string(Todo::Title).not_null())
                    // text 列不设 DEFAULT（MySQL 下只能是 NULL，而本列 NOT NULL），
                    // 默认值由业务层处理
                    .col(text(Todo::Description).not_null())
                    .col(integer(Todo::IsCompleted).not_null().default(0))
                    .col(integer(Todo::Priority).not_null().default(0))
                    .col(date_time_null(Todo::DueDate))
                    .col(date_time_null(Todo::CompletedAt))
                    .col(big_integer_null(Todo::ListId))
                    .col(integer(Todo::SortOrder).not_null().default(0))
                    .col(date_time(Todo::CreateTime).not_null())
                    .col(date_time(Todo::UpdateTime).not_null())
                    .col(date_time_null(Todo::DeletedAt))
                    // P1 字段：一次建全（可空或带默认值），P1 阶段无需再改表结构
                    .col(date_time_null(Todo::StartDate))
                    .col(date_time_null(Todo::RemindAt))
                    .col(integer(Todo::IsReminded).not_null().default(0))
                    .col(big_integer(Todo::ParentId).not_null().default(0))
                    .col(big_integer_null(Todo::NoteId))
                    .col(integer(Todo::RecurrenceType).not_null().default(0))
                    .col(integer(Todo::RecurrenceInterval).not_null().default(1))
                    .col(date_time_null(Todo::RecurrenceEndDate))
                    .col(integer(Todo::McpAccess).not_null().default(0))
                    .to_owned(),
            )
            .await?;

        // 创建待办标签关联表（复用现有 tag 体系，避免维护两套标签）
        manager
            .create_table(
                Table::create()
                    .table(TodoTags::Table)
                    .if_not_exists()
                    .col(big_integer(TodoTags::Id).primary_key().auto_increment())
                    .col(big_integer(TodoTags::TodoId).not_null())
                    .col(big_integer(TodoTags::TagId).not_null())
                    .col(date_time(TodoTags::CreateTime).not_null())
                    .to_owned(),
            )
            .await?;

        // 创建索引（MySQL 不支持 IF NOT EXISTS，需按后端分别处理）
        let db = manager.get_connection();
        let backend = db.get_database_backend();

        for (idx_name, table, columns) in INDEXES {
            match backend {
                sea_orm::DatabaseBackend::Sqlite | sea_orm::DatabaseBackend::Postgres => {
                    let sql = format!("CREATE INDEX IF NOT EXISTS {idx_name} ON {table} {columns}");
                    db.execute_unprepared(&sql).await?;
                }
                sea_orm::DatabaseBackend::MySql => {
                    let check = format!(
                        "SELECT 1 FROM information_schema.statistics WHERE table_schema = DATABASE() AND table_name = '{table}' AND index_name = '{idx_name}' LIMIT 1"
                    );
                    let has: Vec<sea_orm::QueryResult> = db
                        .query_all_raw(sea_orm::Statement::from_string(backend, check))
                        .await?;
                    if has.is_empty() {
                        let sql = format!("CREATE INDEX {idx_name} ON {table} {columns}");
                        db.execute_unprepared(&sql).await?;
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TodoList::Table).to_owned())
            .await
    }
}

/// 待办表定义
#[derive(DeriveIden)]
enum Todo {
    Table,
    Id,
    Title,
    Description,
    IsCompleted,
    Priority,
    DueDate,
    CompletedAt,
    ListId,
    SortOrder,
    CreateTime,
    UpdateTime,
    DeletedAt,
    StartDate,
    RemindAt,
    IsReminded,
    ParentId,
    NoteId,
    RecurrenceType,
    RecurrenceInterval,
    RecurrenceEndDate,
    McpAccess,
}

/// 待办标签关联表定义
#[derive(DeriveIden)]
enum TodoTags {
    Table,
    Id,
    TodoId,
    TagId,
    CreateTime,
}

/// 待办清单表定义
#[derive(DeriveIden)]
enum TodoList {
    Table,
    Id,
    Name,
    Icon,
    Color,
    SortOrder,
    McpAccess,
    CreateTime,
    UpdateTime,
}
