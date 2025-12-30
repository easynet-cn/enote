//! 配置管理模块
//!
//! 本模块负责：
//! - 加载和解析 YAML 配置文件
//! - 自动创建默认配置文件（如不存在）
//! - 管理数据库连接池配置
//! - 提供应用全局状态

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use config::Config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tauri::{AppHandle, Manager};
use tracing::info;

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
    ///
    /// # 配置文件位置
    /// 配置文件位于应用数据目录下的 `application.yml`
    ///
    /// # 自动创建
    /// 如果配置文件不存在，会自动创建包含 SQLite 数据库配置的默认文件
    pub fn new(app_handle: &AppHandle) -> Result<Self> {
        // 获取应用数据目录
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .context("无法获取应用数据目录")?;

        // 确保目录存在
        if !app_data_dir.exists() {
            fs::create_dir_all(&app_data_dir).context("无法创建应用数据目录")?;
        }

        // 配置文件路径
        let config_file_path = app_data_dir.join("application.yml");

        // 如果配置文件不存在，创建默认配置
        if !config_file_path.exists() {
            Self::create_default_config(&app_data_dir, &config_file_path)?;
        }

        let config_path = config_file_path
            .to_str()
            .context("配置文件路径包含无效字符")?;

        // 加载并解析配置文件
        let config = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()
            .context("无法加载配置文件，请确保 application.yml 存在且格式正确")?;

        Ok(Self { config })
    }

    /// 创建默认配置文件
    ///
    /// # 参数
    /// - `app_data_dir`: 应用数据目录
    /// - `config_file_path`: 配置文件完整路径
    ///
    /// # 返回
    /// - `Ok(())`: 创建成功
    /// - `Err`: 创建失败
    ///
    /// # 默认配置
    /// - 使用 SQLite 数据库
    /// - 数据库文件位于应用数据目录下的 `enote.db`
    fn create_default_config(app_data_dir: &PathBuf, config_file_path: &PathBuf) -> Result<()> {
        // SQLite 数据库文件路径
        let db_path = app_data_dir.join("enote.db");
        let db_url = format!(
            "sqlite:{}?mode=rwc",
            db_path.to_str().context("数据库路径包含无效字符")?
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
  # 最大连接数：适应并发操作
  max-connections: 20
  # 最小连接数：保持常驻连接减少重连开销
  min-connections: 2
  # 连接超时（秒）：建立新连接的超时时间
  connect_timeout: 10
  # 获取连接超时（秒）：从池中获取连接的超时时间
  acquire-timeout: 5
  # 空闲超时（秒）：空闲连接保持时间
  idle-time: 300
  # 最大生命周期（秒）：连接最长存活时间
  max-lifetime: 1800
"#,
            db_url
        );

        // 写入配置文件
        fs::write(config_file_path, default_config).context("无法写入配置文件")?;

        info!("已创建默认配置文件: {:?}", config_file_path);
        info!("数据库文件: {:?}", db_path);

        Ok(())
    }

    /// 创建数据库连接
    ///
    /// 根据配置文件中的数据库设置创建连接池
    ///
    /// # 配置项
    /// - `datasource.url`: 数据库连接 URL（必需）
    /// - `datasource.max-connections`: 最大连接数（默认: 100）
    /// - `datasource.min-connections`: 最小连接数（默认: 1）
    /// - `datasource.connect_timeout`: 连接超时时间，秒（默认: 30）
    /// - `datasource.acquire-timeout`: 获取连接超时时间，秒（默认: 8）
    /// - `datasource.idle-time`: 空闲连接超时时间，秒（默认: 10）
    /// - `datasource.max-lifetime`: 连接最大生命周期，秒（默认: 30）
    ///
    /// # 返回
    /// - `Ok(DatabaseConnection)`: 数据库连接创建成功
    /// - `Err`: 连接失败（配置错误、网络问题等）
    pub async fn database_connection(&self) -> Result<DatabaseConnection> {
        // 获取数据库连接 URL（必需配置项）
        let url = self
            .config
            .get_string("datasource.url")
            .context("配置文件中缺少 datasource.url")?;

        // 读取连接池配置，使用优化的默认值作为回退
        let max_connections = self
            .config
            .get_int("datasource.max-connections")
            .unwrap_or(20) as u32;
        let min_connections = self
            .config
            .get_int("datasource.min-connections")
            .unwrap_or(2) as u32;
        let connect_timeout = self
            .config
            .get_int("datasource.connect_timeout")
            .unwrap_or(10) as u64;
        let acquire_timeout = self
            .config
            .get_int("datasource.acquire-timeout")
            .unwrap_or(5) as u64;
        let idle_timeout = self
            .config
            .get_int("datasource.idle-time")
            .unwrap_or(300) as u64;
        let max_lifetime = self
            .config
            .get_int("datasource.max-lifetime")
            .unwrap_or(1800) as u64;

        // 配置数据库连接选项
        let mut opt = ConnectOptions::new(url);

        opt.max_connections(max_connections)
            .min_connections(min_connections)
            .connect_timeout(Duration::from_secs(connect_timeout))
            .acquire_timeout(Duration::from_secs(acquire_timeout))
            .idle_timeout(Duration::from_secs(idle_timeout))
            .max_lifetime(Duration::from_secs(max_lifetime));

        // 建立数据库连接
        Database::connect(opt)
            .await
            .context("无法连接数据库，请检查数据库配置和网络连接")
    }
}

/// 应用全局状态
///
/// 存储应用运行时需要的共享状态，通过 Tauri 状态管理器在各命令间共享
pub struct AppState {
    /// 应用配置
    pub configuration: Configuration,
    /// 数据库连接（连接池）
    pub database_connection: DatabaseConnection,
}
