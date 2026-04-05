//! API 端点映射
//!
//! 将所有 HTTP-capable 的 Tauri 命令映射为 HTTP API 调用。
//! 请求/响应使用与 Tauri IPC 相同的 camelCase JSON 格式。

use std::collections::HashMap;

use crate::error::AppError;
use crate::model::*;
use crate::service::enote_server::error::map_http_error;

use super::client::EnoteServerClient;

// ============================================================================
// Notebook
// ============================================================================

impl EnoteServerClient {
    pub async fn find_all_notebooks(&self) -> Result<Vec<Notebook>, AppError> {
        self.get("/api/notebooks").await
    }

    pub async fn create_notebook(&self, notebook: &Notebook) -> Result<Option<Notebook>, AppError> {
        self.post("/api/notebooks", notebook).await
    }

    pub async fn update_notebook(&self, notebook: &Notebook) -> Result<Option<Notebook>, AppError> {
        self.put(&format!("/api/notebooks/{}", notebook.id), notebook)
            .await
    }

    pub async fn delete_notebook_by_id(&self, id: i64) -> Result<(), AppError> {
        self.delete(&format!("/api/notebooks/{}", id)).await
    }

    pub async fn reorder_notebooks(&self, orders: &[(i64, i32)]) -> Result<(), AppError> {
        self.put_void("/api/notebooks/reorder", &orders).await
    }
}

// ============================================================================
// Tag
// ============================================================================

impl EnoteServerClient {
    pub async fn find_all_tags(&self) -> Result<Vec<Tag>, AppError> {
        self.get("/api/tags").await
    }

    pub async fn create_tag(&self, tag: &Tag) -> Result<Option<Tag>, AppError> {
        self.post("/api/tags", tag).await
    }

    pub async fn update_tag(&self, tag: &Tag) -> Result<Option<Tag>, AppError> {
        self.put(&format!("/api/tags/{}", tag.id), tag).await
    }

    pub async fn delete_tag_by_id(&self, id: i64) -> Result<(), AppError> {
        self.delete(&format!("/api/tags/{}", id)).await
    }

    pub async fn reorder_tags(&self, orders: &[(i64, i32)]) -> Result<(), AppError> {
        self.put_void("/api/tags/reorder", &orders).await
    }
}

// ============================================================================
// Note CRUD
// ============================================================================

impl EnoteServerClient {
    pub async fn create_note(&self, note: &Note) -> Result<Option<Note>, AppError> {
        self.post("/api/notes", note).await
    }

    pub async fn update_note(&self, note: &Note) -> Result<Option<Note>, AppError> {
        self.put(&format!("/api/notes/{}", note.id), note).await
    }

    pub async fn delete_note_by_id(&self, id: i64) -> Result<(), AppError> {
        self.delete(&format!("/api/notes/{}", id)).await
    }

    pub async fn toggle_note_pin(&self, id: i64) -> Result<Option<Note>, AppError> {
        self.put(&format!("/api/notes/{}/toggle-pin", id), &serde_json::json!({}))
            .await
    }

    pub async fn toggle_note_star(&self, id: i64) -> Result<Option<Note>, AppError> {
        self.put(&format!("/api/notes/{}/toggle-star", id), &serde_json::json!({}))
            .await
    }

    pub async fn restore_note(&self, id: i64) -> Result<(), AppError> {
        self.put_void(&format!("/api/notes/{}/restore", id), &serde_json::json!({}))
            .await
    }

    pub async fn permanent_delete_note(&self, id: i64) -> Result<(), AppError> {
        self.delete(&format!("/api/notes/{}/permanent", id)).await
    }

    pub async fn empty_trash(&self) -> Result<(), AppError> {
        self.delete("/api/notes/trash").await
    }

    pub async fn batch_move_notes(
        &self,
        note_ids: &[i64],
        notebook_id: i64,
    ) -> Result<(), AppError> {
        self.post_void(
            "/api/notes/batch-move",
            &serde_json::json!({
                "noteIds": note_ids,
                "notebookId": notebook_id,
            }),
        )
        .await
    }

    pub async fn batch_delete_notes(&self, note_ids: &[i64]) -> Result<(), AppError> {
        self.post_void(
            "/api/notes/batch-delete",
            &serde_json::json!({ "noteIds": note_ids }),
        )
        .await
    }
}

// ============================================================================
// Note Search
// ============================================================================

impl EnoteServerClient {
    pub async fn search_page_notes(
        &self,
        param: &NoteSearchPageParam,
    ) -> Result<PageResult<Note>, AppError> {
        self.post("/api/notes/search", param).await
    }

    pub async fn note_stats(
        &self,
        param: &NoteSearchPageParam,
    ) -> Result<NoteStatsResult, AppError> {
        self.post("/api/notes/stats", param).await
    }

    pub async fn find_deleted_notes(
        &self,
        page_param: &PageParam,
    ) -> Result<PageResult<Note>, AppError> {
        self.post("/api/notes/deleted", page_param).await
    }
}

// ============================================================================
// Note History
// ============================================================================

impl EnoteServerClient {
    pub async fn search_page_note_histories(
        &self,
        param: &NoteHistorySearchPageParam,
    ) -> Result<PageResult<NoteHistory>, AppError> {
        self.post("/api/note-histories/search", param).await
    }
}

// ============================================================================
// Settings
// ============================================================================

impl EnoteServerClient {
    pub async fn get_all_settings(&self) -> Result<HashMap<String, String>, AppError> {
        self.get("/api/settings").await
    }

