//! ENote MCP Server
//!
//! 独立的 MCP Server 进程，通过 stdio 与 AI 工具通信。
//! 数据层复用公共 store 工厂，设置层与桌面端共用同一份文件/远程存储，
//! 因此 Database / Git / Server 三种 profile 均可直接接入。

mod mcp;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use clap::Parser;
use rmcp::{ServiceExt, transport::stdio};
use sea_orm_migration::MigratorTrait;
use tracing_subscriber::EnvFilter;

use enote_lib::config::ProfileBackend;
use enote_lib::service;
use enote_lib::store::DatabaseStore;

use mcp::ENoteMcpServer;

/// ENote MCP Server - 为 AI 工具提供笔记操作能力
#[derive(Parser, Debug)]
#[command(name = "enote-mcp", version, about)]
struct Args {
    /// 使用的 Profile ID（默认取 profiles.yml 中的 active）
    #[arg(long, conflicts_with = "config")]
    profile: Option<String>,

    /// 旧模式：直接指定 application.yml（仅数据库）
    #[arg(short, long)]
    config: Option<String>,
}

/// ENote 应用数据目录（与 Tauri bundle identifier 一致）
fn app_data_dir() -> Result<PathBuf> {
    enote_lib::util::data_dir()
        .map(|dir| dir.join("net.easycloudcn.enote"))
        .context("无法解析应用数据目录")
}

/// 按后端解析 MCP 设置，与桌面端 `get_all_settings` 的读取顺序保持一致：
/// - Server：远程 API
/// - Git：本地 `{app_data_dir}/settings/{profile_id}.yml`
/// - Database：优先本地文件，文件为空时从旧的 settings 表迁移
///
/// MCP 不再直接读数据库 settings 表作为运行时数据源——那套路径在 Git/Server
/// profile 下从来不存在，是早期实现的历史遗留问题。
async fn resolve_settings(
    app_data_dir: &Path,
    profile_id: &str,
    backend: &ProfileBackend,
    db: Option<&sea_orm::DatabaseConnection>,
) -> Result<HashMap<String, String>> {
    match backend {
        ProfileBackend::Server(client) => Ok(client.get_all_settings().await?),
        ProfileBackend::Git => Ok(service::settings_file::get_all(app_data_dir, profile_id)?),
        ProfileBackend::Database => {
            let file_settings = service::settings_file::get_all(app_data_dir, profile_id)?;
            if !file_settings.is_empty() {
                return Ok(file_settings);
            }
            match db {
                Some(db) => Ok(service::settings_file::migrate_from_db(
                    app_data_dir,
                    profile_id,
                    db,
                )
                .await?),
                None => Ok(file_settings),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 日志输出到 stderr（stdout 留给 MCP stdio 协议）
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let args = Args::parse();
    tracing::info!("Starting ENote MCP Server");

    let (store, settings, profile_label) = if let Some(ref config_path) = args.config {
        // ======== 旧模式：application.yml 直连数据库 ========
        let configuration = enote_lib::config::Configuration::from_file(config_path)?;
        if !configuration.is_mcp_enabled() {
            anyhow::bail!(
                "MCP Server is disabled in configuration. Set mcp.enabled to true in application.yml to enable it."
            );
        }
        let db = configuration.database_connection().await?;
        enote_lib::migration::Migrator::up(&db, None).await?;

        let store: Arc<dyn enote_lib::store::AccessStore> =
            Arc::new(DatabaseStore::new(db.clone(), None, app_data_dir()?));
        // 旧模式没有 profile 概念，沿用数据库 settings 表
        let settings = service::settings::get_all(&db).await?;
        (store, settings, "legacy".to_string())
    } else {
        // ======== Profile 模式：走公共 store 工厂 ========
        let dir = app_data_dir()?;
        let index = service::profile::read_index(&dir)?;
        let profile_id = args
            .profile
            .clone()
            .or_else(|| {
                if index.active.is_empty() {
                    None
                } else {
                    Some(index.active.clone())
                }
            })
            .context("未指定 profile，且 profiles.yml 中没有 active profile；请使用 --profile")?;

        let profile_config = service::profile::read_profile(&dir, &profile_id)?;
        tracing::info!("Building stores for profile {}", profile_id);
        let bundle = enote_lib::store::factory::build_profile_stores(
            &dir,
            &profile_id,
            &profile_config,
            None, // 独立进程无 Tauri AppHandle
            enote_lib::store::factory::CredentialPolicy::Skip,
        )
        .await?;

        // 数据库后端由桌面端启动时迁移；此处兜底，保证脱离桌面端也能启动
        if let Some(db) = &bundle.database_connection {
            enote_lib::migration::Migrator::up(db, None).await?;
        }

        tracing::info!("Resolving settings for profile {}", profile_id);
        let settings = resolve_settings(
            &dir,
            &profile_id,
            &bundle.backend,
            bundle.database_connection.as_ref(),
        )
        .await?;
        if settings.get("mcpEnabled").map(String::as_str) != Some("1") {
            anyhow::bail!(
                "MCP 未启用：请在 ENote 设置面板中开启「启用 MCP」（profile: {}）",
                profile_id
            );
        }

        tracing::info!(
            "Profile ready: {} ({})",
            profile_id,
            backend_name(&bundle.backend)
        );
        (bundle.access_store, settings, profile_id)
    };

    tracing::info!("Store ready (profile: {})", profile_label);

    // 启动 MCP Server
    let server = ENoteMcpServer::new(store, settings);
    let service = server.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("MCP serve error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}

fn backend_name(backend: &ProfileBackend) -> &'static str {
    match backend {
        ProfileBackend::Database => "Database",
        ProfileBackend::Git => "Git",
        ProfileBackend::Server(_) => "Server",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn temp_dir(name: &str) -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "enote-mcp-settings-test-{}-{}-{n}",
            std::process::id(),
            name
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[tokio::test]
    async fn git_profile_reads_settings_file() {
        let dir = temp_dir("git");
        let mut expected = HashMap::new();
        expected.insert("mcpEnabled".to_string(), "1".to_string());
        expected.insert(
            "mcpEnabledTools".to_string(),
            "search_notes,get_note".to_string(),
        );
        service::settings_file::save(&dir, "git-notes", &expected).unwrap();

        let actual = resolve_settings(&dir, "git-notes", &ProfileBackend::Git, None)
            .await
            .unwrap();
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn missing_settings_file_returns_empty() {
        let dir = temp_dir("missing");
        let actual = resolve_settings(&dir, "nonexistent", &ProfileBackend::Git, None)
            .await
            .unwrap();
        assert!(actual.is_empty());
    }
}
