use std::sync::Arc;

use chrono::Local;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
    prelude::Expr, sea_query::Asterisk,
};

use crate::{
    config::AppState,
    entity,
    model::{Note, NoteSearchPageParam, Notebook, NotebookResult, PageResult},
    service,
};

#[tauri::command]
pub async fn find_all_notebooks(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<NotebookResult, String> {
    let db = &app_state.database_connection;

    let total_count = service::note::total_count(db)
        .await
        .map_err(|e| e.to_string())?;
    let notebooks = service::notebook::find_all(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(NotebookResult::new(total_count, notebooks))
}

#[tauri::command]
pub async fn create_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    note: Note,
) -> Result<Option<Note>, String> {
    let db = &app_state.database_connection;

    let now = Local::now().naive_local();

    let mut m = note.clone();
    let mut active_model: entity::note::ActiveModel = note.into();

    active_model.create_time = Set(now);
    active_model.update_time = Set(now);

    let id = entity::note::Entity::insert(active_model)
        .exec(db)
        .await
        .map_err(|e| e.to_string())?
        .last_insert_id;

    m.id = id;
    m.create_time = Some(now);
    m.update_time = Some(now);

    Ok(Some(m))
}

#[tauri::command]
pub async fn search_page_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
    search_param: NoteSearchPageParam,
) -> Result<PageResult<Note>, String> {
    let db = &app_state.database_connection;

    let mut count_builder = entity::note::Entity::find();
    let mut query_builder = entity::note::Entity::find();

    if search_param.notebook_id > 0 {
        count_builder =
            count_builder.filter(entity::note::Column::NotebookId.eq(search_param.notebook_id));
        query_builder =
            query_builder.filter(entity::note::Column::NotebookId.eq(search_param.notebook_id));
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
    }

    let total = count_builder
        .select_only()
        .column_as(Expr::col(Asterisk).count(), "count")
        .into_tuple::<i64>()
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    if total > 0 {
        let start = search_param.page_param.start() as u64;
        let page_size = search_param.page_param.page_size as u64;
        let page_configs: Vec<Note> = query_builder
            .offset(start)
            .limit(page_size)
            .all(db)
            .await
            .map_err(|e| e.to_string())?
            .iter()
            .map(Note::from)
            .collect();

        let mut page_result = PageResult::<Note>::with_data(total, page_configs);

        page_result.total_pages(search_param.page_param.page_size);

        return Ok(page_result);
    }

    Ok(PageResult::default())
}
