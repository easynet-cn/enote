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
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tracing::{error, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::AppState;

// 模块声明
mod command; // Tauri 命令处理器
mod config; // 配置管理
mod entity; // SeaORM 数据库实体
mod error; // 错误处理
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
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
                let configuration = match config::Configuration::new(handle) {
                    Ok(config) => config,
                    Err(e) => {
                        error!("配置加载失败: {:#}", e);
                        return Err(e.to_string().into());
                    }
                };

                // 建立数据库连接
                let database_connection = match configuration.database_connection().await {
                    Ok(conn) => conn,
                    Err(e) => {
                        error!("数据库连接失败: {:#}", e);
                        // 显示友好的错误提示对话框
                        handle
                            .dialog()
                            .message(format!(
                                "无法连接到数据库，请检查数据库服务是否已启动。\n\n错误详情：{}",
                                e
                            ))
                            .kind(MessageDialogKind::Error)
                            .title("数据库连接失败")
                            .blocking_show();
                        // 退出应用
                        std::process::exit(1);
                    }
                };

                // 运行数据库迁移，自动创建表结构
                if let Err(e) = migration::Migrator::up(&database_connection, None).await {
                    error!("数据库迁移失败: {:#}", e);
                    // 显示友好的错误提示对话框
                    handle
                        .dialog()
                        .message(format!(
                            "数据库迁移失败，请检查数据库配置。\n\n错误详情：{}",
                            e
                        ))
                        .kind(MessageDialogKind::Error)
                        .title("数据库迁移失败")
                        .blocking_show();
                    // 退出应用
                    std::process::exit(1);
                }
                info!("数据库迁移完成");

                // 创建应用状态并注入到 Tauri 管理器
                let app_state = Arc::new(AppState {
                    configuration,
                    database_connection,
                });

                app.manage(app_state);

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
        ])
        .run(tauri::generate_context!())
        .expect("启动应用失败");
}
