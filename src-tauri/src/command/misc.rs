use super::*;

// ============================================================================
// 图片相关命令
// ============================================================================

/// 保存图片到本地文件
#[tauri::command]
pub async fn save_image(
    app_state: tauri::State<'_, Arc<AppState>>,
    base64_data: String,
) -> Result<String, AppError> {
    let path = service::image::save_image(&app_state.app_data_dir, &base64_data)
        .map_err(AppError::from)?;
    Ok(path)
}

/// 删除本地图片文件
#[tauri::command]
pub async fn delete_image(
    app_state: tauri::State<'_, Arc<AppState>>,
    path: String,
) -> Result<(), AppError> {
    service::image::delete_image(&app_state.app_data_dir, &path).map_err(AppError::from)
}

// ============================================================================
// 模板相关命令
// ============================================================================

#[tauri::command]
pub async fn find_all_templates(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<NoteTemplate>, AppError> {
    let db = require_db(&app_state).await?;
    service::note_template::find_all(&db)
        .await
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn create_template(
    app_state: tauri::State<'_, Arc<AppState>>,
    template: NoteTemplate,
) -> Result<Option<NoteTemplate>, AppError> {
    template.validate().map_err(AppError::from)?;
    let db = require_db(&app_state).await?;
    service::note_template::create(&db, &template)
        .await
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn update_template(
    app_state: tauri::State<'_, Arc<AppState>>,
    template: NoteTemplate,
) -> Result<Option<NoteTemplate>, AppError> {
    template.validate().map_err(AppError::from)?;
    let db = require_db(&app_state).await?;
    service::note_template::update(&db, &template)
        .await
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn delete_template_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::note_template::delete_by_id(&db, id)
        .await
        .map_err(AppError::from)
}

// ============================================================================
// 笔记链接相关命令
// ============================================================================

/// 获取笔记的所有链接
#[tauri::command]
pub async fn find_note_links(
    app_state: tauri::State<'_, Arc<AppState>>,
    note_id: i64,
) -> Result<Vec<NoteLink>, AppError> {
    let db = require_db(&app_state).await?;
    service::note_link::find_links(&db, note_id)
        .await
        .map_err(AppError::from)
}

/// 创建笔记链接
#[tauri::command]
pub async fn create_note_link(
    app_state: tauri::State<'_, Arc<AppState>>,
    source_note_id: i64,
    target_note_id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::note_link::create_link(&db, source_note_id, target_note_id)
        .await
        .map_err(AppError::from)
}

/// 删除笔记链接
#[tauri::command]
pub async fn delete_note_link(
    app_state: tauri::State<'_, Arc<AppState>>,
    link_id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::note_link::delete_link(&db, link_id)
        .await
        .map_err(AppError::from)
}

/// 搜索可链接的笔记
#[tauri::command]
pub async fn search_linkable_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
    note_id: i64,
    keyword: String,
) -> Result<Vec<NoteLink>, AppError> {
    let db = require_db(&app_state).await?;
    service::note_link::search_linkable_notes(&db, note_id, &keyword)
        .await
        .map_err(AppError::from)
}

// ============================================================================
// 加密相关命令
// ============================================================================

/// 加密笔记内容
#[tauri::command]
pub async fn encrypt_content(content: String, password: String) -> Result<String, AppError> {
    service::crypto::encrypt(&content, &password).map_err(AppError::from)
}

/// 解密笔记内容
#[tauri::command]
pub async fn decrypt_content(content: String, password: String) -> Result<String, AppError> {
    service::crypto::decrypt(&content, &password).map_err(AppError::from)
}

/// 检查内容是否已加密
#[tauri::command]
pub async fn is_content_encrypted(content: String) -> Result<bool, AppError> {
    Ok(service::crypto::is_encrypted(&content))
}

// ============================================================================
// 锁屏认证相关命令
// ============================================================================

/// 设置锁屏密码
#[tauri::command]
pub async fn set_lock_password(
    app_state: tauri::State<'_, Arc<AppState>>,
    password: String,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::auth::set_password(&db, &password)
        .await
        .map_err(AppError::from)
}

/// 验证锁屏密码
#[tauri::command]
pub async fn verify_lock_password(
    app_state: tauri::State<'_, Arc<AppState>>,
    password: String,
) -> Result<bool, AppError> {
    let db = require_db(&app_state).await?;
    service::auth::verify_password(&db, &password)
        .await
        .map_err(AppError::from)
}

/// 清除锁屏密码
#[tauri::command]
pub async fn clear_lock_password(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::auth::clear_password(&db)
        .await
        .map_err(AppError::from)
}

// ============================================================================
// 附件相关命令
// ============================================================================

/// 保存附件
#[tauri::command]
pub async fn save_attachment(
    app_state: tauri::State<'_, Arc<AppState>>,
    note_id: i64,
    file_name: String,
    file_data: Vec<u8>,
    mime_type: String,
) -> Result<NoteAttachment, AppError> {
    let db = require_db(&app_state).await?;
    service::attachment::save_attachment(
        &db,
        &app_state.app_data_dir,
        note_id,
        &file_name,
        &file_data,
        &mime_type,
    )
    .await
    .map_err(AppError::from)
}

/// 查询笔记附件列表
#[tauri::command]
pub async fn find_attachments(
    app_state: tauri::State<'_, Arc<AppState>>,
    note_id: i64,
) -> Result<Vec<NoteAttachment>, AppError> {
    let db = require_db(&app_state).await?;
    service::attachment::find_by_note_id(&db, note_id)
        .await
        .map_err(AppError::from)
}

/// 删除附件
#[tauri::command]
pub async fn delete_attachment(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::attachment::delete_by_id(&db, &app_state.app_data_dir, id)
        .await
        .map_err(AppError::from)
}

/// 使用系统默认程序打开附件
#[tauri::command]
pub async fn open_attachment(
    app_handle: tauri::AppHandle,
    app_state: tauri::State<'_, Arc<AppState>>,
    file_path: String,
) -> Result<(), AppError> {
    use tauri_plugin_opener::OpenerExt;

    // 安全校验：防止路径穿越
    if file_path.contains("..") || file_path.contains('/') || file_path.contains('\\') {
        return Err(AppError::Business("Invalid attachment file path".to_string()));
    }

    let full_path = app_state
        .app_data_dir
        .join("attachments")
        .join(&file_path);

    if !full_path.exists() {
        return Err(AppError::code("ATTACHMENT_NOT_FOUND"));
    }

    let path_str = full_path.to_string_lossy().to_string();
    app_handle
        .opener()
        .open_path(&path_str, None::<&str>)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("{}", e)))
}

