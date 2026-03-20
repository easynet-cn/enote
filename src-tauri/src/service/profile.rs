//! Profile 配置管理服务模块
//!
//! 管理多个数据库配置 profile，支持：
//! - 列出所有 profile
//! - 创建/更新/删除 profile
//! - 读取/设置活跃 profile
//!
//! 存储结构：
//! ```text
//! app_data_dir/
//!   profiles/
//!     local-sqlite.yml
//!     work-mysql.yml
//!   profiles.yml          # 索引文件
//! ```

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing::info;

/// Profile 索引文件（profiles.yml）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileIndex {
    /// 当前活跃的 profile 名称
    #[serde(default)]
    pub active: String,
    /// 是否自动连接上次使用的 profile
    #[serde(default, rename = "auto-connect")]
    pub auto_connect: bool,
}

/// SSL 配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SslConfig {
    /// SSL 模式：disabled, preferred, required, verify-ca, verify-identity, verify-full
    #[serde(default)]
    pub mode: String,
    /// CA 证书路径
    #[serde(default, rename = "ca-cert", skip_serializing_if = "String::is_empty")]
    pub ca_cert: String,
    /// 客户端证书路径
    #[serde(
        default,
        rename = "client-cert",
        skip_serializing_if = "String::is_empty"
    )]
    pub client_cert: String,
    /// 客户端私钥路径
    #[serde(
        default,
        rename = "client-key",
        skip_serializing_if = "String::is_empty"
    )]
    pub client_key: String,
}

/// 数据源配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatasourceConfig {
    /// 数据库类型：sqlite, mysql, postgres
    #[serde(default, rename = "type")]
    pub db_type: String,

    // SQLite 专用
    /// 数据库文件路径
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,

    // MySQL / PostgreSQL 通用
    /// 主机地址
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
    /// 端口
    #[serde(default, skip_serializing_if = "is_zero")]
    pub port: u16,
    /// 数据库名
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub database: String,
    /// 用户名（密码存 Keychain）
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub username: String,
    /// 认证方式：password, certificate
    #[serde(
        default,
        rename = "auth-method",
        skip_serializing_if = "String::is_empty"
    )]
    pub auth_method: String,

    /// SSL/TLS 配置
    #[serde(default, skip_serializing_if = "is_ssl_empty")]
    pub ssl: SslConfig,
}

fn is_zero(v: &u16) -> bool {
    *v == 0
}

fn is_ssl_empty(ssl: &SslConfig) -> bool {
    ssl.mode.is_empty() && ssl.ca_cert.is_empty()
}

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    /// 是否启用内容加密
    #[serde(default, rename = "content-encryption")]
    pub content_encryption: bool,
}

/// 单个 Profile 配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileConfig {
    /// Profile 显示名称
    pub name: String,
    /// 图标标识（用于 UI 区分）
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub icon: String,
    /// 数据源配置
    pub datasource: DatasourceConfig,
    /// 安全配置
    #[serde(default)]
    pub security: SecurityConfig,
}

/// Profile 摘要信息（用于列表展示）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSummary {
    /// Profile 文件名（不含 .yml 后缀）
    pub id: String,
    /// 显示名称
    pub name: String,
    /// 图标
    pub icon: String,
    /// 数据库类型
    pub db_type: String,
    /// 连接信息摘要（如 "localhost:3306/enote"）
    pub connection_info: String,
    /// 是否启用内容加密
    pub content_encryption: bool,
    /// 是否是当前活跃的 profile
    pub is_active: bool,
}

// ============================================================================
// 路径工具函数
// ============================================================================

/// 获取 profiles 目录路径
fn profiles_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("profiles")
}

/// 获取 profiles.yml 索引文件路径
fn index_file_path(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("profiles.yml")
}

/// 获取单个 profile 文件路径
fn profile_file_path(app_data_dir: &Path, profile_id: &str) -> PathBuf {
    profiles_dir(app_data_dir).join(format!("{}.yml", profile_id))
}

