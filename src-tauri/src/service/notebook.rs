use chrono::Local;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, QueryOrder};

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

pub async fn create(
    db: &DatabaseConnection,
    notebook: &Notebook,
) -> anyhow::Result<Option<Notebook>> {
    let now = Local::now().naive_local();

    let active_model = entity::notebook::ActiveModel {
        id: Set(0),
        parent_id: Set(notebook.parent_id),
        name: Set(notebook.name.clone()),
        description: Set(notebook.description.clone()),
        icon: Set(notebook.icon.clone()),
        cls: Set(notebook.cls.clone()),
        sort_order: Set(notebook.sort_order),
        create_time: Set(now),
        update_time: Set(now),
    };

    let entity = active_model.insert(db).await?;

    Ok(Some(Notebook::from(entity)))
}
