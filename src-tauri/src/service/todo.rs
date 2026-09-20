//! 待办服务
//!
//! 提供待办（todo）和待办清单（todo_list）的 CRUD、查询、软删除/恢复功能。
//!
//! # 跨数据库兼容说明
//! - `description` 为 `text` 列且 DB 层无 DEFAULT（MySQL 不允许），
//!   因此所有插入路径都必须显式赋值，不能依赖数据库默认值。
//! - 布尔语义字段（`is_completed` / `is_reminded`）统一使用 `i32`（0/1）。
//! - 关键字搜索统一使用 `LIKE`，不依赖 SQLite 专有的 FTS5。
//! - 按 `due_date` 排序时，先按 `due_date IS NULL` 排序，避免不同数据库
//!   对 NULL 排序位置不一致（MySQL 与 PostgreSQL 行为相反）。

use anyhow::{Context, Result};
use chrono::{Duration, Local, Months, NaiveDateTime};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, Order, PaginatorTrait, QueryFilter, QueryOrder,
};
use tracing::info;

use crate::entity;
use crate::model::{PageResult, Todo, TodoList, TodoSearchParam, TodoStats};

/// 获取下一个排序值（新待办默认排在最前）
///
/// 列表按 `sort_order DESC` 排序，因此取当前最大值 + 1。
async fn next_sort_order(db: &DatabaseConnection) -> Result<i32> {
    let max_sort = entity::todo::Entity::find()
        .order_by_desc(entity::todo::Column::SortOrder)
        .one(db)
        .await?
        .map(|m| m.sort_order)
        .unwrap_or(0);
    Ok(max_sort + 1)
}

/// 构造查询条件并应用过滤、排序
fn build_todo_query(
    param: &TodoSearchParam,
    tag_todo_ids: Option<Vec<i64>>,
) -> sea_orm::Select<entity::todo::Entity> {
    let mut query = entity::todo::Entity::find();

    // 标签过滤（ID 列表由调用方从 todo_tags 关联表查出，避免子查询的跨库差异）
    if let Some(ids) = tag_todo_ids {
        query = query.filter(entity::todo::Column::Id.is_in(ids));
    }

    // 软删除过滤：默认只查未删除，deleted = true 时只查回收站
    if param.deleted {
        query = query.filter(entity::todo::Column::DeletedAt.is_not_null());
    } else {
        query = query.filter(entity::todo::Column::DeletedAt.is_null());
    }

    // 完成状态过滤
    if let Some(completed) = param.completed {
        query = query.filter(entity::todo::Column::IsCompleted.eq(if completed { 1 } else { 0 }));
    }

    // 清单过滤
    if let Some(list_id) = param.list_id {
        query = query.filter(entity::todo::Column::ListId.eq(list_id));
    }

    // 优先级过滤
    if let Some(priority) = param.priority {
        query = query.filter(entity::todo::Column::Priority.eq(priority));
    }

    // 智能视图：today / planned / overdue 基于 due_date 计算
    match param.view.as_str() {
        "today" => {
            let now = Local::now().naive_local();
            let today = now.date();
            if let (Some(start), Some(end)) = (
                today.and_hms_opt(0, 0, 0),
                today.and_hms_opt(23, 59, 59),
            ) {
                query = query.filter(entity::todo::Column::DueDate.between(start, end));
            }
        }
        "planned" => {
            let now = Local::now().naive_local();
            if let Some(end) = now.date().and_hms_opt(23, 59, 59) {
                query = query.filter(entity::todo::Column::DueDate.gt(end));
            }
        }
        "overdue" => {
            let now = Local::now().naive_local();
            if let Some(start) = now.date().and_hms_opt(0, 0, 0) {
                query = query
                    .filter(entity::todo::Column::DueDate.lt(start))
                    .filter(entity::todo::Column::IsCompleted.eq(0));
            }
        }
        _ => {}
    }

    // 关键字搜索：跨库统一使用 LIKE
    let keyword = param.keyword.trim();
    if !keyword.is_empty() {
        query = query.filter(
            sea_orm::Condition::any()
                .add(entity::todo::Column::Title.contains(keyword))
                .add(entity::todo::Column::Description.contains(keyword)),
        );
    }

    // 排序
    let desc = param.sort_order.eq_ignore_ascii_case("desc");
    let order = if desc { Order::Desc } else { Order::Asc };

    match param.sort_field.as_str() {
        "dueDate" => {
            // 先按 NULL 排序，保证 NULL 值在三库中都排在最后
            query = query
                .order_by(entity::todo::Column::DueDate.is_null(), Order::Asc)
                .order_by(entity::todo::Column::DueDate, order);
        }
        "priority" => {
            query = query
                .order_by(entity::todo::Column::Priority, order)
                .order_by(entity::todo::Column::SortOrder, Order::Desc);
        }
        "createTime" => {
            query = query.order_by(entity::todo::Column::CreateTime, order);
        }
        // manual（默认）：手动排序
        _ => {
            query = query
                .order_by(entity::todo::Column::SortOrder, Order::Desc)
                .order_by(entity::todo::Column::Id, Order::Desc);
        }
    }

    query
}

