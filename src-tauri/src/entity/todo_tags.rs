//! 待办标签关联实体
//!
//! 复用现有 `tag` 体系，避免维护两套标签。

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "todo_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub todo_id: i64,
    pub tag_id: i64,
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::todo::Entity",
        from = "Column::TodoId",
        to = "super::todo::Column::Id"
    )]
    Todo,
    #[sea_orm(
        belongs_to = "super::tag::Entity",
        from = "Column::TagId",
        to = "super::tag::Column::Id"
    )]
    Tag,
}

impl Related<super::todo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Todo.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
