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

use crate::{
    config::AppState,
    error::AppError,
    model::{
        Note, NoteHistory, NoteHistorySearchPageParam, NoteSearchPageParam, NoteStatsResult,
        Notebook, PageResult, Tag,
    },
    service,
};

// ============================================================================
// 笔记本相关命令
// ============================================================================

/// 获取所有笔记本
///
/// # 返回
/// - `Ok(Vec<Notebook>)`: 笔记本列表
/// - `Err(AppError)`: 错误信息
#[tauri::command]
pub async fn find_all_notebooks(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<Notebook>, AppError> {
    let db = &app_state.database_connection;
    let notebooks = service::notebook::find_all(db).await.map_err(AppError::from)?;
    Ok(notebooks)
}

/// 创建笔记本
///
/// # 参数
/// - `notebook`: 要创建的笔记本数据
///
/// # 返回
/// - `Ok(Some(Notebook))`: 创建成功，返回新笔记本
/// - `Ok(None)`: 创建后未找到（异常情况）
/// - `Err(AppError)`: 创建失败
#[tauri::command]
pub async fn create_notebook(
    app_state: tauri::State<'_, Arc<AppState>>,
    notebook: Notebook,
) -> Result<Option<Notebook>, AppError> {
    let db = &app_state.database_connection;
    service::notebook::create(db, &notebook).await.map_err(AppError::from)
}

/// 根据 ID 删除笔记本
///
/// # 参数
/// - `id`: 笔记本 ID
///
/// # 返回
/// - `Ok(())`: 删除成功
/// - `Err(AppError)`: 删除失败
#[tauri::command]
pub async fn delete_notebook_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::notebook::delete_by_id(db, id).await.map_err(AppError::from)
}

/// 更新笔记本
///
/// # 参数
/// - `notebook`: 包含更新数据的笔记本对象（需包含 ID）
///
/// # 返回
/// - `Ok(Some(Notebook))`: 更新成功，返回更新后的笔记本
/// - `Ok(None)`: 笔记本不存在
/// - `Err(AppError)`: 更新失败
#[tauri::command]
pub async fn update_notebook(
    app_state: tauri::State<'_, Arc<AppState>>,
    notebook: Notebook,
) -> Result<Option<Notebook>, AppError> {
    let db = &app_state.database_connection;
    service::notebook::update(db, &notebook).await.map_err(AppError::from)
}

// ============================================================================
// 标签相关命令
// ============================================================================

/// 获取所有标签
///
/// # 返回
/// - `Ok(Vec<Tag>)`: 标签列表
/// - `Err(AppError)`: 错误信息
#[tauri::command]
pub async fn find_all_tags(app_state: tauri::State<'_, Arc<AppState>>) -> Result<Vec<Tag>, AppError> {
    let db = &app_state.database_connection;
    let tags = service::tag::find_all(db).await.map_err(AppError::from)?;
    Ok(tags)
}

/// 创建标签
///
/// # 参数
/// - `tag`: 要创建的标签数据
///
/// # 返回
/// - `Ok(Some(Tag))`: 创建成功，返回新标签
/// - `Ok(None)`: 创建后未找到（异常情况）
/// - `Err(AppError)`: 创建失败
#[tauri::command]
pub async fn create_tag(
    app_state: tauri::State<'_, Arc<AppState>>,
    tag: Tag,
) -> Result<Option<Tag>, AppError> {
    let db = &app_state.database_connection;
    service::tag::create(db, &tag).await.map_err(AppError::from)
}

/// 根据 ID 删除标签
///
/// # 参数
/// - `id`: 标签 ID
///
/// # 返回
/// - `Ok(())`: 删除成功
/// - `Err(AppError)`: 删除失败
#[tauri::command]
pub async fn delete_tag_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::tag::delete_by_id(db, id).await.map_err(AppError::from)
}

