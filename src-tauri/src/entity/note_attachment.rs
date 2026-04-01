//! 笔记附件实体

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "note_attachment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub note_id: i64,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    /// SHA256 哈希值，用于去重
    #[sea_orm(default_value = "")]
    pub file_hash: String,
    /// 引用计数（多个附件记录可能指向同一物理文件）
    #[sea_orm(default_value = 1)]
    pub ref_count: i32,
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::note::Entity",
        from = "Column::NoteId",
        to = "super::note::Column::Id"
    )]
    Note,
}

impl Related<super::note::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Note.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
