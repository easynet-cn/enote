//! 笔记服务模块
//!
//! 本模块提供笔记相关的所有业务逻辑，是应用程序中最复杂的服务模块。
//!
//! # 功能概述
//! - 笔记的 CRUD 操作
//! - 笔记与标签的关联管理
//! - 笔记历史记录的自动生成
//! - 分页搜索功能（支持多条件过滤）
//!
//! # 事务处理
//! 创建、更新、删除操作都在事务中执行，确保数据一致性：
//! - 笔记数据与标签关联的原子操作
//! - 历史记录的同步写入
//!
//! # 历史记录类型
//! - OperationType::Create (1): 创建
//! - OperationType::Update (2): 更新
//! - OperationType::Delete (3): 删除

use std::collections::{HashMap, HashSet};

use crate::{
    entity::{self, notebook},
    model::{
        Note, NoteHistoryExtra, NoteSearchPageParam, NoteStatsResult, OperateSource,
        OperationType, PageResult, Tag,
    },
};
use chrono::Local;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, Condition, ConnectionTrait, DatabaseBackend, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, QuerySelect, Select, TransactionTrait,
    prelude::Expr,
    sea_query::{Asterisk, Query},
};

/// Build keyword filter condition based on database backend
fn apply_keyword_filter<E: EntityTrait>(
    builder: Select<E>,
    keyword: &str,
    is_sqlite: bool,
) -> Select<E> {
    if is_sqlite {
        let fts_keyword = keyword
            .replace('"', "\"\"")
            .replace('*', "")
            .replace('(', "")
            .replace(')', "")
            .replace('{', "")
            .replace('}', "")
            .replace('^', "")
            .replace(':', "");
        let fts_query = format!("\"{}\"", fts_keyword);
        // 使用 BM25 排序：标题权重 10，内容权重 1
        let fts_condition = Expr::cust_with_values(
            "id IN (SELECT rowid FROM note_fts WHERE note_fts MATCH ? ORDER BY bm25(note_fts, 10.0, 1.0))",
            [fts_query],
        );
        builder.filter(fts_condition)
    } else {
        builder.filter(
            Condition::all().add(
                Condition::any()
                    .add(entity::note::Column::Title.contains(keyword))
                    .add(entity::note::Column::Content.contains(keyword)),
            ),
        )
    }
}

/// Apply common search filters (notebook, tag, keyword) to a query builder
/// 默认排除已软删除的笔记
fn apply_search_filters<E: EntityTrait>(
    builder: Select<E>,
    search_param: &NoteSearchPageParam,
    is_sqlite: bool,
) -> Select<E> {
    let mut b = builder;
    // 排除已软删除的笔记
    b = b.filter(entity::note::Column::DeletedAt.is_null());
    if search_param.notebook_id > 0 {
        b = b.filter(entity::note::Column::NotebookId.eq(search_param.notebook_id));
    }
    if search_param.tag_id > 0 {
        let sub_query = Query::select()
            .column(entity::note_tags::Column::NoteId)
            .distinct()
            .and_where(Expr::col(entity::note_tags::Column::TagId).eq(search_param.tag_id))
            .from(entity::note_tags::Entity)
            .to_owned();
        b = b.filter(Condition::any().add(entity::note::Column::Id.in_subquery(sub_query)));
    }
    if !search_param.keyword.is_empty() {
        b = apply_keyword_filter(b, search_param.keyword.as_str(), is_sqlite);
    }
    b
}

/// 获取笔记关联的标签列表
async fn fetch_note_tags<C: ConnectionTrait>(db: &C, note_id: i64) -> anyhow::Result<Vec<Tag>> {
    Ok(entity::note_tags::Entity::find()
        .filter(entity::note_tags::Column::NoteId.eq(note_id))
        .find_also_related(entity::tag::Entity)
        .order_by_desc(entity::note_tags::Column::SortOrder)
        .all(db)
        .await?
        .into_iter()
        .filter_map(|(_, tag_opt)| tag_opt.map(|t| Tag::from(t)))
        .collect())
}

/// 根据 ID 查询笔记（优化版本：2 次查询代替 4 次）
pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<Option<Note>> {
    // 查询 1: 获取笔记和笔记本（使用 LEFT JOIN）
    let result = entity::note::Entity::find_by_id(id)
        .find_also_related(entity::notebook::Entity)
        .one(db)
        .await?;

    if let Some((note_entity, notebook_opt)) = result {
        let mut note = Note::from(note_entity);

        if let Some(notebook) = notebook_opt {
            note.notebook_name = notebook.name;
        }

        // 查询 2: 使用 JOIN 一次性获取所有标签（代替原来的 2 次查询）
        note.tags = fetch_note_tags(db, id).await?;

        Ok(Some(note))
    } else {
        Ok(None)
    }
}

