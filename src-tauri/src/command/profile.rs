use super::*;

use crate::config::ProfileBackend;
use crate::service::enote_server::auth::{ServerConfig, ServerSecrets};

// ============================================================================
// IPC 键名约定转换
//
// YAML 存储使用 kebab-case（如 auth-method），TypeScript/JSON 使用 camelCase
// （如 authMethod）。以下函数在 IPC 边界做双向转换，使每种语言保持各自惯例。
// ============================================================================

/// 将 kebab-case 字符串转为 camelCase（如 `auth-method` → `authMethod`）
fn kebab_to_camel(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut upper = false;
    for c in s.chars() {
        if c == '-' || c == '_' {
            upper = true;
        } else if upper {
            out.push(c.to_ascii_uppercase());
            upper = false;
        } else {
            out.push(c);
        }
    }
    out
}

/// 将 camelCase 字符串转为 kebab-case（如 `authMethod` → `auth-method`）
///
/// 已经是 kebab-case 的输入（如 `auth-method`）保持不变，因此前端混用两种
/// 风格时也能正确落到 YAML 的 kebab-case。
fn camel_to_kebab(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            out.push('-');
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// 递归将 JSON Value 的所有键名从 kebab-case 转为 camelCase（响应方向）
fn keys_to_camel(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (k, v) in map.iter_mut() {
                keys_to_camel(v);
                new_map.insert(kebab_to_camel(k), v.take());
            }
            *map = new_map;
        }
        serde_json::Value::Array(arr) => {
            for v in arr.iter_mut() {
                keys_to_camel(v);
            }
        }
        _ => {}
    }
}

/// 递归将 JSON Value 的所有键名从 camelCase 转为 kebab-case（请求方向）
fn keys_to_kebab(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (k, v) in map.iter_mut() {
                keys_to_kebab(v);
                new_map.insert(camel_to_kebab(k), v.take());
            }
            *map = new_map;
        }
        serde_json::Value::Array(arr) => {
            for v in arr.iter_mut() {
                keys_to_kebab(v);
            }
        }
        _ => {}
    }
}

