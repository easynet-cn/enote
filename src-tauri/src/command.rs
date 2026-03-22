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

use tracing::info;

use sea_orm::DatabaseConnection;

use crate::{
    config::AppState,
    error::AppError,
    model::{
        Note, NoteHistory, NoteHistorySearchPageParam, NoteLink, NoteSearchPageParam,
        NoteStatsResult, NoteTemplate, Notebook, OperateSource, PageParam, PageResult, SyncLog,
        SyncLogDetail, SyncOptions, SyncPreview, Tag,
    },
    service,
};

/// 从 AppState 获取数据库连接，未连接时返回明确错误。
///
/// `DatabaseConnection` 内部基于 `Arc<Pool>`，clone 为 O(1) 操作。
async fn require_db(app_state: &AppState) -> Result<DatabaseConnection, AppError> {
    app_state
        .database_connection
        .read()
        .await
        .clone()
        .ok_or_else(|| AppError::code("DB_NOT_CONNECTED"))
}

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
            service::keychain::set_db_password(&profile_id, &password).map_err(AppError::from)?;
        }
    }

    // 保存加密密钥到 Keychain
    if let Some(key) = encryption_key {
        if !key.is_empty() {
            service::keychain::set_encryption_key(&profile_id, &key).map_err(AppError::from)?;
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
    service::keychain::delete_profile_secrets(&profile_id).map_err(AppError::from)?;
    // 删除 profile 文件
    service::profile::delete_profile(&app_state.app_data_dir, &profile_id).map_err(AppError::from)
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
#[tauri::command(rename_all = "camelCase")]
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
        Err(e) => Err(AppError::code_with_args("DB_CONNECTION_FAILED", vec![e.to_string()])),
    }
}

/// 生成加密密钥
#[tauri::command]
pub async fn generate_encryption_key() -> Result<String, AppError> {
    Ok(service::keychain::generate_encryption_key())
}

/// 使用指定 profile 重启应用（仅 release 构建使用）
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

