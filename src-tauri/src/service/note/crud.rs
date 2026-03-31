use std::collections::HashSet;

use crate::{
    entity::{self},
    model::{Note, NoteHistoryExtra, OperateSource, OperationType},
};
use chrono::Local;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, TransactionTrait,
    prelude::Expr,
};

use super::crypto_helper::{decrypt_content, encrypt_content};
use super::search::{fetch_note_tags, find_by_id, find_by_id_with_key};

/// 创建笔记
pub async fn create(
    db: &DatabaseConnection,
    note: &Note,
    source: OperateSource,
) -> anyhow::Result<Option<Note>> {
    create_with_key(db, note, source, None).await
}

/// 创建笔记（支持内容加密）
pub async fn create_with_key(
    db: &DatabaseConnection,
    note: &Note,
    source: OperateSource,
    encryption_key: Option<&str>,
) -> anyhow::Result<Option<Note>> {
    let txn = db.begin().await?;

    let now = Local::now().naive_local();

    // 透明加密：对内容进行加密
    let encrypted_content = encrypt_content(&note.content, encryption_key)?;

    let mut active_model: entity::note::ActiveModel = note.into();

    active_model.id = NotSet;
    active_model.is_pinned = Set(0);
    active_model.is_starred = Set(0);
    active_model.content = Set(encrypted_content);
    active_model.create_time = Set(now);
    active_model.update_time = Set(now);
    active_model.deleted_at = Set(None);

    let entity = active_model.insert(&txn).await?;

    if !note.tags.is_empty() {
        let note_tags = note
            .tags
            .iter()
            .map(|e| entity::note_tags::ActiveModel {
                id: NotSet,
                note_id: Set(entity.id),
                tag_id: Set(e.id),
                sort_order: Set(e.sort_order),
                create_time: Set(now),
                update_time: Set(now),
            })
            .collect::<Vec<entity::note_tags::ActiveModel>>();

        entity::note_tags::Entity::insert_many(note_tags)
            .exec(&txn)
            .await?;
    }

    let mut notebook_id = 0_i64;
    let mut notebook_name = String::default();

    if note.notebook_id > 0
        && let Some(notebook) = entity::notebook::Entity::find_by_id(note.notebook_id)
            .one(&txn)
            .await?
        {
            notebook_id = notebook.id;
            notebook_name = notebook.name;
        }

    let extra = serde_json::to_string(&NoteHistoryExtra {
        notebook_id,
        notebook_name,
        content_type: note.content_type,
        title: note.title.clone(),
        tags: note.tags.clone(),
    })?;

    entity::note_history::ActiveModel {
        id: NotSet,
        note_id: Set(entity.id),
        old_content: Set(String::default()),
        new_content: Set(note.content.clone()),
        extra: Set(extra),
        operate_type: Set(OperationType::Create.as_i32()),
        operate_source: Set(source.as_i32()),
        operate_time: Set(now),
        create_time: Set(now),
    }
    .insert(&txn)
    .await?;

    txn.commit().await?;

    find_by_id_with_key(db, entity.id, encryption_key).await
}

/// 创建笔记（跳过历史记录生成，专用于同步场景）
///
/// 同步时原始历史记录会单独同步，此函数避免生成重复的 Create 历史。
pub async fn create_for_sync(
    db: &DatabaseConnection,
    note: &Note,
    encryption_key: Option<&str>,
) -> anyhow::Result<Option<Note>> {
    let txn = db.begin().await?;

    let now = Local::now().naive_local();

    let encrypted_content = encrypt_content(&note.content, encryption_key)?;

    let mut active_model: entity::note::ActiveModel = note.into();

    active_model.id = NotSet;
    active_model.is_pinned = Set(0);
    active_model.is_starred = Set(0);
    active_model.content = Set(encrypted_content);
    active_model.create_time = Set(now);
    active_model.update_time = Set(now);
    active_model.deleted_at = Set(None);

    let entity = active_model.insert(&txn).await?;

    if !note.tags.is_empty() {
        let note_tags = note
            .tags
            .iter()
            .map(|e| entity::note_tags::ActiveModel {
                id: NotSet,
                note_id: Set(entity.id),
                tag_id: Set(e.id),
                sort_order: Set(e.sort_order),
                create_time: Set(now),
                update_time: Set(now),
            })
            .collect::<Vec<entity::note_tags::ActiveModel>>();

        entity::note_tags::Entity::insert_many(note_tags)
            .exec(&txn)
            .await?;
    }

    txn.commit().await?;

    find_by_id_with_key(db, entity.id, encryption_key).await
}

