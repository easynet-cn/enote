//! Tauri 命令处理器模块
//!
//! 本模块定义了所有可从前端调用的 Tauri 命令。
//! 每个命令都是一个异步函数，接收前端传入的参数，
//! 调用相应的服务层方法，并返回结果给前端。
//!
//! # 命令分类
//! - 笔记本命令：CRUD 操作
//! - 标签命令：CRUD 操作
//! - 笔记命令：CRUD 和搜索操作
//! - 历史记录命令：分页搜索

use std::sync::Arc;

use std::collections::HashMap;

use crate::{
    config::AppState,
    error::AppError,
    model::{
        Note, NoteHistory, NoteHistorySearchPageParam, NoteSearchPageParam, NoteStatsResult,
        NoteLink, NoteTemplate, Notebook, OperateSource, PageResult, Tag,
    },
    service,
};

// ============================================================================
// Setup / Profile 相关命令
// ============================================================================

/// 检查是否需要初始化设置
#[tauri::command]
pub async fn check_setup_needed(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<bool, AppError> {
    Ok(service::profile::needs_setup(&app_state.app_data_dir))
}

/// 获取 Profile 索引信息
#[tauri::command]
pub async fn get_profile_index(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<service::profile::ProfileIndex, AppError> {
    service::profile::read_index(&app_state.app_data_dir).map_err(AppError::from)
}

/// 列出所有 Profile
#[tauri::command]
pub async fn list_profiles(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<service::profile::ProfileSummary>, AppError> {
    service::profile::list_profiles(&app_state.app_data_dir).map_err(AppError::from)
}

/// 获取单个 Profile 配置
#[tauri::command]
pub async fn get_profile(
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
) -> Result<service::profile::ProfileConfig, AppError> {
    service::profile::read_profile(&app_state.app_data_dir, &profile_id).map_err(AppError::from)
}

/// 保存 Profile 配置
///
/// 同时处理 Keychain 中的密码和加密密钥存储
#[tauri::command]
pub async fn save_profile_config(
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
    config: service::profile::ProfileConfig,
    db_password: Option<String>,
    encryption_key: Option<String>,
) -> Result<(), AppError> {
    // 保存 profile 文件
    service::profile::save_profile(&app_state.app_data_dir, &profile_id, &config)
        .map_err(AppError::from)?;

    // 保存数据库密码到 Keychain
    if let Some(password) = db_password {
        if !password.is_empty() {
            service::keychain::set_db_password(&profile_id, &password)
                .map_err(AppError::from)?;
        }
    }

    // 保存加密密钥到 Keychain
    if let Some(key) = encryption_key {
        if !key.is_empty() {
            service::keychain::set_encryption_key(&profile_id, &key)
                .map_err(AppError::from)?;
        }
    }

    Ok(())
}

/// 删除 Profile 配置
#[tauri::command]
pub async fn delete_profile_config(
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
) -> Result<(), AppError> {
    // 删除 Keychain 中的密钥
    service::keychain::delete_profile_secrets(&profile_id)
        .map_err(AppError::from)?;
    // 删除 profile 文件
    service::profile::delete_profile(&app_state.app_data_dir, &profile_id)
        .map_err(AppError::from)
}

/// 选择活跃 Profile
#[tauri::command]
pub async fn select_profile(
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
) -> Result<(), AppError> {
    service::profile::set_active_profile(&app_state.app_data_dir, &profile_id)
        .map_err(AppError::from)
}

/// 设置是否自动连接
#[tauri::command]
pub async fn set_auto_connect(
    app_state: tauri::State<'_, Arc<AppState>>,
    auto_connect: bool,
) -> Result<(), AppError> {
    service::profile::set_auto_connect(&app_state.app_data_dir, auto_connect)
        .map_err(AppError::from)
}

/// 测试数据库连接
#[tauri::command]
pub async fn test_db_connection(
    config: service::profile::ProfileConfig,
    db_password: Option<String>,
) -> Result<bool, AppError> {
    match crate::config::database_connection_from_profile(&config, db_password.as_deref()).await {
        Ok(_) => Ok(true),
        Err(e) => Err(AppError::Business(format!("连接失败: {}", e))),
    }
}

/// 生成加密密钥
#[tauri::command]
pub async fn generate_encryption_key() -> Result<String, AppError> {
    Ok(service::keychain::generate_encryption_key())
}

/// 使用指定 profile 重启应用
///
/// 设置活跃 profile 后通过前端触发 app restart
#[tauri::command]
pub async fn restart_with_profile(
    app_handle: tauri::AppHandle,
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
) -> Result<(), AppError> {
    service::profile::set_active_profile(&app_state.app_data_dir, &profile_id)
        .map_err(AppError::from)?;

    // 通过 Tauri 重启应用
    app_handle.restart();
}

// ============================================================================
// 数据备份相关命令
// ============================================================================

/// 导出数据库备份
#[tauri::command]
pub async fn export_backup(
    app_state: tauri::State<'_, Arc<AppState>>,
    format: String,
    path: String,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    match format.as_str() {
        "sql" => service::backup::export_sql(db, &path)
            .await
            .map_err(AppError::from)?,
        "excel" => service::backup::export_excel(db, &path)
            .await
            .map_err(AppError::from)?,
        "csv" => service::backup::export_csv(db, &path)
            .await
            .map_err(AppError::from)?,
        _ => return Err(AppError::Business("不支持的导出格式".to_string())),
    }
    Ok(())
}

/// 导入数据库备份
#[tauri::command]
pub async fn import_backup(
    app_state: tauri::State<'_, Arc<AppState>>,
    format: String,
    path: String,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    match format.as_str() {
        "sql" => service::backup::import_sql(db, &path)
            .await
            .map_err(AppError::from)?,
        "excel" => service::backup::import_excel(db, &path)
            .await
            .map_err(AppError::from)?,
        "csv" => service::backup::import_csv(db, &path)
            .await
            .map_err(AppError::from)?,
        _ => return Err(AppError::Business("不支持的导入格式".to_string())),
    }
    Ok(())
}

// ============================================================================
// 笔记本相关命令
// ============================================================================

/// 获取所有笔记本
///
/// # 返回
/// - `Ok(Vec<Notebook>)`: 笔记本列表
/// - `Err(AppError)`: 错误信息
#[tauri::command]
pub async fn find_all_notebooks(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<Notebook>, AppError> {
    let db = &app_state.database_connection;
    let notebooks = service::notebook::find_all(db)
        .await
        .map_err(AppError::from)?;
    Ok(notebooks)
}

/// 创建笔记本
///
/// # 参数
/// - `notebook`: 要创建的笔记本数据
///
/// # 返回
/// - `Ok(Some(Notebook))`: 创建成功，返回新笔记本
/// - `Ok(None)`: 创建后未找到（异常情况）
/// - `Err(AppError)`: 创建失败
#[tauri::command]
pub async fn create_notebook(
    app_state: tauri::State<'_, Arc<AppState>>,
    notebook: Notebook,
) -> Result<Option<Notebook>, AppError> {
    notebook.validate().map_err(AppError::from)?;
    let db = &app_state.database_connection;
    service::notebook::create(db, &notebook)
        .await
        .map_err(AppError::from)
}

/// 根据 ID 删除笔记本
///
/// # 参数
/// - `id`: 笔记本 ID
///
/// # 返回
/// - `Ok(())`: 删除成功
/// - `Err(AppError)`: 删除失败
#[tauri::command]
pub async fn delete_notebook_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::notebook::delete_by_id(db, id)
        .await
        .map_err(AppError::from)
}

/// 更新笔记本
///
/// # 参数
/// - `notebook`: 包含更新数据的笔记本对象（需包含 ID）
///
/// # 返回
/// - `Ok(Some(Notebook))`: 更新成功，返回更新后的笔记本
/// - `Ok(None)`: 笔记本不存在
/// - `Err(AppError)`: 更新失败
#[tauri::command]
pub async fn update_notebook(
    app_state: tauri::State<'_, Arc<AppState>>,
    notebook: Notebook,
) -> Result<Option<Notebook>, AppError> {
    notebook.validate().map_err(AppError::from)?;
    let db = &app_state.database_connection;
    service::notebook::update(db, &notebook)
        .await
        .map_err(AppError::from)
}

// ============================================================================
// 标签相关命令
// ============================================================================

/// 获取所有标签
///
/// # 返回
/// - `Ok(Vec<Tag>)`: 标签列表
/// - `Err(AppError)`: 错误信息
#[tauri::command]
pub async fn find_all_tags(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<Tag>, AppError> {
    let db = &app_state.database_connection;
    let tags = service::tag::find_all(db).await.map_err(AppError::from)?;
    Ok(tags)
}

/// 创建标签
///
/// # 参数
/// - `tag`: 要创建的标签数据
///
/// # 返回
/// - `Ok(Some(Tag))`: 创建成功，返回新标签
/// - `Ok(None)`: 创建后未找到（异常情况）
/// - `Err(AppError)`: 创建失败
#[tauri::command]
pub async fn create_tag(
    app_state: tauri::State<'_, Arc<AppState>>,
    tag: Tag,
) -> Result<Option<Tag>, AppError> {
    tag.validate().map_err(AppError::from)?;
    let db = &app_state.database_connection;
    service::tag::create(db, &tag).await.map_err(AppError::from)
}

/// 根据 ID 删除标签
///
/// # 参数
/// - `id`: 标签 ID
///
/// # 返回
/// - `Ok(())`: 删除成功
/// - `Err(AppError)`: 删除失败
#[tauri::command]
pub async fn delete_tag_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::tag::delete_by_id(db, id)
        .await
        .map_err(AppError::from)
}

/// 更新标签
///
/// # 参数
/// - `tag`: 包含更新数据的标签对象（需包含 ID）
///
/// # 返回
/// - `Ok(Some(Tag))`: 更新成功，返回更新后的标签
/// - `Ok(None)`: 标签不存在
/// - `Err(AppError)`: 更新失败
#[tauri::command]
pub async fn update_tag(
    app_state: tauri::State<'_, Arc<AppState>>,
    tag: Tag,
) -> Result<Option<Tag>, AppError> {
    tag.validate().map_err(AppError::from)?;
    let db = &app_state.database_connection;
    service::tag::update(db, &tag).await.map_err(AppError::from)
}

// ============================================================================
// 笔记相关命令
// ============================================================================

/// 创建笔记
#[tauri::command]
pub async fn create_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    note: Note,
) -> Result<Option<Note>, AppError> {
    note.validate().map_err(AppError::from)?;
    let db = &app_state.database_connection;
    service::note::create_with_key(
        db,
        &note,
        OperateSource::User,
        app_state.encryption_key.as_deref(),
    )
    .await
    .map_err(AppError::from)
}

/// 更新笔记
#[tauri::command]
pub async fn update_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    note: Note,
) -> Result<Option<Note>, AppError> {
    note.validate().map_err(AppError::from)?;
    let db = &app_state.database_connection;
    service::note::update_with_key(
        db,
        &note,
        OperateSource::User,
        app_state.encryption_key.as_deref(),
    )
    .await
    .map_err(AppError::from)
}

/// 根据 ID 删除笔记
///
/// # 参数
/// - `id`: 笔记 ID
///
/// # 返回
/// - `Ok(())`: 删除成功
/// - `Err(AppError)`: 删除失败
///
/// # 说明
/// 删除操作会自动创建历史记录，并清理关联的标签关系
#[tauri::command]
pub async fn delete_note_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::note::delete_by_id(db, id)
        .await
        .map_err(AppError::from)
}

