use chrono::Local;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection, EntityTrait, IntoActiveModel, QueryOrder,
};

use crate::{entity, model::Notebook};

pub async fn find_all(db: &DatabaseConnection) -> anyhow::Result<Vec<Notebook>> {
    let notebooks = entity::notebook::Entity::find()
        .order_by_desc(entity::notebook::Column::SortOrder)
        .order_by_desc(entity::notebook::Column::UpdateTime)
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
        id: NotSet,
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

pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
    entity::notebook::Entity::delete_by_id(id).exec(db).await?;

    Ok(())
}

pub async fn update(
    db: &DatabaseConnection,
    notebook: &Notebook,
) -> anyhow::Result<Option<Notebook>> {
    if let Some(entity) = entity::notebook::Entity::find_by_id(notebook.id)
        .one(db)
        .await?
    {
        let mut m = notebook.clone();
        let mut active_model: entity::notebook::ActiveModel = entity.into_active_model();

        active_model.name = Set(notebook.name.clone());
        active_model.description = Set(notebook.description.clone());
        active_model.icon = Set(notebook.icon.clone());
        active_model.cls = Set(notebook.cls.clone());
        active_model.sort_order = Set(notebook.sort_order);

        if active_model.is_changed() {
            let now = Local::now().naive_local();

            active_model.update_time = Set(now);

            active_model.update(db).await?;

            m.update_time = Some(now);
        }

        Ok(Some(m))
    } else {
        Ok(None)
    }
}
