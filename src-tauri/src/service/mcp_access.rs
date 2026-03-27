//! MCP 访问控制服务模块
//!
//! 实现笔记的三层访问控制权限解析：
//! 1. 加密笔记 → 强制 Deny
//! 2. note.mcp_access != Inherit → 使用笔记自身设置
//! 3. 笔记的标签中，取所有非 Inherit 标签的最严格值
//! 4. notebook.mcp_access != Inherit → 使用笔记本设置
//! 5. 以上都是 Inherit → 默认 ReadWrite

use std::collections::{HashMap, HashSet};

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entity,
    model::{McpAccess, McpPermission, Note},
    service::crypto,
};

/// 解析单条笔记的有效 MCP 权限
pub async fn resolve_note_access(
    db: &DatabaseConnection,
    note_id: i64,
) -> anyhow::Result<McpPermission> {
    let result = entity::note::Entity::find_by_id(note_id)
        .find_also_related(entity::notebook::Entity)
        .one(db)
        .await?;

    let Some((note_entity, notebook_opt)) = result else {
        anyhow::bail!("Note ID {} not found", note_id);
    };

    // 1. 加密笔记 → 强制 Deny
    if crypto::is_encrypted(&note_entity.content) {
        return Ok(McpPermission::Deny);
    }

    // 2. 笔记自身设置
    let note_access = McpAccess::from(note_entity.mcp_access);
    if note_access != McpAccess::Inherit {
        return Ok(mcp_access_to_permission(note_access));
    }

    // 3. 标签权限（取最严格值）
    let tag_permission = resolve_tags_access(db, note_id).await?;
    if let Some(perm) = tag_permission {
        // 4. 与笔记本权限比较，取最严格值
        if let Some(notebook) = &notebook_opt {
            let nb_access = McpAccess::from(notebook.mcp_access);
            if nb_access != McpAccess::Inherit {
                return Ok(perm.stricter(mcp_access_to_permission(nb_access)));
            }
        }
        return Ok(perm);
    }

    // 4. 笔记本设置
    if let Some(notebook) = notebook_opt {
        let nb_access = McpAccess::from(notebook.mcp_access);
        if nb_access != McpAccess::Inherit {
            return Ok(mcp_access_to_permission(nb_access));
        }
    }

    // 5. 默认 ReadWrite
    Ok(McpPermission::ReadWrite)
}

/// 批量解析笔记的有效 MCP 权限
///
/// 使用 3 次查询完成（notebook 批量查、tag 批量查、内存计算）
pub async fn resolve_notes_access(
    db: &DatabaseConnection,
    notes: &[Note],
) -> anyhow::Result<HashMap<i64, McpPermission>> {
    let mut result = HashMap::with_capacity(notes.len());

    if notes.is_empty() {
        return Ok(result);
    }

    // 收集需要查询的笔记本 ID
    let notebook_ids: HashSet<i64> = notes
        .iter()
        .filter(|n| n.notebook_id > 0)
        .map(|n| n.notebook_id)
        .collect();

    // 批量查询笔记本的 mcp_access
    let notebook_access: HashMap<i64, i32> = if !notebook_ids.is_empty() {
        entity::notebook::Entity::find()
            .filter(entity::notebook::Column::Id.is_in(notebook_ids))
            .all(db)
            .await?
            .into_iter()
            .map(|nb| (nb.id, nb.mcp_access))
            .collect()
    } else {
        HashMap::new()
    };

    // 收集所有笔记 ID
    let note_ids: Vec<i64> = notes.iter().map(|n| n.id).collect();

    // 批量查询笔记-标签关联
    let note_tags = entity::note_tags::Entity::find()
        .filter(entity::note_tags::Column::NoteId.is_in(note_ids))
        .all(db)
        .await?;

    // 收集需要查询的标签 ID
    let tag_ids: HashSet<i64> = note_tags.iter().map(|nt| nt.tag_id).collect();

    // 批量查询标签的 mcp_access
    let tag_access: HashMap<i64, i32> = if !tag_ids.is_empty() {
        entity::tag::Entity::find()
            .filter(entity::tag::Column::Id.is_in(tag_ids))
            .all(db)
            .await?
            .into_iter()
            .map(|t| (t.id, t.mcp_access))
            .collect()
    } else {
        HashMap::new()
    };

    // 构建 note_id -> tag_ids 映射
    let mut note_tag_map: HashMap<i64, Vec<i64>> = HashMap::new();
    for nt in &note_tags {
        note_tag_map.entry(nt.note_id).or_default().push(nt.tag_id);
    }

    // 内存计算每条笔记的权限
    for note in notes {
        // 1. 加密笔记 → Deny
        if crypto::is_encrypted(&note.content) {
            result.insert(note.id, McpPermission::Deny);
            continue;
        }

        // 2. 笔记自身设置
        let note_mcp = McpAccess::from(note.mcp_access);
        if note_mcp != McpAccess::Inherit {
            result.insert(note.id, mcp_access_to_permission(note_mcp));
            continue;
        }

        // 3. 标签权限（取最严格值）
        let tag_perm = if let Some(tids) = note_tag_map.get(&note.id) {
            let mut strictest: Option<McpPermission> = None;
            for tid in tids {
                if let Some(&access) = tag_access.get(tid) {
                    let a = McpAccess::from(access);
                    if a != McpAccess::Inherit {
                        let p = mcp_access_to_permission(a);
                        strictest = Some(match strictest {
                            Some(s) => s.stricter(p),
                            None => p,
                        });
                    }
                }
            }
            strictest
        } else {
            None
        };

        // 4. 笔记本设置
        let nb_perm = notebook_access
            .get(&note.notebook_id)
            .map(|&a| McpAccess::from(a))
            .filter(|a| *a != McpAccess::Inherit)
            .map(mcp_access_to_permission);

        // 合并：标签和笔记本取最严格值
        let effective = match (tag_perm, nb_perm) {
            (Some(tp), Some(np)) => tp.stricter(np),
            (Some(tp), None) => tp,
            (None, Some(np)) => np,
            (None, None) => McpPermission::ReadWrite, // 5. 默认
        };

        result.insert(note.id, effective);
    }

    Ok(result)
}

