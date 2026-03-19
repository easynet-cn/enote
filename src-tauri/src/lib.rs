//! ENote 应用程序入口模块
//!
//! 本模块负责初始化 Tauri 应用程序，包括：
//! - 检测是否需要初始化设置
//! - 支持 Profile 模式（多数据库配置）
//! - 加载配置文件或进入设置向导
//! - 建立数据库连接
//! - 运行数据库迁移（自动创建表）
//! - 注册命令处理器
//! - 启动应用程序

use std::sync::Arc;

use sea_orm_migration::MigratorTrait;
use tauri::Manager;
use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::AppState;
use crate::i18n::{t, t_simple};

// 模块声明
mod command; // Tauri 命令处理器
pub mod config; // 配置管理
pub mod entity; // SeaORM 数据库实体
mod error; // 错误处理
pub mod i18n; // 国际化支持（必须在 config 之前声明）
pub mod migration; // 数据库迁移
pub mod model; // 数据传输对象（DTO）
pub mod service; // 业务逻辑服务层

/// 应用程序入口函数
///
/// 启动流程：
/// 1. 检查是否存在 profile 配置
/// 2. 如果没有 profile → 进入设置向导模式（精简命令集）
/// 3. 如果有 profile → 正常连接数据库并启动
///
/// # 参数
/// - `config_path`: 可选的配置文件路径，如果为 None 则使用 Profile 模式
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(config_path: Option<String>) {
    // 初始化 tracing 日志系统
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer())
        .init();

    tauri::Builder::default()
        // 注册 Tauri 插件
        .plugin(tauri_plugin_fs::init()) // 文件系统访问
        .plugin(tauri_plugin_opener::init()) // 打开外部链接/文件
        .plugin(tauri_plugin_shell::init()) // Shell 命令执行
        .plugin(tauri_plugin_dialog::init()) // 文件对话框
        // 应用初始化设置
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let handle = app.handle();

                // 获取应用数据目录
                let app_data_dir = handle
                    .path()
                    .app_data_dir()
                    .expect("无法获取应用数据目录");

                // 确保目录存在
                if !app_data_dir.exists() {
                    std::fs::create_dir_all(&app_data_dir)
                        .expect("无法创建应用数据目录");
                }

                // 决定启动模式
                if config_path.is_some() || !service::profile::needs_setup(&app_data_dir) {
                    // 正常模式：有配置，连接数据库
                    setup_normal_mode(app, config_path).await?;
                } else {
                    // 设置向导模式：没有任何 profile
                    info!("未找到 profile 配置，进入设置向导模式");
                    setup_wizard_mode(app).await?;
                }

                Ok(())
            })
        })
        // 注册前端可调用的命令处理器（包含所有命令，精简模式下部分命令会返回错误）
        .invoke_handler(tauri::generate_handler![
            // Setup / Profile 相关命令（始终可用）
            command::check_setup_needed,
            command::list_profiles,
            command::get_profile,
            command::save_profile_config,
            command::delete_profile_config,
            command::select_profile,
            command::test_db_connection,
            command::generate_encryption_key,
            command::get_profile_index,
            command::set_auto_connect,
            command::restart_with_profile,
            // 笔记本相关命令
            command::find_all_notebooks,
            command::create_notebook,
            command::delete_notebook_by_id,
            command::update_notebook,
            // 标签相关命令
            command::find_all_tags,
            command::create_tag,
            command::delete_tag_by_id,
            command::update_tag,
            // 笔记相关命令
            command::create_note,
            command::update_note,
            command::delete_note_by_id,
            command::search_page_notes,
            command::note_stats,
            // 历史记录相关命令
            command::search_page_note_histories,
            // 数据备份相关命令
            command::export_backup,
            command::import_backup,
            // 设置相关命令
            command::get_all_settings,
            command::save_settings,
            // 笔记置顶
            command::toggle_note_pin,
            // 回收站相关命令
            command::restore_note,
            command::permanent_delete_note,
            command::empty_trash,
            command::find_deleted_notes,
            // 排序相关命令
            command::reorder_notebooks,
            command::reorder_tags,
            // 图片相关命令
            command::save_image,
            command::delete_image,
            // 自动备份相关命令
            command::auto_backup,
            command::cleanup_old_backups,
            command::list_auto_backups,
            // 模板相关命令
            command::find_all_templates,
            command::create_template,
            command::update_template,
            command::delete_template_by_id,
            // 笔记链接相关命令
            command::find_note_links,
            command::create_note_link,
            command::delete_note_link,
            command::search_linkable_notes,
            // 加密相关命令
            command::encrypt_content,
            command::decrypt_content,
            command::is_content_encrypted,
            // 锁屏认证相关命令
            command::set_lock_password,
            command::verify_lock_password,
            command::clear_lock_password,
        ])
        .run(tauri::generate_context!())
        .expect(&t_simple("error.appStartFailed"));
}

