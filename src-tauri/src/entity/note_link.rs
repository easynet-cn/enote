//! `SeaORM` Entity - 笔记链接

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "note_link")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub source_note_id: i64,
    pub target_note_id: i64,
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
