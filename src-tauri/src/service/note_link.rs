//! 笔记链接服务
//!
//! 实现笔记间的双向链接功能

use anyhow::Result;
use chrono::Local;
use sea_orm::*;

use crate::entity::{note, note_link};
use crate::model::NoteLink;

/// 获取笔记的所有链接（双向）
///
/// 返回与指定笔记关联的所有笔记（包括 source 和 target 方向）
pub async fn find_links(db: &DatabaseConnection, note_id: i64) -> Result<Vec<NoteLink>> {
    let links = note_link::Entity::find()
        .filter(
            Condition::any()
                .add(note_link::Column::SourceNoteId.eq(note_id))
                .add(note_link::Column::TargetNoteId.eq(note_id)),
        )
        .all(db)
        .await?;

    if links.is_empty() {
        return Ok(Vec::new());
    }

    // 批量收集所有关联笔记 ID
    let linked_note_ids: Vec<i64> = links
        .iter()
        .map(|link| {
            if link.source_note_id == note_id {
                link.target_note_id
            } else {
                link.source_note_id
            }
        })
        .collect();

    // 一次查询获取所有关联笔记标题
    let notes_map: std::collections::HashMap<i64, String> = note::Entity::find()
        .filter(note::Column::Id.is_in(linked_note_ids))
        .all(db)
        .await?
        .into_iter()
        .map(|n| (n.id, n.title))
        .collect();

    let mut result = Vec::new();
    for link in &links {
        let linked_note_id = if link.source_note_id == note_id {
            link.target_note_id
        } else {
            link.source_note_id
        };

        if let Some(title) = notes_map.get(&linked_note_id) {
            result.push(NoteLink {
                id: link.id,
                note_id: linked_note_id,
                note_title: title.clone(),
                create_time: Some(link.create_time),
            });
        }
    }

    Ok(result)
}

/// 创建笔记链接
///
/// 在两个笔记之间建立双向链接，使用较小 ID 作为 source 确保唯一性
pub async fn create_link(
    db: &DatabaseConnection,
    source_note_id: i64,
    target_note_id: i64,
) -> Result<()> {
    if source_note_id == target_note_id {
        anyhow::bail!("不能链接到自身");
    }

    // 标准化方向：较小 ID 作为 source
    let (src, tgt) = if source_note_id < target_note_id {
        (source_note_id, target_note_id)
    } else {
        (target_note_id, source_note_id)
    };

    // 检查是否已存在
    let existing = note_link::Entity::find()
        .filter(note_link::Column::SourceNoteId.eq(src))
        .filter(note_link::Column::TargetNoteId.eq(tgt))
        .one(db)
        .await?;

    if existing.is_some() {
        return Ok(()); // 已存在，不重复创建
    }

    let now = Local::now().naive_local();
    let model = note_link::ActiveModel {
        id: NotSet,
        source_note_id: Set(src),
        target_note_id: Set(tgt),
        create_time: Set(now),
    };

    note_link::Entity::insert(model).exec(db).await?;
    Ok(())
}

/// 删除链接
pub async fn delete_link(db: &DatabaseConnection, link_id: i64) -> Result<()> {
    note_link::Entity::delete_by_id(link_id).exec(db).await?;
    Ok(())
}

/// 搜索可链接的笔记（排除已链接的和自身）
pub async fn search_linkable_notes(
    db: &DatabaseConnection,
    note_id: i64,
    keyword: &str,
) -> Result<Vec<NoteLink>> {
    // 获取已链接的笔记 ID
    let existing_links = note_link::Entity::find()
        .filter(
            Condition::any()
                .add(note_link::Column::SourceNoteId.eq(note_id))
                .add(note_link::Column::TargetNoteId.eq(note_id)),
        )
        .all(db)
        .await?;

    let mut excluded_ids: Vec<i64> = existing_links
        .iter()
        .map(|l| {
            if l.source_note_id == note_id {
                l.target_note_id
            } else {
                l.source_note_id
            }
        })
        .collect();
    excluded_ids.push(note_id); // 排除自身

    // 搜索笔记
    let mut query = note::Entity::find()
        .filter(note::Column::Id.is_not_in(excluded_ids))
        .filter(note::Column::DeletedAt.is_null());

    if !keyword.is_empty() {
        query = query.filter(note::Column::Title.contains(keyword));
    }

    let notes = query
        .order_by_desc(note::Column::UpdateTime)
        .limit(20)
        .all(db)
        .await?;

    Ok(notes
        .into_iter()
        .map(|n| NoteLink {
            id: 0,
            note_id: n.id,
            note_title: n.title,
            create_time: None,
        })
        .collect())
}
