//! 笔记历史服务模块
//!
//! 本模块提供笔记历史记录的查询功能。
//!
//! # 功能概述
//! - 分页查询指定笔记的历史记录
//!
//! # 历史记录说明
//! 历史记录由笔记服务在以下操作时自动创建：
//! - 创建笔记（操作类型 1）
//! - 更新笔记（操作类型 2）
//! - 删除笔记（操作类型 3）
//!
//! 每条历史记录包含：
//! - 操作前后的内容差异
//! - 操作时的笔记元信息（标题、笔记本、标签）

use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
    prelude::Expr, sea_query::Asterisk,
};

use crate::{
    entity::{self},
    model::{NoteHistory, NoteHistorySearchPageParam, PageResult},
};

/// 分页搜索笔记历史记录
///
/// # 参数
/// - `db`: 数据库连接
/// - `search_param`: 搜索参数，包含：
///   - `page_param`: 分页参数（页码、每页数量）
///   - `note_id`: 笔记 ID（必填，用于过滤指定笔记的历史）
///
/// # 返回
/// - `Ok(PageResult<NoteHistory>)`: 分页结果，包含历史记录列表和分页信息
/// - `Err`: 查询失败
///
/// # 排序
/// 按 ID 降序排列（即最新的历史记录在前）
pub async fn search_page(
    db: &DatabaseConnection,
    search_param: &NoteHistorySearchPageParam,
) -> anyhow::Result<PageResult<NoteHistory>> {
    // 构建计数查询和数据查询
    let mut count_builder = entity::note_history::Entity::find();
    let mut query_builder = entity::note_history::Entity::find();

    // 按笔记 ID 过滤
    if search_param.note_id > 0 {
        count_builder =
            count_builder.filter(entity::note_history::Column::NoteId.eq(search_param.note_id));
        query_builder =
            query_builder.filter(entity::note_history::Column::NoteId.eq(search_param.note_id));
    }

    // 查询总数
    let total = count_builder
        .select_only()
        .column_as(Expr::col(Asterisk).count(), "count")
        .into_tuple::<i64>()
        .one(db)
        .await?
        .unwrap_or_default();

    // 有数据时查询分页结果
    if total > 0 {
        let start = search_param.page_param.start() as u64;
        let page_size = search_param.page_param.page_size as u64;
        let histories: Vec<NoteHistory> = query_builder
            .offset(start)
            .limit(page_size)
            .order_by_desc(entity::note_history::Column::Id)
            .all(db)
            .await?
            .iter()
            .map(NoteHistory::from)
            .collect();

        let mut page_result = PageResult::<NoteHistory>::with_data(total, histories);

        page_result.total_pages(search_param.page_param.page_size);

        return Ok(page_result);
    }

    // 无数据时返回空结果
    Ok(PageResult::<NoteHistory>::default())
}
