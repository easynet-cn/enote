//! Profile 存储后端工厂
//!
//! 根据 [`ProfileConfig`] 构造 [`NoteStore`] / [`TodoStore`] 及配套运行状态，
//! 供 Tauri 主程序（启动、Profile 热切换）与独立 MCP 进程共用，
//! 避免多处各自实现后端分发、Keychain 取密钥、Git 远程配置等逻辑。
//!
//! # 设计约定
//! - 工厂**不依赖 Tauri `AppHandle`**，因此可被独立进程调用；
//! - Git 后端的启动同步不在此处执行：工厂仅返回同步所需的仓库路径与远程配置，
//!   由调用方决定是否同步（Tauri 侧带 `AppHandle` 推送事件，MCP 侧可跳过）；
//! - 错误码与原有 command 层保持一致，前端 i18n 无需新增词条。

use std::path::{Path, PathBuf};
use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tauri::AppHandle;

use crate::config::{ProfileBackend, database_connection_from_profile};
use crate::error::AppError;
use crate::service::enote_server::auth::ServerSecrets;
use crate::service::enote_server::EnoteServerClient;
use crate::service::profile::ProfileConfig;
use crate::store::git_remote::GitRemoteConfig;
use crate::store::git_sync::GitCredentials;
use crate::store::{AccessStore, DatabaseStore, GitStore, NoteStore, ServerStore, TodoStore};

/// 工厂构造出的存储后端集合
pub struct StoreBundle {
    /// Profile ID
    pub profile_id: String,
    /// 当前后端类型
    pub backend: ProfileBackend,
    /// 数据库连接（仅 Database 后端有值）
    pub database_connection: Option<DatabaseConnection>,
    /// 内容加密密钥（仅 Database / Git 后端可能有值）
    pub encryption_key: Option<String>,
    /// 笔记域存储后端
    pub note_store: Arc<dyn NoteStore>,
    /// 待办域存储后端
    pub todo_store: Arc<dyn TodoStore>,
    /// MCP 访问能力（与上面两个视图共享同一实例）
    pub access_store: Arc<dyn AccessStore>,
    /// Git 后端启动同步所需信息（仓库路径 + 远程配置），其他后端为 None
    pub git_sync: Option<(PathBuf, GitRemoteConfig)>,
}

/// 凭据读取策略
///
/// 桌面端持有 GUI 会话，可以正常访问系统 Keychain；独立 MCP 进程没有 GUI 上下文，
/// 首次访问会触发 macOS 授权弹窗而阻塞 stdio 通信，因此默认跳过凭据读取。
/// 跳过后本地笔记读写不受影响，仅远程认证与内容解密不可用。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CredentialPolicy {
    /// 从系统 Keychain 读取密钥与令牌（桌面端）
    Keychain,
    /// 不访问 Keychain（独立 MCP 进程）
    Skip,
}

/// 根据 Profile 配置与 Keychain 凭据构造 Git 远程运行时配置
///
/// 密码/PAT 仅运行时从 Keychain 读取，不落配置文件。
pub fn build_git_remote(
    profile_id: &str,
    git_config: &crate::service::profile::GitStoreConfig,
    policy: CredentialPolicy,
) -> GitRemoteConfig {
    let credentials = match git_config.remote_auth.as_str() {
        "https" => GitCredentials::Https {
            username: git_config.remote_username.clone(),
            // 缺失时给空串（认证失败会明确报错）
            password: if policy == CredentialPolicy::Keychain {
                crate::service::keychain::get_git_password(profile_id)
                    .ok()
                    .flatten()
                    .unwrap_or_default()
            } else {
                String::new()
            },
        },
        "ssh" => GitCredentials::Ssh,
        _ => GitCredentials::None,
    };
    GitRemoteConfig {
        url: git_config.remote_url.clone(),
        credentials,
    }
}

/// 从 Keychain 读取内容加密密钥（如果 Profile 启用了加密）
fn resolve_encryption_key(
    profile_id: &str,
    profile_config: &ProfileConfig,
    policy: CredentialPolicy,
) -> Result<Option<String>, AppError> {
    if !profile_config.security.content_encryption || policy == CredentialPolicy::Skip {
        return Ok(None);
    }
    #[cfg(any(feature = "desktop", feature = "db-full"))]
    {
        crate::service::keychain::get_encryption_key(profile_id).map_err(AppError::from)
    }
    #[cfg(not(any(feature = "desktop", feature = "db-full")))]
    {
        Ok(None)
    }
}

