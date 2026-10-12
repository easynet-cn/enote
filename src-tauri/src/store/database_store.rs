//! 数据库后端实现
//!
//! 基于 SeaORM 的 [`NoteStore`] 实现，支持 SQLite / MySQL / PostgreSQL。
//! 复用 `service` 层既有函数（事务、加解密、历史记录生成均保持原行为），
//! 并保留原有的操作日志（app_log）写入习惯。

use std::path::PathBuf;

use sea_orm::DatabaseConnection;

use crate::error::AppError;
use crate::model::{
    AttachmentStats, Note, NoteAttachment, NoteHistory, NoteHistorySearchPageParam, NoteLink,
    NoteSearchPageParam, NoteStatsResult, NoteTemplate, Notebook, OperateSource, PageParam,
    PageResult, Tag, Todo, TodoList, TodoSearchParam, TodoStats,
};
use crate::service;

use super::{NoteStore, TodoStore};

/// 数据库后端
pub struct DatabaseStore {
    db: DatabaseConnection,
    /// 内容加密密钥（未启用加密时为 None）
    encryption_key: Option<String>,
    /// 应用数据目录（附件等文件操作的根路径）
    app_data_dir: PathBuf,
}

impl DatabaseStore {
    /// 创建数据库后端
    pub fn new(
        db: DatabaseConnection,
        encryption_key: Option<String>,
        app_data_dir: PathBuf,
    ) -> Self {
        Self {
            db,
            encryption_key,
            app_data_dir,
        }
    }

    /// 写入操作日志（失败不影响主流程）
    async fn log_action(
        &self,
        module: &str,
        action: &str,
        target_id: Option<&str>,
        target_title: Option<&str>,
        message: &str,
    ) {
        let _ = service::app_log::log_action(
            &self.db,
            module,
            action,
            target_id,
            target_title,
            message,
            None,
        )
        .await;
    }
}

#[async_trait::async_trait]
impl NoteStore for DatabaseStore {
    // ======== 笔记 ========

    async fn create_note(&self, note: &Note) -> Result<Option<Note>, AppError> {
        let is_encrypted = service::app_log::should_skip_content(&note.content);
        let result = service::note::create_with_key(
            &self.db,
            note,
            OperateSource::User,
            self.encryption_key.as_deref(),
        )
        .await
        .map_err(AppError::from)?;
        if let Some(ref n) = result {
            let msg = if is_encrypted {
                format!("Created note: {} [encrypted content]", n.title)
            } else {
                format!("Created note: {}", n.title)
            };
            self.log_action("note", "create", Some(&n.id.to_string()), Some(&n.title), &msg)
                .await;
        }
        Ok(result)
    }

    async fn update_note(&self, note: &Note) -> Result<Option<Note>, AppError> {
        let is_encrypted = service::app_log::should_skip_content(&note.content);
        let result = service::note::update_with_key(
            &self.db,
            note,
            OperateSource::User,
            self.encryption_key.as_deref(),
        )
        .await
        .map_err(AppError::from)?;
        if result.is_some() {
            let msg = if is_encrypted {
                format!("Updated note: {} [encrypted content]", note.title)
            } else {
                format!("Updated note: {}", note.title)
            };
            self.log_action(
                "note",
                "update",
                Some(&note.id.to_string()),
                Some(&note.title),
                &msg,
            )
            .await;
        }
        Ok(result)
    }

    async fn delete_note_by_id(&self, id: i64) -> Result<(), AppError> {
        service::note::delete_by_id(&self.db, id)
            .await
            .map_err(AppError::from)?;
        self.log_action(
            "note",
            "delete",
            Some(&id.to_string()),
            None,
            &format!("Moved note id={} to trash", id),
        )
        .await;
        Ok(())
    }

    async fn search_page_notes(
        &self,
        param: &NoteSearchPageParam,
    ) -> Result<PageResult<Note>, AppError> {
        service::note::search_page_with_key(&self.db, param, self.encryption_key.as_deref())
            .await
            .map_err(AppError::from)
    }

    async fn note_stats(&self, param: &NoteSearchPageParam) -> Result<NoteStatsResult, AppError> {
        service::note::stats(&self.db, param)
            .await
            .map_err(AppError::from)
    }

    async fn batch_move_notes(&self, note_ids: &[i64], notebook_id: i64) -> Result<(), AppError> {
        service::note::batch_move(&self.db, note_ids, notebook_id)
            .await
            .map_err(AppError::from)
    }

