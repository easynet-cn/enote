//! 配置管理模块
//!
//! 本模块负责：
//! - 加载和解析 YAML 配置文件
//! - 支持 Profile 模式（多数据库配置）
//! - 管理数据库连接池配置
//! - 支持 SSL/TLS 证书认证
//! - 提供应用全局状态

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use config::Config;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Statement};
use tauri::{AppHandle, Manager};
use tracing::info;

use crate::i18n::t_simple;
use crate::service::profile::{self, ProfileConfig};

/// 应用配置结构体
///
/// 封装了从 YAML 配置文件加载的所有配置项
#[derive(Clone, Debug, Default)]
pub struct Configuration {
    /// 底层配置对象，用于读取具体配置值
    pub config: Config,
}

impl Configuration {
    /// 创建新的配置实例
    ///
    /// 如果配置文件不存在，会自动创建默认配置文件。
    ///
    /// # 参数
    /// - `app_handle`: Tauri 应用句柄，用于获取应用数据目录
    ///
    /// # 返回
    /// - `Ok(Configuration)`: 配置加载成功
    /// - `Err`: 配置加载失败（目录不存在、文件格式错误等）
    pub fn new(app_handle: &AppHandle, custom_config_path: Option<&str>) -> Result<Self> {
        // 获取应用数据目录
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .context(t_simple("config.get_app_data_dir.failed"))?;

        // 确保目录存在
        if !app_data_dir.exists() {
            fs::create_dir_all(&app_data_dir)
                .context(t_simple("config.create_app_data_dir.failed"))?;
        }

        // 确定配置文件路径：优先使用命令行指定的路径
        let config_file_path = if let Some(custom_path) = custom_config_path {
            let path = PathBuf::from(custom_path);
            // 支持相对路径：相对于当前工作目录解析
            if path.is_relative() {
                std::env::current_dir()
                    .context("Failed to get current directory")?
                    .join(path)
            } else {
                path
            }
        } else {
            let default_path = app_data_dir.join("application.yml");
            // 如果默认配置文件不存在，创建默认配置
            if !default_path.exists() {
                Self::create_default_config(&app_data_dir, &default_path)?;
            }
            default_path
        };

        info!("Using config file: {:?}", config_file_path);

        let config_path = config_file_path
            .to_str()
            .context(t_simple("config.invalid_config_path"))?;

        // 加载并解析配置文件
        let config = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()
            .context(t_simple("config.load_file.failed"))?;

        Ok(Self { config })
    }

    /// 从指定配置文件路径创建配置实例（无需 Tauri AppHandle）
    ///
    /// 用于 MCP Server 等独立进程
    pub fn from_file(config_path: &str) -> Result<Self> {
        let path = std::path::Path::new(config_path);
        let path = if path.is_relative() {
            std::env::current_dir()
                .context("Failed to get current directory")?
                .join(path)
        } else {
            path.to_path_buf()
        };

        let config_str = path.to_str().context("Config file path contains invalid characters")?;

        let config = Config::builder()
            .add_source(config::File::with_name(config_str))
            .build()
            .context("Failed to load configuration file")?;

        Ok(Self { config })
    }

    /// 检查 MCP Server 是否启用
    pub fn is_mcp_enabled(&self) -> bool {
        self.config.get_bool("mcp.enabled").unwrap_or(false)
    }

    /// 创建默认配置文件
    fn create_default_config(app_data_dir: &Path, config_file_path: &Path) -> Result<()> {
        // SQLite 数据库文件路径
        let db_path = app_data_dir.join("enote.db");
        let db_url = format!(
            "sqlite:{}?mode=rwc",
            db_path
                .to_str()
                .context(t_simple("config.invalid_db_path"))?
        );

        // 默认配置内容（优化的连接池参数）
        let default_config = format!(
            r#"# ENote 应用配置文件
# 此文件由应用自动生成

datasource:
  # SQLite 数据库连接 URL
  # mode=rwc 表示读写模式，如果文件不存在则创建
  url: "{}"

  # 连接池配置
  # 最大连接数：SQLite 推荐 5 以内
  max-connections: 5
  # 最小连接数：保持常驻连接减少重连开销
  min-connections: 1
  # 连接超时（秒）：建立新连接的超时时间
  connect_timeout: 10
  # 获取连接超时（秒）：从池中获取连接的超时时间
  acquire-timeout: 5
  # 空闲超时（秒）：空闲连接保持时间
  idle-time: 300
  # 最大生命周期（秒）：连接最长存活时间
  max-lifetime: 1800

# MCP Server 配置
# 启用后可通过 AI 工具（如 Claude Desktop）操作笔记
mcp:
  enabled: false
"#,
            db_url
        );

        // 写入配置文件
        fs::write(config_file_path, default_config)
            .context(t_simple("config.write_file.failed"))?;

        info!("Default config file created: {:?}", config_file_path);
        info!("Database file: {:?}", db_path);

        Ok(())
    }

