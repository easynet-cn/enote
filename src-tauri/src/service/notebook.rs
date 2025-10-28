use std::collections::HashMap;

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect};

use crate::{entity, model::Notebook};

pub async fn find_all(db: &DatabaseConnection) -> anyhow::Result<Vec<Notebook>> {
    let mut notebooks = entity::notebook::Entity::find()
        .order_by_desc(entity::notebook::Column::SortOrder)
        .all(db)
        .await?
        .iter()
        .map(Notebook::from)
        .collect::<Vec<Notebook>>();

    let count_map = entity::note::Entity::find()
        .select_only()
        .column(entity::note::Column::NotebookId)
        .column_as(entity::note::Column::Id.count(), "n")
        .group_by(entity::note::Column::NotebookId)
        .into_tuple::<(i64, i64)>()
        .all(db)
        .await?
        .into_iter()
        .collect::<HashMap<i64, i64>>();

    for notebook in notebooks.iter_mut() {
        notebook.count = *count_map.get(&notebook.id).unwrap_or(&0);
    }

    Ok(notebooks)
}