/// 创建笔记
///
/// 在事务中执行以下操作：
/// 1. 插入笔记记录
/// 2. 创建笔记与标签的关联
/// 3. 记录创建历史（操作类型 1）
///
/// # 参数
/// - `db`: 数据库连接
/// - `note`: 笔记数据（包含标签列表）
///
/// # 返回
/// - `Ok(Some(Note))`: 创建成功，返回完整的笔记对象（含标签和笔记本信息）
/// - `Ok(None)`: 创建后未找到（异常情况）
/// - `Err`: 创建失败
///
/// # 事务安全
/// 所有操作在同一事务中执行，失败时自动回滚
pub async fn create(
    db: &DatabaseConnection,
    note: &Note,
    source: OperateSource,
) -> anyhow::Result<Option<Note>> {
    let txn = db.begin().await?;

    let now = Local::now().naive_local();

    let mut active_model: entity::note::ActiveModel = note.into();

    active_model.id = NotSet;
    active_model.is_pinned = Set(0);
    active_model.create_time = Set(now);
    active_model.update_time = Set(now);
    active_model.deleted_at = Set(None);

    let entity = active_model.insert(&txn).await?;

    if !note.tags.is_empty() {
        let note_tags = note
            .tags
            .iter()
            .map(|e| entity::note_tags::ActiveModel {
                id: NotSet,
                note_id: Set(entity.id),
                tag_id: Set(e.id),
                sort_order: Set(e.sort_order),
                create_time: Set(now),
                update_time: Set(now),
            })
            .collect::<Vec<entity::note_tags::ActiveModel>>();

        entity::note_tags::Entity::insert_many(note_tags)
            .exec(&txn)
            .await?;
    }

    let mut notebook_id = 0_i64;
    let mut notebook_name = String::default();

    if note.notebook_id > 0 {
        if let Some(notebook) = entity::notebook::Entity::find_by_id(note.notebook_id)
            .one(&txn)
            .await?
        {
            notebook_id = notebook.id;
            notebook_name = notebook.name.clone();
        }
    }

    let note_history_extra = NoteHistoryExtra {
        notebook_id,
        notebook_name: notebook_name.clone(),
        content_type: note.content_type,
        title: note.title.clone(),
        tags: note.tags.clone(),
    };

    let extra = serde_json::to_string(&note_history_extra)?;

    entity::note_history::ActiveModel {
        id: NotSet,
        note_id: Set(entity.id),
        old_content: Set(String::default()),
        new_content: Set(note.content.clone()),
        extra: Set(extra),
        operate_type: Set(OperationType::Create.as_i32()),
        operate_source: Set(source.as_i32()),
        operate_time: Set(now),
        create_time: Set(now),
    }
    .insert(&txn)
    .await?;

    txn.commit().await?;

    find_by_id(db, entity.id).await
}

/// 软删除笔记（移入回收站）
///
/// 设置 deleted_at 时间戳，不会真正删除数据
pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
    if let Some(entity) = entity::note::Entity::find_by_id(id).one(db).await? {
        let now = Local::now().naive_local();
        let mut active_model: entity::note::ActiveModel = entity.into_active_model();
        active_model.deleted_at = Set(Some(now));
        active_model.update_time = Set(now);
        active_model.update(db).await?;
    }
    Ok(())
}

/// 从回收站恢复笔记
pub async fn restore_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
    if let Some(entity) = entity::note::Entity::find_by_id(id).one(db).await? {
        let now = Local::now().naive_local();
        let mut active_model: entity::note::ActiveModel = entity.into_active_model();
        active_model.deleted_at = Set(None);
        active_model.update_time = Set(now);
        active_model.update(db).await?;
    }
    Ok(())
}