    async fn batch_delete_notes(&self, note_ids: &[i64]) -> Result<(), AppError> {
        service::note::batch_delete(&self.db, note_ids)
            .await
            .map_err(AppError::from)
    }

    async fn toggle_note_star(&self, id: i64) -> Result<Option<Note>, AppError> {
        service::note::toggle_star(&self.db, id)
            .await
            .map_err(AppError::from)
    }

    async fn toggle_note_pin(&self, id: i64) -> Result<Option<Note>, AppError> {
        service::note::toggle_pin(&self.db, id)
            .await
            .map_err(AppError::from)
    }

    async fn restore_note(&self, id: i64) -> Result<(), AppError> {
        service::note::restore_by_id(&self.db, id)
            .await
            .map_err(AppError::from)?;
        self.log_action(
            "note",
            "restore",
            Some(&id.to_string()),
            None,
            &format!("Restored note id={} from trash", id),
        )
        .await;
        Ok(())
    }

    async fn permanent_delete_note(&self, id: i64) -> Result<(), AppError> {
        service::note::permanent_delete_by_id(
            &self.db,
            id,
            OperateSource::User,
            self.encryption_key.as_deref(),
        )
        .await
        .map_err(AppError::from)?;
        self.log_action(
            "note",
            "permanent_delete",
            Some(&id.to_string()),
            None,
            &format!("Permanently deleted note id={}", id),
        )
        .await;
        Ok(())
    }

    async fn empty_trash(&self) -> Result<(), AppError> {
        service::note::empty_trash(&self.db, self.encryption_key.as_deref())
            .await
            .map_err(AppError::from)?;
        self.log_action("note", "empty_trash", None, None, "Emptied trash")
            .await;
        Ok(())
    }

    async fn find_deleted_notes(
        &self,
        page_param: &PageParam,
    ) -> Result<PageResult<Note>, AppError> {
        service::note::find_deleted_with_key(&self.db, page_param, self.encryption_key.as_deref())
            .await
            .map_err(AppError::from)
    }

    async fn search_page_note_histories(
        &self,
        param: &NoteHistorySearchPageParam,
    ) -> Result<PageResult<NoteHistory>, AppError> {
        service::note_history::search_page(&self.db, param)
            .await
            .map_err(AppError::from)
    }

    // ======== 笔记本 ========

    async fn find_all_notebooks(&self) -> Result<Vec<Notebook>, AppError> {
        service::notebook::find_all(&self.db)
            .await
            .map_err(AppError::from)
    }

    async fn create_notebook(&self, notebook: &Notebook) -> Result<Option<Notebook>, AppError> {
        let result = service::notebook::create(&self.db, notebook)
            .await
            .map_err(AppError::from)?;
        if let Some(ref nb) = result {
            self.log_action(
                "notebook",
                "create",
                Some(&nb.id.to_string()),
                Some(&nb.name),
                &format!("Created notebook: {}", nb.name),
            )
            .await;
        }
        Ok(result)
    }

    async fn update_notebook(&self, notebook: &Notebook) -> Result<Option<Notebook>, AppError> {
        let result = service::notebook::update(&self.db, notebook)
            .await
            .map_err(AppError::from)?;
        if result.is_some() {
            self.log_action(
                "notebook",
                "update",
                Some(&notebook.id.to_string()),
                Some(&notebook.name),
                &format!("Updated notebook: {}", notebook.name),
            )
            .await;
        }
        Ok(result)
    }

    async fn delete_notebook_by_id(&self, id: i64) -> Result<(), AppError> {
        service::notebook::delete_by_id(&self.db, id)
            .await
            .map_err(AppError::from)?;
        self.log_action(
            "notebook",
            "delete",
            Some(&id.to_string()),
            None,
            &format!("Deleted notebook id={}", id),
        )
        .await;
        Ok(())
    }

    async fn reorder_notebooks(&self, orders: Vec<(i64, i32)>) -> Result<(), AppError> {
        service::notebook::reorder(&self.db, orders)
            .await
            .map_err(AppError::from)
    }

    // ======== 标签 ========

    async fn find_all_tags(&self) -> Result<Vec<Tag>, AppError> {
        service::tag::find_all(&self.db)
            .await
            .map_err(AppError::from)
    }