/// 查询待办列表
///
/// # 参数
/// - `db`: 数据库连接
/// - `param`: 查询参数（关键字、清单、完成状态、视图、排序）
///
/// # 返回
/// 符合条件的待办列表
/// 解析标签过滤对应的待办 ID 列表
async fn resolve_tag_todo_ids(
    db: &DatabaseConnection,
    tag_id: Option<i64>,
) -> Result<Option<Vec<i64>>> {
    match tag_id {
        Some(tag_id) => {
            let rows = entity::todo_tags::Entity::find()
                .filter(entity::todo_tags::Column::TagId.eq(tag_id))
                .all(db)
                .await?;
            Ok(Some(rows.into_iter().map(|r| r.todo_id).collect::<Vec<i64>>()))
        }
        None => Ok(None),
    }
}

/// 查询待办列表（不分页，返回全部匹配项）
pub async fn search(db: &DatabaseConnection, param: &TodoSearchParam) -> Result<Vec<Todo>> {
    let tag_todo_ids = resolve_tag_todo_ids(db, param.tag_id).await?;
    let models = build_todo_query(param, tag_todo_ids).all(db).await?;
    Ok(models.into_iter().map(Todo::from).collect())
}

/// 分页查询待办
///
/// 供 MCP 等外部调用方使用，避免一次拉取全量数据。
/// `page_index` / `page_size` 为非法值时回退为默认值（第 1 页、每页 20 条，上限 50）。
pub async fn search_page(
    db: &DatabaseConnection,
    param: &TodoSearchParam,
) -> Result<PageResult<Todo>> {
    let page_index = if param.page_index < 1 {
        1
    } else {
        param.page_index
    } as u64;
    let page_size = if param.page_size < 1 {
        20
    } else {
        param.page_size.min(50)
    } as u64;

    let tag_todo_ids = resolve_tag_todo_ids(db, param.tag_id).await?;
    let paginator = build_todo_query(param, tag_todo_ids).paginate(db, page_size);

    let total = paginator.num_items().await? as i64;
    let total_pages = paginator.num_pages().await? as i64;
    let models = paginator.fetch_page(page_index - 1).await?;

    Ok(PageResult {
        total,
        total_pages,
        data: models.into_iter().map(Todo::from).collect(),
    })
}

/// 根据 ID 查询待办
pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> Result<Option<Todo>> {
    let model = entity::todo::Entity::find_by_id(id).one(db).await?;
    Ok(model.map(Todo::from))
}

