use std::collections::HashMap;

use chrono::NaiveDateTime;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnNull, serde_as};

use crate::entity;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageParam {
    #[serde(default = "PageParam::default_page_index")]
    pub page_index: i64,
    #[serde(default = "PageParam::default_page_size")]
    pub page_size: i64,
}

impl PageParam {
    pub fn start(&self) -> i64 {
        (self.page_index - 1) * self.page_size
    }

    fn default_page_index() -> i64 {
        1
    }

    fn default_page_size() -> i64 {
        50
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    #[serde(default = "PageResult::<T>::default_total")]
    pub total: i64,
    #[serde(default = "PageResult::<T>::default_total_pages")]
    pub total_pages: i64,
    #[serde(default = "PageResult::<T>::default_data")]
    pub data: Vec<T>,
}

impl<T> PageResult<T> {
    pub fn new(total: i64, total_pages: i64, data: Vec<T>) -> Self {
        Self {
            total: total,
            total_pages: total_pages,
            data: data,
        }
    }

    pub fn with_data(total: i64, data: Vec<T>) -> Self {
        Self {
            total: total,
            total_pages: 0,
            data: data,
        }
    }

    pub fn total_pages(&mut self, page_size: i64) {
        if self.total > 0 && page_size > 0 {
            self.total_pages = ((self.total as f64) / (page_size as f64)).ceil() as i64
        }
    }

    fn default_total() -> i64 {
        0
    }

    fn default_total_pages() -> i64 {
        0
    }

    fn default_data() -> Vec<T> {
        vec![]
    }
}

impl<T> Default for PageResult<T> {
    fn default() -> Self {
        Self {
            total: 0,
            total_pages: 0,
            data: vec![],
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Notebook {
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: i64,
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
    #[serde(
        serialize_with = "serialize_option_dt",
        deserialize_with = "deserialize_option_dt"
    )]
    pub create_time: Option<NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_option_dt",
        deserialize_with = "deserialize_option_dt"
    )]
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

impl From<entity::notebook::Model> for Notebook {
    fn from(value: entity::notebook::Model) -> Self {
        Self {
            id: value.id,
            parent_id: value.parent_id,
            name: value.name,
            description: value.description,
            icon: value.icon,
            cls: value.cls,
            sort_order: value.sort_order,
            create_time: Some(value.create_time),
            update_time: Some(value.update_time),
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Note {
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: i64,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub notebook_id: i64,
    pub notebook_name: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub title: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub content: String,
    #[serde(
        serialize_with = "serialize_option_dt",
        deserialize_with = "deserialize_option_dt"
    )]
    pub create_time: Option<NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_option_dt",
        deserialize_with = "deserialize_option_dt"
    )]
    pub update_time: Option<NaiveDateTime>,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tags: Vec<Tag>,
}

impl From<entity::note::Model> for Note {
    fn from(value: entity::note::Model) -> Self {
        Self {
            id: value.id,
            notebook_id: value.notebook_id,
            title: value.title,
            content: value.content,
            create_time: Some(value.create_time),
            update_time: Some(value.update_time),
            ..Default::default()
        }
    }
}

impl From<&entity::note::Model> for Note {
    fn from(value: &entity::note::Model) -> Self {
        Self {
            id: value.id,
            notebook_id: value.notebook_id,
            title: value.title.clone(),
            content: value.content.clone(),
            create_time: Some(value.create_time),
            update_time: Some(value.update_time),
            ..Default::default()
        }
    }
}

impl Into<entity::note::ActiveModel> for Note {
    fn into(self) -> entity::note::ActiveModel {
        entity::note::ActiveModel {
            id: Set(self.id),
            notebook_id: Set(self.notebook_id),
            title: Set(self.title.clone()),
            content: Set(self.content.clone()),
            create_time: Set(self.create_time.unwrap_or_default()),
            update_time: Set(self.update_time.unwrap_or_default()),
        }
    }
}

impl Into<entity::note::ActiveModel> for &Note {
    fn into(self) -> entity::note::ActiveModel {
        entity::note::ActiveModel {
            id: Set(self.id),
            notebook_id: Set(self.notebook_id),
            title: Set(self.title.clone()),
            content: Set(self.content.clone()),
            create_time: Set(self.create_time.unwrap_or_default()),
            update_time: Set(self.update_time.unwrap_or_default()),
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NotePageResult {
    #[serde(flatten)]
    page_result: PageResult<Note>,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    notebook_counts: HashMap<i64, i64>,
}

impl NotePageResult {
    pub fn new(page_result: PageResult<Note>, notebook_counts: HashMap<i64, i64>) -> Self {
        Self {
            page_result: page_result,
            notebook_counts: notebook_counts,
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Tag {
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: i64,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub icon: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub cls: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub sort_order: i32,
    #[serde(
        serialize_with = "serialize_option_dt",
        deserialize_with = "deserialize_option_dt"
    )]
    pub create_time: Option<NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_option_dt",
        deserialize_with = "deserialize_option_dt"
    )]
    pub update_time: Option<NaiveDateTime>,
}

impl From<entity::tag::Model> for Tag {
    fn from(value: entity::tag::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            icon: value.icon,
            cls: value.cls,
            sort_order: value.sort_order,
            create_time: Some(value.create_time),
            update_time: Some(value.update_time),
        }
    }
}

impl From<&entity::tag::Model> for Tag {
    fn from(value: &entity::tag::Model) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            icon: value.icon.clone(),
            cls: value.cls.clone(),
            sort_order: value.sort_order,
            create_time: Some(value.create_time),
            update_time: Some(value.update_time),
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteSearchPageParam {
    #[serde(flatten)]
    pub page_param: PageParam,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub notebook_id: i64,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tag_id: i64,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub keyword: String,
}

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteHistoryExtra {
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub title: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tags: Vec<Tag>,
}

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteHistory {
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: i64,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub note_id: i64,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub old_content: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub new_content: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub extra: NoteHistoryExtra,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub operate_type: i32,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub operate_time: NaiveDateTime,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub create_time: NaiveDateTime,
}

impl From<entity::note_history::Model> for NoteHistory {
    fn from(value: entity::note_history::Model) -> Self {
        Self {
            id: value.id,
            note_id: value.note_id,
            old_content: value.old_content,
            new_content: value.new_content,
            extra: serde_json::from_str(&value.extra).unwrap_or_default(),
            operate_type: value.operate_type,
            operate_time: value.operate_time,
            create_time: value.create_time,
        }
    }
}

impl From<&entity::note_history::Model> for NoteHistory {
    fn from(value: &entity::note_history::Model) -> Self {
        Self {
            id: value.id,
            note_id: value.note_id,
            old_content: value.old_content.clone(),
            new_content: value.new_content.clone(),
            extra: serde_json::from_str(&value.extra).unwrap_or_default(),
            operate_type: value.operate_type,
            operate_time: value.operate_time,
            create_time: value.create_time,
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteHistorySearchPageParam {
    #[serde(flatten)]
    pub page_param: PageParam,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub note_id: i64,
}

pub fn serialize_dt<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&dt.format("%Y-%m-%d %H:%M:%S").to_string())
}

pub fn deserialize_dt<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
}

pub fn serialize_option_dt<S>(
    value: &Option<NaiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(dt) => serializer.serialize_some(&dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_option_dt<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(str) => NaiveDateTime::parse_from_str(&str, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)
            .map(|ndt| Some(ndt)),

        None => Ok(None),
    }
}
