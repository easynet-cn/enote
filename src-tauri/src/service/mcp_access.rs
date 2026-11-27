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
    model::{McpAccess, McpPermission, Note, Notebook, Todo, TodoList},
    service::crypto,
};

// ============================================================================
// 后端无关的权限计算
//
// 以下函数只依赖已加载的数据模型，不持有任何数据库连接。
// store trait 的默认实现用它们在内存中解析权限，使 Database / Git / Server
// 三种后端共享同一套规则，避免权限逻辑出现三份实现而逐渐漂移。
// ============================================================================

/// 取一组访问控制值中最严格的非 Inherit 权限
fn strictest_access<I>(accesses: I) -> Option<McpPermission>
where
    I: IntoIterator<Item = i32>,
{
    let mut strictest: Option<McpPermission> = None;
    for access in accesses {
        let a = McpAccess::from(access);
        if a != McpAccess::Inherit {
            let perm = mcp_access_to_permission(a);
            strictest = Some(match strictest {
                Some(s) => s.stricter(perm),
                None => perm,
            });
        }
    }
    strictest
}

/// 基于已加载的 Note（含 tags）与笔记本列表计算单条笔记权限
///
/// 优先级与 [`resolve_note_access`] 完全一致：
/// 加密 → 笔记自身 → 标签 → 笔记本 → 默认 ReadWrite
pub fn note_permission(note: &Note, notebooks: &[Notebook]) -> McpPermission {
    // 1. 加密笔记 → 强制 Deny
    if crypto::is_encrypted(&note.content) {
        return McpPermission::Deny;
    }

    // 2. 笔记自身设置
    let note_access = McpAccess::from(note.mcp_access);
    if note_access != McpAccess::Inherit {
        return mcp_access_to_permission(note_access);
    }

    // 3. 标签权限（Note.tags 由各 Store 在查询时填充，已带 mcp_access）
    let tag_perm = strictest_access(note.tags.iter().map(|t| t.mcp_access));

    // 4. 笔记本设置
    let notebook_perm = notebooks
        .iter()
        .find(|nb| nb.id == note.notebook_id)
        .map(|nb| McpAccess::from(nb.mcp_access))
        .filter(|a| *a != McpAccess::Inherit)
        .map(mcp_access_to_permission);

    // 5. 合并并回退到默认 ReadWrite
    match (tag_perm, notebook_perm) {
        (Some(tp), Some(np)) => tp.stricter(np),
        (Some(tp), None) => tp,
        (None, Some(np)) => np,
        (None, None) => McpPermission::ReadWrite,
    }
}

/// 批量计算笔记权限
pub fn notes_permissions(notes: &[Note], notebooks: &[Notebook]) -> HashMap<i64, McpPermission> {
    notes
        .iter()
        .map(|n| (n.id, note_permission(n, notebooks)))
        .collect()
}

/// 基于已加载的 Todo 计算单条待办权限
///
/// `tag_accesses` 为该待办关联标签的 mcp_access 值；`linked_note_perm` 为关联笔记的
/// 有效权限（None 表示无关联笔记或未解析）。
pub fn todo_permission(
    todo: &Todo,
    lists: &[TodoList],
    tag_accesses: &[i32],
    linked_note_perm: Option<McpPermission>,
) -> McpPermission {
    // 1. 关联笔记被 Deny → 连带 Deny（安全优先，不可被自身设置覆盖）
    if linked_note_perm == Some(McpPermission::Deny) {
        return McpPermission::Deny;
    }

    // 2. 待办自身设置
    let todo_access = McpAccess::from(todo.mcp_access);
    if todo_access != McpAccess::Inherit {
        return mcp_access_to_permission(todo_access);
    }

    // 3. 标签权限
    let tag_perm = strictest_access(tag_accesses.iter().copied());

    // 4. 清单设置
    let list_perm = lists
        .iter()
        .find(|l| Some(l.id) == todo.list_id)
        .map(|l| McpAccess::from(l.mcp_access))
        .filter(|a| *a != McpAccess::Inherit)
        .map(mcp_access_to_permission);

    // 5. 合并并回退到默认 ReadWrite
    match (tag_perm, list_perm) {
        (Some(tp), Some(lp)) => tp.stricter(lp),
        (Some(tp), None) => tp,
        (None, Some(lp)) => lp,
        (None, None) => McpPermission::ReadWrite,
    }
}