    /// 创建数据库连接（兼容旧的 application.yml 方式）
    pub async fn database_connection(&self) -> Result<DatabaseConnection> {
        // 获取数据库连接 URL（必需配置项）
        let mut url = self
            .config
            .get_string("datasource.url")
            .context(t_simple("config.missing_datasource_url"))?;

        // 展开 ~ 为用户主目录
        if url.contains("~/")
            && let Some(home) = dirs::home_dir() {
                let home_str = home.to_string_lossy();
                url = url.replace("~", &home_str);
            }

        let is_sqlite = url.starts_with("sqlite:");

        // 读取连接池配置with clamp validation
        let default_max = if is_sqlite { 5 } else { 20 };
        let max_connections = (self
            .config
            .get_int("datasource.max-connections")
            .unwrap_or(default_max) as u32)
            .clamp(1, 1000);
        let min_connections = (self
            .config
            .get_int("datasource.min-connections")
            .unwrap_or(1) as u32)
            .clamp(0, max_connections);
        let connect_timeout = (self
            .config
            .get_int("datasource.connect_timeout")
            .unwrap_or(10) as u64)
            .clamp(1, 300);
        let acquire_timeout = (self
            .config
            .get_int("datasource.acquire-timeout")
            .unwrap_or(5) as u64)
            .clamp(1, 300);
        let idle_timeout =
            (self.config.get_int("datasource.idle-time").unwrap_or(300) as u64).clamp(1, 86400);
        let max_lifetime = (self
            .config
            .get_int("datasource.max-lifetime")
            .unwrap_or(1800) as u64)
            .clamp(1, 86400);

        let mut opt = ConnectOptions::new(url);

        opt.max_connections(max_connections)
            .min_connections(min_connections)
            .connect_timeout(Duration::from_secs(connect_timeout))
            .acquire_timeout(Duration::from_secs(acquire_timeout))
            .idle_timeout(Duration::from_secs(idle_timeout))
            .max_lifetime(Duration::from_secs(max_lifetime));

        let db = Database::connect(opt)
            .await
            .context(t_simple("config.database.connect.failed"))?;

        // Enable WAL mode and optimize SQLite for better performance
        if is_sqlite {
            Self::optimize_sqlite(&db).await?;
        }

        Ok(db)
    }
}

// ============================================================================
// Profile 模式的数据库连接
// ============================================================================

/// 从 Profile 配置创建数据库连接
pub async fn database_connection_from_profile(
    profile_config: &ProfileConfig,
    db_password: Option<&str>,
) -> Result<DatabaseConnection> {
    let url = profile::build_database_url(profile_config, db_password);

    let is_sqlite = url.starts_with("sqlite:");
    // SQLite 使用 WAL 模式允许并发读，提高到 10 连接
    // MySQL/PostgreSQL 默认 20 连接
    let default_max = if is_sqlite { 10 } else { 20 };

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(default_max)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800));

    // SSL 证书配置通过 URL 参数传递（无需直接依赖 sqlx 类型）
    // MySQL: ?ssl-mode=REQUIRED&ssl-ca=/path/to/ca.pem&ssl-cert=...&ssl-key=...
    // PostgreSQL: ?sslmode=require&sslrootcert=...&sslcert=...&sslkey=...

    let db = Database::connect(opt)
        .await
        .context(t_simple("config.database.connect.failed"))?;

    if is_sqlite {
        Configuration::optimize_sqlite(&db).await?;
    }

    Ok(db)
}

impl Configuration {
    /// 优化 SQLite 连接性能
    async fn optimize_sqlite(db: &DatabaseConnection) -> Result<()> {
        let backend = db.get_database_backend();
        for pragma in [
            "PRAGMA journal_mode=WAL",
            "PRAGMA synchronous=NORMAL",
            "PRAGMA journal_size_limit=67108864",
            // 内存映射 I/O（256MB），加速大数据库读取
            "PRAGMA mmap_size=268435456",
            // 页缓存大小（负值表示 KB，-16000 ≈ 16MB）
            "PRAGMA cache_size=-16000",
            // 临时表存储在内存中
            "PRAGMA temp_store=MEMORY",
            // 启用外键约束
            "PRAGMA foreign_keys=ON",
        ] {
            db.execute(Statement::from_string(backend, pragma.to_string()))
                .await
                .context(format!(
                    "{}: {}",
                    t_simple("config.database.pragma.failed"),
                    pragma
                ))?;
        }
        Ok(())
    }
}

/// 应用全局状态
///
/// 存储应用运行时需要的共享状态，通过 Tauri 状态管理器在各命令间共享
pub struct AppState {
    /// 应用配置（保留以备将来使用）
    #[allow(dead_code)]
    pub configuration: Configuration,
    /// 数据库连接（连接池），使用 RwLock 支持 Profile 热切换
    /// 在 Profile 选择模式下为 None，待用户选择后通过 reconnect_profile 填充
    pub database_connection: tokio::sync::RwLock<Option<DatabaseConnection>>,
    /// 应用数据目录（用于图片等文件存储）
    pub app_data_dir: PathBuf,
    /// 当前活跃的 profile ID，使用 RwLock 支持 Profile 热切换
    pub active_profile_id: tokio::sync::RwLock<String>,
    /// 内容加密密钥（启动时从 Keychain 加载，如果启用了加密），使用 RwLock 支持 Profile 热切换
    pub encryption_key: tokio::sync::RwLock<Option<String>>,
    /// 设置缓存（避免频繁查询数据库）
    pub settings_cache: tokio::sync::RwLock<Option<std::collections::HashMap<String, String>>>,
}