/// 创建待办
///
/// # 参数
/// - `db`: 数据库连接
/// - `dto`: 待办数据（id 由数据库生成）
///
/// # 返回
/// 创建后的待办
pub async fn create(db: &DatabaseConnection, dto: &Todo) -> Result<Todo> {
    let title = dto.title.trim();
    if title.is_empty() {
        anyhow::bail!("Todo title cannot be empty");
    }

    let now = Local::now().naive_local();
    let sort_order = if dto.sort_order != 0 {
        dto.sort_order
    } else {
        next_sort_order(db).await?
    };
    let is_completed = if dto.is_completed == 1 { 1 } else { 0 };

    let model = entity::todo::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(title.to_string()),
        // text 列在 MySQL 下无 DEFAULT，必须显式赋值
        description: ActiveValue::Set(dto.description.clone()),
        is_completed: ActiveValue::Set(is_completed),
        priority: ActiveValue::Set(dto.priority.clamp(0, 3)),
        due_date: ActiveValue::Set(dto.due_date),
        completed_at: ActiveValue::Set(if is_completed == 1 { Some(now) } else { None }),
        list_id: ActiveValue::Set(dto.list_id),
        sort_order: ActiveValue::Set(sort_order),
        create_time: ActiveValue::Set(now),
        update_time: ActiveValue::Set(now),
        deleted_at: ActiveValue::Set(None),
        // P1 字段
        start_date: ActiveValue::Set(dto.start_date),
        remind_at: ActiveValue::Set(dto.remind_at),
        is_reminded: ActiveValue::Set(0),
        parent_id: ActiveValue::Set(dto.parent_id),
        note_id: ActiveValue::Set(dto.note_id),
        recurrence_type: ActiveValue::Set(dto.recurrence_type.clamp(0, 4)),
        recurrence_interval: ActiveValue::Set(if dto.recurrence_interval < 1 {
            1
        } else {
            dto.recurrence_interval
        }),
        recurrence_end_date: ActiveValue::Set(dto.recurrence_end_date),
        mcp_access: ActiveValue::Set(dto.mcp_access),
    };

    let entity = model
        .insert(db)
        .await
        .context("Failed to create todo")?;
    info!("Todo created: id={}, title={}", entity.id, entity.title);
    Ok(Todo::from(entity))
}

/// 更新待办
///
/// # 参数
/// - `db`: 数据库连接
/// - `dto`: 待办数据（需带 id）
///
/// # 返回
/// 更新后的待办，不存在时返回 None
pub async fn update(db: &DatabaseConnection, dto: &Todo) -> Result<Option<Todo>> {
    let Some(existing) = entity::todo::Entity::find_by_id(dto.id).one(db).await? else {
        return Ok(None);
    };

    let title = dto.title.trim();
    if title.is_empty() {
        anyhow::bail!("Todo title cannot be empty");
    }

    let mut am: entity::todo::ActiveModel = existing.clone().into_active_model();
    am.title = ActiveValue::Set(title.to_string());
    am.description = ActiveValue::Set(dto.description.clone());
    am.priority = ActiveValue::Set(dto.priority.clamp(0, 3));
    am.due_date = ActiveValue::Set(dto.due_date);
    am.list_id = ActiveValue::Set(dto.list_id);
    if dto.sort_order != 0 {
        am.sort_order = ActiveValue::Set(dto.sort_order);
    }
    am.update_time = ActiveValue::Set(Local::now().naive_local());

    // P1 字段
    am.start_date = ActiveValue::Set(dto.start_date);
    am.remind_at = ActiveValue::Set(dto.remind_at);
    am.parent_id = ActiveValue::Set(dto.parent_id);
    am.note_id = ActiveValue::Set(dto.note_id);
    am.recurrence_type = ActiveValue::Set(dto.recurrence_type.clamp(0, 4));
    am.recurrence_interval = ActiveValue::Set(if dto.recurrence_interval < 1 {
        1
    } else {
        dto.recurrence_interval
    });
    am.recurrence_end_date = ActiveValue::Set(dto.recurrence_end_date);
    am.mcp_access = ActiveValue::Set(dto.mcp_access);

    let updated = am.update(db).await.context("Failed to update todo")?;
    info!("Todo updated: id={}", dto.id);
    Ok(Some(Todo::from(updated)))
}