/// 软删除笔记（移入回收站）
pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
    if let Some(entity) = entity::note::Entity::find_by_id(id).one(db).await? {
        let now = Local::now().naive_local();
        let mut active_model: entity::note::ActiveModel = entity.into_active_model();
        active_model.deleted_at = Set(Some(now));
        active_model.update_time = Set(now);
        active_model.update(db).await?;
    }
    Ok(())
}

/// 从回收站恢复笔记
pub async fn restore_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
    if let Some(entity) = entity::note::Entity::find_by_id(id).one(db).await? {
        let now = Local::now().naive_local();
        let mut active_model: entity::note::ActiveModel = entity.into_active_model();
        active_model.deleted_at = Set(None);
        active_model.update_time = Set(now);
        active_model.update(db).await?;
    }
    Ok(())
}

/// 在给定连接上执行单条笔记的永久删除（含历史记录生成）
async fn permanent_delete_one<C: ConnectionTrait>(
    db: &C,
    id: i64,
    source: OperateSource,
    encryption_key: Option<&str>,
) -> anyhow::Result<()> {
    let result = entity::note::Entity::find_by_id(id)
        .find_also_related(entity::notebook::Entity)
        .one(db)
        .await?;

    if let Some((entity, notebook_opt)) = result {
        let tags = fetch_note_tags(db, id).await?;

        let (notebook_id, notebook_name) = match notebook_opt {
            Some(notebook) => (notebook.id, notebook.name.clone()),
            None => (0, String::default()),
        };

        let now = Local::now().naive_local();

        let note_history_extra = NoteHistoryExtra {
            notebook_id,
            notebook_name,
            content_type: entity.content_type,
            title: entity.title.clone(),
            tags,
        };

        let extra = serde_json::to_string(&note_history_extra)?;

        // 历史记录统一存明文（与 create/update 保持一致）
        let old_content_plain = decrypt_content(&entity.content, encryption_key);

        entity::note_history::ActiveModel {
            id: NotSet,
            note_id: Set(id),
            old_content: Set(old_content_plain),
            new_content: Set(String::default()),
            extra: Set(extra),
            operate_type: Set(OperationType::Delete.as_i32()),
            operate_source: Set(source.as_i32()),
            operate_time: Set(now),
            create_time: Set(now),
        }
        .insert(db)
        .await?;

        entity::note_tags::Entity::delete_many()
            .filter(entity::note_tags::Column::NoteId.eq(id))
            .exec(db)
            .await?;
        entity::note_link::Entity::delete_many()
            .filter(
                Condition::any()
                    .add(entity::note_link::Column::SourceNoteId.eq(id))
                    .add(entity::note_link::Column::TargetNoteId.eq(id)),
            )
            .exec(db)
            .await?;
        entity::note::Entity::delete_by_id(id).exec(db).await?;
    }

    Ok(())
}

/// 永久删除笔记（从回收站中彻底删除）
pub async fn permanent_delete_by_id(
    db: &DatabaseConnection,
    id: i64,
    source: OperateSource,
    encryption_key: Option<&str>,
) -> anyhow::Result<()> {
    let txn = db.begin().await?;
    permanent_delete_one(&txn, id, source, encryption_key).await?;
    txn.commit().await?;
    Ok(())
}