/// 检查笔记是否可读，不可读则返回错误
pub async fn check_read(db: &DatabaseConnection, note_id: i64) -> anyhow::Result<()> {
    let perm = resolve_note_access(db, note_id).await?;
    if !perm.can_read() {
        anyhow::bail!("AI access denied for this note (mcp_access=Deny)");
    }
    Ok(())
}

/// 检查笔记是否可写，不可写则返回错误
pub async fn check_write(db: &DatabaseConnection, note_id: i64) -> anyhow::Result<()> {
    let perm = resolve_note_access(db, note_id).await?;
    if !perm.can_write() {
        if !perm.can_read() {
            anyhow::bail!("AI access denied for this note (mcp_access=Deny)");
        }
        anyhow::bail!("This note is read-only for AI (mcp_access=ReadOnly)");
    }
    Ok(())
}

/// 检查笔记本是否可写（用于 create_note 时检查目标笔记本）
pub async fn check_notebook_write(db: &DatabaseConnection, notebook_id: i64) -> anyhow::Result<()> {
    if notebook_id == 0 {
        return Ok(()); // 不归属笔记本，允许
    }

    let notebook = entity::notebook::Entity::find_by_id(notebook_id)
        .one(db)
        .await?;

    if let Some(nb) = notebook {
        let access = McpAccess::from(nb.mcp_access);
        match access {
            McpAccess::Deny => {
                anyhow::bail!("AI access denied for target notebook (mcp_access=Deny)");
            }
            McpAccess::ReadOnly => {
                anyhow::bail!("Target notebook is read-only, AI cannot create notes here (mcp_access=ReadOnly)");
            }
            _ => {}
        }
    }

    Ok(())
}

/// 将 McpAccess 转换为 McpPermission（不含 Inherit）
fn mcp_access_to_permission(access: McpAccess) -> McpPermission {
    match access {
        McpAccess::ReadWrite | McpAccess::Inherit => McpPermission::ReadWrite,
        McpAccess::ReadOnly => McpPermission::ReadOnly,
        McpAccess::Deny => McpPermission::Deny,
    }
}

/// 解析笔记标签的聚合权限
async fn resolve_tags_access(
    db: &DatabaseConnection,
    note_id: i64,
) -> anyhow::Result<Option<McpPermission>> {
    let note_tags = entity::note_tags::Entity::find()
        .filter(entity::note_tags::Column::NoteId.eq(note_id))
        .all(db)
        .await?;

    if note_tags.is_empty() {
        return Ok(None);
    }

    let tag_ids: Vec<i64> = note_tags.iter().map(|nt| nt.tag_id).collect();

    let tags = entity::tag::Entity::find()
        .filter(entity::tag::Column::Id.is_in(tag_ids))
        .all(db)
        .await?;

    let mut strictest: Option<McpPermission> = None;
    for tag in &tags {
        let access = McpAccess::from(tag.mcp_access);
        if access != McpAccess::Inherit {
            let perm = mcp_access_to_permission(access);
            strictest = Some(match strictest {
                Some(s) => s.stricter(perm),
                None => perm,
            });
        }
    }

    Ok(strictest)
}
