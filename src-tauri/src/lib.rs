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

use std::sync::{Arc, OnceLock};

use sea_orm_migration::MigratorTrait;
use tauri::{Emitter, Manager};
#[cfg(feature = "desktop")]
use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder};
#[cfg(feature = "desktop")]
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::Layer;

use crate::config::AppState;
use crate::i18n::{t, t_simple};

/// 静态存储 tracing guards，避免 Box::leak 内存泄漏
static TRACING_GUARDS: OnceLock<(
    tracing_appender::non_blocking::WorkerGuard,
    tracing_appender::non_blocking::WorkerGuard,
)> = OnceLock::new();

// 模块声明
mod command; // Tauri 命令处理器
pub mod config; // 配置管理
pub mod entity; // SeaORM 数据库实体
mod error; // 错误处理
pub mod i18n; // 国际化支持（必须在 config 之前声明）
pub mod migration; // 数据库迁移
pub mod model; // 数据传输对象（DTO）
pub mod service; // 业务逻辑服务层

/// 移动端入口（无参数）
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    run_with_config(None);
}

/// 桌面端入口（支持配置文件路径参数）
pub fn run_with_config(config_path: Option<String>) {
    // 初始化 tracing 日志系统（三层：stdout + 全量文件 + error 单独文件）
    let log_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("net.easycloudcn.enote")
        .join("logs");
    let _ = std::fs::create_dir_all(&log_dir);

    // 全量日志文件（按天 rolling）
    let all_appender = tracing_appender::rolling::daily(&log_dir, "enote.log");
    let (all_writer, _all_guard) = tracing_appender::non_blocking(all_appender);

    // Error 日志单独文件（按天 rolling）
    let error_appender = tracing_appender::rolling::daily(&log_dir, "enote-error.log");
    let (error_writer, _error_guard) = tracing_appender::non_blocking(error_appender);

    // 将 guard 存入静态变量以确保生命周期（优于 Box::leak，程序退出时会正常 flush）
    let _ = TRACING_GUARDS.set((_all_guard, _error_guard));

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        // stdout 输出（开发调试用）
        .with(fmt::layer())
        // 全量日志文件
        .with(
            fmt::layer()
                .with_writer(all_writer)
                .with_ansi(false),
        )
        // Error 日志单独文件（仅 ERROR 级别）
        .with(
            fmt::layer()
                .with_writer(error_writer)
                .with_ansi(false)
                .with_filter(tracing_subscriber::filter::LevelFilter::ERROR),
        )
        .init();

    tauri::Builder::default()
        // 注册 Tauri 插件
        .plugin(tauri_plugin_fs::init()) // 文件系统访问
        .plugin(tauri_plugin_opener::init()) // 打开外部链接/文件
        .plugin(tauri_plugin_shell::init()) // Shell 命令执行
        .plugin(tauri_plugin_dialog::init()) // 文件对话框
        .plugin(tauri_plugin_updater::Builder::new().build()) // 自动更新
        .plugin(tauri_plugin_process::init()) // 进程管理（重启）
        // 应用初始化设置
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let handle = app.handle();

                // 获取应用数据目录
                let app_data_dir = handle.path().app_data_dir()
                    .map_err(|e| {
                        error!("Failed to get app data directory: {:#}", e);
                        e
                    })?;

                // 确保目录存在
                if !app_data_dir.exists() {
                    std::fs::create_dir_all(&app_data_dir)
                        .map_err(|e| {
                            error!("Failed to create app data directory: {:#}", e);
                            e
                        })?;
                }

                // 决定启动模式
                if config_path.is_some() || !service::profile::needs_setup(&app_data_dir) {
                    // 正常模式：有配置，连接数据库
                    setup_normal_mode(app, config_path).await?;
                } else if cfg!(target_os = "android") || cfg!(target_os = "ios") {
                    // 移动端：自动创建默认 SQLite profile，跳过向导
                    info!("Mobile first launch, auto-creating default SQLite profile");
                    auto_create_mobile_profile(&app_data_dir)?;
                    setup_normal_mode(app, None).await?;
                } else {
                    // 桌面端设置向导模式：没有任何 profile
                    info!("No profile found, entering setup wizard mode");
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
            command::test_server_connection,
            command::generate_encryption_key,
            command::get_profile_index,
            command::set_auto_connect,
            command::restart_with_profile,
            command::reconnect_profile,
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
            // 批量操作相关命令
            command::batch_move_notes,
            command::batch_delete_notes,
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
            // 笔记收藏/星标
            command::toggle_note_star,
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
            // 云备份相关命令
            command::test_cloud_connection,
            command::save_cloud_backup_config,
            command::upload_backup_to_cloud,
            command::cloud_backup_now,
            command::list_cloud_backups,
            command::download_cloud_backup,
            command::cleanup_cloud_backups,
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
            // 跨 Profile 同步相关命令
            command::get_sync_preview,
            command::start_sync,
            command::find_sync_logs,
            command::find_sync_log_details,
            command::delete_sync_log,
            command::clear_sync_logs,
            command::export_sync_log,
            // 帮助手册
            command::read_help_manual,
            // 应用日志相关命令
            command::search_page_app_logs,
            command::delete_app_log,
            command::delete_batch_app_logs,
            command::clear_app_logs,
            command::cleanup_app_logs_before,
            command::write_frontend_log,
            command::list_log_files,
            command::read_log_file,
            command::delete_log_files,
            command::cleanup_old_log_files,
            // 附件相关命令
            command::save_attachment,
            command::find_attachments,
            command::delete_attachment,
            command::open_attachment,
            command::get_attachment_stats,
            command::cleanup_orphan_attachments,
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            error!("Application start failed: {:#}", e);
            eprintln!("{}: {}", t_simple("error.appStartFailed"), e);
            std::process::exit(1);
        });
}

