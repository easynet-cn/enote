//! 待办相关命令
//!
//! 提供待办和待办清单的 CRUD、查询、软删除/恢复能力。
//!
//! # 说明
//! Server 后端（ENote Server）目前尚未提供待办接口，因此这些命令
//! 直接走本地数据库连接（SQLite / MySQL / PostgreSQL）。

use super::*;

/// 查询待办列表
#[tauri::command]
pub async fn search_todos(
    app_state: tauri::State<'_, Arc<AppState>>,
    param: TodoSearchParam,
) -> Result<Vec<Todo>, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::search(&db, &param)
        .await
        .map_err(AppError::from)
}

/// 根据 ID 查询待办
#[tauri::command]
pub async fn find_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<Option<Todo>, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::find_by_id(&db, id)
        .await
        .map_err(AppError::from)
}

/// 创建待办
#[tauri::command]
pub async fn create_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    todo: Todo,
) -> Result<Todo, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::create(&db, &todo)
        .await
        .map_err(AppError::from)
}

/// 更新待办
#[tauri::command]
pub async fn update_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    todo: Todo,
) -> Result<Option<Todo>, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::update(&db, &todo)
        .await
        .map_err(AppError::from)
}

/// 切换待办完成状态
#[tauri::command]
pub async fn toggle_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<Option<Todo>, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::toggle_complete(&db, id)
        .await
        .map_err(AppError::from)
}

/// 软删除待办（移入回收站）
#[tauri::command]
pub async fn delete_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::todo::delete_by_id(&db, id)
        .await
        .map_err(AppError::from)
}

/// 恢复回收站中的待办
#[tauri::command]
pub async fn restore_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::todo::restore(&db, id).await.map_err(AppError::from)
}

/// 彻底删除待办（物理删除）
#[tauri::command]
pub async fn permanent_delete_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::todo::permanent_delete(&db, id)
        .await
        .map_err(AppError::from)
}

/// 清空待办回收站
#[tauri::command]
pub async fn empty_todo_trash(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<u64, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::empty_trash(&db)
        .await
        .map_err(AppError::from)
}

/// 查询所有待办清单
#[tauri::command]
pub async fn find_all_todo_lists(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<TodoList>, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::find_all_lists(&db)
        .await
        .map_err(AppError::from)
}

/// 创建待办清单
#[tauri::command]
pub async fn create_todo_list(
    app_state: tauri::State<'_, Arc<AppState>>,
    list: TodoList,
) -> Result<TodoList, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::create_list(&db, &list)
        .await
        .map_err(AppError::from)
}

/// 更新待办清单
#[tauri::command]
pub async fn update_todo_list(
    app_state: tauri::State<'_, Arc<AppState>>,
    list: TodoList,
) -> Result<Option<TodoList>, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::update_list(&db, &list)
        .await
        .map_err(AppError::from)
}

/// 删除待办清单
///
/// 清单下的待办不会被删除，仅解除关联。
#[tauri::command]
pub async fn delete_todo_list(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::todo::delete_list(&db, id)
        .await
        .map_err(AppError::from)
}

/// 批量更新待办排序（拖拽排序后持久化）
#[tauri::command]
pub async fn reorder_todos(
    app_state: tauri::State<'_, Arc<AppState>>,
    orders: Vec<(i64, i32)>,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::todo::reorder(&db, orders)
        .await
        .map_err(AppError::from)
}

/// 查询待办关联的标签 ID 列表
#[tauri::command]
pub async fn find_todo_tags(
    app_state: tauri::State<'_, Arc<AppState>>,
    todo_id: i64,
) -> Result<Vec<i64>, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::find_tag_ids_by_todo(&db, todo_id)
        .await
        .map_err(AppError::from)
}

/// 设置待办关联的标签（全量替换）
#[tauri::command]
pub async fn set_todo_tags(
    app_state: tauri::State<'_, Arc<AppState>>,
    todo_id: i64,
    tag_ids: Vec<i64>,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::todo::set_todo_tags(&db, todo_id, tag_ids)
        .await
        .map_err(AppError::from)
}

/// 批量切换待办完成状态
#[tauri::command]
pub async fn batch_toggle_todos(
    app_state: tauri::State<'_, Arc<AppState>>,
    ids: Vec<i64>,
) -> Result<u64, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::batch_toggle(&db, ids)
        .await
        .map_err(AppError::from)
}

/// 批量软删除待办（移入回收站）
#[tauri::command]
pub async fn batch_delete_todos(
    app_state: tauri::State<'_, Arc<AppState>>,
    ids: Vec<i64>,
) -> Result<u64, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::batch_delete(&db, ids)
        .await
        .map_err(AppError::from)
}

/// 批量移动待办到指定清单（list_id 为 null 表示移出清单）
#[tauri::command]
pub async fn batch_move_todos(
    app_state: tauri::State<'_, Arc<AppState>>,
    ids: Vec<i64>,
    list_id: Option<i64>,
) -> Result<u64, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::batch_move(&db, ids, list_id)
        .await
        .map_err(AppError::from)
}

/// 获取待办统计信息
#[tauri::command]
pub async fn todo_stats(app_state: tauri::State<'_, Arc<AppState>>) -> Result<TodoStats, AppError> {
    let db = require_db(&app_state).await?;
    service::todo::stats(&db).await.map_err(AppError::from)
}
