use chrono::Local;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection, EntityTrait, QueryOrder,
};

use crate::{entity, model::Tag};

pub async fn find_all(db: &DatabaseConnection) -> anyhow::Result<Vec<Tag>> {
    let tags = entity::tag::Entity::find()
        .order_by_desc(entity::tag::Column::SortOrder)
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