    async fn create_tag(&self, tag: &Tag) -> Result<Option<Tag>, AppError> {
        let result = service::tag::create(&self.db, tag)
            .await
            .map_err(AppError::from)?;
        if let Some(ref t) = result {
            self.log_action(
                "tag",
                "create",
                Some(&t.id.to_string()),
                Some(&t.name),
                &format!("Created tag: {}", t.name),
            )
            .await;
        }
        Ok(result)
    }

    async fn update_tag(&self, tag: &Tag) -> Result<Option<Tag>, AppError> {
        let result = service::tag::update(&self.db, tag)
            .await
            .map_err(AppError::from)?;
        if result.is_some() {
            self.log_action(
                "tag",
                "update",
                Some(&tag.id.to_string()),
                Some(&tag.name),
                &format!("Updated tag: {}", tag.name),
            )
            .await;
        }
        Ok(result)
    }

    async fn delete_tag_by_id(&self, id: i64) -> Result<(), AppError> {
        service::tag::delete_by_id(&self.db, id)
            .await
            .map_err(AppError::from)?;
        self.log_action(
            "tag",
            "delete",
            Some(&id.to_string()),
            None,
            &format!("Deleted tag id={}", id),
        )
        .await;
        Ok(())
    }

    async fn reorder_tags(&self, orders: Vec<(i64, i32)>) -> Result<(), AppError> {
        service::tag::reorder(&self.db, orders)
            .await
            .map_err(AppError::from)
    }

    // ======== 模板 ========

    async fn find_all_templates(&self) -> Result<Vec<NoteTemplate>, AppError> {
        service::note_template::find_all(&self.db)
            .await
            .map_err(AppError::from)
    }

    async fn create_template(
        &self,
        template: &NoteTemplate,
    ) -> Result<Option<NoteTemplate>, AppError> {
        service::note_template::create(&self.db, template)
            .await
            .map_err(AppError::from)
    }

    async fn update_template(
        &self,
        template: &NoteTemplate,
    ) -> Result<Option<NoteTemplate>, AppError> {
        service::note_template::update(&self.db, template)
            .await
            .map_err(AppError::from)
    }

    async fn delete_template_by_id(&self, id: i64) -> Result<(), AppError> {
        service::note_template::delete_by_id(&self.db, id)
            .await
            .map_err(AppError::from)
    }

    // ======== 笔记链接 ========

    async fn find_note_links(&self, note_id: i64) -> Result<Vec<NoteLink>, AppError> {
        service::note_link::find_links(&self.db, note_id)
            .await
            .map_err(AppError::from)
    }

    async fn create_note_link(
        &self,
        source_note_id: i64,
        target_note_id: i64,
    ) -> Result<(), AppError> {
        service::note_link::create_link(&self.db, source_note_id, target_note_id)
            .await
            .map_err(AppError::from)
    }

    async fn delete_note_link(&self, link_id: i64) -> Result<(), AppError> {
        service::note_link::delete_link(&self.db, link_id)
            .await
            .map_err(AppError::from)
    }

    async fn search_linkable_notes(
        &self,
        note_id: i64,
        keyword: &str,
    ) -> Result<Vec<NoteLink>, AppError> {
        service::note_link::search_linkable_notes(&self.db, note_id, keyword)
            .await
            .map_err(AppError::from)
    }

    // ======== 附件 ========

    async fn save_attachment(
        &self,
        note_id: i64,
        file_name: &str,
        file_data: &[u8],
        mime_type: &str,
    ) -> Result<NoteAttachment, AppError> {
        service::attachment::save_attachment(
            &self.db,
            &self.app_data_dir,
            note_id,
            file_name,
            file_data,
            mime_type,
        )
        .await
        .map_err(AppError::from)
    }

    async fn find_attachments(&self, note_id: i64) -> Result<Vec<NoteAttachment>, AppError> {
        service::attachment::find_by_note_id(&self.db, note_id)
            .await
            .map_err(AppError::from)
    }

    async fn delete_attachment(&self, id: i64) -> Result<(), AppError> {
        service::attachment::delete_by_id(&self.db, &self.app_data_dir, id)
            .await
            .map_err(AppError::from)
    }

    async fn get_attachment_stats(&self) -> Result<AttachmentStats, AppError> {
        service::attachment::get_stats(&self.db, &self.app_data_dir)
            .await
            .map_err(AppError::from)
    }

