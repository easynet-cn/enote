//! Tauri 命令处理器模块
//!
//! 本模块定义了所有可从前端调用的 Tauri 命令。
//! 每个命令都是一个异步函数，接收前端传入的参数，
//! 调用相应的服务层方法，并返回结果给前端。
//!
//! # 命令分类
//! - 笔记本命令：CRUD 操作
//! - 标签命令：CRUD 操作
//! - 笔记命令：CRUD 和搜索操作
//! - 历史记录命令：分页搜索

use std::sync::Arc;

use std::collections::HashMap;

use tracing::info;

use sea_orm::DatabaseConnection;
use tauri::Manager;

use crate::{
    config::{AppState, ProfileBackend},
    error::AppError,
    model::{
        AppLog, AppLogSearchParam, AttachmentStats, LogFileInfo, Note, NoteAttachment, NoteHistory,
        NoteHistorySearchPageParam, NoteLink, NoteSearchPageParam, NoteStatsResult, NoteTemplate,
        Notebook, PageParam, PageResult, SyncLog, SyncLogDetail, SyncOptions, SyncPreview, Tag,
        Todo, TodoList, TodoSearchParam, TodoStats,
    },
    service,
    service::enote_server::EnoteServerClient,
};

/// 从 AppState 获取数据库连接，未连接时返回明确错误。
///
/// `DatabaseConnection` 内部基于 `Arc<Pool>`，clone 为 O(1) 操作。
pub(crate) async fn require_db(app_state: &AppState) -> Result<DatabaseConnection, AppError> {
    app_state
        .database_connection
        .read()
        .await
        .clone()
        .ok_or_else(|| AppError::code("DB_NOT_CONNECTED"))
}

/// 获取 EnoteServerClient，仅 Server 后端可用
pub(crate) async fn require_server(app_state: &AppState) -> Result<EnoteServerClient, AppError> {
    match &*app_state.backend.read().await {
        ProfileBackend::Server(client) => Ok(client.clone()),
        ProfileBackend::Database | ProfileBackend::Git => Err(AppError::code("NOT_SERVER_BACKEND")),
    }
}

/// 从 AppState 获取当前笔记域存储后端（NoteStore 抽象层），未连接时返回明确错误。
///
/// `Arc<dyn NoteStore>` clone 为 O(1) 操作。
pub(crate) async fn require_note_store(
    app_state: &AppState,
) -> Result<std::sync::Arc<dyn crate::store::NoteStore>, AppError> {
    app_state
        .note_store
        .read()
        .await
        .clone()
        .ok_or_else(|| AppError::code("DB_NOT_CONNECTED"))
}

/// 获取待办存储后端（Server 后端暂不支持，返回 TODO_NOT_SUPPORTED）
pub(crate) async fn require_todo_store(
    app_state: &AppState,
) -> Result<std::sync::Arc<dyn crate::store::TodoStore>, AppError> {
    app_state
        .todo_store
        .read()
        .await
        .clone()
        .ok_or_else(|| AppError::code("DB_NOT_CONNECTED"))
}

mod app_log;
mod backup;
mod cloud_backup;
mod misc;
mod note;
mod notebook;
mod profile;
mod screen_saver;
mod settings;
mod sync;
mod tag;
mod todo;

pub use app_log::*;
pub use backup::*;
pub use cloud_backup::*;
pub use misc::*;
pub use note::*;
pub use notebook::*;
pub use profile::*;
pub use screen_saver::*;
pub use settings::*;
pub use sync::*;
pub use tag::*;
pub use todo::*;
