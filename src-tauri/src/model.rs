use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnNull, serde_as};

use crate::entity;

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Notebook {
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: i32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub parent_id: i64,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub description: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub icon: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub cls: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub sort_order: i32,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}

impl From<&entity::notebook::Model> for Notebook {
    fn from(value: &entity::notebook::Model) -> Self {
        Self {
            id: value.id,
            parent_id: value.parent_id,
            name: value.name.clone(),
            description: value.description.clone(),
            icon: value.icon.clone(),
            cls: value.cls.clone(),
            sort_order: value.sort_order,
            create_time: Some(value.create_time),
            update_time: Some(value.update_time),
        }
    }
}
