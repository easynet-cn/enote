//! `SeaORM` Entity - SyncLogDetail

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sync_log_detail")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub sync_log_id: i64,
    pub table_name: String,
    pub source_id: i64,
    pub target_id: Option<i64>,
    pub record_name: String,
    pub status: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub error_message: Option<String>,
    pub synced_at: DateTime,
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sync_log::Entity",
        from = "Column::SyncLogId",
        to = "super::sync_log::Column::Id"
    )]
    SyncLog,
}

impl Related<super::sync_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SyncLog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
