//! ENote MCP Server
//!
//! 独立的 MCP Server 进程，通过 stdio 与 AI 工具通信，
//! 直接复用 enote_lib 的 service 层操作笔记数据库。

mod mcp;

use anyhow::Result;
use clap::Parser;
use rmcp::{ServiceExt, transport::stdio};
use sea_orm_migration::MigratorTrait;
use tracing_subscriber::EnvFilter;

use mcp::ENoteMcpServer;

/// ENote MCP Server - 为 AI 工具提供笔记操作能力
#[derive(Parser, Debug)]
#[command(name = "enote-mcp", version, about)]
struct Args {
    /// 配置文件路径（application.yml）
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 日志输出到 stderr（stdout 留给 MCP stdio 协议）
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let args = Args::parse();

    tracing::info!("Starting ENote MCP Server");

    // 加载配置并连接数据库
    let configuration = enote_lib::config::Configuration::from_file(&args.config)?;

    // 检查 MCP 是否启用
    if !configuration.is_mcp_enabled() {
        anyhow::bail!("MCP Server is disabled in configuration. Set mcp.enabled to true in application.yml to enable it.");
    }

    let db = configuration.database_connection().await?;

    // 运行迁移
    enote_lib::migration::Migrator::up(&db, None).await?;
    tracing::info!("Database ready");

    // 启动 MCP Server
    let server = ENoteMcpServer::new(db);
    let service = server.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("MCP serve error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}
