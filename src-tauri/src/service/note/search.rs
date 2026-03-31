use std::collections::{HashMap, HashSet};

use crate::{
    entity::{self, notebook},
    model::{Note, NoteSearchPageParam, NoteStatsResult, PageResult, Tag},
};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DatabaseBackend, DatabaseConnection, EntityTrait,
    Order, QueryFilter, QueryOrder, QuerySelect, Select,
    prelude::Expr,
    sea_query::{Asterisk, Query},
};

use super::crypto_helper::decrypt_note;

/// Build keyword filter condition based on database backend
///
/// SQLite: 使用 FTS5 全文搜索（title + content + tags）
/// MySQL/PG: 使用 LIKE 搜索 title、content 和 tag name
pub(super) fn apply_keyword_filter<E: EntityTrait>(
    builder: Select<E>,
    keyword: &str,
    is_sqlite: bool,
) -> Select<E> {
    if is_sqlite {
        let fts_keyword = keyword
            .replace('"', "\"\"")
            .replace(['*', '(', ')', '{', '}', '^', ':'], "");
        let fts_query = format!("\"{}\"", fts_keyword);
        // 使用 BM25 排序：标题权重 10，内容权重 1，标签权重 5
        let fts_condition = Expr::cust_with_values(
            "id IN (SELECT rowid FROM note_fts WHERE note_fts MATCH ? ORDER BY bm25(note_fts, 10.0, 1.0, 5.0))",
            [fts_query],
        );
        builder.filter(fts_condition)
    } else {
        // MySQL/PG: LIKE 搜索 title、content，加上 tag name 子查询
        let tag_subquery = Query::select()
            .column(entity::note_tags::Column::NoteId)
            .distinct()
            .from(entity::note_tags::Entity)
            .inner_join(
                entity::tag::Entity,
                Expr::col((entity::tag::Entity, entity::tag::Column::Id))
                    .equals((entity::note_tags::Entity, entity::note_tags::Column::TagId)),
            )
            .and_where(entity::tag::Column::Name.contains(keyword))
            .to_owned();

        builder.filter(
            Condition::all().add(
                Condition::any()
                    .add(entity::note::Column::Title.contains(keyword))
                    .add(entity::note::Column::Content.contains(keyword))
                    .add(entity::note::Column::Id.in_subquery(tag_subquery)),
            ),
        )
    }
}

/// Apply common search filters (notebook, tag, keyword) to a query builder
/// 默认排除已软删除的笔记
pub(super) fn apply_search_filters<E: EntityTrait>(
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
    if search_param.is_starred {
        b = b.filter(entity::note::Column::IsStarred.eq(1));
    }
    if !search_param.keyword.is_empty() {
        b = apply_keyword_filter(b, search_param.keyword.as_str(), is_sqlite);
    }
    b
}

/// 获取笔记关联的标签列表
pub(super) async fn fetch_note_tags<C: ConnectionTrait>(
    db: &C,
    note_id: i64,
) -> anyhow::Result<Vec<Tag>> {
    Ok(entity::note_tags::Entity::find()
        .filter(entity::note_tags::Column::NoteId.eq(note_id))
        .find_also_related(entity::tag::Entity)
        .order_by_desc(entity::note_tags::Column::SortOrder)
        .all(db)
        .await?
        .into_iter()
        .filter_map(|(_, tag_opt)| tag_opt.map(Tag::from))
        .collect())
}

/// 根据 ID 查询笔记（优化版本：2 次查询代替 4 次）
pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<Option<Note>> {
    find_by_id_with_key(db, id, None).await
}

/// 根据 ID 查询笔记（支持解密）
pub async fn find_by_id_with_key(
    db: &DatabaseConnection,
    id: i64,
    encryption_key: Option<&str>,
) -> anyhow::Result<Option<Note>> {
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

        // 透明解密
        decrypt_note(&mut note, encryption_key);

        Ok(Some(note))
    } else {
        Ok(None)
    }
}

/// 获取回收站中的笔记列表（分页）
pub async fn find_deleted(
    db: &DatabaseConnection,
    page_param: &crate::model::PageParam,
) -> anyhow::Result<PageResult<Note>> {
    find_deleted_with_key(db, page_param, None).await
}

/// 获取回收站中的笔记列表（分页 + 支持解密）
pub async fn find_deleted_with_key(
    db: &DatabaseConnection,
    page_param: &crate::model::PageParam,
    encryption_key: Option<&str>,
) -> anyhow::Result<PageResult<Note>> {
    use sea_orm::PaginatorTrait;

    let page_index = page_param.page_index.max(1) as u64;
    let page_size = page_param.page_size.clamp(1, 200) as u64;

    let query = entity::note::Entity::find()
        .filter(entity::note::Column::DeletedAt.is_not_null())
        .order_by_desc(entity::note::Column::UpdateTime);

    let total = query.clone().count(db).await? as i64;
    let total_pages = if total == 0 {
        0
    } else {
        (total as u64).div_ceil(page_size) as i64
    };

    let mut notes: Vec<Note> = query
        .offset((page_index - 1) * page_size)
        .limit(page_size)
        .all(db)
        .await?
        .into_iter()
        .map(Note::from)
        .collect();

    // 透明解密
    for note in notes.iter_mut() {
        decrypt_note(note, encryption_key);
    }

    Ok(PageResult {
        total,
        total_pages,
        data: notes,
    })
}

/// 分页搜索笔记
pub async fn search_page(
    db: &DatabaseConnection,
    search_param: &NoteSearchPageParam,
) -> anyhow::Result<PageResult<Note>> {
    search_page_with_key(db, search_param, None).await
}

/// 分页搜索笔记（支持解密）
pub async fn search_page_with_key(
    db: &DatabaseConnection,
    search_param: &NoteSearchPageParam,
    encryption_key: Option<&str>,
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

        let order = if search_param.sort_order == "asc" {
            Order::Asc
        } else {
            Order::Desc
        };
        let sort_col = match search_param.sort_field.as_str() {
            "title" => entity::note::Column::Title,
            "create_time" => entity::note::Column::CreateTime,
            _ => entity::note::Column::UpdateTime,
        };

        let mut notes: Vec<Note> = query_builder
            .offset(start)
            .limit(page_size)
            .order_by_desc(entity::note::Column::IsPinned)
            .order_by(sort_col, order.clone())
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

        // 透明解密
        for note in notes.iter_mut() {
            decrypt_note(note, encryption_key);
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
