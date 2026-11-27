//! 存储抽象层
//!
//! 定义笔记域的统一存储接口 [`NoteStore`] 与待办域接口 [`TodoStore`]，屏蔽底层实现差异：
//! - [`DatabaseStore`]：数据库后端（SQLite / MySQL / PostgreSQL）
//! - [`GitStore`]：本地 Git 仓库后端
//! - [`ServerStore`]：远程 ENote Server HTTP 后端
//!
//! [`AccessStore`] 在两者之上提供 MCP 所需的写入来源与权限解析能力，
//! 所有方法均有默认实现，因此三种后端自动共享同一套权限规则。
//!
//! # 设计约定
//! - ID 生成、加解密、操作日志等细节由各 Store 自行负责
//! - 所有方法返回 `Result<_, AppError>`，与命令层错误类型保持一致
//! - 通过 `Arc<dyn NoteStore>` 持有，可在 Profile 热切换时整体替换

mod database_store;
pub mod factory;
mod git_store;
pub(crate) mod git_remote;
pub(crate) mod git_sync;
mod server_store;

pub use database_store::DatabaseStore;
pub use factory::StoreBundle;
pub use git_store::GitStore;
pub use server_store::ServerStore;

use std::collections::HashMap;

use crate::error::AppError;
use crate::model::{
    AttachmentStats, McpPermission, Note, NoteAttachment, NoteHistory, NoteHistorySearchPageParam,
    NoteLink, NoteSearchPageParam, NoteStatsResult, NoteTemplate, Notebook, OperateSource,
    PageParam, PageResult, Tag, Todo, TodoList, TodoSearchParam, TodoStats,
};

/// 笔记域存储接口（抽象层）
///
/// 覆盖笔记、笔记本、标签、笔记历史四类数据的读写能力，
/// 与现有 Tauri 命令的行为一一对应。
#[async_trait::async_trait]
pub trait NoteStore: Send + Sync {
    // ======== 笔记 ========

    /// 根据 ID 查询单条笔记（含关联标签）
    async fn find_note_by_id(&self, id: i64) -> Result<Option<Note>, AppError>;

    /// 创建笔记
    async fn create_note(&self, note: &Note) -> Result<Option<Note>, AppError>;

    /// 更新笔记
    async fn update_note(&self, note: &Note) -> Result<Option<Note>, AppError>;

    /// 根据 ID 删除笔记（软删除，移入回收站）
    async fn delete_note_by_id(&self, id: i64) -> Result<(), AppError>;

    /// 分页搜索笔记
    async fn search_page_notes(
        &self,
        param: &NoteSearchPageParam,
    ) -> Result<PageResult<Note>, AppError>;

    /// 笔记统计信息
    async fn note_stats(&self, param: &NoteSearchPageParam) -> Result<NoteStatsResult, AppError>;

    /// 批量移动笔记到指定笔记本
    async fn batch_move_notes(&self, note_ids: &[i64], notebook_id: i64) -> Result<(), AppError>;

    /// 批量软删除笔记
    async fn batch_delete_notes(&self, note_ids: &[i64]) -> Result<(), AppError>;

    /// 切换笔记收藏/星标状态
    async fn toggle_note_star(&self, id: i64) -> Result<Option<Note>, AppError>;

    /// 切换笔记置顶状态
    async fn toggle_note_pin(&self, id: i64) -> Result<Option<Note>, AppError>;

    /// 从回收站恢复笔记
    async fn restore_note(&self, id: i64) -> Result<(), AppError>;

    /// 永久删除笔记
    async fn permanent_delete_note(&self, id: i64) -> Result<(), AppError>;

    /// 清空回收站
    async fn empty_trash(&self) -> Result<(), AppError>;

    /// 获取回收站笔记列表（分页）
    async fn find_deleted_notes(
        &self,
        page_param: &PageParam,
    ) -> Result<PageResult<Note>, AppError>;

    /// 分页搜索笔记历史记录
    async fn search_page_note_histories(
        &self,
        param: &NoteHistorySearchPageParam,
    ) -> Result<PageResult<NoteHistory>, AppError>;

    // ======== 笔记本 ========

    /// 获取所有笔记本
    async fn find_all_notebooks(&self) -> Result<Vec<Notebook>, AppError>;