/// 检查是否需要初始化设置
#[tauri::command]
pub async fn check_setup_needed(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<bool, AppError> {
    Ok(service::profile::needs_setup(&app_state.app_data_dir))
}

/// 获取 Profile 索引信息
#[tauri::command]
pub async fn get_profile_index(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<service::profile::ProfileIndex, AppError> {
    service::profile::read_index(&app_state.app_data_dir).map_err(AppError::from)
}

/// 列出所有 Profile
#[tauri::command]
pub async fn list_profiles(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<service::profile::ProfileSummary>, AppError> {
    service::profile::list_profiles(&app_state.app_data_dir).map_err(AppError::from)
}

/// 获取单个 Profile 配置
///
/// YAML 中使用 kebab-case 存储，返回前端时统一转为 camelCase。
#[tauri::command]
pub async fn get_profile(
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
) -> Result<serde_json::Value, AppError> {
    let config = service::profile::read_profile(&app_state.app_data_dir, &profile_id)
        .map_err(AppError::from)?;
    let mut value = serde_json::to_value(config)
        .map_err(|e| AppError::code_with_args("JSON_SERIALIZE_FAILED", vec![e.to_string()]))?;
    keys_to_camel(&mut value);
    Ok(value)
}

/// 保存 Profile 配置
///
/// 前端传入 camelCase，转为 kebab-case 后反序列化为 ProfileConfig 并写入 YAML。
/// 同时处理 Keychain 中的密码、加密密钥和服务器认证信息存储。
#[tauri::command]
pub async fn save_profile_config(
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
    mut config: serde_json::Value,
    db_password: Option<String>,
    encryption_key: Option<String>,
    server_token: Option<String>,
    server_password: Option<String>,
    server_client_secret: Option<String>,
    git_password: Option<String>,
) -> Result<(), AppError> {
    keys_to_kebab(&mut config);
    let config: service::profile::ProfileConfig = serde_json::from_value(config)
        .map_err(|e| AppError::code_with_args("JSON_DESERIALIZE_FAILED", vec![e.to_string()]))?;

    // 保存 profile 文件
    service::profile::save_profile(&app_state.app_data_dir, &profile_id, &config)
        .map_err(AppError::from)?;

    // 保存数据库密码到 Keychain
    if let Some(password) = db_password
        && !password.is_empty()
    {
        service::keychain::set_db_password(&profile_id, &password).map_err(AppError::from)?;
    }

    // 保存加密密钥到 Keychain
    if let Some(key) = encryption_key
        && !key.is_empty()
    {
        service::keychain::set_encryption_key(&profile_id, &key).map_err(AppError::from)?;
    }

    // 保存服务器认证信息到 Keychain
    if let Some(token) = server_token
        && !token.is_empty()
    {
        service::keychain::set_server_token(&profile_id, &token).map_err(AppError::from)?;
    }
    if let Some(password) = server_password
        && !password.is_empty()
    {
        service::keychain::set_server_password(&profile_id, &password).map_err(AppError::from)?;
    }
    if let Some(secret) = server_client_secret
        && !secret.is_empty()
    {
        service::keychain::set_server_client_secret(&profile_id, &secret).map_err(AppError::from)?;
    }

    // 保存 Git HTTPS 密码/访问令牌到 Keychain
    if let Some(password) = git_password
        && !password.is_empty()
    {
        service::keychain::set_git_password(&profile_id, &password).map_err(AppError::from)?;
    }

    Ok(())
}

/// 删除 Profile 配置
#[tauri::command]
pub async fn delete_profile_config(
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
) -> Result<(), AppError> {
    // 删除 Keychain 中的密钥
    service::keychain::delete_profile_secrets(&profile_id).map_err(AppError::from)?;
    // 删除 profile 文件
    service::profile::delete_profile(&app_state.app_data_dir, &profile_id).map_err(AppError::from)
}

/// 选择活跃 Profile
#[tauri::command]
pub async fn select_profile(
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
) -> Result<(), AppError> {
    service::profile::set_active_profile(&app_state.app_data_dir, &profile_id)
        .map_err(AppError::from)
}

/// 设置是否自动连接
#[tauri::command(rename_all = "camelCase")]
pub async fn set_auto_connect(
    app_state: tauri::State<'_, Arc<AppState>>,
    auto_connect: bool,
) -> Result<(), AppError> {
    service::profile::set_auto_connect(&app_state.app_data_dir, auto_connect)
        .map_err(AppError::from)
}

/// 测试数据库连接
#[tauri::command]
pub async fn test_db_connection(
    mut config: serde_json::Value,
    db_password: Option<String>,
) -> Result<bool, AppError> {
    keys_to_kebab(&mut config);
    let config: service::profile::ProfileConfig = serde_json::from_value(config)
        .map_err(|e| AppError::code_with_args("JSON_DESERIALIZE_FAILED", vec![e.to_string()]))?;
    match crate::config::database_connection_from_profile(&config, db_password.as_deref()).await {
        Ok(_) => Ok(true),
        Err(e) => Err(AppError::code_with_args(
            "DB_CONNECTION_FAILED",
            vec![e.to_string()],
        )),
    }
}

/// 测试服务器连接
#[tauri::command]
pub async fn test_server_connection(
    mut config: serde_json::Value,
    server_token: Option<String>,
    server_password: Option<String>,
    server_header_value: Option<String>,
) -> Result<bool, AppError> {
    keys_to_kebab(&mut config);
    let config: ServerConfig = serde_json::from_value(config)
        .map_err(|e| AppError::code_with_args("JSON_DESERIALIZE_FAILED", vec![e.to_string()]))?;
    let secrets = ServerSecrets {
        token: server_token,
        password: server_password,
        header_value: server_header_value,
        ..Default::default()
    };
    let client = EnoteServerClient::new(&config, secrets)
        .map_err(|e| AppError::code_with_args("SERVER_CONNECTION_FAILED", vec![e.to_string()]))?;
    client.test_connection().await?;
    Ok(true)
}

/// 生成加密密钥
#[tauri::command]
pub async fn generate_encryption_key() -> Result<String, AppError> {
    Ok(service::keychain::generate_encryption_key())
}

/// 检测系统是否安装 git（git 后端 Profile 的可用性前置条件）
#[tauri::command]
pub async fn git_available() -> bool {
    crate::store::git_sync::is_available()
}

/// 根据 Profile 配置与 Keychain 凭据构造 Git 远程运行时配置
pub(crate) fn build_git_remote(
    profile_id: &str,
    git_config: &service::profile::GitStoreConfig,
) -> crate::store::git_remote::GitRemoteConfig {
    use crate::store::git_sync::GitCredentials;

    let credentials = match git_config.remote_auth.as_str() {
        "https" => GitCredentials::Https {
            username: git_config.remote_username.clone(),
            // 密码/PAT 从 Keychain 读取，缺失时给空串（认证失败会明确报错）
            password: service::keychain::get_git_password(profile_id)
                .ok()
                .flatten()
                .unwrap_or_default(),
        },
        "ssh" => GitCredentials::Ssh,
        _ => GitCredentials::None,
    };
    crate::store::git_remote::GitRemoteConfig {
        url: git_config.remote_url.clone(),
        credentials,
    }
}

/// 测试 Git 远程仓库连接（向导「测试连接」使用，不读写本地仓库配置）
#[tauri::command(rename_all = "camelCase")]
pub async fn test_git_connection(
    url: String,
    auth: String,
    username: Option<String>,
    password: Option<String>,
) -> Result<bool, AppError> {
    use crate::store::git_remote;
    use crate::store::git_sync::GitCredentials;

    if !crate::store::git_sync::is_available() {
        return Err(AppError::code("GIT_NOT_AVAILABLE"));
    }

    let credentials = match auth.as_str() {
        "https" => GitCredentials::Https {
            username: username.unwrap_or_default(),
            password: password.unwrap_or_default(),
        },
        "ssh" => GitCredentials::Ssh,
        _ => GitCredentials::None,
    };

    // git 命令为阻塞调用，放到阻塞线程池执行，避免卡住 async 运行时
    tauri::async_runtime::spawn_blocking(move || git_remote::test_remote(&url, &credentials))
        .await
        .map_err(|e| AppError::code_with_args("GIT_SYNC_FAILED", vec![e.to_string()]))?
        .map_err(|e| AppError::code_with_args(e.code.as_str(), vec![e.message]))?;
    Ok(true)
}

/// 使用指定 profile 重启应用（仅 release 构建使用）
///
/// 设置活跃 profile 后通过前端触发 app restart
#[tauri::command]
pub async fn restart_with_profile(
    app_handle: tauri::AppHandle,
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
) -> Result<(), AppError> {
    service::profile::set_active_profile(&app_state.app_data_dir, &profile_id)
        .map_err(AppError::from)?;

    // 通过 Tauri 重启应用
    app_handle.restart();
}

/// 热切换 Profile（不重启进程）
///
/// 根据 profile 后端类型，创建数据库连接或 HTTP 客户端，替换 AppState。
/// 前端调用后需刷新所有数据。
#[tauri::command]
pub async fn reconnect_profile(
    app_handle: tauri::AppHandle,
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
) -> Result<(), AppError> {
    info!("Hot-switching profile: {}", profile_id);

    // 防止并发切换：切换进行中收到新请求直接失败，避免交替写状态
    let _switch_guard = app_state
        .profile_switch_lock
        .try_lock()
        .map_err(|_| AppError::code("PROFILE_SWITCH_IN_PROGRESS"))?;

    // 1. 读取新 profile 配置
    let profile_config = service::profile::read_profile(&app_state.app_data_dir, &profile_id)
        .map_err(AppError::from)?;

    if profile_config.is_server_backend() {
        // ======== Server 后端路径 ========
        let server_config = profile_config
            .server
            .as_ref()
            .ok_or_else(|| AppError::code("SERVER_CONFIG_MISSING"))?;

        // 从 Keychain 加载服务器认证信息
        let secrets = ServerSecrets {
            token: service::keychain::get_server_token(&profile_id).unwrap_or(None),
            refresh_token: service::keychain::get_server_refresh_token(&profile_id).unwrap_or(None),
            password: service::keychain::get_server_password(&profile_id).unwrap_or(None),
            client_secret: service::keychain::get_server_client_secret(&profile_id).unwrap_or(None),
            header_value: service::keychain::get_server_header_value(&profile_id).unwrap_or(None),
        };

        let client = EnoteServerClient::new(server_config, secrets).map_err(|e| {
            AppError::code_with_args("SERVER_CONNECTION_FAILED", vec![e.to_string()])
        })?;

        // 替换 AppState
        {
            let mut db_guard = app_state.database_connection.write().await;
            *db_guard = None; // Server 后端不使用数据库连接
        }
        {
            let mut backend_guard = app_state.backend.write().await;
            *backend_guard = ProfileBackend::Server(client.clone());
        }
        {
            // 笔记域存储后端切换为远程服务器实现
            let mut store_guard = app_state.note_store.write().await;
            *store_guard = Some(std::sync::Arc::new(crate::store::ServerStore::new(
                client.clone(),
            )));
        }
        {
            // 待办域存储后端切换为远程服务器实现
            let mut todo_guard = app_state.todo_store.write().await;
            *todo_guard = Some(std::sync::Arc::new(crate::store::ServerStore::new(
                client.clone(),
            )));
        }
        {
            let mut key_guard = app_state.encryption_key.write().await;
            *key_guard = None; // Server 后端的加密由服务端处理
        }
    } else if profile_config.is_git_backend() {
        // ======== Git 后端路径 ========
        // git 后端依赖系统 git CLI，缺失时明确拒绝连接
        if !crate::store::git_sync::is_available() {
            return Err(AppError::code("GIT_NOT_AVAILABLE"));
        }

        let git_config = profile_config
            .git
            .as_ref()
            .ok_or_else(|| AppError::code("GIT_CONFIG_MISSING"))?;

        // 从 Keychain 获取加密密钥（如果启用加密）
        #[cfg(any(feature = "desktop", feature = "db-full"))]
        let enc_key = if profile_config.security.content_encryption {
            service::keychain::get_encryption_key(&profile_id).map_err(AppError::from)?
        } else {
            None
        };
        #[cfg(not(any(feature = "desktop", feature = "db-full")))]
        let enc_key: Option<String> = None;

        let repo_path = std::path::PathBuf::from(&git_config.repo_path);

        // 构造远程配置（URL + Keychain 凭据）
        let remote = build_git_remote(&profile_id, git_config);

        // GitStore 在锁内创建（目录初始化 + git init + 索引构建）。
        // 即使远程同步尚未完成，GitStore 也能处理——空仓库 + fetch 远程后 pull 即可。
        let git_store = std::sync::Arc::new(
            crate::store::GitStore::new(repo_path.clone(), enc_key.clone())
                .with_remote(Some(remote.clone()), Some(app_handle.clone())),
        );

        // 替换 AppState
        {
            let mut db_guard = app_state.database_connection.write().await;
            *db_guard = None;
        }
        {
            let mut backend_guard = app_state.backend.write().await;
            *backend_guard = ProfileBackend::Git;
        }
        {
            // 笔记域存储后端切换为 Git 仓库实现
            let mut store_guard = app_state.note_store.write().await;
            *store_guard = Some(git_store.clone());
        }
        {
            // 待办域存储后端切换为 Git 仓库实现
            let mut todo_guard = app_state.todo_store.write().await;
            *todo_guard = Some(git_store.clone());
        }
        {
            let mut key_guard = app_state.encryption_key.write().await;
            *key_guard = enc_key;
        }

        // ← profile_switch_lock 在这里释放（_switch_guard drop）
        // 下面的 Git 同步在锁外 fire-and-forget，不阻塞 UI。
        // 成功/失败都会通过 git-sync-result 事件推给前端。
        if !remote.url.is_empty() {
            let sync_remote = remote.clone();
            let sync_path = repo_path.clone();
            let sync_handle = app_handle.clone();
            tauri::async_runtime::spawn_blocking(move || {
                crate::store::git_remote::ensure_and_sync(
                    &sync_path,
                    &sync_remote,
                    Some(&sync_handle),
                )
            });
        }
    } else {
        // ======== Database 后端路径（原有逻辑不变） ========

        // 2. 从 Keychain 获取数据库密码
        #[cfg(any(feature = "desktop", feature = "db-full"))]
        let db_password = if profile_config.datasource.db_type != "sqlite"
            && profile_config.datasource.auth_method != "certificate"
        {
            service::keychain::get_db_password(&profile_id).map_err(AppError::from)?
        } else {
            None
        };
        #[cfg(not(any(feature = "desktop", feature = "db-full")))]
        let db_password: Option<String> = None;

        // 3. 创建新数据库连接
        let new_db = crate::config::database_connection_from_profile(
            &profile_config,
            db_password.as_deref(),
        )
        .await
        .map_err(|e| AppError::code_with_args("DB_CONNECTION_FAILED", vec![e.to_string()]))?;

        // 4. 运行数据库迁移
        use sea_orm_migration::MigratorTrait;
        crate::migration::Migrator::up(&new_db, None)
            .await
            .map_err(|e| AppError::code_with_args("DB_MIGRATION_FAILED", vec![e.to_string()]))?;

        // 5. 从 Keychain 获取加密密钥
        #[cfg(any(feature = "desktop", feature = "db-full"))]
        let new_encryption_key = if profile_config.security.content_encryption {
            service::keychain::get_encryption_key(&profile_id).map_err(AppError::from)?
        } else {
            None
        };
        #[cfg(not(any(feature = "desktop", feature = "db-full")))]
        let new_encryption_key: Option<String> = None;

        // 6. 替换 AppState 中的连接和相关状态
        {
            let mut db_guard = app_state.database_connection.write().await;
            *db_guard = Some(new_db.clone());
        }
        {
            let mut backend_guard = app_state.backend.write().await;
            *backend_guard = ProfileBackend::Database;
        }
        {
            // 笔记域存储后端切换为数据库实现
            let mut store_guard = app_state.note_store.write().await;
            *store_guard = Some(std::sync::Arc::new(crate::store::DatabaseStore::new(
                new_db.clone(),
                new_encryption_key.clone(),
                app_state.app_data_dir.clone(),
            )));
        }
        {
            // 待办域存储后端切换为数据库实现
            let mut todo_guard = app_state.todo_store.write().await;
            *todo_guard = Some(std::sync::Arc::new(crate::store::DatabaseStore::new(
                new_db.clone(),
                new_encryption_key.clone(),
                app_state.app_data_dir.clone(),
            )));
        }
        {
            let mut key_guard = app_state.encryption_key.write().await;
            *key_guard = new_encryption_key;
        }
    }

    // 公共状态更新
    // 新后端全部就绪后才写入活跃 profile：切换失败时 active 保持原值，
    // 避免指向一个连接失败的配置（下次启动 auto-connect 会失败）
    service::profile::set_active_profile(&app_state.app_data_dir, &profile_id)
        .map_err(AppError::from)?;
    {
        let mut profile_guard = app_state.active_profile_id.write().await;
        *profile_guard = profile_id.clone();
    }
    // 清空设置缓存（新 profile 有不同的设置）
    {
        let mut cache = app_state.settings_cache.write().await;
        *cache = None;
    }

    info!("Profile hot-switch completed: {}", profile_id);
    Ok(())
}