/// 切换待办完成状态
///
/// 完成时写入 `completed_at`，取消完成时清空。
///
/// # 重复任务
/// 若待办设置了重复规则且本次为"完成"操作，则不真正完成，而是将
/// `due_date` 顺延到下一个周期，并重置完成状态与提醒标记。
/// 当顺延后的日期超过 `recurrence_end_date` 时，视为最后一次，正常完成。
pub async fn toggle_complete(db: &DatabaseConnection, id: i64) -> Result<Option<Todo>> {
    let Some(existing) = entity::todo::Entity::find_by_id(id).one(db).await? else {
        return Ok(None);
    };

    let now = Local::now().naive_local();
    let new_completed = if existing.is_completed == 1 { 0 } else { 1 };

    // 重复任务：完成时顺延而非真正完成
    let mut next_due: Option<NaiveDateTime> = None;
    if new_completed == 1 && existing.recurrence_type > 0 {
        if let Some(due) = existing.due_date {
            if let Some(next) =
                next_due_date(due, existing.recurrence_type, existing.recurrence_interval)
            {
                let within_end = match existing.recurrence_end_date {
                    Some(end) => next <= end,
                    None => true,
                };
                if within_end {
                    next_due = Some(next);
                }
            }
        }
    }

    let mut am: entity::todo::ActiveModel = existing.into_active_model();
    match next_due {
        Some(next) => {
            // 顺延：保持未完成，推进截止日期并重置提醒
            am.due_date = ActiveValue::Set(Some(next));
            am.is_completed = ActiveValue::Set(0);
            am.completed_at = ActiveValue::Set(None);
            am.is_reminded = ActiveValue::Set(0);
        }
        None => {
            am.is_completed = ActiveValue::Set(new_completed);
            am.completed_at = ActiveValue::Set(if new_completed == 1 { Some(now) } else { None });
        }
    }
    am.update_time = ActiveValue::Set(now);

    let updated = am
        .update(db)
        .await
        .context("Failed to toggle todo completion")?;
    info!(
        "Todo completion toggled: id={}, repeated={}",
        id,
        next_due.is_some()
    );
    Ok(Some(Todo::from(updated)))
}

/// 软删除待办（移入回收站）
pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> Result<()> {
    let Some(existing) = entity::todo::Entity::find_by_id(id).one(db).await? else {
        return Ok(());
    };

    let mut am: entity::todo::ActiveModel = existing.into_active_model();
    am.deleted_at = ActiveValue::Set(Some(Local::now().naive_local()));
    am.update_time = ActiveValue::Set(Local::now().naive_local());
    am.update(db).await.context("Failed to delete todo")?;
    info!("Todo soft deleted: id={}", id);
    Ok(())
}

/// 恢复回收站中的待办
pub async fn restore(db: &DatabaseConnection, id: i64) -> Result<()> {
    let Some(existing) = entity::todo::Entity::find_by_id(id).one(db).await? else {
        return Ok(());
    };

    let mut am: entity::todo::ActiveModel = existing.into_active_model();
    am.deleted_at = ActiveValue::Set(None);
    am.update_time = ActiveValue::Set(Local::now().naive_local());
    am.update(db).await.context("Failed to restore todo")?;
    info!("Todo restored: id={}", id);
    Ok(())
}

/// 彻底删除待办（物理删除）
pub async fn permanent_delete(db: &DatabaseConnection, id: i64) -> Result<()> {
    entity::todo::Entity::delete_by_id(id)
        .exec(db)
        .await
        .context("Failed to permanently delete todo")?;
    info!("Todo permanently deleted: id={}", id);
    Ok(())
}

/// 清空回收站（彻底删除所有软删除的待办）
pub async fn empty_trash(db: &DatabaseConnection) -> Result<u64> {
    let result = entity::todo::Entity::delete_many()
        .filter(entity::todo::Column::DeletedAt.is_not_null())
        .exec(db)
        .await
        .context("Failed to empty todo trash")?;
    info!("Todo trash emptied: {} deleted", result.rows_affected);
    Ok(result.rows_affected)
}