    async fn cleanup_orphan_attachments(&self) -> Result<u32, AppError> {
        service::attachment::cleanup_orphans(&self.db, &self.app_data_dir)
            .await
            .map_err(AppError::from)
    }
}

#[async_trait::async_trait]
impl TodoStore for DatabaseStore {
    async fn search_todos(&self, param: &TodoSearchParam) -> Result<Vec<Todo>, AppError> {
        service::todo::search(&self.db, param)
            .await
            .map_err(AppError::from)
    }

    async fn find_todo(&self, id: i64) -> Result<Option<Todo>, AppError> {
        service::todo::find_by_id(&self.db, id)
            .await
            .map_err(AppError::from)
    }

    async fn create_todo(&self, todo: &Todo) -> Result<Todo, AppError> {
        service::todo::create(&self.db, todo)
            .await
            .map_err(AppError::from)
    }

    async fn update_todo(&self, todo: &Todo) -> Result<Option<Todo>, AppError> {
        service::todo::update(&self.db, todo)
            .await
            .map_err(AppError::from)
    }

    async fn toggle_todo(&self, id: i64) -> Result<Option<Todo>, AppError> {
        service::todo::toggle_complete(&self.db, id)
            .await
            .map_err(AppError::from)
    }

    async fn delete_todo(&self, id: i64) -> Result<(), AppError> {
        service::todo::delete_by_id(&self.db, id)
            .await
            .map_err(AppError::from)
    }

    async fn restore_todo(&self, id: i64) -> Result<(), AppError> {
        service::todo::restore(&self.db, id)
            .await
            .map_err(AppError::from)
    }

    async fn permanent_delete_todo(&self, id: i64) -> Result<(), AppError> {
        service::todo::permanent_delete(&self.db, id)
            .await
            .map_err(AppError::from)
    }

    async fn empty_todo_trash(&self) -> Result<u64, AppError> {
        service::todo::empty_trash(&self.db)
            .await
            .map_err(AppError::from)
    }

    async fn find_all_todo_lists(&self) -> Result<Vec<TodoList>, AppError> {
        service::todo::find_all_lists(&self.db)
            .await
            .map_err(AppError::from)
    }

    async fn create_todo_list(&self, list: &TodoList) -> Result<TodoList, AppError> {
        service::todo::create_list(&self.db, list)
            .await
            .map_err(AppError::from)
    }

    async fn update_todo_list(&self, list: &TodoList) -> Result<Option<TodoList>, AppError> {
        service::todo::update_list(&self.db, list)
            .await
            .map_err(AppError::from)
    }

    async fn delete_todo_list(&self, id: i64) -> Result<(), AppError> {
        service::todo::delete_list(&self.db, id)
            .await
            .map_err(AppError::from)
    }

    async fn reorder_todos(&self, orders: Vec<(i64, i32)>) -> Result<(), AppError> {
        service::todo::reorder(&self.db, orders)
            .await
            .map_err(AppError::from)
    }

    async fn find_todo_tags(&self, todo_id: i64) -> Result<Vec<i64>, AppError> {
        service::todo::find_tag_ids_by_todo(&self.db, todo_id)
            .await
            .map_err(AppError::from)
    }

    async fn set_todo_tags(&self, todo_id: i64, tag_ids: Vec<i64>) -> Result<(), AppError> {
        service::todo::set_todo_tags(&self.db, todo_id, tag_ids)
            .await
            .map_err(AppError::from)
    }

    async fn batch_toggle_todos(&self, ids: &[i64]) -> Result<u64, AppError> {
        service::todo::batch_toggle(&self.db, ids.to_vec())
            .await
            .map_err(AppError::from)
    }

    async fn batch_delete_todos(&self, ids: &[i64]) -> Result<u64, AppError> {
        service::todo::batch_delete(&self.db, ids.to_vec())
            .await
            .map_err(AppError::from)
    }

    async fn batch_move_todos(
        &self,
        ids: &[i64],
        list_id: Option<i64>,
    ) -> Result<u64, AppError> {
        service::todo::batch_move(&self.db, ids.to_vec(), list_id)
            .await
            .map_err(AppError::from)
    }

    async fn todo_stats(&self) -> Result<TodoStats, AppError> {
        service::todo::stats(&self.db)
            .await
            .map_err(AppError::from)
    }
}