/// 根据 Profile 配置构造存储后端
///
/// 供 Tauri 启动、Profile 热切换与独立 MCP 进程共用。
/// 数据库迁移、Git 启动同步由调用方根据返回的字段自行执行。
pub async fn build_profile_stores(
    app_data_dir: &Path,
    profile_id: &str,
    profile_config: &ProfileConfig,
    app_handle: Option<AppHandle>,
    credentials: CredentialPolicy,
) -> Result<StoreBundle, AppError> {
    if profile_config.is_server_backend() {
        // ======== Server 后端 ========
        let server_config = profile_config
            .server
            .as_ref()
            .ok_or_else(|| AppError::code("SERVER_CONFIG_MISSING"))?;

        let secrets = if credentials == CredentialPolicy::Keychain {
            ServerSecrets {
                token: crate::service::keychain::get_server_token(profile_id).unwrap_or(None),
                refresh_token: crate::service::keychain::get_server_refresh_token(profile_id)
                    .unwrap_or(None),
                password: crate::service::keychain::get_server_password(profile_id).unwrap_or(None),
                client_secret: crate::service::keychain::get_server_client_secret(profile_id)
                    .unwrap_or(None),
                header_value: crate::service::keychain::get_server_header_value(profile_id)
                    .unwrap_or(None),
            }
        } else {
            ServerSecrets::default()
        };

        let client = EnoteServerClient::new(server_config, secrets).map_err(|e| {
            AppError::code_with_args("SERVER_CONNECTION_FAILED", vec![e.to_string()])
        })?;

        let store = Arc::new(ServerStore::new(client.clone()));
        let note_store: Arc<dyn NoteStore> = store.clone();
        let todo_store: Arc<dyn TodoStore> = store.clone();
        let access_store: Arc<dyn AccessStore> = store;

        Ok(StoreBundle {
            profile_id: profile_id.to_string(),
            backend: ProfileBackend::Server(client),
            database_connection: None,
            encryption_key: None,
            note_store,
            todo_store,
            access_store,
            git_sync: None,
        })
    } else if profile_config.is_git_backend() {
        // ======== Git 后端 ========
        if !crate::store::git_sync::is_available() {
            return Err(AppError::code("GIT_NOT_AVAILABLE"));
        }

        let git_config = profile_config
            .git
            .as_ref()
            .ok_or_else(|| AppError::code("GIT_CONFIG_MISSING"))?;

        let encryption_key = resolve_encryption_key(profile_id, profile_config, credentials)?;
        let repo_path = PathBuf::from(&git_config.repo_path);
        let remote = build_git_remote(profile_id, git_config, credentials);

        // Tauri 侧传入 AppHandle 以推送 git-sync-result 事件；
        // 独立 MCP 进程传 None，后台推送线程仍可工作，只是无前端事件。
        let git_store = Arc::new(GitStore::new(repo_path.clone(), encryption_key.clone())
            .with_remote(Some(remote.clone()), app_handle));

        let note_store: Arc<dyn NoteStore> = git_store.clone();
        let todo_store: Arc<dyn TodoStore> = git_store.clone();
        let access_store: Arc<dyn AccessStore> = git_store;

        Ok(StoreBundle {
            profile_id: profile_id.to_string(),
            backend: ProfileBackend::Git,
            database_connection: None,
            encryption_key,
            note_store,
            todo_store,
            access_store,
            git_sync: Some((repo_path, remote)),
        })
    } else {
        // ======== Database 后端 ========
        #[cfg(any(feature = "desktop", feature = "db-full"))]
        let db_password = if credentials == CredentialPolicy::Keychain
            && profile_config.datasource.db_type != "sqlite"
            && profile_config.datasource.auth_method != "certificate"
        {
            crate::service::keychain::get_db_password(profile_id).map_err(AppError::from)?
        } else {
            None
        };
        #[cfg(not(any(feature = "desktop", feature = "db-full")))]
        let db_password: Option<String> = None;

        let db = database_connection_from_profile(profile_config, db_password.as_deref())
            .await
            .map_err(|e| AppError::code_with_args("DB_CONNECTION_FAILED", vec![e.to_string()]))?;

        let encryption_key = resolve_encryption_key(profile_id, profile_config, credentials)?;

        let store = Arc::new(DatabaseStore::new(
            db.clone(),
            encryption_key.clone(),
            app_data_dir.to_path_buf(),
        ));
        let note_store: Arc<dyn NoteStore> = store.clone();
        let todo_store: Arc<dyn TodoStore> = store.clone();
        let access_store: Arc<dyn AccessStore> = store;

        Ok(StoreBundle {
            profile_id: profile_id.to_string(),
            backend: ProfileBackend::Database,
            database_connection: Some(db),
            encryption_key,
            note_store,
            todo_store,
            access_store,
            git_sync: None,
        })
    }
}