/// 计算重复任务的下一个截止日期
///
/// # 参数
/// - `current`: 当前截止日期
/// - `recurrence_type`: 重复类型（1 = 每天，2 = 每周，3 = 每月，4 = 每年）
/// - `interval`: 重复间隔（每 N 个周期，最小为 1）
fn next_due_date(
    current: NaiveDateTime,
    recurrence_type: i32,
    interval: i32,
) -> Option<NaiveDateTime> {
    let interval = if interval < 1 { 1 } else { interval } as u32;
    match recurrence_type {
        1 => current.checked_add_signed(Duration::days(interval as i64)),
        2 => current.checked_add_signed(Duration::weeks(interval as i64)),
        3 => current.checked_add_months(Months::new(interval)),
        4 => current.checked_add_months(Months::new(interval.saturating_mul(12))),
        _ => None,
    }
}

/// 查询到期待提醒的待办
///
/// 条件：未完成、未提醒、提醒时刻已到、未软删除。
pub async fn find_pending_reminders(db: &DatabaseConnection) -> Result<Vec<Todo>> {
    let now = Local::now().naive_local();
    let models = entity::todo::Entity::find()
        .filter(entity::todo::Column::IsCompleted.eq(0))
        .filter(entity::todo::Column::IsReminded.eq(0))
        .filter(entity::todo::Column::RemindAt.is_not_null())
        .filter(entity::todo::Column::RemindAt.lte(now))
        .filter(entity::todo::Column::DeletedAt.is_null())
        .all(db)
        .await?;
    Ok(models.into_iter().map(Todo::from).collect())
}

/// 标记待办已提醒
pub async fn mark_reminded(db: &DatabaseConnection, id: i64) -> Result<()> {
    let Some(existing) = entity::todo::Entity::find_by_id(id).one(db).await? else {
        return Ok(());
    };

    let mut am: entity::todo::ActiveModel = existing.into_active_model();
    am.is_reminded = ActiveValue::Set(1);
    am.update(db)
        .await
        .context("Failed to mark todo as reminded")?;
    Ok(())
}

/// 批量更新待办排序
///
/// # 参数
/// - `db`: 数据库连接
/// - `orders`: (待办 ID, 新的排序值) 列表
pub async fn reorder(db: &DatabaseConnection, orders: Vec<(i64, i32)>) -> Result<()> {
    let count = orders.len();
    let now = Local::now().naive_local();

    for (id, sort_order) in orders {
        let Some(existing) = entity::todo::Entity::find_by_id(id).one(db).await? else {
            continue;
        };

        let mut am: entity::todo::ActiveModel = existing.into_active_model();
        am.sort_order = ActiveValue::Set(sort_order);
        am.update_time = ActiveValue::Set(now);
        am.update(db).await.context("Failed to reorder todo")?;
    }

    info!("Todos reordered: {} items", count);
    Ok(())
}

// ============================================================================
// 待办标签
// ============================================================================

/// 查询待办关联的标签 ID 列表
pub async fn find_tag_ids_by_todo(db: &DatabaseConnection, todo_id: i64) -> Result<Vec<i64>> {
    let rows = entity::todo_tags::Entity::find()
        .filter(entity::todo_tags::Column::TodoId.eq(todo_id))
        .all(db)
        .await?;
    Ok(rows.into_iter().map(|r| r.tag_id).collect())
}

/// 设置待办关联的标签（全量替换）
///
/// # 参数
/// - `db`: 数据库连接
/// - `todo_id`: 待办 ID
/// - `tag_ids`: 标签 ID 列表，传空表示清空所有标签
pub async fn set_todo_tags(db: &DatabaseConnection, todo_id: i64, tag_ids: Vec<i64>) -> Result<()> {
    let count = tag_ids.len();

    // 先删除现有关联
    entity::todo_tags::Entity::delete_many()
        .filter(entity::todo_tags::Column::TodoId.eq(todo_id))
        .exec(db)
        .await?;

    // 批量插入新关联
    if !tag_ids.is_empty() {
        let now = Local::now().naive_local();
        let models: Vec<entity::todo_tags::ActiveModel> = tag_ids
            .into_iter()
            .map(|tag_id| entity::todo_tags::ActiveModel {
                id: ActiveValue::NotSet,
                todo_id: ActiveValue::Set(todo_id),
                tag_id: ActiveValue::Set(tag_id),
                create_time: ActiveValue::Set(now),
            })
            .collect();
        entity::todo_tags::Entity::insert_many(models)
            .exec(db)
            .await
            .context("Failed to set todo tags")?;
    }

    info!("Todo tags updated: todo_id={}, count={}", todo_id, count);
    Ok(())
}