/// 分页搜索笔记
#[tauri::command]
pub async fn search_page_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut search_param: NoteSearchPageParam,
) -> Result<PageResult<Note>, AppError> {
    search_param.normalize();
    let db = &app_state.database_connection;
    service::note::search_page_with_key(db, &search_param, app_state.encryption_key.as_deref())
        .await
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn note_stats(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut search_param: NoteSearchPageParam,
) -> Result<NoteStatsResult, AppError> {
    search_param.normalize();
    let db = &app_state.database_connection;
    service::note::stats(db, &search_param)
        .await
        .map_err(AppError::from)
}

// ============================================================================
// 历史记录相关命令
// ============================================================================

/// 分页搜索笔记历史记录
///
/// # 参数
/// - `search_param`: 搜索参数，包含：
///   - `page_index`: 页码（从 1 开始）
///   - `page_size`: 每页数量
///   - `note_id`: 笔记 ID
///
/// # 返回
/// - `Ok(PageResult<NoteHistory>)`: 历史记录分页结果
/// - `Err(AppError)`: 搜索失败
#[tauri::command]
pub async fn search_page_note_histories(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut search_param: NoteHistorySearchPageParam,
) -> Result<PageResult<NoteHistory>, AppError> {
    search_param.page_param.normalize();
    let db = &app_state.database_connection;
    service::note_history::search_page(db, &search_param)
        .await
        .map_err(AppError::from)
}

// ============================================================================
// 设置相关命令
// ============================================================================

/// 获取所有设置（带内存缓存）
#[tauri::command]
pub async fn get_all_settings(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<HashMap<String, String>, AppError> {
    // 尝试从缓存读取
    {
        let cache = app_state.settings_cache.read().await;
        if let Some(ref cached) = *cache {
            return Ok(cached.clone());
        }
    }

    // 缓存未命中，从数据库读取
    let db = &app_state.database_connection;
    let settings = service::settings::get_all(db).await.map_err(AppError::from)?;

    // 写入缓存
    {
        let mut cache = app_state.settings_cache.write().await;
        *cache = Some(settings.clone());
    }

    Ok(settings)
}

/// 保存设置（同时失效缓存）
#[tauri::command]
pub async fn save_settings(
    app_state: tauri::State<'_, Arc<AppState>>,
    settings: HashMap<String, String>,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::settings::save(db, settings)
        .await
        .map_err(AppError::from)?;

    // 失效缓存
    {
        let mut cache = app_state.settings_cache.write().await;
        *cache = None;
    }

    Ok(())
}

// ============================================================================
// 笔记置顶命令
// ============================================================================

/// 切换笔记置顶状态
#[tauri::command]
pub async fn toggle_note_pin(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<Option<Note>, AppError> {
    let db = &app_state.database_connection;
    service::note::toggle_pin(db, id)
        .await
        .map_err(AppError::from)
}

// ============================================================================
// 回收站相关命令
// ============================================================================

/// 从回收站恢复笔记
#[tauri::command]
pub async fn restore_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::note::restore_by_id(db, id)
        .await
        .map_err(AppError::from)
}

/// 永久删除笔记
#[tauri::command]
pub async fn permanent_delete_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::note::permanent_delete_by_id(db, id, OperateSource::User)
        .await
        .map_err(AppError::from)
}

/// 清空回收站
#[tauri::command]
pub async fn empty_trash(app_state: tauri::State<'_, Arc<AppState>>) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::note::empty_trash(db).await.map_err(AppError::from)
}

/// 获取回收站笔记列表
#[tauri::command]
pub async fn find_deleted_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<Note>, AppError> {
    let db = &app_state.database_connection;
    service::note::find_deleted_with_key(db, app_state.encryption_key.as_deref())
        .await
        .map_err(AppError::from)
}

// ============================================================================
// 排序相关命令
// ============================================================================

/// 批量更新笔记本排序
#[tauri::command]
pub async fn reorder_notebooks(
    app_state: tauri::State<'_, Arc<AppState>>,
    orders: Vec<(i64, i32)>,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::notebook::reorder(db, orders)
        .await
        .map_err(AppError::from)
}

/// 批量更新标签排序
#[tauri::command]
pub async fn reorder_tags(
    app_state: tauri::State<'_, Arc<AppState>>,
    orders: Vec<(i64, i32)>,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::tag::reorder(db, orders)
        .await
        .map_err(AppError::from)
}

// ============================================================================
// 图片相关命令
// ============================================================================

/// 保存图片到本地文件
///
/// 将 Base64 编码的图片数据保存为本地文件，返回 asset protocol URL
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
pub async fn delete_image(path: String) -> Result<(), AppError> {
    service::image::delete_image(&path).map_err(AppError::from)
}

// ============================================================================
// 自动备份相关命令
// ============================================================================

/// 执行一次自动备份
#[tauri::command]
pub async fn auto_backup(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<String, AppError> {
    let db = &app_state.database_connection;
    let filename = service::backup::auto_backup(db, &app_state.app_data_dir)
        .await
        .map_err(AppError::from)?;
    Ok(filename)
}

/// 清理旧备份文件
#[tauri::command]
pub async fn cleanup_old_backups(
    app_state: tauri::State<'_, Arc<AppState>>,
    max_count: usize,
) -> Result<u32, AppError> {
    service::backup::cleanup_old_backups(&app_state.app_data_dir, max_count)
        .map_err(AppError::from)
}

/// 列出所有自动备份
#[tauri::command]
pub async fn list_auto_backups(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<(String, u64)>, AppError> {
    service::backup::list_backups(&app_state.app_data_dir).map_err(AppError::from)
}

// ============================================================================
// 模板相关命令
// ============================================================================

#[tauri::command]
pub async fn find_all_templates(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<NoteTemplate>, AppError> {
    let db = &app_state.database_connection;
    service::note_template::find_all(db)
        .await
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn create_template(
    app_state: tauri::State<'_, Arc<AppState>>,
    template: NoteTemplate,
) -> Result<Option<NoteTemplate>, AppError> {
    template.validate().map_err(AppError::from)?;
    let db = &app_state.database_connection;
    service::note_template::create(db, &template)
        .await
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn update_template(
    app_state: tauri::State<'_, Arc<AppState>>,
    template: NoteTemplate,
) -> Result<Option<NoteTemplate>, AppError> {
    template.validate().map_err(AppError::from)?;
    let db = &app_state.database_connection;
    service::note_template::update(db, &template)
        .await
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn delete_template_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::note_template::delete_by_id(db, id)
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
    let db = &app_state.database_connection;
    service::note_link::find_links(db, note_id)
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
    let db = &app_state.database_connection;
    service::note_link::create_link(db, source_note_id, target_note_id)
        .await
        .map_err(AppError::from)
}

/// 删除笔记链接
#[tauri::command]
pub async fn delete_note_link(
    app_state: tauri::State<'_, Arc<AppState>>,
    link_id: i64,
) -> Result<(), AppError> {
    let db = &app_state.database_connection;
    service::note_link::delete_link(db, link_id)
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
    let db = &app_state.database_connection;
    service::note_link::search_linkable_notes(db, note_id, &keyword)
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
    service::auth::set_password(&app_state.database_connection, &password)
        .await
        .map_err(AppError::from)
}

/// 验证锁屏密码
#[tauri::command]
pub async fn verify_lock_password(
    app_state: tauri::State<'_, Arc<AppState>>,
    password: String,
) -> Result<bool, AppError> {
    service::auth::verify_password(&app_state.database_connection, &password)
        .await
        .map_err(AppError::from)
}

/// 清除锁屏密码
#[tauri::command]
pub async fn clear_lock_password(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<(), AppError> {
    service::auth::clear_password(&app_state.database_connection)
        .await
        .map_err(AppError::from)
}
