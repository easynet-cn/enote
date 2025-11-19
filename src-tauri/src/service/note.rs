use std::collections::{HashMap, HashSet};

use chrono::Local;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QueryOrder, QuerySelect, TransactionTrait,
    prelude::Expr,
    sea_query::Asterisk,
};

use crate::{
    entity::{self},
    model::{Note, NoteHistoryExtra, NotePageResult, NoteSearchPageParam, PageResult, Tag},
};

pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<Option<Note>> {
    if let Some(entity) = entity::note::Entity::find_by_id(id).one(db).await? {
        let mut note = Note::from(entity);

        if let Some(notebook) = entity::notebook::Entity::find_by_id(note.id)
            .one(db)
            .await?
        {
            note.notebook_name = notebook.name;
        }

        let tag_ids = entity::note_tags::Entity::find()
            .filter(entity::note_tags::Column::NoteId.eq(id))
            .order_by_desc(entity::note_tags::Column::SortOrder)
            .all(db)
            .await?
            .iter()
            .map(|e| e.tag_id)
            .collect::<Vec<i64>>();

        if !tag_ids.is_empty() {
            let tags = entity::tag::Entity::find()
                .filter(entity::tag::Column::Id.is_in(tag_ids))
                .all(db)
                .await?
                .iter()
                .map(Tag::from)
                .collect::<Vec<Tag>>();

            note.tags = tags;
        }

        Ok(Some(note))
    } else {
        Ok(None)
    }
}

pub async fn total_count(db: &DatabaseConnection) -> anyhow::Result<i64> {
    let total_count = entity::note::Entity::find()
        .select_only()
        .column_as(Expr::col(Asterisk).count(), "count")
        .into_tuple::<i64>()
        .one(db)
        .await?
        .unwrap_or_default();

    Ok(total_count)
}

pub async fn create(db: &DatabaseConnection, note: &Note) -> anyhow::Result<Option<Note>> {
    let txn = db.begin().await?;

    let now = Local::now().naive_local();

    let mut active_model: entity::note::ActiveModel = note.into();

    active_model.id = NotSet;
    active_model.create_time = Set(now);
    active_model.update_time = Set(now);

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
                upate_time: Set(now),
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
            .one(db)
            .await?
        {
            notebook_id = notebook.id;
            notebook_name = notebook.name.clone();
        }
    }

    let note_history_extra = NoteHistoryExtra {
        notebook_id: notebook_id,
        notebook_name: notebook_name.clone(),
        title: note.title.clone(),
        tags: note.tags.clone(),
    };

    let extra = serde_json::to_string(&note_history_extra).unwrap_or_default();

    entity::note_history::ActiveModel {
        id: NotSet,
        note_id: Set(entity.id),
        old_content: Set(String::default()),
        new_content: Set(note.content.clone()),
        extra: Set(extra),
        operate_type: Set(1),
        operate_time: Set(now),
        create_time: Set(now),
    }
    .insert(&txn)
    .await?;

    txn.commit().await?;

    find_by_id(db, entity.id).await
}

pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
    if let Some(entity) = entity::note::Entity::find_by_id(id).one(db).await? {
        let tag_ids = entity::note_tags::Entity::find()
            .filter(entity::note_tags::Column::NoteId.eq(id))
            .order_by_desc(entity::note_tags::Column::SortOrder)
            .all(db)
            .await?
            .iter()
            .map(|e| e.tag_id)
            .collect::<Vec<i64>>();

        let mut tags = Vec::<Tag>::with_capacity(tag_ids.len());

        if !tag_ids.is_empty() {
            tags = entity::tag::Entity::find()
                .filter(entity::tag::Column::Id.is_in(tag_ids))
                .all(db)
                .await?
                .iter()
                .map(Tag::from)
                .collect::<Vec<Tag>>();
        }

        let mut notebook_id = 0_i64;
        let mut notebook_name = String::default();

        if entity.notebook_id > 0 {
            if let Some(notebook) = entity::notebook::Entity::find_by_id(entity.notebook_id)
                .one(db)
                .await?
            {
                notebook_id = notebook.id;
                notebook_name = notebook.name.clone();
            }
        }

        let txn = db.begin().await?;

        let now = Local::now().naive_local();

        let note_history_extra = NoteHistoryExtra {
            notebook_id: notebook_id,
            notebook_name: notebook_name.clone(),
            title: entity.title.clone(),
            tags: tags,
        };

        let extra = serde_json::to_string(&note_history_extra).unwrap_or_default();

        entity::note_history::ActiveModel {
            id: Set(0),
            note_id: Set(id),
            old_content: Set(entity.content),
            new_content: Set(String::default()),
            extra: Set(extra),
            operate_type: Set(3),
            operate_time: Set(now),
            create_time: Set(now),
        }
        .insert(&txn)
        .await?;

        entity::note::Entity::delete_by_id(id).exec(&txn).await?;
        entity::note_tags::Entity::delete_many()
            .filter(entity::note_tags::Column::NoteId.eq(id))
            .exec(&txn)
            .await?;

        txn.commit().await?;
    }

    Ok(())
}