/// 更新标签
///
/// # 参数
/// - `tag`: 包含更新数据的标签对象（需包含 ID）
///
/// # 返回
/// - `Ok(Some(Tag))`: 更新成功，返回更新后的标签
/// - `Ok(None)`: 标签不存在
/// - `Err(AppError)`: 更新失败
#[tauri::command]
pub async fn update_tag(
    app_state: tauri::State<'_, Arc<AppState>>,
    tag: Tag,
) -> Result<Option<Tag>, AppError> {
    let db = &app_state.database_connection;
    service::tag::update(db, &tag).await.map_err(AppError::from)
}

// ============================================================================
// 笔记相关命令
// ============================================================================

/// 创建笔记
///
/// # 参数
/// - `note`: 要创建的笔记数据（包含标题、内容、标签等）
///
/// # 返回
/// - `Ok(Some(Note))`: 创建成功，返回新笔记（包含关联的标签信息）
/// - `Ok(None)`: 创建后未找到（异常情况）
/// - `Err(AppError)`: 创建失败
#[tauri::command]
pub async fn create_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    note: Note,
) -> Result<Option<Note>, AppError> {
    let db = &app_state.database_connection;
    service::note::create(db, &note).await.map_err(AppError::from)
}

/// 更新笔记
///
/// # 参数
/// - `note`: 包含更新数据的笔记对象（需包含 ID）
///
/// # 返回
/// - `Ok(Some(Note))`: 更新成功，返回更新后的笔记
/// - `Ok(None)`: 笔记不存在
/// - `Err(AppError)`: 更新失败
///
/// # 说明
/// 更新操作会自动创建历史记录
#[tauri::command]
pub async fn update_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    note: Note,
) -> Result<Option<Note>, AppError> {
    let db = &app_state.database_connection;
    service::note::update(db, &note).await.map_err(AppError::from)
}

/// 根据 ID 删除笔记
///
/// # 参数
/// - `id`: 笔记 ID
///
/// # 返回
/// - `Ok(())`: 删除成功
/// - `Err(AppError)`: 删除失败
///
/// # 说明
/// 删除操作会自动创建历史记录，并清理关联的标签关系
#[tauri::command]
pub async fn delete_note_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::note::delete_by_id(db, id).await.map_err(AppError::from)
}

/// 分页搜索笔记
///
/// # 参数
/// - `search_param`: 搜索参数，包含：
///   - `page_index`: 页码（从 1 开始）
///   - `page_size`: 每页数量
///   - `notebook_id`: 筛选笔记本 ID（0 表示不筛选）
///   - `tag_id`: 筛选标签 ID（0 表示不筛选）
///   - `keyword`: 搜索关键词（搜索标题和内容）
///
/// # 返回
/// - `Ok(NotePageResult)`: 搜索结果
/// - `Err(AppError)`: 搜索失败
#[tauri::command]
pub async fn search_page_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
    search_param: NoteSearchPageParam,
) -> Result<PageResult<Note>, AppError> {
    let db = &app_state.database_connection;
    service::note::search_page(db, &search_param).await.map_err(AppError::from)
}

#[tauri::command]
pub async fn note_stats(
    app_state: tauri::State<'_, Arc<AppState>>,
    search_param: NoteSearchPageParam,
) -> Result<NoteStatsResult, AppError> {
    let db = &app_state.database_connection;
    service::note::stats(db, &search_param).await.map_err(AppError::from)
}

// ============================================================================
// 历史记录相关命令
// ============================================================================

/// 分页搜索笔记历史记录
///
/// # 参数
/// - `search_param`: 搜索参数，包含：
///   - `page_index`: 页码（从 1 开始）
///   - `page_size`: 每页数量
///   - `note_id`: 笔记 ID
///
/// # 返回
/// - `Ok(PageResult<NoteHistory>)`: 历史记录分页结果
/// - `Err(AppError)`: 搜索失败
#[tauri::command]
pub async fn search_page_note_histories(
    app_state: tauri::State<'_, Arc<AppState>>,
    search_param: NoteHistorySearchPageParam,
) -> Result<PageResult<NoteHistory>, AppError> {
    let db = &app_state.database_connection;
    service::note_history::search_page(db, &search_param).await.map_err(AppError::from)
}