/// 永久删除笔记（从回收站中彻底删除）
pub async fn permanent_delete_by_id(
    db: &DatabaseConnection,
    id: i64,
    source: OperateSource,
) -> anyhow::Result<()> {
    let txn = db.begin().await?;

    let result = entity::note::Entity::find_by_id(id)
        .find_also_related(entity::notebook::Entity)
        .one(&txn)
        .await?;

    if let Some((entity, notebook_opt)) = result {
        let tags = fetch_note_tags(&txn, id).await?;

        let (notebook_id, notebook_name) = match notebook_opt {
            Some(notebook) => (notebook.id, notebook.name.clone()),
            None => (0, String::default()),
        };

        let now = Local::now().naive_local();

        let note_history_extra = NoteHistoryExtra {
            notebook_id,
            notebook_name,
            content_type: entity.content_type,
            title: entity.title.clone(),
            tags,
        };

        let extra = serde_json::to_string(&note_history_extra)?;

        entity::note_history::ActiveModel {
            id: NotSet,
            note_id: Set(id),
            old_content: Set(entity.content),
            new_content: Set(String::default()),
            extra: Set(extra),
            operate_type: Set(OperationType::Delete.as_i32()),
            operate_source: Set(source.as_i32()),
            operate_time: Set(now),
            create_time: Set(now),
        }
        .insert(&txn)
        .await?;

        entity::note_tags::Entity::delete_many()
            .filter(entity::note_tags::Column::NoteId.eq(id))
            .exec(&txn)
            .await?;
        entity::note::Entity::delete_by_id(id).exec(&txn).await?;

        txn.commit().await?;
    }

    Ok(())
}

/// 清空回收站（永久删除所有软删除的笔记）
pub async fn empty_trash(db: &DatabaseConnection) -> anyhow::Result<()> {
    let deleted_notes = entity::note::Entity::find()
        .filter(entity::note::Column::DeletedAt.is_not_null())
        .all(db)
        .await?;

    for note in deleted_notes {
        permanent_delete_by_id(db, note.id, OperateSource::User).await?;
    }

    Ok(())
}

/// 获取回收站中的笔记列表
pub async fn find_deleted(db: &DatabaseConnection) -> anyhow::Result<Vec<Note>> {
    let notes: Vec<Note> = entity::note::Entity::find()
        .filter(entity::note::Column::DeletedAt.is_not_null())
        .order_by_desc(entity::note::Column::UpdateTime)
        .all(db)
        .await?
        .into_iter()
        .map(Note::from)
        .collect();

    Ok(notes)
}

/// 切换笔记置顶状态
pub async fn toggle_pin(db: &DatabaseConnection, id: i64) -> anyhow::Result<Option<Note>> {
    if let Some(entity) = entity::note::Entity::find_by_id(id).one(db).await? {
        let now = Local::now().naive_local();
        let new_pinned = if entity.is_pinned == 1 { 0 } else { 1 };
        let mut active_model: entity::note::ActiveModel = entity.into_active_model();
        active_model.is_pinned = Set(new_pinned);
        active_model.update_time = Set(now);
        active_model.update(db).await?;
    }
    find_by_id(db, id).await
}

