use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
    prelude::Expr, sea_query::Asterisk,
};

use crate::{
    entity::{self},
    model::{NoteHistory, NoteHistorySearchPageParam, PageResult},
};

pub async fn search_page(
    db: &DatabaseConnection,
    search_param: &NoteHistorySearchPageParam,
) -> anyhow::Result<PageResult<NoteHistory>> {
    let mut count_builder = entity::note_history::Entity::find();
    let mut query_builder = entity::note_history::Entity::find();

    if search_param.note_id > 0 {
        count_builder =
            count_builder.filter(entity::note_history::Column::NoteId.eq(search_param.note_id));
        query_builder =
            query_builder.filter(entity::note_history::Column::NoteId.eq(search_param.note_id));
    }

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

    Ok(PageResult::<NoteHistory>::default())
}