/// 确保 profiles 目录存在
fn ensure_profiles_dir(app_data_dir: &Path) -> Result<()> {
    let dir = profiles_dir(app_data_dir);
    if !dir.exists() {
        fs::create_dir_all(&dir).context("创建 profiles 目录失败")?;
    }
    Ok(())
}

// ============================================================================
// 索引文件操作
// ============================================================================

/// 读取 profile 索引
pub fn read_index(app_data_dir: &Path) -> Result<ProfileIndex> {
    let path = index_file_path(app_data_dir);
    if !path.exists() {
        return Ok(ProfileIndex::default());
    }
    let content = fs::read_to_string(&path).context("读取 profiles.yml 失败")?;
    serde_yaml::from_str(&content).context("解析 profiles.yml 失败")
}

/// 保存 profile 索引
pub fn save_index(app_data_dir: &Path, index: &ProfileIndex) -> Result<()> {
    let path = index_file_path(app_data_dir);
    let content = serde_yaml::to_string(index).context("序列化 profiles.yml 失败")?;
    fs::write(&path, content).context("写入 profiles.yml 失败")?;
    Ok(())
}

// ============================================================================
// Profile CRUD
// ============================================================================

/// 检查是否需要初始化设置（没有任何 profile）
pub fn needs_setup(app_data_dir: &Path) -> bool {
    let dir = profiles_dir(app_data_dir);
    if !dir.exists() {
        return true;
    }
    match fs::read_dir(&dir) {
        Ok(entries) => !entries.filter_map(|e| e.ok()).any(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "yml" || ext == "yaml")
        }),
        Err(_) => true,
    }
}

/// 列出所有 profile 的摘要信息
pub fn list_profiles(app_data_dir: &Path) -> Result<Vec<ProfileSummary>> {
    let dir = profiles_dir(app_data_dir);
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let index = read_index(app_data_dir)?;
    let mut profiles = Vec::new();

    let mut entries: Vec<_> = fs::read_dir(&dir)
        .context("读取 profiles 目录失败")?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "yml" || ext == "yaml")
        })
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();

        if let Ok(config) = read_profile(app_data_dir, &id) {
            let connection_info = match config.datasource.db_type.as_str() {
                "sqlite" => config.datasource.path.clone(),
                "mysql" | "postgres" => {
                    format!(
                        "{}:{}/{}",
                        config.datasource.host, config.datasource.port, config.datasource.database
                    )
                }
                _ => String::new(),
            };

            profiles.push(ProfileSummary {
                is_active: index.active == id,
                id,
                name: config.name,
                icon: config.icon,
                db_type: config.datasource.db_type,
                connection_info,
                content_encryption: config.security.content_encryption,
            });
        }
    }

    Ok(profiles)
}

/// 读取单个 profile 配置
pub fn read_profile(app_data_dir: &Path, profile_id: &str) -> Result<ProfileConfig> {
    let path = profile_file_path(app_data_dir, profile_id);
    let content =
        fs::read_to_string(&path).with_context(|| format!("读取 profile 文件失败: {:?}", path))?;
    serde_yaml::from_str(&content).with_context(|| format!("解析 profile 文件失败: {:?}", path))
}

/// 保存 profile 配置
pub fn save_profile(app_data_dir: &Path, profile_id: &str, config: &ProfileConfig) -> Result<()> {
    ensure_profiles_dir(app_data_dir)?;
    let path = profile_file_path(app_data_dir, profile_id);
    let content = serde_yaml::to_string(config).context("序列化 profile 配置失败")?;
    fs::write(&path, content).context("写入 profile 文件失败")?;
    info!("Profile 已保存: {:?}", path);
    Ok(())
}

