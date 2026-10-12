//! 待办相关命令
//!
//! 提供待办和待办清单的 CRUD、查询、软删除/恢复能力。
//!
//! # 说明
//! 通过 TodoStore 抽象层操作，Server 后端暂不支持待办功能
//!（todo_store 为 None 时返回 TODO_NOT_SUPPORTED）。

use super::*;

/// 查询待办列表
#[tauri::command]
pub async fn search_todos(
    app_state: tauri::State<'_, Arc<AppState>>,
    param: TodoSearchParam,
) -> Result<Vec<Todo>, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.search_todos(&param).await
}

/// 根据 ID 查询待办
#[tauri::command]
pub async fn find_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<Option<Todo>, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.find_todo(id).await
}

/// 创建待办
#[tauri::command]
pub async fn create_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    todo: Todo,
) -> Result<Todo, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.create_todo(&todo).await
}

/// 更新待办
#[tauri::command]
pub async fn update_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    todo: Todo,
) -> Result<Option<Todo>, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.update_todo(&todo).await
}

/// 切换待办完成状态
#[tauri::command]
pub async fn toggle_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<Option<Todo>, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.toggle_todo(id).await
}

/// 软删除待办（移入回收站）
#[tauri::command]
pub async fn delete_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let store = require_todo_store(&app_state).await?;
    store.delete_todo(id).await
}

/// 恢复回收站中的待办
#[tauri::command]
pub async fn restore_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let store = require_todo_store(&app_state).await?;
    store.restore_todo(id).await
}

/// 彻底删除待办（物理删除）
#[tauri::command]
pub async fn permanent_delete_todo(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let store = require_todo_store(&app_state).await?;
    store.permanent_delete_todo(id).await
}

/// 清空待办回收站
#[tauri::command]
pub async fn empty_todo_trash(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<u64, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.empty_todo_trash().await
}

/// 查询所有待办清单
#[tauri::command]
pub async fn find_all_todo_lists(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<TodoList>, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.find_all_todo_lists().await
}

/// 创建待办清单
#[tauri::command]
pub async fn create_todo_list(
    app_state: tauri::State<'_, Arc<AppState>>,
    list: TodoList,
) -> Result<TodoList, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.create_todo_list(&list).await
}

/// 更新待办清单
#[tauri::command]
pub async fn update_todo_list(
    app_state: tauri::State<'_, Arc<AppState>>,
    list: TodoList,
) -> Result<Option<TodoList>, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.update_todo_list(&list).await
}

/// 删除待办清单
///
/// 清单下的待办不会被删除，仅解除关联。
#[tauri::command]
pub async fn delete_todo_list(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let store = require_todo_store(&app_state).await?;
    store.delete_todo_list(id).await
}

/// 批量更新待办排序（拖拽排序后持久化）
#[tauri::command]
pub async fn reorder_todos(
    app_state: tauri::State<'_, Arc<AppState>>,
    orders: Vec<(i64, i32)>,
) -> Result<(), AppError> {
    let store = require_todo_store(&app_state).await?;
    store.reorder_todos(orders).await
}

/// 查询待办关联的标签 ID 列表
#[tauri::command]
pub async fn find_todo_tags(
    app_state: tauri::State<'_, Arc<AppState>>,
    todo_id: i64,
) -> Result<Vec<i64>, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.find_todo_tags(todo_id).await
}

/// 设置待办关联的标签（全量替换）
#[tauri::command]
pub async fn set_todo_tags(
    app_state: tauri::State<'_, Arc<AppState>>,
    todo_id: i64,
    tag_ids: Vec<i64>,
) -> Result<(), AppError> {
    let store = require_todo_store(&app_state).await?;
    store.set_todo_tags(todo_id, tag_ids).await
}

/// 批量切换待办完成状态
#[tauri::command]
pub async fn batch_toggle_todos(
    app_state: tauri::State<'_, Arc<AppState>>,
    ids: Vec<i64>,
) -> Result<u64, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.batch_toggle_todos(&ids).await
}

/// 批量软删除待办（移入回收站）
#[tauri::command]
pub async fn batch_delete_todos(
    app_state: tauri::State<'_, Arc<AppState>>,
    ids: Vec<i64>,
) -> Result<u64, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.batch_delete_todos(&ids).await
}

/// 批量移动待办到指定清单（list_id 为 null 表示移出清单）
#[tauri::command]
pub async fn batch_move_todos(
    app_state: tauri::State<'_, Arc<AppState>>,
    ids: Vec<i64>,
    list_id: Option<i64>,
) -> Result<u64, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.batch_move_todos(&ids, list_id).await
}

/// 获取待办统计信息
#[tauri::command]
pub async fn todo_stats(app_state: tauri::State<'_, Arc<AppState>>) -> Result<TodoStats, AppError> {
    let store = require_todo_store(&app_state).await?;
    store.todo_stats().await
}