/// 热切换 Profile（不重启进程）
///
/// 关闭旧数据库连接，重新连接新 profile 的数据库，替换 AppState 中的连接。
/// 前端调用后需刷新所有数据。
#[tauri::command]
pub async fn reconnect_profile(
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
) -> Result<(), AppError> {
    info!("热切换 Profile: {}", profile_id);

    // 1. 设置活跃 profile
    service::profile::set_active_profile(&app_state.app_data_dir, &profile_id)
        .map_err(AppError::from)?;

    // 2. 读取新 profile 配置
    let profile_config = service::profile::read_profile(&app_state.app_data_dir, &profile_id)
        .map_err(AppError::from)?;

    // 3. 从 Keychain 获取数据库密码（桌面端/db-full 模式 MySQL/PG 需要）
    #[cfg(any(feature = "desktop", feature = "db-full"))]
    let db_password = if profile_config.datasource.db_type != "sqlite"
        && profile_config.datasource.auth_method != "certificate"
    {
        service::keychain::get_db_password(&profile_id).map_err(AppError::from)?
    } else {
        None
    };
    #[cfg(not(any(feature = "desktop", feature = "db-full")))]
    let db_password: Option<String> = None;

    // 4. 创建新数据库连接
    let new_db =
        crate::config::database_connection_from_profile(&profile_config, db_password.as_deref())
            .await
            .map_err(|e| AppError::code_with_args("DB_CONNECTION_FAILED", vec![e.to_string()]))?;

    // 5. 运行数据库迁移
    use sea_orm_migration::MigratorTrait;
    crate::migration::Migrator::up(&new_db, None)
        .await
        .map_err(|e| AppError::code_with_args("DB_MIGRATION_FAILED", vec![e.to_string()]))?;

    // 6. 从 Keychain 获取加密密钥（桌面端/db-full 模式）
    #[cfg(any(feature = "desktop", feature = "db-full"))]
    let new_encryption_key = if profile_config.security.content_encryption {
        service::keychain::get_encryption_key(&profile_id).map_err(AppError::from)?
    } else {
        None
    };
    #[cfg(not(any(feature = "desktop", feature = "db-full")))]
    let new_encryption_key: Option<String> = None;

    // 7. 替换 AppState 中的连接和相关状态
    {
        let mut db_guard = app_state.database_connection.write().await;
        *db_guard = Some(new_db);
    }
    {
        let mut profile_guard = app_state.active_profile_id.write().await;
        *profile_guard = profile_id.clone();
    }
    {
        let mut key_guard = app_state.encryption_key.write().await;
        *key_guard = new_encryption_key;
    }
    // 清空设置缓存（新 profile 有不同的设置）
    {
        let mut cache = app_state.settings_cache.write().await;
        *cache = None;
    }

    info!("Profile 热切换完成: {}", profile_id);
    Ok(())
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
    let db = require_db(&app_state).await?;
    match format.as_str() {
        "sql" => service::backup::export_sql(&db, &path)
            .await
            .map_err(AppError::from)?,
        "excel" => service::backup::export_excel(&db, &path)
            .await
            .map_err(AppError::from)?,
        "csv" => service::backup::export_csv(&db, &path)
            .await
            .map_err(AppError::from)?,
        _ => return Err(AppError::code("UNSUPPORTED_EXPORT_FORMAT")),
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
    let db = require_db(&app_state).await?;
    match format.as_str() {
        "sql" => service::backup::import_sql(&db, &path)
            .await
            .map_err(AppError::from)?,
        "excel" => service::backup::import_excel(&db, &path)
            .await
            .map_err(AppError::from)?,
        "csv" => service::backup::import_csv(&db, &path)
            .await
            .map_err(AppError::from)?,
        _ => return Err(AppError::code("UNSUPPORTED_IMPORT_FORMAT")),
    }
    Ok(())
}

// ============================================================================
// 笔记本相关命令
// ============================================================================

/// 获取所有笔记本
#[tauri::command]
pub async fn find_all_notebooks(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<Notebook>, AppError> {
    let db = require_db(&app_state).await?;
    let notebooks = service::notebook::find_all(&db)
        .await
        .map_err(AppError::from)?;
    Ok(notebooks)
}

/// 创建笔记本
#[tauri::command]
pub async fn create_notebook(
    app_state: tauri::State<'_, Arc<AppState>>,
    notebook: Notebook,
) -> Result<Option<Notebook>, AppError> {
    notebook.validate().map_err(AppError::from)?;
    let db = require_db(&app_state).await?;
    service::notebook::create(&db, &notebook)
        .await
        .map_err(AppError::from)
}

/// 根据 ID 删除笔记本
#[tauri::command]
pub async fn delete_notebook_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::notebook::delete_by_id(&db, id)
        .await
        .map_err(AppError::from)
}

/// 更新笔记本
#[tauri::command]
pub async fn update_notebook(
    app_state: tauri::State<'_, Arc<AppState>>,
    notebook: Notebook,
) -> Result<Option<Notebook>, AppError> {
    notebook.validate().map_err(AppError::from)?;
    let db = require_db(&app_state).await?;
    service::notebook::update(&db, &notebook)
        .await
        .map_err(AppError::from)
}

// ============================================================================
// 标签相关命令
// ============================================================================

/// 获取所有标签
#[tauri::command]
pub async fn find_all_tags(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<Tag>, AppError> {
    let db = require_db(&app_state).await?;
    let tags = service::tag::find_all(&db).await.map_err(AppError::from)?;
    Ok(tags)
}

/// 创建标签
#[tauri::command]
pub async fn create_tag(
    app_state: tauri::State<'_, Arc<AppState>>,
    tag: Tag,
) -> Result<Option<Tag>, AppError> {
    tag.validate().map_err(AppError::from)?;
    let db = require_db(&app_state).await?;
    service::tag::create(&db, &tag).await.map_err(AppError::from)
}

/// 根据 ID 删除标签
#[tauri::command]
pub async fn delete_tag_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::tag::delete_by_id(&db, id)
        .await
        .map_err(AppError::from)
}

/// 更新标签
#[tauri::command]
pub async fn update_tag(
    app_state: tauri::State<'_, Arc<AppState>>,
    tag: Tag,
) -> Result<Option<Tag>, AppError> {
    tag.validate().map_err(AppError::from)?;
    let db = require_db(&app_state).await?;
    service::tag::update(&db, &tag).await.map_err(AppError::from)
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
    let db = require_db(&app_state).await?;
    let enc_key = app_state.encryption_key.read().await;
    service::note::create_with_key(
        &db,
        &note,
        OperateSource::User,
        enc_key.as_deref(),
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
    let db = require_db(&app_state).await?;
    let enc_key = app_state.encryption_key.read().await;
    service::note::update_with_key(
        &db,
        &note,
        OperateSource::User,
        enc_key.as_deref(),
    )
    .await
    .map_err(AppError::from)
}

/// 根据 ID 删除笔记
#[tauri::command]
pub async fn delete_note_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::note::delete_by_id(&db, id)
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
    let db = require_db(&app_state).await?;
    let enc_key = app_state.encryption_key.read().await;
    service::note::search_page_with_key(&db, &search_param, enc_key.as_deref())
        .await
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn note_stats(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut search_param: NoteSearchPageParam,
) -> Result<NoteStatsResult, AppError> {
    search_param.normalize();
    let db = require_db(&app_state).await?;
    service::note::stats(&db, &search_param)
        .await
        .map_err(AppError::from)
}

// ============================================================================
// 历史记录相关命令
// ============================================================================

/// 分页搜索笔记历史记录
#[tauri::command]
pub async fn search_page_note_histories(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut search_param: NoteHistorySearchPageParam,
) -> Result<PageResult<NoteHistory>, AppError> {
    search_param.page_param.normalize();
    let db = require_db(&app_state).await?;
    service::note_history::search_page(&db, &search_param)
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
    let db = require_db(&app_state).await?;
    let settings = service::settings::get_all(&db)
        .await
        .map_err(AppError::from)?;

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
    let db = require_db(&app_state).await?;
    service::settings::save(&db, settings)
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
    let db = require_db(&app_state).await?;
    service::note::toggle_pin(&db, id)
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
    let db = require_db(&app_state).await?;
    service::note::restore_by_id(&db, id)
        .await
        .map_err(AppError::from)
}

/// 永久删除笔记
#[tauri::command]
pub async fn permanent_delete_note(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::note::permanent_delete_by_id(&db, id, OperateSource::User)
        .await
        .map_err(AppError::from)
}

/// 清空回收站
#[tauri::command]
pub async fn empty_trash(app_state: tauri::State<'_, Arc<AppState>>) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::note::empty_trash(&db).await.map_err(AppError::from)
}

/// 获取回收站笔记列表
#[tauri::command]
pub async fn find_deleted_notes(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<Note>, AppError> {
    let db = require_db(&app_state).await?;
    let enc_key = app_state.encryption_key.read().await;
    service::note::find_deleted_with_key(&db, enc_key.as_deref())
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
    let db = require_db(&app_state).await?;
    service::notebook::reorder(&db, orders)
        .await
        .map_err(AppError::from)
}

/// 批量更新标签排序
#[tauri::command]
pub async fn reorder_tags(
    app_state: tauri::State<'_, Arc<AppState>>,
    orders: Vec<(i64, i32)>,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::tag::reorder(&db, orders)
        .await
        .map_err(AppError::from)
}

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
pub async fn delete_image(path: String) -> Result<(), AppError> {
    service::image::delete_image(&path).map_err(AppError::from)
}

// ============================================================================
// 自动备份相关命令
// ============================================================================

/// 执行一次自动备份
#[tauri::command]
pub async fn auto_backup(app_state: tauri::State<'_, Arc<AppState>>) -> Result<String, AppError> {
    let db = require_db(&app_state).await?;
    let filename = service::backup::auto_backup(&db, &app_state.app_data_dir)
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
    service::backup::cleanup_old_backups(&app_state.app_data_dir, max_count).map_err(AppError::from)
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
// 跨 Profile 同步相关命令
// ============================================================================

/// 获取同步预览信息
#[tauri::command]
pub async fn get_sync_preview(
    app_state: tauri::State<'_, Arc<AppState>>,
    target_profile_id: String,
    target_db_password: Option<String>,
) -> Result<SyncPreview, AppError> {
    let db = require_db(&app_state).await?;
    let profile_id = app_state.active_profile_id.read().await;
    service::sync::get_preview(
        &db,
        &app_state.app_data_dir,
        &*profile_id,
        &target_profile_id,
        target_db_password.as_deref(),
    )
    .await
    .map_err(AppError::from)
}

/// 开始同步
#[tauri::command]
pub async fn start_sync(
    app_state: tauri::State<'_, Arc<AppState>>,
    app_handle: tauri::AppHandle,
    options: SyncOptions,
    target_db_password: Option<String>,
    target_encryption_key: Option<String>,
) -> Result<SyncLog, AppError> {
    let db = require_db(&app_state).await?;
    let profile_id = app_state.active_profile_id.read().await;
    let enc_key = app_state.encryption_key.read().await;

    // 获取目标端加密密钥：优先使用前端传入的，否则从 Keychain 获取
    let target_key = if let Some(key) = target_encryption_key {
        if key.is_empty() { None } else { Some(key) }
    } else {
        service::keychain::get_encryption_key(&options.target_profile_id).unwrap_or(None)
    };

    service::sync::sync_to_profile(
        &db,
        &app_state.app_data_dir,
        &*profile_id,
        &options,
        target_db_password.as_deref(),
        enc_key.as_deref(),
        target_key.as_deref(),
        &app_handle,
    )
    .await
    .map_err(AppError::from)
}

/// 查询同步日志列表（分页）
#[tauri::command]
pub async fn find_sync_logs(
    app_state: tauri::State<'_, Arc<AppState>>,
    mut page: PageParam,
) -> Result<PageResult<SyncLog>, AppError> {
    page.normalize();
    let db = require_db(&app_state).await?;
    service::sync_log::search_page(&db, &page)
        .await
        .map_err(AppError::from)
}

/// 查询同步明细（分页，支持过滤）
#[tauri::command]
pub async fn find_sync_log_details(
    app_state: tauri::State<'_, Arc<AppState>>,
    sync_log_id: i64,
    table_name: Option<String>,
    status: Option<String>,
    mut page: PageParam,
) -> Result<PageResult<SyncLogDetail>, AppError> {
    page.normalize();
    let db = require_db(&app_state).await?;
    service::sync_log::search_detail_page(
        &db,
        sync_log_id,
        table_name.as_deref(),
        status.as_deref(),
        &page,
    )
    .await
    .map_err(AppError::from)
}

/// 删除同步日志
#[tauri::command]
pub async fn delete_sync_log(
    app_state: tauri::State<'_, Arc<AppState>>,
    sync_log_id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::sync_log::delete_by_id(&db, sync_log_id)
        .await
        .map_err(AppError::from)
}

/// 清空所有同步日志
#[tauri::command]
pub async fn clear_sync_logs(app_state: tauri::State<'_, Arc<AppState>>) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::sync_log::clear_all(&db)
        .await
        .map_err(AppError::from)
}

/// 导出同步日志为 JSON
#[tauri::command]
pub async fn export_sync_log(
    app_state: tauri::State<'_, Arc<AppState>>,
    sync_log_id: i64,
    path: String,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;

    // 获取日志
    let log = service::sync_log::find_by_id(&db, sync_log_id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::code("SYNC_LOG_NOT_FOUND"))?;

    // 获取所有明细
    let page = PageParam {
        page_index: 1,
        page_size: 10000,
    };
    let details = service::sync_log::search_detail_page(&db, sync_log_id, None, None, &page)
        .await
        .map_err(AppError::from)?;

    // 构造导出数据
    let export_data = serde_json::json!({
        "syncLog": log,
        "details": details.data,
    });

    // 写入文件
    let json_str = serde_json::to_string_pretty(&export_data)
        .map_err(|e| AppError::code_with_args("JSON_SERIALIZE_FAILED", vec![e.to_string()]))?;
    std::fs::write(&path, json_str)
        .map_err(|e| AppError::code_with_args("FILE_WRITE_FAILED", vec![e.to_string()]))?;

    Ok(())
}
