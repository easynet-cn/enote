//! 待办实体

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    /// 备注
    ///
    /// 注意：`text` 列在 MySQL 下不允许 DEFAULT 值，因此 DB 层未设默认值，
    /// 插入时必须由业务层显式赋值（通常为空字符串）。
    pub description: String,
    /// 是否已完成：0 = 未完成，1 = 已完成
    pub is_completed: i32,
    /// 优先级：0 = 无，1 = 低，2 = 中，3 = 高
    pub priority: i32,
    /// 截止日期
    pub due_date: Option<DateTime>,
    /// 完成时间
    pub completed_at: Option<DateTime>,
    /// 所属清单 ID，NULL 表示未归类
    pub list_id: Option<i64>,
    pub sort_order: i32,
    pub create_time: DateTime,
    pub update_time: DateTime,
    /// 软删除时间，NULL 表示未删除
    pub deleted_at: Option<DateTime>,
    // ↓ P1 字段（建表时已一次建全，P1 阶段无需再改表结构）
    /// 计划开始日期
    pub start_date: Option<DateTime>,
    /// 提醒时刻
    pub remind_at: Option<DateTime>,
    /// 是否已提醒：0 = 未提醒，1 = 已提醒
    pub is_reminded: i32,
    /// 父待办 ID，0 表示无父待办（用于子任务）
    pub parent_id: i64,
    /// 关联笔记 ID
    pub note_id: Option<i64>,
    /// 重复类型：0 = 不重复，1 = 每天，2 = 每周，3 = 每月，4 = 每年
    pub recurrence_type: i32,
    /// 重复间隔（每 N 个周期）
    pub recurrence_interval: i32,
    /// 重复终止日期
    pub recurrence_end_date: Option<DateTime>,
    /// MCP 访问控制：0=继承, 1=读写, 2=只读, 3=禁止
    pub mcp_access: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::todo_list::Entity",
        from = "Column::ListId",
        to = "super::todo_list::Column::Id"
    )]
    TodoList,
}

impl Related<super::todo_list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TodoList.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
