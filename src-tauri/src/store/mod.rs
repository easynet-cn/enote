//! 存储抽象层
//!
//! 定义笔记域的统一存储接口 [`NoteStore`]，屏蔽底层实现差异：
//! - [`DatabaseStore`]：数据库后端（SQLite / MySQL / PostgreSQL）
//! - [`ServerStore`]：远程 ENote Server HTTP 后端
//!
//! 未来可按同一接口接入更多实现，例如 Git 仓库、OSS 对象存储等。
//!
//! # 设计约定
//! - ID 生成、加解密、操作日志等细节由各 Store 自行负责
//! - 所有方法返回 `Result<_, AppError>`，与命令层错误类型保持一致
//! - 通过 `Arc<dyn NoteStore>` 持有，可在 Profile 热切换时整体替换

mod database_store;
mod git_store;
pub(crate) mod git_remote;
pub(crate) mod git_sync;
mod server_store;

pub use database_store::DatabaseStore;
pub use git_store::GitStore;
pub use server_store::ServerStore;

use crate::error::AppError;
use crate::model::{
    AttachmentStats, Note, NoteAttachment, NoteHistory, NoteHistorySearchPageParam, NoteLink,
    NoteSearchPageParam, NoteStatsResult, NoteTemplate, Notebook, PageParam, PageResult, Tag, Todo,
    TodoList, TodoSearchParam, TodoStats,
};

/// 笔记域存储接口（抽象层）
///
/// 覆盖笔记、笔记本、标签、笔记历史四类数据的读写能力，
/// 与现有 Tauri 命令的行为一一对应。
#[async_trait::async_trait]
pub trait NoteStore: Send + Sync {
    // ======== 笔记 ========

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
/// Server 后端暂不支持待办功能（todo_store 为 None 时返回 NOT_SUPPORTED）。
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