    /// 创建笔记本
    async fn create_notebook(&self, notebook: &Notebook) -> Result<Option<Notebook>, AppError>;

    /// 更新笔记本
    async fn update_notebook(&self, notebook: &Notebook) -> Result<Option<Notebook>, AppError>;

    /// 根据 ID 删除笔记本
    async fn delete_notebook_by_id(&self, id: i64) -> Result<(), AppError>;

    /// 批量更新笔记本排序
    async fn reorder_notebooks(&self, orders: Vec<(i64, i32)>) -> Result<(), AppError>;

    // ======== 标签 ========

    /// 获取所有标签
    async fn find_all_tags(&self) -> Result<Vec<Tag>, AppError>;

    /// 创建标签
    async fn create_tag(&self, tag: &Tag) -> Result<Option<Tag>, AppError>;

    /// 更新标签
    async fn update_tag(&self, tag: &Tag) -> Result<Option<Tag>, AppError>;

    /// 根据 ID 删除标签
    async fn delete_tag_by_id(&self, id: i64) -> Result<(), AppError>;

    /// 批量更新标签排序
    async fn reorder_tags(&self, orders: Vec<(i64, i32)>) -> Result<(), AppError>;

    // ======== 模板 ========

    /// 获取所有笔记模板
    async fn find_all_templates(&self) -> Result<Vec<NoteTemplate>, AppError>;

    /// 创建笔记模板
    async fn create_template(
        &self,
        template: &NoteTemplate,
    ) -> Result<Option<NoteTemplate>, AppError>;

    /// 更新笔记模板
    async fn update_template(
        &self,
        template: &NoteTemplate,
    ) -> Result<Option<NoteTemplate>, AppError>;

    /// 根据 ID 删除笔记模板
    async fn delete_template_by_id(&self, id: i64) -> Result<(), AppError>;

    // ======== 笔记链接 ========

    /// 获取笔记的所有链接
    async fn find_note_links(&self, note_id: i64) -> Result<Vec<NoteLink>, AppError>;

    /// 创建笔记链接
    async fn create_note_link(
        &self,
        source_note_id: i64,
        target_note_id: i64,
    ) -> Result<(), AppError>;

    /// 删除笔记链接
    async fn delete_note_link(&self, link_id: i64) -> Result<(), AppError>;

    /// 搜索可链接的笔记
    async fn search_linkable_notes(
        &self,
        note_id: i64,
        keyword: &str,
    ) -> Result<Vec<NoteLink>, AppError>;

    // ======== 附件 ========

    /// 保存附件
    async fn save_attachment(
        &self,
        note_id: i64,
        file_name: &str,
        file_data: &[u8],
        mime_type: &str,
    ) -> Result<NoteAttachment, AppError>;

    /// 查询笔记附件列表
    async fn find_attachments(&self, note_id: i64) -> Result<Vec<NoteAttachment>, AppError>;

    /// 删除附件
    async fn delete_attachment(&self, id: i64) -> Result<(), AppError>;

    /// 获取附件统计信息
    async fn get_attachment_stats(&self) -> Result<AttachmentStats, AppError>;

    /// 清理孤立附件文件
    async fn cleanup_orphan_attachments(&self) -> Result<u32, AppError>;
}

/// 待办域存储接口（抽象层）
///
/// 覆盖待办和待办清单的 CRUD、查询、批量操作、标签关联、统计。
/// Database / Git / Server 三种后端均完整实现。
#[async_trait::async_trait]
pub trait TodoStore: Send + Sync {
    /// 查询待办列表
    async fn search_todos(&self, param: &TodoSearchParam) -> Result<Vec<Todo>, AppError>;

    /// 根据 ID 查询待办
    async fn find_todo(&self, id: i64) -> Result<Option<Todo>, AppError>;

    /// 创建待办
    async fn create_todo(&self, todo: &Todo) -> Result<Todo, AppError>;

    /// 更新待办
    async fn update_todo(&self, todo: &Todo) -> Result<Option<Todo>, AppError>;

    /// 切换待办完成状态
    async fn toggle_todo(&self, id: i64) -> Result<Option<Todo>, AppError>;

    /// 软删除待办（移入回收站）
    async fn delete_todo(&self, id: i64) -> Result<(), AppError>;

    /// 恢复回收站中的待办
    async fn restore_todo(&self, id: i64) -> Result<(), AppError>;

