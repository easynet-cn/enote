use std::collections::HashMap;

use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, QuerySelect, prelude::Expr, sea_query::Asterisk,
};

use crate::{
    entity,
    model::{Note, NotePageResult, NoteSearchPageParam, PageResult},
};

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
    let now = Local::now().naive_local();

    let mut m = note.clone();
    let mut active_model: entity::note::ActiveModel = note.into();

    active_model.create_time = Set(now);
    active_model.update_time = Set(now);

    let id = entity::note::Entity::insert(active_model)
        .exec(db)
        .await?
        .last_insert_id;

    m.id = id;
    m.create_time = Some(now);
    m.update_time = Some(now);

    Ok(Some(m))
}

pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
    entity::note::Entity::delete_by_id(id).exec(db).await?;

    Ok(())
}

pub async fn update(db: &DatabaseConnection, note: &Note) -> anyhow::Result<Option<Note>> {
    if let Some(entity) = entity::note::Entity::find_by_id(note.id).one(db).await? {
        let mut m = note.clone();

        let mut active_model: entity::note::ActiveModel = entity.into_active_model();

        active_model.title = Set(note.title.clone());
        active_model.content = Set(note.content.clone());

        if active_model.is_changed() {
            let now = Local::now().naive_local();

            active_model.update_time = Set(now);

            active_model.update(db).await?;

            m.update_time = Some(now);
        }

        return Ok(Some(m));
    }

    Ok(None)
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
        let notebooks: Vec<Note> = query_builder
            .offset(start)
            .limit(page_size)
            .order_by_desc(entity::note::Column::UpdateTime)
            .order_by_desc(entity::note::Column::Id)
            .all(db)
            .await?
            .iter()
            .map(Note::from)
            .collect();

        let mut page_result = PageResult::<Note>::with_data(total, notebooks);

        page_result.total_pages(search_param.page_param.page_size);

        return Ok(NotePageResult::new(page_result, notbook_counts));
    }

    Ok(NotePageResult::default())
}
