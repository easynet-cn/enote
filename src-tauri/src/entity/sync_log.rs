//! `SeaORM` Entity - SyncLog

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sync_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub source_profile: String,
    pub target_profile: String,
    pub source_db_type: String,
    pub target_db_type: String,
    pub sync_mode: String,
    #[sea_orm(column_type = "Text")]
    pub sync_scope: String,
    pub backup_format: Option<String>,
    pub source_backup: Option<String>,
    pub target_backup: Option<String>,
    pub status: String,
    pub total_count: i32,
    pub success_count: i32,
    pub failed_count: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub error_message: Option<String>,
    pub started_at: DateTime,
    pub finished_at: Option<DateTime>,
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
