use chrono::Local;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection, EntityTrait, IntoActiveModel, QueryOrder,
};

use crate::{entity, model::Tag};

pub async fn find_all(db: &DatabaseConnection) -> anyhow::Result<Vec<Tag>> {
    let tags = entity::tag::Entity::find()
        .order_by_desc(entity::tag::Column::SortOrder)
        .order_by_desc(entity::tag::Column::UpdateTime)
        .all(db)
        .await?
        .iter()
        .map(Tag::from)
        .collect::<Vec<Tag>>();

    Ok(tags)
}
pub async fn create(db: &DatabaseConnection, tag: &Tag) -> anyhow::Result<Option<Tag>> {
    let now = Local::now().naive_local();

    let active_model = entity::tag::ActiveModel {
        id: NotSet,
        name: Set(tag.name.clone()),
        icon: Set(tag.icon.clone()),
        cls: Set(tag.cls.clone()),
        sort_order: Set(tag.sort_order),
        create_time: Set(now),
        update_time: Set(now),
    };

    let entity = active_model.insert(db).await?;

    Ok(Some(Tag::from(entity)))
}

pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
    entity::tag::Entity::delete_by_id(id).exec(db).await?;

    Ok(())
}

pub async fn update(db: &DatabaseConnection, tag: &Tag) -> anyhow::Result<Option<Tag>> {
    if let Some(entity) = entity::tag::Entity::find_by_id(tag.id).one(db).await? {
        let mut m = tag.clone();
        let mut active_model: entity::tag::ActiveModel = entity.into_active_model();

        active_model.name = Set(tag.name.clone());
        active_model.icon = Set(tag.icon.clone());
        active_model.cls = Set(tag.cls.clone());
        active_model.sort_order = Set(tag.sort_order);

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