/// 批量计算待办权限
///
/// `tag_accesses_by_todo` 的 key 为待办 ID，value 为其关联标签的 mcp_access 列表。
pub fn todos_permissions(
    todos: &[Todo],
    lists: &[TodoList],
    tag_accesses_by_todo: &HashMap<i64, Vec<i32>>,
    note_perms: &HashMap<i64, McpPermission>,
) -> HashMap<i64, McpPermission> {
    todos
        .iter()
        .map(|t| {
            let linked_perm = t.note_id.and_then(|id| note_perms.get(&id).copied());
            let tag_accesses = tag_accesses_by_todo
                .get(&t.id)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            (t.id, todo_permission(t, lists, tag_accesses, linked_perm))
        })
        .collect()
}

/// 校验权限是否满足读取/写入要求，错误信息与原有 MCP 工具保持一致
pub fn ensure_permission(
    perm: McpPermission,
    entity_kind: &str,
    write: bool,
) -> anyhow::Result<()> {
    if !perm.can_read() {
        anyhow::bail!("AI access denied for this {} (mcp_access=Deny)", entity_kind);
    }
    if write && !perm.can_write() {
        anyhow::bail!("This {} is read-only for AI (mcp_access=ReadOnly)", entity_kind);
    }
    Ok(())
}

/// 检查目标笔记本是否允许 AI 写入
pub fn ensure_notebook_write(notebook: Option<&Notebook>) -> anyhow::Result<()> {
    let Some(nb) = notebook else {
        return Ok(()); // 不归属笔记本，允许
    };
    match McpAccess::from(nb.mcp_access) {
        McpAccess::Deny => {
            anyhow::bail!("AI access denied for target notebook (mcp_access=Deny)");
        }
        McpAccess::ReadOnly => {
            anyhow::bail!(
                "Target notebook is read-only, AI cannot create notes here (mcp_access=ReadOnly)"
            );
        }
        _ => Ok(()),
    }
}