    /// 彻底删除待办（物理删除）
    async fn permanent_delete_todo(&self, id: i64) -> Result<(), AppError>;

    /// 清空待办回收站
    async fn empty_todo_trash(&self) -> Result<u64, AppError>;

    /// 查询所有待办清单
    async fn find_all_todo_lists(&self) -> Result<Vec<TodoList>, AppError>;

    /// 创建待办清单
    async fn create_todo_list(&self, list: &TodoList) -> Result<TodoList, AppError>;

    /// 更新待办清单
    async fn update_todo_list(&self, list: &TodoList) -> Result<Option<TodoList>, AppError>;

    /// 删除待办清单
    async fn delete_todo_list(&self, id: i64) -> Result<(), AppError>;

    /// 批量更新待办排序
    async fn reorder_todos(&self, orders: Vec<(i64, i32)>) -> Result<(), AppError>;

    /// 查询待办关联的标签 ID 列表
    async fn find_todo_tags(&self, todo_id: i64) -> Result<Vec<i64>, AppError>;

    /// 设置待办关联的标签（全量替换）
    async fn set_todo_tags(&self, todo_id: i64, tag_ids: Vec<i64>) -> Result<(), AppError>;

    /// 批量切换待办完成状态
    async fn batch_toggle_todos(&self, ids: &[i64]) -> Result<u64, AppError>;

    /// 批量软删除待办
    async fn batch_delete_todos(&self, ids: &[i64]) -> Result<u64, AppError>;

    /// 批量移动待办到指定清单
    async fn batch_move_todos(&self, ids: &[i64], list_id: Option<i64>) -> Result<u64, AppError>;

    /// 获取待办统计信息
    async fn todo_stats(&self) -> Result<TodoStats, AppError>;
}

/// MCP 访问控制与写入上下文（组合 trait）
///
/// 同时持有笔记域与待办域的存储能力，在其上提供：
/// - 带 [`OperateSource`] 的笔记写方法，保证 AI 工具的操作来源不被丢失
/// - 笔记/待办的 MCP 权限解析与读写校验
///
/// 所有方法都有默认实现，基于 [`NoteStore`] / [`TodoStore`] 已有的查询能力
/// 在内存中完成权限计算。因此任何同时实现两个 trait 的后端都会自动获得
/// 完全一致的行为，无需各自复制权限逻辑。
#[async_trait::async_trait]
pub trait AccessStore: NoteStore + TodoStore {
    // ======== 写入来源 ========

    /// 以指定操作来源创建笔记
    ///
    /// 默认实现忽略 source 并委托给 [`NoteStore::create_note`]，
    /// 保持尚不支持操作日志的后端可用；支持审计的后端应覆盖此方法。
    async fn create_note_with_source(
        &self,
        note: &Note,
        source: OperateSource,
    ) -> Result<Option<Note>, AppError> {
        let _ = source;
        self.create_note(note).await
    }

    /// 以指定操作来源更新笔记
    async fn update_note_with_source(
        &self,
        note: &Note,
        source: OperateSource,
    ) -> Result<Option<Note>, AppError> {
        let _ = source;
        self.update_note(note).await
    }

    // ======== 笔记权限 ========

    /// 批量解析笔记权限
    async fn resolve_notes_access(
        &self,
        notes: &[Note],
    ) -> Result<HashMap<i64, McpPermission>, AppError> {
        let notebooks = self.find_all_notebooks().await?;
        Ok(crate::service::mcp_access::notes_permissions(notes, &notebooks))
    }

    /// 解析单条笔记权限
    ///
    /// 复用批量解析，确保覆盖批量方法的后端（如 DatabaseStore 需读取原始
    /// 加密内容）在单条场景下行为一致。
    async fn resolve_note_access(&self, note_id: i64) -> Result<McpPermission, AppError> {
        let Some(note) = self.find_note_by_id(note_id).await? else {
            return Err(AppError::from(anyhow::anyhow!(
                "Note ID {} not found",
                note_id
            )));
        };
        let perms = self.resolve_notes_access(std::slice::from_ref(&note)).await?;
        perms.get(&note_id).copied().ok_or_else(|| {
            AppError::from(anyhow::anyhow!("Failed to resolve access for note {}", note_id))
        })
    }