/// 清空回收站（永久删除所有软删除的笔记）
pub async fn empty_trash(
    db: &DatabaseConnection,
    encryption_key: Option<&str>,
) -> anyhow::Result<()> {
    let deleted_notes = entity::note::Entity::find()
        .filter(entity::note::Column::DeletedAt.is_not_null())
        .all(db)
        .await?;

    if deleted_notes.is_empty() {
        return Ok(());
    }

    let txn = db.begin().await?;

    for note in deleted_notes {
        permanent_delete_one(&txn, note.id, OperateSource::User, encryption_key).await?;
    }

    txn.commit().await?;
    Ok(())
}

/// 切换笔记收藏/星标状态
pub async fn toggle_star(db: &DatabaseConnection, id: i64) -> anyhow::Result<Option<Note>> {
    if let Some(entity) = entity::note::Entity::find_by_id(id).one(db).await? {
        let now = Local::now().naive_local();
        let new_starred = if entity.is_starred == 1 { 0 } else { 1 };
        let mut active_model: entity::note::ActiveModel = entity.into_active_model();
        active_model.is_starred = Set(new_starred);
        active_model.update_time = Set(now);
        active_model.update(db).await?;
    }
    find_by_id(db, id).await
}

/// 切换笔记置顶状态
pub async fn toggle_pin(db: &DatabaseConnection, id: i64) -> anyhow::Result<Option<Note>> {
    if let Some(entity) = entity::note::Entity::find_by_id(id).one(db).await? {
        let now = Local::now().naive_local();
        let new_pinned = if entity.is_pinned == 1 { 0 } else { 1 };
        let mut active_model: entity::note::ActiveModel = entity.into_active_model();
        active_model.is_pinned = Set(new_pinned);
        active_model.update_time = Set(now);
        active_model.update(db).await?;
    }
    find_by_id(db, id).await
}

/// 更新笔记
pub async fn update(
    db: &DatabaseConnection,
    note: &Note,
    source: OperateSource,
) -> anyhow::Result<Option<Note>> {
    update_with_key(db, note, source, None).await
}