// ============================================================================
// 待办清单
// ============================================================================

/// 查询所有待办清单
pub async fn find_all_lists(db: &DatabaseConnection) -> Result<Vec<TodoList>> {
    let models = entity::todo_list::Entity::find()
        .order_by_desc(entity::todo_list::Column::SortOrder)
        .order_by_desc(entity::todo_list::Column::UpdateTime)
        .all(db)
        .await?;
    Ok(models.into_iter().map(TodoList::from).collect())
}

/// 创建待办清单
pub async fn create_list(db: &DatabaseConnection, dto: &TodoList) -> Result<TodoList> {
    let name = dto.name.trim();
    if name.is_empty() {
        anyhow::bail!("Todo list name cannot be empty");
    }

    let now = Local::now().naive_local();
    let max_sort = entity::todo_list::Entity::find()
        .order_by_desc(entity::todo_list::Column::SortOrder)
        .one(db)
        .await?
        .map(|m| m.sort_order)
        .unwrap_or(0);

    let model = entity::todo_list::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(name.to_string()),
        icon: ActiveValue::Set(dto.icon.clone()),
        color: ActiveValue::Set(dto.color.clone()),
        sort_order: ActiveValue::Set(max_sort + 1),
        mcp_access: ActiveValue::Set(dto.mcp_access),
        create_time: ActiveValue::Set(now),
        update_time: ActiveValue::Set(now),
    };

    let entity = model
        .insert(db)
        .await
        .context("Failed to create todo list")?;
    info!("Todo list created: id={}, name={}", entity.id, entity.name);
    Ok(TodoList::from(entity))
}

/// 更新待办清单
pub async fn update_list(db: &DatabaseConnection, dto: &TodoList) -> Result<Option<TodoList>> {
    let Some(existing) = entity::todo_list::Entity::find_by_id(dto.id).one(db).await? else {
        return Ok(None);
    };

    let name = dto.name.trim();
    if name.is_empty() {
        anyhow::bail!("Todo list name cannot be empty");
    }

    let mut am: entity::todo_list::ActiveModel = existing.into_active_model();
    am.name = ActiveValue::Set(name.to_string());
    am.icon = ActiveValue::Set(dto.icon.clone());
    am.color = ActiveValue::Set(dto.color.clone());
    am.mcp_access = ActiveValue::Set(dto.mcp_access);
    am.update_time = ActiveValue::Set(Local::now().naive_local());

    let updated = am
        .update(db)
        .await
        .context("Failed to update todo list")?;
    info!("Todo list updated: id={}", dto.id);
    Ok(Some(TodoList::from(updated)))
}

/// 删除待办清单
///
/// 清单下的待办不会删除，而是解除关联（`list_id` 置为 NULL）。
pub async fn delete_list(db: &DatabaseConnection, id: i64) -> Result<()> {
    // 先解除该清单下所有待办的关联
    entity::todo::Entity::update_many()
        .col_expr(
            entity::todo::Column::ListId,
            Expr::value(Option::<i64>::None),
        )
        .filter(entity::todo::Column::ListId.eq(id))
        .exec(db)
        .await
        .context("Failed to unlink todos from list")?;

    entity::todo_list::Entity::delete_by_id(id)
        .exec(db)
        .await
        .context("Failed to delete todo list")?;
    info!("Todo list deleted: id={}", id);
    Ok(())
}

// ============================================================================
// 批量操作
// ============================================================================