/// 获取附件统计信息
#[tauri::command]
pub async fn get_attachment_stats(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<AttachmentStats, AppError> {
    let db = require_db(&app_state).await?;
    service::attachment::get_stats(&db, &app_state.app_data_dir)
        .await
        .map_err(AppError::from)
}

/// 清理孤立附件文件
#[tauri::command]
pub async fn cleanup_orphan_attachments(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<u32, AppError> {
    let db = require_db(&app_state).await?;
    service::attachment::cleanup_orphans(&db, &app_state.app_data_dir)
        .await
        .map_err(AppError::from)
}

// ============================================================================
// 帮助手册相关命令
// ============================================================================

/// 读取帮助手册内容
#[tauri::command]
pub async fn read_help_manual(app_handle: tauri::AppHandle, lang: String) -> Result<String, AppError> {
    let filename = match lang.as_str() {
        "en-US" => "manual-en-US.md",
        _ => "manual-zh-CN.md",
    };

    let resource_path = app_handle
        .path()
        .resource_dir()
        .map_err(|e| AppError::code_with_args("RESOURCE_DIR_ERROR", vec![e.to_string()]))?
        .join("resources/help")
        .join(filename);

    std::fs::read_to_string(&resource_path).map_err(|e| {
        AppError::code_with_args("HELP_MANUAL_READ_FAILED", vec![e.to_string()])
    })
}
