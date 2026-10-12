//! 远程服务器后端实现
//!
//! 基于 [`EnoteServerClient`] 的 [`NoteStore`] 实现，
//! 所有操作通过 HTTP API 由远程 ENote Server 完成，
//! 加密与操作日志均由服务端处理，本地仅做透传。

use crate::error::AppError;
use crate::model::{
    AttachmentStats, Note, NoteAttachment, NoteHistory, NoteHistorySearchPageParam, NoteLink,
    NoteSearchPageParam, NoteStatsResult, NoteTemplate, Notebook, PageParam, PageResult, Tag,
    Todo, TodoList, TodoSearchParam, TodoStats,
};
use crate::service::enote_server::EnoteServerClient;

use super::{NoteStore, TodoStore};

/// 远程 ENote Server 后端
pub struct ServerStore {
    client: EnoteServerClient,
}

impl ServerStore {
    /// 创建远程服务器后端
    pub fn new(client: EnoteServerClient) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl NoteStore for ServerStore {
    // ======== 笔记 ========

    async fn create_note(&self, note: &Note) -> Result<Option<Note>, AppError> {
        self.client.create_note(note).await
    }

    async fn update_note(&self, note: &Note) -> Result<Option<Note>, AppError> {
        self.client.update_note(note).await
    }

    async fn delete_note_by_id(&self, id: i64) -> Result<(), AppError> {
        self.client.delete_note_by_id(id).await
    }

    async fn search_page_notes(
        &self,
        param: &NoteSearchPageParam,
    ) -> Result<PageResult<Note>, AppError> {
        self.client.search_page_notes(param).await
    }

    async fn note_stats(&self, param: &NoteSearchPageParam) -> Result<NoteStatsResult, AppError> {
        self.client.note_stats(param).await
    }

    async fn batch_move_notes(&self, note_ids: &[i64], notebook_id: i64) -> Result<(), AppError> {
        self.client.batch_move_notes(note_ids, notebook_id).await
    }

    async fn batch_delete_notes(&self, note_ids: &[i64]) -> Result<(), AppError> {
        self.client.batch_delete_notes(note_ids).await
    }

    async fn toggle_note_star(&self, id: i64) -> Result<Option<Note>, AppError> {
        self.client.toggle_note_star(id).await
    }

    async fn toggle_note_pin(&self, id: i64) -> Result<Option<Note>, AppError> {
        self.client.toggle_note_pin(id).await
    }

    async fn restore_note(&self, id: i64) -> Result<(), AppError> {
        self.client.restore_note(id).await
    }

    async fn permanent_delete_note(&self, id: i64) -> Result<(), AppError> {
        self.client.permanent_delete_note(id).await
    }

    async fn empty_trash(&self) -> Result<(), AppError> {
        self.client.empty_trash().await
    }

    async fn find_deleted_notes(
        &self,
        page_param: &PageParam,
    ) -> Result<PageResult<Note>, AppError> {
        self.client.find_deleted_notes(page_param).await
    }

    async fn search_page_note_histories(
        &self,
        param: &NoteHistorySearchPageParam,
    ) -> Result<PageResult<NoteHistory>, AppError> {
        self.client.search_page_note_histories(param).await
    }

    // ======== 笔记本 ========

    async fn find_all_notebooks(&self) -> Result<Vec<Notebook>, AppError> {
        self.client.find_all_notebooks().await
    }

    async fn create_notebook(&self, notebook: &Notebook) -> Result<Option<Notebook>, AppError> {
        self.client.create_notebook(notebook).await
    }

    async fn update_notebook(&self, notebook: &Notebook) -> Result<Option<Notebook>, AppError> {
        self.client.update_notebook(notebook).await
    }

    async fn delete_notebook_by_id(&self, id: i64) -> Result<(), AppError> {
        self.client.delete_notebook_by_id(id).await
    }

    async fn reorder_notebooks(&self, orders: Vec<(i64, i32)>) -> Result<(), AppError> {
        self.client.reorder_notebooks(&orders).await
    }

    // ======== 标签 ========

    async fn find_all_tags(&self) -> Result<Vec<Tag>, AppError> {
        self.client.find_all_tags().await
    }

    async fn create_tag(&self, tag: &Tag) -> Result<Option<Tag>, AppError> {
        self.client.create_tag(tag).await
    }

    async fn update_tag(&self, tag: &Tag) -> Result<Option<Tag>, AppError> {
        self.client.update_tag(tag).await
    }

    async fn delete_tag_by_id(&self, id: i64) -> Result<(), AppError> {
        self.client.delete_tag_by_id(id).await
    }

    async fn reorder_tags(&self, orders: Vec<(i64, i32)>) -> Result<(), AppError> {
        self.client.reorder_tags(&orders).await
    }

    // ======== 模板 ========

    async fn find_all_templates(&self) -> Result<Vec<NoteTemplate>, AppError> {
        self.client.find_all_templates().await
    }

    async fn create_template(
        &self,
        template: &NoteTemplate,
    ) -> Result<Option<NoteTemplate>, AppError> {
        self.client.create_template(template).await
    }

    async fn update_template(
        &self,
        template: &NoteTemplate,
    ) -> Result<Option<NoteTemplate>, AppError> {
        self.client.update_template(template).await
    }

    async fn delete_template_by_id(&self, id: i64) -> Result<(), AppError> {
        self.client.delete_template_by_id(id).await
    }

    // ======== 笔记链接 ========

    async fn find_note_links(&self, note_id: i64) -> Result<Vec<NoteLink>, AppError> {
        self.client.find_note_links(note_id).await
    }

    async fn create_note_link(
        &self,
        source_note_id: i64,
        target_note_id: i64,
    ) -> Result<(), AppError> {
        self.client
            .create_note_link(source_note_id, target_note_id)
            .await
    }

    async fn delete_note_link(&self, link_id: i64) -> Result<(), AppError> {
        self.client.delete_note_link(link_id).await
    }

    async fn search_linkable_notes(
        &self,
        note_id: i64,
        keyword: &str,
    ) -> Result<Vec<NoteLink>, AppError> {
        self.client.search_linkable_notes(note_id, keyword).await
    }

    // ======== 附件 ========

    async fn save_attachment(
        &self,
        note_id: i64,
        file_name: &str,
        file_data: &[u8],
        mime_type: &str,
    ) -> Result<NoteAttachment, AppError> {
        self.client
            .save_attachment(note_id, file_name, file_data, mime_type)
            .await
    }

    async fn find_attachments(&self, note_id: i64) -> Result<Vec<NoteAttachment>, AppError> {
        self.client.find_attachments(note_id).await
    }

    async fn delete_attachment(&self, id: i64) -> Result<(), AppError> {
        self.client.delete_attachment(id).await
    }

    async fn get_attachment_stats(&self) -> Result<AttachmentStats, AppError> {
        self.client.get_attachment_stats().await
    }

    async fn cleanup_orphan_attachments(&self) -> Result<u32, AppError> {
        self.client.cleanup_orphan_attachments().await
    }
}

#[async_trait::async_trait]
impl TodoStore for ServerStore {
    async fn search_todos(&self, param: &TodoSearchParam) -> Result<Vec<Todo>, AppError> {
        self.client.search_todos(param).await
    }