/// 批量切换完成状态
///
/// # 返回
/// 实际处理的待办数量
pub async fn batch_toggle(db: &DatabaseConnection, ids: Vec<i64>) -> Result<u64> {
    let now = Local::now().naive_local();
    let mut count = 0u64;

    for id in ids {
        let Some(existing) = entity::todo::Entity::find_by_id(id).one(db).await? else {
            continue;
        };

        let new_completed = if existing.is_completed == 1 { 0 } else { 1 };
        let mut am: entity::todo::ActiveModel = existing.into_active_model();
        am.is_completed = ActiveValue::Set(new_completed);
        am.completed_at = ActiveValue::Set(if new_completed == 1 { Some(now) } else { None });
        am.update_time = ActiveValue::Set(now);
        am.update(db)
            .await
            .context("Failed to batch toggle todo")?;
        count += 1;
    }

    info!("Todos batch toggled: {} items", count);
    Ok(count)
}

/// 批量软删除（移入回收站）
pub async fn batch_delete(db: &DatabaseConnection, ids: Vec<i64>) -> Result<u64> {
    let now = Local::now().naive_local();
    let mut count = 0u64;

    for id in ids {
        let Some(existing) = entity::todo::Entity::find_by_id(id).one(db).await? else {
            continue;
        };

        let mut am: entity::todo::ActiveModel = existing.into_active_model();
        am.deleted_at = ActiveValue::Set(Some(now));
        am.update_time = ActiveValue::Set(now);
        am.update(db)
            .await
            .context("Failed to batch delete todo")?;
        count += 1;
    }

    info!("Todos batch deleted: {} items", count);
    Ok(count)
}

/// 批量移动待办到指定清单
///
/// # 参数
/// - `list_id`: 目标清单 ID，None 表示移出清单（设为未归类）
pub async fn batch_move(
    db: &DatabaseConnection,
    ids: Vec<i64>,
    list_id: Option<i64>,
) -> Result<u64> {
    let now = Local::now().naive_local();
    let mut count = 0u64;

    for id in ids {
        let Some(existing) = entity::todo::Entity::find_by_id(id).one(db).await? else {
            continue;
        };

        let mut am: entity::todo::ActiveModel = existing.into_active_model();
        am.list_id = ActiveValue::Set(list_id);
        am.update_time = ActiveValue::Set(now);
        am.update(db)
            .await
            .context("Failed to batch move todo")?;
        count += 1;
    }

    info!("Todos batch moved: {} items", count);
    Ok(count)
}

/// 统计待办概况
///
/// 统计范围：未软删除的待办。完成率 = 已完成 / 总数 × 100。
pub async fn stats(db: &DatabaseConnection) -> Result<TodoStats> {
    let now = Local::now().naive_local();
    let today = now.date();

    let total = entity::todo::Entity::find()
        .filter(entity::todo::Column::DeletedAt.is_null())
        .count(db)
        .await?;

    let completed = entity::todo::Entity::find()
        .filter(entity::todo::Column::DeletedAt.is_null())
        .filter(entity::todo::Column::IsCompleted.eq(1))
        .count(db)
        .await?;

    let mut today_count = 0u64;
    let mut overdue_count = 0u64;

    if let (Some(start), Some(end)) = (today.and_hms_opt(0, 0, 0), today.and_hms_opt(23, 59, 59)) {
        today_count = entity::todo::Entity::find()
            .filter(entity::todo::Column::DeletedAt.is_null())
            .filter(entity::todo::Column::IsCompleted.eq(0))
            .filter(entity::todo::Column::DueDate.between(start, end))
            .count(db)
            .await?;

        overdue_count = entity::todo::Entity::find()
            .filter(entity::todo::Column::DeletedAt.is_null())
            .filter(entity::todo::Column::IsCompleted.eq(0))
            .filter(entity::todo::Column::DueDate.lt(start))
            .count(db)
            .await?;
    }

    let pending_count = total.saturating_sub(completed);
    let completion_rate = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    Ok(TodoStats {
        total_count: total,
        completed_count: completed,
        pending_count,
        today_count,
        overdue_count,
        completion_rate,
    })
}
