use chrono::Local;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

use crate::{entity, model::Tag};

pub async fn create(db: &DatabaseConnection, tag: &Tag) -> anyhow::Result<Option<Tag>> {
    let now = Local::now().naive_local();

    let active_model = entity::tag::ActiveModel {
        id: Set(0),
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