/// 更新笔记（支持内容加密）
pub async fn update_with_key(
    db: &DatabaseConnection,
    note: &Note,
    source: OperateSource,
    encryption_key: Option<&str>,
) -> anyhow::Result<Option<Note>> {
    if let Some(entity) = entity::note::Entity::find_by_id(note.id).one(db).await? {
        let old_title = entity.title.clone();

        let txn = db.begin().await?;

        // 获取旧标签关联 ID（用于差集计算）
        let old_tag_ids: Vec<i64> = entity::note_tags::Entity::find()
            .filter(entity::note_tags::Column::NoteId.eq(note.id))
            .order_by_desc(entity::note_tags::Column::SortOrder)
            .all(&txn)
            .await?
            .into_iter()
            .map(|nt| nt.tag_id)
            .collect();

        // 获取旧标签详情（用于历史记录）
        let old_tags = fetch_note_tags(&txn, note.id).await?;

        let old_content_type = entity.content_type;
        let old_content = entity.content.clone();

        // 透明加密：对新内容进行加密
        let encrypted_content = encrypt_content(&note.content, encryption_key)?;

        let mut active_model: entity::note::ActiveModel = entity.into_active_model();

        active_model.notebook_id.set_if_not_equals(note.notebook_id);
        active_model.title.set_if_not_equals(note.title.clone());
        active_model.content.set_if_not_equals(encrypted_content);
        active_model
            .content_type
            .set_if_not_equals(note.content_type);

        let now = Local::now().naive_local();

        let note_changed = active_model.is_changed();

        if note_changed {
            active_model.update_time = Set(now);

            active_model.update(&txn).await?;
        }

        let new_tag_ids = note.tags.iter().map(|e| e.id).collect::<Vec<i64>>();

        let tags_changed: bool = old_tag_ids != new_tag_ids;

        if tags_changed {
            // 使用差集计算，只删除和添加变化的部分
            let old_set: HashSet<i64> = old_tag_ids.iter().cloned().collect();
            let new_set: HashSet<i64> = new_tag_ids.iter().cloned().collect();

            // 需要删除的标签：旧的有，新的没有
            let to_delete: Vec<i64> = old_set.difference(&new_set).cloned().collect();
            // 需要添加的标签：新的有，旧的没有
            let to_add: Vec<i64> = new_set.difference(&old_set).cloned().collect();

            // 删除不再需要的标签关联
            if !to_delete.is_empty() {
                entity::note_tags::Entity::delete_many()
                    .filter(entity::note_tags::Column::NoteId.eq(note.id))
                    .filter(entity::note_tags::Column::TagId.is_in(to_delete))
                    .exec(&txn)
                    .await?;
            }

            // 添加新的标签关联
            if !to_add.is_empty() {
                let new_note_tags = note
                    .tags
                    .iter()
                    .filter(|e| to_add.contains(&e.id))
                    .map(|e| entity::note_tags::ActiveModel {
                        id: NotSet,
                        note_id: Set(note.id),
                        tag_id: Set(e.id),
                        sort_order: Set(e.sort_order),
                        create_time: Set(now),
                        update_time: Set(now),
                    })
                    .collect::<Vec<entity::note_tags::ActiveModel>>();

                entity::note_tags::Entity::insert_many(new_note_tags)
                    .exec(&txn)
                    .await?;
            }
        }

        if note_changed || tags_changed {
            let mut notebook_id = 0_i64;
            let mut notebook_name = String::default();

            if note.notebook_id > 0
                && let Some(notebook) = entity::notebook::Entity::find_by_id(note.notebook_id)
                    .one(&txn)
                    .await?
                {
                    notebook_id = notebook.id;
                    notebook_name = notebook.name;
                }

            let extra = serde_json::to_string(&NoteHistoryExtra {
                notebook_id,
                notebook_name,
                content_type: old_content_type,
                title: old_title,
                tags: old_tags,
            })?;

            entity::note_history::ActiveModel {
                id: NotSet,
                note_id: Set(note.id),
                old_content: Set(old_content),
                new_content: Set(note.content.clone()),
                extra: Set(extra),
                operate_type: Set(OperationType::Update.as_i32()),
                operate_source: Set(source.as_i32()),
                operate_time: Set(now),
                create_time: Set(now),
            }
            .insert(&txn)
            .await?;
        }

        txn.commit().await?;
    }

    find_by_id_with_key(db, note.id, encryption_key).await
}

/// 批量移动笔记到指定笔记本
pub async fn batch_move(
    db: &DatabaseConnection,
    note_ids: &[i64],
    notebook_id: i64,
) -> anyhow::Result<()> {
    if note_ids.is_empty() {
        return Ok(());
    }
    let now = Local::now().naive_local();
    let txn = db.begin().await?;
    entity::note::Entity::update_many()
        .col_expr(entity::note::Column::NotebookId, Expr::value(notebook_id))
        .col_expr(entity::note::Column::UpdateTime, Expr::value(now))
        .filter(entity::note::Column::Id.is_in(note_ids.to_vec()))
        .filter(entity::note::Column::DeletedAt.is_null())
        .exec(&txn)
        .await?;
    txn.commit().await?;
    Ok(())
}

/// 批量软删除笔记
pub async fn batch_delete(
    db: &DatabaseConnection,
    note_ids: &[i64],
) -> anyhow::Result<()> {
    if note_ids.is_empty() {
        return Ok(());
    }
    let now = Local::now().naive_local();
    let txn = db.begin().await?;
    entity::note::Entity::update_many()
        .col_expr(
            entity::note::Column::DeletedAt,
            Expr::value(Some(now)),
        )
        .col_expr(entity::note::Column::UpdateTime, Expr::value(now))
        .filter(entity::note::Column::Id.is_in(note_ids.to_vec()))
        .filter(entity::note::Column::DeletedAt.is_null())
        .exec(&txn)
        .await?;
    txn.commit().await?;
    Ok(())
}