/// 更新笔记
///
/// 在事务中执行以下操作：
/// 1. 查询原笔记数据（用于历史记录和变更检测）
/// 2. 检测字段变更，仅更新有变化的字段
/// 3. 处理标签关联的变更（删除旧关联，创建新关联）
/// 4. 如有变更，记录更新历史（操作类型 2）
///
/// # 参数
/// - `db`: 数据库连接
/// - `note`: 包含更新数据的笔记对象
///
/// # 返回
/// - `Ok(Some(Note))`: 更新成功，返回更新后的完整笔记
/// - `Ok(None)`: 笔记不存在
/// - `Err`: 更新失败
///
/// # 智能更新
/// - 使用 `set_if_not_equals` 仅更新有变化的字段
/// - 标签变更时先清空再重建关联
/// - 只有实际发生变更时才记录历史
pub async fn update(
    db: &DatabaseConnection,
    note: &Note,
    source: OperateSource,
) -> anyhow::Result<Option<Note>> {
    if let Some(entity) = entity::note::Entity::find_by_id(note.id).one(db).await? {
        let old_title = entity.title.clone();

        let txn = db.begin().await?;

        // 获取旧标签关联 ID（用于差集计算）
        let old_tag_ids: Vec<i64> = entity::note_tags::Entity::find()
            .filter(entity::note_tags::Column::NoteId.eq(note.id))
            .order_by_desc(entity::note_tags::Column::SortOrder)
            .all(&txn)
            .await?
            .into_iter()
            .map(|nt| nt.tag_id)
            .collect();

        // 获取旧标签详情（用于历史记录）
        let old_tags = fetch_note_tags(&txn, note.id).await?;

        let old_content_type = entity.content_type;
        let old_content = entity.content.clone();

        let mut active_model: entity::note::ActiveModel = entity.into_active_model();

        active_model.notebook_id.set_if_not_equals(note.notebook_id);
        active_model.title.set_if_not_equals(note.title.clone());
        active_model.content.set_if_not_equals(note.content.clone());
        active_model
            .content_type
            .set_if_not_equals(note.content_type);

        let now = Local::now().naive_local();

        let note_changed = active_model.is_changed();

        if note_changed {
            active_model.update_time = Set(now);

            active_model.update(&txn).await?;
        }

        let new_tag_ids = note.tags.iter().map(|e| e.id).collect::<Vec<i64>>();

        let tags_changed: bool = old_tag_ids != new_tag_ids;

        if tags_changed {
            // 使用差集计算，只删除和添加变化的部分
            let old_set: HashSet<i64> = old_tag_ids.iter().cloned().collect();
            let new_set: HashSet<i64> = new_tag_ids.iter().cloned().collect();

            // 需要删除的标签：旧的有，新的没有
            let to_delete: Vec<i64> = old_set.difference(&new_set).cloned().collect();
            // 需要添加的标签：新的有，旧的没有
            let to_add: Vec<i64> = new_set.difference(&old_set).cloned().collect();

            // 删除不再需要的标签关联
            if !to_delete.is_empty() {
                entity::note_tags::Entity::delete_many()
                    .filter(entity::note_tags::Column::NoteId.eq(note.id))
                    .filter(entity::note_tags::Column::TagId.is_in(to_delete))
                    .exec(&txn)
                    .await?;
            }

            // 添加新的标签关联
            if !to_add.is_empty() {
                let new_note_tags = note
                    .tags
                    .iter()
                    .filter(|e| to_add.contains(&e.id))
                    .map(|e| entity::note_tags::ActiveModel {
                        id: NotSet,
                        note_id: Set(note.id),
                        tag_id: Set(e.id),
                        sort_order: Set(e.sort_order),
                        create_time: Set(now),
                        update_time: Set(now),
                    })
                    .collect::<Vec<entity::note_tags::ActiveModel>>();

                entity::note_tags::Entity::insert_many(new_note_tags)
                    .exec(&txn)
                    .await?;
            }
        }

        if note_changed || tags_changed {
            let mut notebook_id = 0_i64;
            let mut notebook_name = String::default();

            if note.notebook_id > 0 {
                if let Some(notebook) = entity::notebook::Entity::find_by_id(note.notebook_id)
                    .one(&txn)
                    .await?
                {
                    notebook_id = notebook.id;
                    notebook_name = notebook.name.clone();
                }
            }

            let note_history_extra = NoteHistoryExtra {
                notebook_id,
                notebook_name: notebook_name.clone(),
                content_type: old_content_type,
                title: old_title,
                tags: old_tags,
            };

            let extra = serde_json::to_string(&note_history_extra)?;

            entity::note_history::ActiveModel {
                id: NotSet,
                note_id: Set(note.id),
                old_content: Set(old_content),
                new_content: Set(note.content.clone()),
                extra: Set(extra),
                operate_type: Set(OperationType::Update.as_i32()),
                operate_source: Set(source.as_i32()),
                operate_time: Set(now),
                create_time: Set(now),
            }
            .insert(&txn)
            .await?;
        }

        txn.commit().await?;
    }

    find_by_id(db, note.id).await
}

