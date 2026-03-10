//! 设置服务模块
//!
//! 提供应用设置的读取和保存功能（KV 存储）

use std::collections::HashMap;

use chrono::Local;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use crate::entity;

/// 获取所有设置
pub async fn get_all(db: &DatabaseConnection) -> anyhow::Result<HashMap<String, String>> {
    let settings = entity::settings::Entity::find().all(db).await?;
    Ok(settings.into_iter().map(|s| (s.key, s.value)).collect())
}

/// 保存设置（upsert）
pub async fn save(
    db: &DatabaseConnection,
    settings: HashMap<String, String>,
) -> anyhow::Result<()> {
    let now = Local::now().naive_local();

    for (key, value) in settings {
        let existing = entity::settings::Entity::find()
            .filter(entity::settings::Column::Key.eq(&key))
            .one(db)
            .await?;

        if let Some(model) = existing {
            let mut active: entity::settings::ActiveModel = model.into();
            active.value = Set(value);
            active.update_time = Set(now);
            active.update(db).await?;
        } else {
            entity::settings::ActiveModel {
                id: NotSet,
                key: Set(key),
                value: Set(value),
                create_time: Set(now),
                update_time: Set(now),
            }
            .insert(db)
            .await?;
        }
    }

    Ok(())
}
