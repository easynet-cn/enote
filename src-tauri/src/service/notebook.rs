use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};

use crate::{entity, model::Notebook};

pub async fn find_all(db: &DatabaseConnection) -> anyhow::Result<Vec<Notebook>> {
    let notebooks = entity::notebook::Entity::find()
        .order_by_desc(entity::notebook::Column::SortOrder)
        .all(db)
        .await?
        .iter()
        .map(Notebook::from)
        .collect::<Vec<Notebook>>();

    Ok(notebooks)
}