    /// 检查笔记是否允许 AI 读取
    async fn check_note_read(&self, note_id: i64) -> Result<(), AppError> {
        let perm = self.resolve_note_access(note_id).await?;
        crate::service::mcp_access::ensure_permission(perm, "note", false).map_err(AppError::from)
    }

    /// 检查笔记是否允许 AI 写入
    async fn check_note_write(&self, note_id: i64) -> Result<(), AppError> {
        let perm = self.resolve_note_access(note_id).await?;
        crate::service::mcp_access::ensure_permission(perm, "note", true).map_err(AppError::from)
    }

    /// 检查目标笔记本是否允许 AI 写入
    async fn check_notebook_write(&self, notebook_id: i64) -> Result<(), AppError> {
        if notebook_id == 0 {
            return Ok(()); // 不归属笔记本，允许
        }
        let notebook = self
            .find_all_notebooks()
            .await?
            .into_iter()
            .find(|nb| nb.id == notebook_id);
        crate::service::mcp_access::ensure_notebook_write(notebook.as_ref())
            .map_err(AppError::from)
    }

    // ======== 待办权限 ========

    /// 批量解析待办权限
    async fn resolve_todos_access(
        &self,
        todos: &[Todo],
    ) -> Result<HashMap<i64, McpPermission>, AppError> {
        if todos.is_empty() {
            return Ok(HashMap::new());
        }

        let lists = self.find_all_todo_lists().await?;
        let tag_access: HashMap<i64, i32> = self
            .find_all_tags()
            .await?
            .into_iter()
            .map(|t| (t.id, t.mcp_access))
            .collect();

        let mut tag_accesses_by_todo: HashMap<i64, Vec<i32>> = HashMap::new();
        let mut linked_notes: Vec<Note> = Vec::new();

        for todo in todos {
            let tag_ids = self.find_todo_tags(todo.id).await?;
            let accesses = tag_ids
                .iter()
                .filter_map(|id| tag_access.get(id).copied())
                .collect();
            tag_accesses_by_todo.insert(todo.id, accesses);

            // 关联笔记的 Deny 会连带禁止待办访问
            if let Some(note_id) = todo.note_id {
                if let Some(note) = self.find_note_by_id(note_id).await? {
                    linked_notes.push(note);
                }
            }
        }

        let note_perms = self.resolve_notes_access(&linked_notes).await?;
        Ok(crate::service::mcp_access::todos_permissions(
            todos,
            &lists,
            &tag_accesses_by_todo,
            &note_perms,
        ))
    }

    /// 解析单条待办权限
    async fn resolve_todo_access(&self, todo_id: i64) -> Result<McpPermission, AppError> {
        let Some(todo) = self.find_todo(todo_id).await? else {
            return Err(AppError::from(anyhow::anyhow!(
                "Todo ID {} not found",
                todo_id
            )));
        };
        let perms = self.resolve_todos_access(std::slice::from_ref(&todo)).await?;
        perms.get(&todo_id).copied().ok_or_else(|| {
            AppError::from(anyhow::anyhow!("Failed to resolve access for todo {}", todo_id))
        })
    }

    /// 检查待办是否允许 AI 读取
    async fn check_todo_read(&self, todo_id: i64) -> Result<(), AppError> {
        let perm = self.resolve_todo_access(todo_id).await?;
        crate::service::mcp_access::ensure_permission(perm, "todo", false).map_err(AppError::from)
    }

    /// 检查待办是否允许 AI 写入
    async fn check_todo_write(&self, todo_id: i64) -> Result<(), AppError> {
        let perm = self.resolve_todo_access(todo_id).await?;
        crate::service::mcp_access::ensure_permission(perm, "todo", true).map_err(AppError::from)
    }

    /// 检查目标待办清单是否允许 AI 写入
    async fn check_todo_list_write(&self, list_id: Option<i64>) -> Result<(), AppError> {
        let list = match list_id {
            Some(id) => self
                .find_all_todo_lists()
                .await?
                .into_iter()
                .find(|l| l.id == id),
            None => None,
        };
        crate::service::mcp_access::ensure_todo_list_write(list.as_ref()).map_err(AppError::from)
    }
}

// 各后端显式接入；DatabaseStore 在自身模块中覆盖写方法以保留 OperateSource
impl AccessStore for GitStore {}
impl AccessStore for ServerStore {}
