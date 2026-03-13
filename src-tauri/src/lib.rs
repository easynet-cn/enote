//! ENote 应用程序入口模块
//!
//! 本模块负责初始化 Tauri 应用程序，包括：
//! - 加载配置文件（自动创建默认配置）
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
mod config; // 配置管理
mod entity; // SeaORM 数据库实体
mod error; // 错误处理
mod i18n; // 国际化支持（必须在 config 之前声明）
mod migration; // 数据库迁移
mod model; // 数据传输对象（DTO）
mod service; // 业务逻辑服务层

/// 应用程序入口函数
///
/// 该函数执行以下操作：
/// 1. 初始化日志系统
/// 2. 初始化 Tauri 插件（文件系统、打开器、Shell）
/// 3. 加载应用配置（自动创建默认配置文件）
/// 4. 建立数据库连接
/// 5. 运行数据库迁移（自动创建表结构）
/// 6. 将应用状态注入到 Tauri 管理器
/// 7. 注册所有前端可调用的命令
/// 8. 启动应用程序主循环
///
/// # 参数
/// - `config_path`: 可选的配置文件路径，如果为 None 则使用默认路径
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(config_path: Option<String>) {
    // 初始化 tracing 日志系统
    // 默认日志级别为 info，可通过 RUST_LOG 环境变量覆盖
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

                // 加载应用配置
                let configuration = match config::Configuration::new(handle, config_path.as_deref()) {
                    Ok(config) => config,
                    Err(e) => {
                        error!("配置加载失败: {:#}", e);
                        // 显示友好的错误提示对话框（根据系统语言国际化）
                        let error_msg = format!("{}", e);
                        handle
                            .dialog()
                            .message(t("config.load.failed.message", &[&error_msg]))
                            .kind(MessageDialogKind::Error)
                            .title(t_simple("config.load.failed.title"))
                            .blocking_show();
                        // 退出应用
                        std::process::exit(1);
                    }
                };

                // 建立数据库连接
                let database_connection = match configuration.database_connection().await {
                    Ok(conn) => conn,
                    Err(e) => {
                        error!("数据库连接失败: {:#}", e);
                        // 显示友好的错误提示对话框（根据系统语言国际化）
                        let error_msg = format!("{}", e);
                        handle
                            .dialog()
                            .message(t("database.connection.failed.message", &[&error_msg]))
                            .kind(MessageDialogKind::Error)
                            .title(t_simple("database.connection.failed.title"))
                            .blocking_show();
                        // 退出应用
                        std::process::exit(1);
                    }
                };

                // 运行数据库迁移，自动创建表结构
                if let Err(e) = migration::Migrator::up(&database_connection, None).await {
                    error!("数据库迁移失败: {:#}", e);
                    // 显示友好的错误提示对话框（根据系统语言国际化）
                    let error_msg = format!("{}", e);
                    handle
                        .dialog()
                        .message(t("database.migration.failed.message", &[&error_msg]))
                        .kind(MessageDialogKind::Error)
                        .title(t_simple("database.migration.failed.title"))
                        .blocking_show();
                    // 退出应用
                    std::process::exit(1);
                }
                info!("数据库迁移完成");

                // 获取应用数据目录
                let app_data_dir = handle
                    .path()
                    .app_data_dir()
                    .expect("无法获取应用数据目录");

                // 创建应用状态并注入到 Tauri 管理器
                let app_state = Arc::new(AppState {
                    configuration,
                    database_connection,
                    app_data_dir,
                });

                app.manage(app_state);

                // 设置系统托盘
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
            })
        })
        // 注册前端可调用的命令处理器
        .invoke_handler(tauri::generate_handler![
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
