//! 待办清单实体

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "todo_list")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    /// 图标标识
    pub icon: String,
    /// 主题色
    pub color: String,
    pub sort_order: i32,
    /// MCP 访问控制：0=继承, 1=读写, 2=只读, 3=禁止
    pub mcp_access: i32,
    pub create_time: DateTime,
    pub update_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::todo::Entity")]
    Todo,
}

impl Related<super::todo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Todo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