/// 删除 profile
pub fn delete_profile(app_data_dir: &Path, profile_id: &str) -> Result<()> {
    let path = profile_file_path(app_data_dir, profile_id);
    if path.exists() {
        fs::remove_file(&path).context("删除 profile 文件失败")?;
    }

    // 如果当前活跃的就是被删除的 profile，清除 active
    let mut index = read_index(app_data_dir)?;
    if index.active == profile_id {
        index.active = String::new();
        save_index(app_data_dir, &index)?;
    }

    info!("Profile 已删除: {}", profile_id);
    Ok(())
}

/// 设置活跃 profile
pub fn set_active_profile(app_data_dir: &Path, profile_id: &str) -> Result<()> {
    // 验证 profile 存在
    let path = profile_file_path(app_data_dir, profile_id);
    if !path.exists() {
        anyhow::bail!("Profile 不存在: {}", profile_id);
    }

    let mut index = read_index(app_data_dir)?;
    index.active = profile_id.to_string();
    save_index(app_data_dir, &index)?;
    info!("活跃 Profile 已设置为: {}", profile_id);
    Ok(())
}

/// 设置是否自动连接
pub fn set_auto_connect(app_data_dir: &Path, auto_connect: bool) -> Result<()> {
    let mut index = read_index(app_data_dir)?;
    index.auto_connect = auto_connect;
    save_index(app_data_dir, &index)?;
    Ok(())
}

/// 根据 profile 配置生成数据库连接 URL（包含 SSL 参数）
pub fn build_database_url(config: &ProfileConfig, password: Option<&str>) -> String {
    match config.datasource.db_type.as_str() {
        "mysql" => {
            let user = &config.datasource.username;
            let host = &config.datasource.host;
            let port = config.datasource.port;
            let db = &config.datasource.database;
            let base = if let Some(pwd) = password {
                format!("mysql://{}:{}@{}:{}/{}", user, pwd, host, port, db)
            } else {
                format!("mysql://{}@{}:{}/{}", user, host, port, db)
            };
            // 附加 SSL 参数
            let ssl = &config.datasource.ssl;
            let mut params = Vec::new();
            if !ssl.mode.is_empty() && ssl.mode != "disabled" {
                params.push(format!("ssl-mode={}", ssl.mode.to_uppercase()));
            }
            if !ssl.ca_cert.is_empty() {
                params.push(format!("ssl-ca={}", ssl.ca_cert));
            }
            if !ssl.client_cert.is_empty() {
                params.push(format!("ssl-cert={}", ssl.client_cert));
            }
            if !ssl.client_key.is_empty() {
                params.push(format!("ssl-key={}", ssl.client_key));
            }
            if params.is_empty() {
                base
            } else {
                format!("{}?{}", base, params.join("&"))
            }
        }
        "postgres" => {
            let user = &config.datasource.username;
            let host = &config.datasource.host;
            let port = config.datasource.port;
            let db = &config.datasource.database;
            let base = if let Some(pwd) = password {
                format!("postgres://{}:{}@{}:{}/{}", user, pwd, host, port, db)
            } else {
                format!("postgres://{}@{}:{}/{}", user, host, port, db)
            };
            // 附加 SSL 参数
            let ssl = &config.datasource.ssl;
            let mut params = Vec::new();
            if !ssl.mode.is_empty() && ssl.mode != "disabled" {
                params.push(format!("sslmode={}", ssl.mode));
            }
            if !ssl.ca_cert.is_empty() {
                params.push(format!("sslrootcert={}", ssl.ca_cert));
            }
            if !ssl.client_cert.is_empty() {
                params.push(format!("sslcert={}", ssl.client_cert));
            }
            if !ssl.client_key.is_empty() {
                params.push(format!("sslkey={}", ssl.client_key));
            }
            if params.is_empty() {
                base
            } else {
                format!("{}?{}", base, params.join("&"))
            }
        }
        _ => {
            // SQLite
            let path = &config.datasource.path;
            format!("sqlite:{}?mode=rwc", path)
        }
    }
}

/// 将名称转为文件名安全的 profile ID
pub fn name_to_id(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else if c == ' ' {
                '-'
            } else {
                '_'
            }
        })
        .collect::<String>()
        .to_lowercase()
}