    async fn find_todo(&self, id: i64) -> Result<Option<Todo>, AppError> {
        self.client.find_todo(id).await
    }

    async fn create_todo(&self, todo: &Todo) -> Result<Todo, AppError> {
        self.client.create_todo(todo).await
    }

    async fn update_todo(&self, todo: &Todo) -> Result<Option<Todo>, AppError> {
        self.client.update_todo(todo).await
    }

    async fn toggle_todo(&self, id: i64) -> Result<Option<Todo>, AppError> {
        self.client.toggle_todo(id).await
    }

    async fn delete_todo(&self, id: i64) -> Result<(), AppError> {
        self.client.delete_todo(id).await
    }

    async fn restore_todo(&self, id: i64) -> Result<(), AppError> {
        self.client.restore_todo(id).await
    }

    async fn permanent_delete_todo(&self, id: i64) -> Result<(), AppError> {
        self.client.permanent_delete_todo(id).await
    }

    async fn empty_todo_trash(&self) -> Result<u64, AppError> {
        self.client.empty_todo_trash().await
    }

    async fn find_all_todo_lists(&self) -> Result<Vec<TodoList>, AppError> {
        self.client.find_all_todo_lists().await
    }

    async fn create_todo_list(&self, list: &TodoList) -> Result<TodoList, AppError> {
        self.client.create_todo_list(list).await
    }

    async fn update_todo_list(&self, list: &TodoList) -> Result<Option<TodoList>, AppError> {
        self.client.update_todo_list(list).await
    }

    async fn delete_todo_list(&self, id: i64) -> Result<(), AppError> {
        self.client.delete_todo_list(id).await
    }

    async fn reorder_todos(&self, orders: Vec<(i64, i32)>) -> Result<(), AppError> {
        self.client.reorder_todos(&orders).await
    }

    async fn find_todo_tags(&self, todo_id: i64) -> Result<Vec<i64>, AppError> {
        self.client.find_todo_tags(todo_id).await
    }

    async fn set_todo_tags(&self, todo_id: i64, tag_ids: Vec<i64>) -> Result<(), AppError> {
        self.client.set_todo_tags(todo_id, &tag_ids).await
    }

    async fn batch_toggle_todos(&self, ids: &[i64]) -> Result<u64, AppError> {
        self.client.batch_toggle_todos(ids).await
    }

    async fn batch_delete_todos(&self, ids: &[i64]) -> Result<u64, AppError> {
        self.client.batch_delete_todos(ids).await
    }

    async fn batch_move_todos(
        &self,
        ids: &[i64],
        list_id: Option<i64>,
    ) -> Result<u64, AppError> {
        self.client.batch_move_todos(ids, list_id).await
    }

    async fn todo_stats(&self) -> Result<TodoStats, AppError> {
        self.client.todo_stats().await
    }
}