/// 移动端首次启动时自动创建默认 SQLite profile
fn auto_create_mobile_profile(
    app_data_dir: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    use service::profile::{
        DatasourceConfig, ProfileConfig, ProfileIndex, SecurityConfig,
    };

    let db_path = app_data_dir.join("enote.db");
    let db_path_str = db_path.to_string_lossy().to_string();

    let config = ProfileConfig {
        name: "Local".to_string(),
        icon: "database".to_string(),
        backend: "database".to_string(),
        datasource: DatasourceConfig {
            db_type: "sqlite".to_string(),
            path: db_path_str,
            ..Default::default()
        },
        server: None,
        security: SecurityConfig::default(),
    };

    let profile_id = "local-sqlite";
    service::profile::save_profile(app_data_dir, profile_id, &config)?;
    service::profile::save_index(
        app_data_dir,
        &ProfileIndex {
            active: profile_id.to_string(),
            auto_connect: true,
        },
    )?;

    info!("Mobile default profile created: {}", db_path.display());
    Ok(())
}

/// 正常模式启动：连接数据库并初始化完整应用状态
async fn setup_normal_mode(
    app: &mut tauri::App,
    config_path: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let app_data_dir = handle.path().app_data_dir()?;

    let (configuration, database_connection, active_profile_id, encryption_key): (
        config::Configuration,
        Option<sea_orm::DatabaseConnection>,
        String,
        Option<String>,
    ) = if let Some(ref _config_path) = config_path {
            // 兼容旧模式：使用 application.yml
            let configuration = match config::Configuration::new(handle, config_path.as_deref()) {
                Ok(config) => config,
                Err(e) => {
                    error!("Config load failed: {:#}", e);
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
                    error!("Database connection failed: {:#}", e);
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

            (configuration, Some(database_connection), String::new(), None)
        } else {
            // Profile 模式
            let index = service::profile::read_index(&app_data_dir)?;
            let profiles = service::profile::list_profiles(&app_data_dir)?;
            info!(
                "Profile mode: auto_connect={}, profiles_count={}",
                index.auto_connect,
                profiles.len()
            );

            // 多 Profile + 非自动连接 → 延迟连接，等待用户在前端选择
            if !index.auto_connect && profiles.len() > 1 {
                info!("Multiple profiles without auto-connect, waiting for user selection");
                (
                    config::Configuration::default(),
                    None,
                    String::new(),
                    None,
                )
            } else {
                let profile_id = if index.active.is_empty() {
                    // 没有活跃 profile，使用第一个
                    if profiles.is_empty() {
                        // 不应该到这里，但以防万一进入向导模式
                        setup_wizard_mode_state(app).await?;
                        return Ok(());
                    }
                    profiles[0].id.clone()
                } else {
                    index.active.clone()
                };

                let profile_config =
                    service::profile::read_profile(&app_data_dir, &profile_id)?;

                // 从 Keychain 获取数据库密码（桌面端/db-full 模式 MySQL/PG 需要）
                #[cfg(any(feature = "desktop", feature = "db-full"))]
                let db_password = if profile_config.datasource.db_type != "sqlite"
                    && profile_config.datasource.auth_method != "certificate"
                {
                    service::keychain::get_db_password(&profile_id)?
                } else {
                    None
                };
                #[cfg(not(any(feature = "desktop", feature = "db-full")))]
                let db_password: Option<String> = None;

                let database_connection = match config::database_connection_from_profile(
                    &profile_config,
                    db_password.as_deref(),
                )
                .await
                {
                    Ok(conn) => conn,
                    Err(e) => {
                        error!("Database connection failed: {:#}", e);
                        let error_msg = format!("{}", e);
                        handle
                            .dialog()
                            .message(t(
                                "database.connection.failed.message",
                                &[&error_msg],
                            ))
                            .kind(MessageDialogKind::Error)
                            .title(t_simple("database.connection.failed.title"))
                            .blocking_show();
                        // 进入向导模式让用户重新配置
                        setup_wizard_mode_state(app).await?;
                        return Ok(());
                    }
                };

                // 从 Keychain 获取加密密钥（桌面端/db-full 模式）
                #[cfg(any(feature = "desktop", feature = "db-full"))]
                let encryption_key = if profile_config.security.content_encryption {
                    service::keychain::get_encryption_key(&profile_id)?
                } else {
                    None
                };
                #[cfg(not(any(feature = "desktop", feature = "db-full")))]
                let encryption_key: Option<String> = None;

                let configuration = config::Configuration::default();
                (
                    configuration,
                    Some(database_connection),
                    profile_id,
                    encryption_key,
                )
            }
        };

    // 运行数据库迁移（仅在有数据库连接时）
    if let Some(ref db) = database_connection {
        if let Err(e) = migration::Migrator::up(db, None).await {
            error!("Database migration failed: {:#}", e);
            let error_msg = format!("{}", e);
            handle
                .dialog()
                .message(t("database.migration.failed.message", &[&error_msg]))
                .kind(MessageDialogKind::Error)
                .title(t_simple("database.migration.failed.title"))
                .blocking_show();
            std::process::exit(1);
        }
        info!("Database migration completed");
    }

    let app_state = Arc::new(AppState {
        configuration,
        database_connection: tokio::sync::RwLock::new(database_connection),
        backend: tokio::sync::RwLock::new(config::ProfileBackend::Database),
        app_data_dir,
        active_profile_id: tokio::sync::RwLock::new(active_profile_id),
        encryption_key: tokio::sync::RwLock::new(encryption_key),
        settings_cache: tokio::sync::RwLock::new(None),
    });

    app.manage(app_state);

    // 设置应用菜单和系统托盘（仅桌面端）
    #[cfg(feature = "desktop")]
    {
        setup_app_menu(app)?;
        setup_tray(app)?;
    }

    Ok(())
}

/// 设置向导模式：不连接数据库，仅提供 setup 命令
async fn setup_wizard_mode(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    setup_wizard_mode_state(app).await
}

/// 创建向导/选择模式的 AppState（无数据库连接）
async fn setup_wizard_mode_state(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let app_data_dir = handle.path().app_data_dir()?;

    let app_state = Arc::new(AppState {
        configuration: config::Configuration::default(),
        database_connection: tokio::sync::RwLock::new(None),
        backend: tokio::sync::RwLock::new(config::ProfileBackend::Database),
        app_data_dir,
        active_profile_id: tokio::sync::RwLock::new(String::new()),
        encryption_key: tokio::sync::RwLock::new(None),
        settings_cache: tokio::sync::RwLock::new(None),
    });

    app.manage(app_state);

    // 设置应用菜单和系统托盘（仅桌面端）
    #[cfg(feature = "desktop")]
    {
        setup_app_menu(app)?;
        setup_tray(app)?;
    }

    Ok(())
}

/// 设置应用菜单栏（仅桌面端编译）
#[cfg(feature = "desktop")]
fn setup_app_menu(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let help_item = MenuItemBuilder::with_id("help-manual", t_simple("help.menuItem"))
        .build(app)?;
    let check_update_item = MenuItemBuilder::with_id("check-update", t_simple("help.checkUpdate"))
        .build(app)?;

    let help_menu = SubmenuBuilder::new(app, t_simple("help.menuTitle"))
        .item(&help_item)
        .separator()
        .item(&check_update_item)
        .build()?;

    // macOS 需要应用子菜单（包含隐藏、退出等）
    #[cfg(target_os = "macos")]
    let app_submenu = SubmenuBuilder::new(app, app.package_info().name.clone())
        .about(None)
        .separator()
        .hide()
        .hide_others()
        .show_all()
        .separator()
        .quit()
        .build()?;

    // 编辑菜单（包含复制、粘贴、撤销等系统快捷键）
    let edit_menu = SubmenuBuilder::new(app, "Edit")
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .select_all()
        .build()?;

    let mut menu_builder = MenuBuilder::new(app);

    #[cfg(target_os = "macos")]
    {
        menu_builder = menu_builder.item(&app_submenu);
    }

    let app_menu = menu_builder
        .item(&edit_menu)
        .item(&help_menu)
        .build()?;

    app.set_menu(app_menu)?;

    let handle = app.handle().clone();
    app.on_menu_event(move |_app, event| {
        match event.id().as_ref() {
            "help-manual" => {
                if let Some(window) = handle.get_webview_window("main") {
                    let _ = window.emit("menu-help-manual", ());
                }
            }
            "check-update" => {
                if let Some(window) = handle.get_webview_window("main") {
                    let _ = window.emit("menu-check-update", ());
                }
            }
            _ => {}
        }
    });

    Ok(())
}

/// 设置系统托盘（仅桌面端编译）
#[cfg(feature = "desktop")]
fn setup_tray(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItemBuilder::with_id("show", i18n::t_simple("tray.show")).build(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", i18n::t_simple("tray.quit")).build(app)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let tray_menu = MenuBuilder::new(app)
        .item(&show_item)
        .item(&separator)
        .item(&quit_item)
        .build()?;

    let tray_icon = app.default_window_icon().cloned()
        .ok_or_else(|| {
            error!("Default window icon not found for tray");
            Box::<dyn std::error::Error>::from("Default window icon not found")
        })?;

    TrayIconBuilder::new()
        .icon(tray_icon)
        .tooltip("ENote")
        .menu(&tray_menu)
        .on_menu_event(|app_handle, event| match event.id().as_ref() {
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