/// 分页搜索笔记
///
/// 支持多条件组合搜索：
/// - 按笔记本 ID 过滤
/// - 按标签 ID 过滤（使用子查询）
/// - 按关键词搜索（标题和内容模糊匹配）
///
/// # 参数
/// - `db`: 数据库连接
/// - `search_param`: 搜索参数，包含：
///   - `page_param`: 分页参数（页码、每页数量）
///   - `notebook_id`: 笔记本 ID（0 表示不过滤）
///   - `tag_id`: 标签 ID（0 表示不过滤）
///   - `keyword`: 搜索关键词（空字符串表示不过滤）
///
/// # 返回
/// - `Ok(PageResult)`: 搜索结果，包含：
///   - 笔记列表（含标签信息）
///   - 分页信息
/// - `Err`: 搜索失败
///
/// # 查询优化
/// - 使用批量查询获取标签信息，避免 N+1 问题
/// - SQLite 数据库使用 FTS5 全文搜索（性能提升 10-100 倍）
pub async fn search_page(
    db: &DatabaseConnection,
    search_param: &NoteSearchPageParam,
) -> anyhow::Result<PageResult<Note>> {
    search_param.validate()?;

    let is_sqlite = db.get_database_backend() == DatabaseBackend::Sqlite;

    let count_builder = apply_search_filters(entity::note::Entity::find(), search_param, is_sqlite);
    let query_builder = apply_search_filters(entity::note::Entity::find(), search_param, is_sqlite);

    let total = count_builder
        .select_only()
        .column_as(Expr::col(Asterisk).count(), "count")
        .into_tuple::<i64>()
        .one(db)
        .await?
        .unwrap_or_default();

    if total > 0 {
        let start = search_param.page_param.start() as u64;
        let page_size = search_param.page_param.page_size as u64;

        let mut notes: Vec<Note> = query_builder
            .offset(start)
            .limit(page_size)
            .order_by_desc(entity::note::Column::IsPinned)
            .order_by_desc(entity::note::Column::UpdateTime)
            .order_by_desc(entity::note::Column::Id)
            .all(db)
            .await?
            .into_iter()
            .map(Note::from)
            .collect();

        let mut note_ids = Vec::<i64>::with_capacity(notes.len());
        let mut notebook_ids = HashSet::<i64>::new();

        for note in notes.iter() {
            note_ids.push(note.id);

            if note.notebook_id > 0 {
                notebook_ids.insert(note.notebook_id);
            }
        }

        if !note_ids.is_empty() {
            let mut notebook_map = HashMap::<i64, String>::with_capacity(notebook_ids.len());

            if !notebook_ids.is_empty() {
                notebook_map = notebook::Entity::find()
                    .filter(notebook::Column::Id.is_in(notebook_ids))
                    .all(db)
                    .await?
                    .into_iter()
                    .map(|e| (e.id, e.name))
                    .collect::<HashMap<i64, String>>();
            }

            for note in notes.iter_mut() {
                if let Some(notebook_name) = notebook_map.get(&note.notebook_id) {
                    note.notebook_name = notebook_name.clone();
                }
            }

            let note_tags = entity::note_tags::Entity::find()
                .filter(entity::note_tags::Column::NoteId.is_in(note_ids))
                .order_by_asc(entity::note_tags::Column::SortOrder)
                .order_by_asc(entity::note_tags::Column::Id)
                .all(db)
                .await?;

            if !note_tags.is_empty() {
                let mut note_tags_map = HashMap::<i64, Vec<i64>>::new();

                for note_tag in note_tags.iter() {
                    note_tags_map
                        .entry(note_tag.note_id)
                        .or_default()
                        .push(note_tag.tag_id);
                }

                let tag_ids = note_tags.iter().map(|e| e.tag_id).collect::<HashSet<i64>>();
                let tag_map = entity::tag::Entity::find()
                    .filter(entity::tag::Column::Id.is_in(tag_ids))
                    .all(db)
                    .await?
                    .into_iter()
                    .map(|e| {
                        let id = e.id;
                        (id, Tag::from(e))
                    })
                    .collect::<HashMap<i64, Tag>>();

                for note in notes.iter_mut() {
                    if let Some(tag_ids) = note_tags_map.get(&note.id) {
                        note.tags = tag_ids
                            .iter()
                            .filter_map(|id| tag_map.get(id).cloned())
                            .collect();
                    }
                }
            }
        }

        let mut page_result = PageResult::<Note>::with_data(total, notes);

        page_result.total_pages(search_param.page_param.page_size);

        return Ok(page_result);
    }

    Ok(PageResult::default())
}

pub async fn stats(
    db: &DatabaseConnection,
    search_param: &NoteSearchPageParam,
) -> anyhow::Result<NoteStatsResult> {
    search_param.validate()?;

    let is_sqlite = db.get_database_backend() == DatabaseBackend::Sqlite;

    let count_builder = apply_search_filters(entity::note::Entity::find(), search_param, is_sqlite);
    let count_map_builder = apply_search_filters(
        entity::note::Entity::find()
            .select_only()
            .column(entity::note::Column::NotebookId)
            .column_as(entity::note::Column::Id.count(), "n"),
        search_param,
        is_sqlite,
    );

    let total = count_builder
        .select_only()
        .column_as(Expr::col(Asterisk).count(), "count")
        .into_tuple::<i64>()
        .one(db)
        .await?
        .unwrap_or_default();

    if total > 0 {
        let notebook_counts = count_map_builder
            .group_by(entity::note::Column::NotebookId)
            .into_tuple::<(i64, i64)>()
            .all(db)
            .await?
            .into_iter()
            .collect::<HashMap<i64, i64>>();

        return Ok(NoteStatsResult {
            total,
            notebook_counts,
        });
    }

    Ok(NoteStatsResult::default())
}