    pub async fn save_settings(&self, settings: &HashMap<String, String>) -> Result<(), AppError> {
        self.put_void("/api/settings", settings).await
    }
}

// ============================================================================
// Note Template
// ============================================================================

impl EnoteServerClient {
    pub async fn find_all_templates(&self) -> Result<Vec<NoteTemplate>, AppError> {
        self.get("/api/templates").await
    }

    pub async fn create_template(
        &self,
        template: &NoteTemplate,
    ) -> Result<Option<NoteTemplate>, AppError> {
        self.post("/api/templates", template).await
    }

    pub async fn update_template(
        &self,
        template: &NoteTemplate,
    ) -> Result<Option<NoteTemplate>, AppError> {
        self.put(&format!("/api/templates/{}", template.id), template)
            .await
    }

    pub async fn delete_template_by_id(&self, id: i64) -> Result<(), AppError> {
        self.delete(&format!("/api/templates/{}", id)).await
    }
}

// ============================================================================
// Note Link
// ============================================================================

impl EnoteServerClient {
    pub async fn find_note_links(&self, note_id: i64) -> Result<Vec<NoteLink>, AppError> {
        self.get(&format!("/api/notes/{}/links", note_id)).await
    }

    pub async fn create_note_link(
        &self,
        source_note_id: i64,
        target_note_id: i64,
    ) -> Result<(), AppError> {
        self.post_void(
            "/api/note-links",
            &serde_json::json!({
                "sourceNoteId": source_note_id,
                "targetNoteId": target_note_id,
            }),
        )
        .await
    }

    pub async fn delete_note_link(&self, id: i64) -> Result<(), AppError> {
        self.delete(&format!("/api/note-links/{}", id)).await
    }

    pub async fn search_linkable_notes(
        &self,
        note_id: i64,
        keyword: &str,
    ) -> Result<Vec<NoteLink>, AppError> {
        self.post(
            "/api/note-links/search",
            &serde_json::json!({
                "noteId": note_id,
                "keyword": keyword,
            }),
        )
        .await
    }
}

// ============================================================================
// Lock Screen Auth
// ============================================================================

impl EnoteServerClient {
    pub async fn set_lock_password(&self, password: &str) -> Result<(), AppError> {
        self.put_void(
            "/api/auth/lock-password",
            &serde_json::json!({ "password": password }),
        )
        .await
    }

    pub async fn verify_lock_password(&self, password: &str) -> Result<bool, AppError> {
        self.post(
            "/api/auth/verify-lock",
            &serde_json::json!({ "password": password }),
        )
        .await
    }

    pub async fn clear_lock_password(&self) -> Result<(), AppError> {
        self.delete("/api/auth/lock-password").await
    }
}

// ============================================================================
// App Log
// ============================================================================

impl EnoteServerClient {
    pub async fn search_page_app_logs(
        &self,
        param: &AppLogSearchParam,
    ) -> Result<PageResult<AppLog>, AppError> {
        self.post("/api/app-logs/search", param).await
    }

    pub async fn delete_app_log(&self, id: i64) -> Result<(), AppError> {
        self.delete(&format!("/api/app-logs/{}", id)).await
    }

    pub async fn delete_batch_app_logs(&self, ids: &[i64]) -> Result<u64, AppError> {
        self.post(
            "/api/app-logs/batch-delete",
            &serde_json::json!({ "ids": ids }),
        )
        .await
    }

    pub async fn clear_app_logs(&self) -> Result<u64, AppError> {
        self.delete_with_response("/api/app-logs").await
    }

    pub async fn cleanup_app_logs_before(&self, before: &str) -> Result<u64, AppError> {
        self.post(
            "/api/app-logs/cleanup",
            &serde_json::json!({ "before": before }),
        )
        .await
    }

    pub async fn write_frontend_log(&self, log: &AppLog) -> Result<(), AppError> {
        self.post_void("/api/app-logs", log).await
    }
}

// ============================================================================
// Attachment
// ============================================================================

impl EnoteServerClient {
    pub async fn save_attachment(
        &self,
        note_id: i64,
        file_name: &str,
        file_data: &[u8],
        mime_type: &str,
    ) -> Result<NoteAttachment, AppError> {
        use reqwest::multipart::{Form, Part};

        let part = Part::bytes(file_data.to_vec())
            .file_name(file_name.to_string())
            .mime_str(mime_type)
            .map_err(|e| AppError::code_with_args("SERVER_REQUEST_FAILED", vec![e.to_string()]))?;

        let form = Form::new()
            .text("noteId", note_id.to_string())
            .part("file", part);

        let response = self.upload_multipart("/api/attachments", form).await?;
        let status = response.status();

        if status.is_success() {
            response.json::<NoteAttachment>().await.map_err(|e| {
                AppError::code_with_args("SERVER_PARSE_ERROR", vec![e.to_string()])
            })
        } else {
            let body = response.text().await.unwrap_or_default();
            Err(map_http_error(status, &body))
        }
    }

    pub async fn find_attachments(&self, note_id: i64) -> Result<Vec<NoteAttachment>, AppError> {
        self.get(&format!("/api/notes/{}/attachments", note_id))
            .await
    }

    pub async fn delete_attachment(&self, id: i64) -> Result<(), AppError> {
        self.delete(&format!("/api/attachments/{}", id)).await
    }

    pub async fn get_attachment_stats(&self) -> Result<AttachmentStats, AppError> {
        self.get("/api/attachments/stats").await
    }

    pub async fn cleanup_orphan_attachments(&self) -> Result<u32, AppError> {
        self.post("/api/attachments/cleanup", &serde_json::json!({}))
            .await
    }
}