/// 正常模式启动：连接数据库并初始化完整应用状态
async fn setup_normal_mode(
    app: &mut tauri::App,
    config_path: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let app_data_dir = handle.path().app_data_dir()?;

    let (configuration, database_connection, active_profile_id, encryption_key) =
        if let Some(ref _config_path) = config_path {
            // 兼容旧模式：使用 application.yml
            let configuration =
                match config::Configuration::new(handle, config_path.as_deref()) {
                    Ok(config) => config,
                    Err(e) => {
                        error!("配置加载失败: {:#}", e);
                        let error_msg = format!("{}", e);
                        handle
                            .dialog()
                            .message(t("config.load.failed.message", &[&error_msg]))
                            .kind(MessageDialogKind::Error)
                            .title(t_simple("config.load.failed.title"))
                            .blocking_show();
                        std::process::exit(1);
                    }
                };

            let database_connection = match configuration.database_connection().await {
                Ok(conn) => conn,
                Err(e) => {
                    error!("数据库连接失败: {:#}", e);
                    let error_msg = format!("{}", e);
                    handle
                        .dialog()
                        .message(t("database.connection.failed.message", &[&error_msg]))
                        .kind(MessageDialogKind::Error)
                        .title(t_simple("database.connection.failed.title"))
                        .blocking_show();
                    std::process::exit(1);
                }
            };

            (configuration, database_connection, String::new(), None)
        } else {
            // Profile 模式
            let index = service::profile::read_index(&app_data_dir)?;
            let profile_id = if index.active.is_empty() {
                // 没有活跃 profile，使用第一个
                let profiles = service::profile::list_profiles(&app_data_dir)?;
                if profiles.is_empty() {
                    // 不应该到这里，但以防万一进入向导模式
                    setup_wizard_mode_state(app).await?;
                    return Ok(());
                }
                profiles[0].id.clone()
            } else {
                index.active.clone()
            };

            let profile_config = service::profile::read_profile(&app_data_dir, &profile_id)?;

            // 从 Keychain 获取数据库密码（MySQL/PG）
            let db_password = if profile_config.datasource.db_type != "sqlite"
                && profile_config.datasource.auth_method != "certificate"
            {
                service::keychain::get_db_password(&profile_id)?
            } else {
                None
            };

            let database_connection = match config::database_connection_from_profile(
                &profile_config,
                db_password.as_deref(),
            )
            .await
            {
                Ok(conn) => conn,
                Err(e) => {
                    error!("数据库连接失败: {:#}", e);
                    let error_msg = format!("{}", e);
                    handle
                        .dialog()
                        .message(t("database.connection.failed.message", &[&error_msg]))
                        .kind(MessageDialogKind::Error)
                        .title(t_simple("database.connection.failed.title"))
                        .blocking_show();
                    // 进入向导模式让用户重新配置
                    setup_wizard_mode_state(app).await?;
                    return Ok(());
                }
            };

            // 从 Keychain 获取加密密钥
            let encryption_key = if profile_config.security.content_encryption {
                service::keychain::get_encryption_key(&profile_id)?
            } else {
                None
            };

            let configuration = config::Configuration::default();
            (configuration, database_connection, profile_id, encryption_key)
        };

    // 运行数据库迁移
    if let Err(e) = migration::Migrator::up(&database_connection, None).await {
        error!("数据库迁移失败: {:#}", e);
        let error_msg = format!("{}", e);
        handle
            .dialog()
            .message(t("database.migration.failed.message", &[&error_msg]))
            .kind(MessageDialogKind::Error)
            .title(t_simple("database.migration.failed.title"))
            .blocking_show();
        std::process::exit(1);
    }
    info!("数据库迁移完成");

    let app_state = Arc::new(AppState {
        configuration,
        database_connection,
        app_data_dir,
        active_profile_id,
        encryption_key,
        settings_cache: tokio::sync::RwLock::new(None),
    });

    app.manage(app_state);

    // 设置系统托盘
    setup_tray(app)?;

    Ok(())
}

/// 设置向导模式：不连接数据库，仅提供 setup 命令
async fn setup_wizard_mode(
    app: &mut tauri::App,
) -> Result<(), Box<dyn std::error::Error>> {
    setup_wizard_mode_state(app).await
}

/// 创建向导模式的 AppState（无数据库连接）
async fn setup_wizard_mode_state(
    app: &mut tauri::App,
) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let app_data_dir = handle.path().app_data_dir()?;

    // 创建一个临时的内存 SQLite 连接（仅用于满足类型要求）
    let db = sea_orm::Database::connect("sqlite::memory:")
        .await
        .expect("无法创建临时数据库连接");

    let app_state = Arc::new(AppState {
        configuration: config::Configuration::default(),
        database_connection: db,
        app_data_dir,
        active_profile_id: String::new(),
        encryption_key: None,
        settings_cache: tokio::sync::RwLock::new(None),
    });

    app.manage(app_state);

    // 设置系统托盘
    setup_tray(app)?;

    Ok(())
}

/// 设置系统托盘
fn setup_tray(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItemBuilder::with_id("show", "显示窗口").build(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let tray_menu = MenuBuilder::new(app)
        .item(&show_item)
        .item(&separator)
        .item(&quit_item)
        .build()?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().cloned().unwrap())
        .tooltip("ENote")
        .menu(&tray_menu)
        .on_menu_event(|app_handle, event| {
            match event.id().as_ref() {
                "show" => {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app_handle = tray.app_handle();
                if let Some(window) = app_handle.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