pub async fn update(db: &DatabaseConnection, note: &Note) -> anyhow::Result<Option<Note>> {
    if let Some(entity) = entity::note::Entity::find_by_id(note.id).one(db).await? {
        let old_title = entity.title.clone();
        let old_tag_ids = entity::note_tags::Entity::find()
            .filter(entity::note_tags::Column::NoteId.eq(note.id))
            .order_by_desc(entity::note_tags::Column::SortOrder)
            .all(db)
            .await?
            .iter()
            .map(|e| e.tag_id)
            .collect::<Vec<i64>>();

        let mut old_tags = Vec::<Tag>::with_capacity(old_tag_ids.len());

        if !old_tag_ids.is_empty() {
            let tags = entity::tag::Entity::find()
                .filter(entity::tag::Column::Id.is_in(old_tag_ids.clone()))
                .all(db)
                .await?
                .iter()
                .map(Tag::from)
                .collect::<Vec<Tag>>();

            old_tags = tags;
        }

        let txn = db.begin().await?;

        let old_content = entity.content.clone();

        let mut active_model: entity::note::ActiveModel = entity.into_active_model();

        active_model.notebook_id.set_if_not_equals(note.notebook_id);
        active_model.title.set_if_not_equals(note.title.clone());
        active_model.content.set_if_not_equals(note.content.clone());

        let now = Local::now().naive_local();

        let note_changed = active_model.is_changed();

        if active_model.is_changed() {
            active_model.update_time = Set(now);

            active_model.update(&txn).await?;
        }

        let tag_ids = note.tags.iter().map(|e| e.id).collect::<Vec<i64>>();

        let tags_changed: bool = old_tag_ids != tag_ids;

        if tags_changed {
            if !old_tag_ids.is_empty() && note.tags.is_empty() {
                entity::note_tags::Entity::delete_many()
                    .filter(entity::note_tags::Column::Id.is_in(old_tag_ids))
                    .exec(&txn)
                    .await?;
            }

            if !note.tags.is_empty() {
                let note_tags = note
                    .tags
                    .iter()
                    .map(|e| entity::note_tags::ActiveModel {
                        id: NotSet,
                        note_id: Set(note.id),
                        tag_id: Set(e.id),
                        sort_order: Set(e.sort_order),
                        create_time: Set(now),
                        upate_time: Set(now),
                    })
                    .collect::<Vec<entity::note_tags::ActiveModel>>();

                entity::note_tags::Entity::insert_many(note_tags)
                    .exec(&txn)
                    .await?;
            }
        }

        if note_changed || tags_changed {
            let mut notebook_id = 0_i64;
            let mut notebook_name = String::default();

            if note.notebook_id > 0 {
                if let Some(notebook) = entity::notebook::Entity::find_by_id(note.notebook_id)
                    .one(db)
                    .await?
                {
                    notebook_id = notebook.id;
                    notebook_name = notebook.name.clone();
                }
            }

            let note_history_extra = NoteHistoryExtra {
                notebook_id: notebook_id,
                notebook_name: notebook_name.clone(),
                title: old_title,
                tags: old_tags,
            };

            let extra = serde_json::to_string(&note_history_extra).unwrap_or_default();

            entity::note_history::ActiveModel {
                id: NotSet,
                note_id: Set(note.id),
                old_content: Set(old_content),
                new_content: Set(note.content.clone()),
                extra: Set(extra),
                operate_type: Set(2),
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

pub async fn search_page(
    db: &DatabaseConnection,
    search_param: &NoteSearchPageParam,
) -> anyhow::Result<NotePageResult> {
    let mut count_builder = entity::note::Entity::find();
    let mut query_builder = entity::note::Entity::find();
    let mut count_map_builder = entity::note::Entity::find()
        .select_only()
        .column(entity::note::Column::NotebookId)
        .column_as(entity::note::Column::Id.count(), "n");

    if search_param.notebook_id > 0 {
        count_builder =
            count_builder.filter(entity::note::Column::NotebookId.eq(search_param.notebook_id));
        query_builder =
            query_builder.filter(entity::note::Column::NotebookId.eq(search_param.notebook_id));
        count_map_builder =
            count_map_builder.filter(entity::note::Column::NotebookId.eq(search_param.notebook_id));
    }
    if !search_param.keyword.is_empty() {
        let keyword = search_param.keyword.as_str();

        count_builder = count_builder.filter(
            Condition::all().add(
                Condition::any()
                    .add(entity::note::Column::Title.contains(keyword))
                    .add(entity::note::Column::Content.contains(keyword)),
            ),
        );
        query_builder = query_builder.filter(
            Condition::all().add(
                Condition::any()
                    .add(entity::note::Column::Title.contains(keyword))
                    .add(entity::note::Column::Content.contains(keyword)),
            ),
        );
        count_map_builder = count_map_builder.filter(
            Condition::all().add(
                Condition::any()
                    .add(entity::note::Column::Title.contains(keyword))
                    .add(entity::note::Column::Content.contains(keyword)),
            ),
        );
    }

    let total = count_builder
        .select_only()
        .column_as(Expr::col(Asterisk).count(), "count")
        .into_tuple::<i64>()
        .one(db)
        .await?
        .unwrap_or_default();

    if total > 0 {
        let notbook_counts = count_map_builder
            .group_by(entity::note::Column::NotebookId)
            .into_tuple::<(i64, i64)>()
            .all(db)
            .await?
            .into_iter()
            .collect::<HashMap<i64, i64>>();

        let start = search_param.page_param.start() as u64;
        let page_size = search_param.page_param.page_size as u64;
        let mut notes: Vec<Note> = query_builder
            .offset(start)
            .limit(page_size)
            .order_by_desc(entity::note::Column::UpdateTime)
            .order_by_desc(entity::note::Column::Id)
            .all(db)
            .await?
            .iter()
            .map(Note::from)
            .collect();

        let note_ids = notes.iter().map(|e| e.id).collect::<Vec<i64>>();

        if !note_ids.is_empty() {
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
                    .iter()
                    .map(|e| (e.id, Tag::from(e)))
                    .collect::<HashMap<i64, Tag>>();

                for note in notes.iter_mut() {
                    if let Some(tag_ids) = note_tags_map.get(&note.id) {
                        let mut tags = Vec::<Tag>::new();

                        for tag_id in tag_ids.iter() {
                            if let Some(tag) = tag_map.get(tag_id) {
                                tags.push(tag.clone());
                            }
                        }

                        note.tags = tags;
                    }
                }
            }
        }

        let mut page_result = PageResult::<Note>::with_data(total, notes);

        page_result.total_pages(search_param.page_param.page_size);

        return Ok(NotePageResult::new(page_result, notbook_counts));
    }

    Ok(NotePageResult::default())
}