/// 检查目标待办清单是否允许 AI 写入
pub fn ensure_todo_list_write(list: Option<&TodoList>) -> anyhow::Result<()> {
    let Some(list) = list else {
        return Ok(()); // 未归类，放行
    };
    match McpAccess::from(list.mcp_access) {
        McpAccess::Deny => {
            anyhow::bail!("AI access denied for target todo list (mcp_access=Deny)");
        }
        McpAccess::ReadOnly => {
            anyhow::bail!(
                "Target todo list is read-only, AI cannot create todos here (mcp_access=ReadOnly)"
            );
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Tag;

    fn tag(id: i64, access: i32) -> Tag {
        Tag {
            id,
            mcp_access: access,
            ..Default::default()
        }
    }

    fn notebook(id: i64, access: i32) -> Notebook {
        Notebook {
            id,
            mcp_access: access,
            ..Default::default()
        }
    }

    fn note(id: i64, access: i32, tags: Vec<Tag>) -> Note {
        Note {
            id,
            mcp_access: access,
            tags,
            ..Default::default()
        }
    }

    fn todo(id: i64, access: i32, list_id: Option<i64>, note_id: Option<i64>) -> Todo {
        Todo {
            id,
            mcp_access: access,
            list_id,
            note_id,
            ..Default::default()
        }
    }

    fn list(id: i64, access: i32) -> TodoList {
        TodoList {
            id,
            mcp_access: access,
            ..Default::default()
        }
    }

    #[test]
    fn note_permission_priority() {
        // 全 Inherit → 默认 ReadWrite
        assert_eq!(note_permission(&note(1, 0, vec![]), &[]), McpPermission::ReadWrite);

        // 笔记自身设置优先于标签和笔记本
        let n = note(1, 2, vec![tag(1, 1)]);
        assert_eq!(note_permission(&n, &[notebook(9, 1)]), McpPermission::ReadOnly);

        // 标签与笔记本取更严格者
        let mut n = note(1, 0, vec![tag(1, 1), tag(2, 2)]);
        n.notebook_id = 9;
        assert_eq!(note_permission(&n, &[notebook(9, 1)]), McpPermission::ReadOnly);

        // 标签 Deny 压过笔记本 ReadWrite
        let n = note(1, 0, vec![tag(1, 3)]);
        assert_eq!(note_permission(&n, &[notebook(9, 1)]), McpPermission::Deny);

        // 笔记本 ReadOnly 生效
        let mut n = note(1, 0, vec![]);
        n.notebook_id = 9;
        assert_eq!(note_permission(&n, &[notebook(9, 2)]), McpPermission::ReadOnly);
    }

    #[test]
    fn note_permission_denies_encrypted_content() {
        let mut n = note(1, 1, vec![]);
        n.content = crypto::encrypt("secret", "key").unwrap();
        assert_eq!(note_permission(&n, &[]), McpPermission::Deny);
    }

    #[test]
    fn todo_permission_priority() {
        // 关联笔记 Deny 连带禁止，且不可被待办自身 ReadWrite 覆盖
        let t = todo(1, 1, Some(1), Some(9));
        assert_eq!(
            todo_permission(&t, &[list(1, 1)], &[], Some(McpPermission::Deny)),
            McpPermission::Deny
        );

        // 待办自身 ReadOnly 优先于标签与清单
        let t = todo(1, 2, Some(1), None);
        assert_eq!(
            todo_permission(&t, &[list(1, 1)], &[1], None),
            McpPermission::ReadOnly
        );

        // 标签 Deny 压过清单 ReadWrite
        let t = todo(1, 0, Some(1), None);
        assert_eq!(
            todo_permission(&t, &[list(1, 1)], &[1, 3], None),
            McpPermission::Deny
        );

        // 全 Inherit → 默认 ReadWrite
        let t = todo(1, 0, Some(1), None);
        assert_eq!(todo_permission(&t, &[list(1, 0)], &[0], None), McpPermission::ReadWrite);
    }

    #[test]
    fn batch_permissions_match_single() {
        let n1 = note(1, 0, vec![tag(1, 2)]);
        let n2 = note(2, 3, vec![]);
        let notebooks = [notebook(9, 1)];
        let perms = notes_permissions(&[n1.clone(), n2.clone()], &notebooks);
        assert_eq!(perms[&1], note_permission(&n1, &notebooks));
        assert_eq!(perms[&2], note_permission(&n2, &notebooks));

        let lists = [list(1, 2)];
        let t1 = todo(1, 0, Some(1), Some(1));
        let mut tag_map = HashMap::new();
        tag_map.insert(1, vec![0]);
        let note_perms = HashMap::from([(1, McpPermission::ReadWrite)]);
        let todo_perms = todos_permissions(&[t1.clone()], &lists, &tag_map, &note_perms);
        assert_eq!(
            todo_perms[&1],
            todo_permission(&t1, &lists, &[0], Some(McpPermission::ReadWrite))
        );
    }

    #[test]
    fn ensure_permission_messages() {
        assert!(ensure_permission(McpPermission::ReadWrite, "note", true).is_ok());
        assert!(ensure_permission(McpPermission::ReadOnly, "note", false).is_ok());
        assert_eq!(
            ensure_permission(McpPermission::ReadOnly, "note", true)
                .unwrap_err()
                .to_string(),
            "This note is read-only for AI (mcp_access=ReadOnly)"
        );
        assert_eq!(
            ensure_permission(McpPermission::Deny, "todo", false)
                .unwrap_err()
                .to_string(),
            "AI access denied for this todo (mcp_access=Deny)"
        );
    }

    #[test]
    fn write_guard_messages() {
        assert!(ensure_notebook_write(None).is_ok());
        assert!(ensure_notebook_write(Some(&notebook(1, 1))).is_ok());
        assert_eq!(
            ensure_notebook_write(Some(&notebook(1, 3)))
                .unwrap_err()
                .to_string(),
            "AI access denied for target notebook (mcp_access=Deny)"
        );
        assert!(ensure_todo_list_write(None).is_ok());
        assert!(ensure_todo_list_write(Some(&list(1, 1))).is_ok());
        assert_eq!(
            ensure_todo_list_write(Some(&list(1, 2)))
                .unwrap_err()
                .to_string(),
            "Target todo list is read-only, AI cannot create todos here (mcp_access=ReadOnly)"
        );
    }
}

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
                anyhow::bail!(
                    "Target notebook is read-only, AI cannot create notes here (mcp_access=ReadOnly)"
                );
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

// ============================================================================
// 待办访问控制
// ============================================================================

/// 解析待办标签的聚合权限（取所有非 Inherit 标签中最严格的值）
async fn resolve_todo_tags_access(
    db: &DatabaseConnection,
    todo_id: i64,
) -> anyhow::Result<Option<McpPermission>> {
    let todo_tags = entity::todo_tags::Entity::find()
        .filter(entity::todo_tags::Column::TodoId.eq(todo_id))
        .all(db)
        .await?;

    if todo_tags.is_empty() {
        return Ok(None);
    }

    let tag_ids: Vec<i64> = todo_tags.iter().map(|tt| tt.tag_id).collect();

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

/// 解析单条待办实体的有效 MCP 权限
///
/// 解析优先级（与笔记体系对齐，并增加关联笔记的连带禁止）：
/// 1. 关联笔记（note_id）解析为 Deny → 连带 Deny（安全优先，不可被待办自身设置覆盖）
/// 2. 待办自身设置（mcp_access != Inherit）
/// 3. 待办标签中非 Inherit 的最严格值
/// 4. 所属清单（todo_list）设置
/// 5. 3 与 4 取更严格者
/// 6. 以上均 Inherit → ReadWrite
async fn resolve_todo_model_access(
    db: &DatabaseConnection,
    todo: &entity::todo::Model,
) -> anyhow::Result<McpPermission> {
    // 1. 关联笔记连带禁止
    if let Some(note_id) = todo.note_id {
        if let Ok(perm) = resolve_note_access(db, note_id).await {
            if perm == McpPermission::Deny {
                return Ok(McpPermission::Deny);
            }
        }
    }

    // 2. 待办自身设置
    let todo_access = McpAccess::from(todo.mcp_access);
    if todo_access != McpAccess::Inherit {
        return Ok(mcp_access_to_permission(todo_access));
    }

    // 3. 标签权限（取最严格值）
    let tag_perm = resolve_todo_tags_access(db, todo.id).await?;

    // 4. 清单设置
    let list_perm = match todo.list_id {
        Some(list_id) => entity::todo_list::Entity::find_by_id(list_id)
            .one(db)
            .await?
            .and_then(|l| {
                let access = McpAccess::from(l.mcp_access);
                if access != McpAccess::Inherit {
                    Some(mcp_access_to_permission(access))
                } else {
                    None
                }
            }),
        None => None,
    };

    // 5. 合并：标签与清单取更严格者，均无表态则默认 ReadWrite
    Ok(match (tag_perm, list_perm) {
        (Some(tp), Some(lp)) => tp.stricter(lp),
        (Some(tp), None) => tp,
        (None, Some(lp)) => lp,
        (None, None) => McpPermission::ReadWrite,
    })
}

/// 解析单条待办的有效 MCP 权限
pub async fn resolve_todo_access(
    db: &DatabaseConnection,
    todo_id: i64,
) -> anyhow::Result<McpPermission> {
    let Some(todo) = entity::todo::Entity::find_by_id(todo_id).one(db).await? else {
        anyhow::bail!("Todo ID {} not found", todo_id);
    };

    resolve_todo_model_access(db, &todo).await
}

/// 批量解析待办的有效 MCP 权限
///
/// 使用 4 次查询完成（清单批量查、待办-标签批量查、标签批量查、关联笔记批量解析），
/// 其余在内存中计算。
pub async fn resolve_todos_access(
    db: &DatabaseConnection,
    todos: &[Todo],
) -> anyhow::Result<HashMap<i64, McpPermission>> {
    let mut result = HashMap::with_capacity(todos.len());

    if todos.is_empty() {
        return Ok(result);
    }

    let todo_ids: Vec<i64> = todos.iter().map(|t| t.id).collect();

    // 批量查询清单的 mcp_access
    let list_ids: HashSet<i64> = todos.iter().filter_map(|t| t.list_id).collect();
    let list_access: HashMap<i64, i32> = if !list_ids.is_empty() {
        entity::todo_list::Entity::find()
            .filter(entity::todo_list::Column::Id.is_in(list_ids))
            .all(db)
            .await?
            .into_iter()
            .map(|l| (l.id, l.mcp_access))
            .collect()
    } else {
        HashMap::new()
    };

    // 批量查询待办-标签关联
    let todo_tags = entity::todo_tags::Entity::find()
        .filter(entity::todo_tags::Column::TodoId.is_in(todo_ids))
        .all(db)
        .await?;

    let tag_ids: HashSet<i64> = todo_tags.iter().map(|tt| tt.tag_id).collect();
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

    // 构建 todo_id -> tag_ids 映射
    let mut todo_tag_map: HashMap<i64, Vec<i64>> = HashMap::new();
    for tt in &todo_tags {
        todo_tag_map.entry(tt.todo_id).or_default().push(tt.tag_id);
    }

    // 批量解析关联笔记权限（用于连带禁止）
    let note_ids: Vec<i64> = todos.iter().filter_map(|t| t.note_id).collect();
    let note_access: HashMap<i64, McpPermission> = if note_ids.is_empty() {
        HashMap::new()
    } else {
        let notes = entity::note::Entity::find()
            .filter(entity::note::Column::Id.is_in(note_ids))
            .all(db)
            .await?;
        let note_dtos: Vec<Note> = notes.into_iter().map(Note::from).collect();
        resolve_notes_access(db, &note_dtos).await?
    };

    // 内存计算每条待办的权限
    for todo in todos {
        // 1. 关联笔记连带禁止
        if let Some(note_id) = todo.note_id {
            if note_access.get(&note_id) == Some(&McpPermission::Deny) {
                result.insert(todo.id, McpPermission::Deny);
                continue;
            }
        }

        // 2. 待办自身设置
        let todo_mcp = McpAccess::from(todo.mcp_access);
        if todo_mcp != McpAccess::Inherit {
            result.insert(todo.id, mcp_access_to_permission(todo_mcp));
            continue;
        }

        // 3. 标签权限（取最严格值）
        let tag_perm = if let Some(tids) = todo_tag_map.get(&todo.id) {
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

        // 4. 清单设置
        let list_perm = todo
            .list_id
            .and_then(|lid| list_access.get(&lid))
            .map(|&a| McpAccess::from(a))
            .filter(|a| *a != McpAccess::Inherit)
            .map(mcp_access_to_permission);

        // 5. 合并：标签与清单取更严格者，均无表态则默认 ReadWrite
        let effective = match (tag_perm, list_perm) {
            (Some(tp), Some(lp)) => tp.stricter(lp),
            (Some(tp), None) => tp,
            (None, Some(lp)) => lp,
            (None, None) => McpPermission::ReadWrite,
        };

        result.insert(todo.id, effective);
    }

    Ok(result)
}

/// 检查待办是否可读，不可读则返回错误
pub async fn check_todo_read(db: &DatabaseConnection, todo_id: i64) -> anyhow::Result<()> {
    let perm = resolve_todo_access(db, todo_id).await?;
    if !perm.can_read() {
        anyhow::bail!("AI access denied for this todo (mcp_access=Deny)");
    }
    Ok(())
}

/// 检查待办是否可写，不可写则返回错误
pub async fn check_todo_write(db: &DatabaseConnection, todo_id: i64) -> anyhow::Result<()> {
    let perm = resolve_todo_access(db, todo_id).await?;
    if !perm.can_write() {
        if !perm.can_read() {
            anyhow::bail!("AI access denied for this todo (mcp_access=Deny)");
        }
        anyhow::bail!("This todo is read-only for AI (mcp_access=ReadOnly)");
    }
    Ok(())
}

/// 检查待办清单是否可写（用于 create_todo 时检查目标清单）
///
/// `list_id` 为 None 表示未归类，直接放行。
pub async fn check_todo_list_write(
    db: &DatabaseConnection,
    list_id: Option<i64>,
) -> anyhow::Result<()> {
    let Some(list_id) = list_id else {
        return Ok(());
    };

    if let Some(list) = entity::todo_list::Entity::find_by_id(list_id)
        .one(db)
        .await?
    {
        let access = McpAccess::from(list.mcp_access);
        match access {
            McpAccess::Deny => {
                anyhow::bail!("AI access denied for target todo list (mcp_access=Deny)");
            }
            McpAccess::ReadOnly => {
                anyhow::bail!(
                    "Target todo list is read-only, AI cannot create todos here (mcp_access=ReadOnly)"
                );
            }
            _ => {}
        }
    }

    Ok(())
}
