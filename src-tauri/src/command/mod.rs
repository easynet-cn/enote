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
    config::AppState,
    error::AppError,
    model::{
        AppLog, AppLogSearchParam, LogFileInfo, Note, NoteAttachment, NoteHistory,
        NoteHistorySearchPageParam, NoteLink, NoteSearchPageParam, NoteStatsResult, NoteTemplate,
        Notebook, OperateSource, PageParam, PageResult, SyncLog, SyncLogDetail, SyncOptions,
        SyncPreview, Tag,
    },
    service,
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

mod profile;
mod notebook;
mod tag;
mod note;
mod backup;
mod settings;
mod sync;
mod app_log;
mod misc;

pub use profile::*;
pub use notebook::*;
pub use tag::*;
pub use note::*;
pub use backup::*;
pub use settings::*;
pub use sync::*;
pub use app_log::*;
pub use misc::*;
