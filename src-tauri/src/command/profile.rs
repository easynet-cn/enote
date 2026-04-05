use super::*;

use crate::config::ProfileBackend;
use crate::service::enote_server::auth::{ServerConfig, ServerSecrets};

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
#[tauri::command]
pub async fn get_profile(
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
) -> Result<service::profile::ProfileConfig, AppError> {
    service::profile::read_profile(&app_state.app_data_dir, &profile_id).map_err(AppError::from)
}

/// 保存 Profile 配置
///
/// 同时处理 Keychain 中的密码、加密密钥和服务器认证信息存储
#[tauri::command]
pub async fn save_profile_config(
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
    config: service::profile::ProfileConfig,
    db_password: Option<String>,
    encryption_key: Option<String>,
    server_token: Option<String>,
    server_password: Option<String>,
    server_client_secret: Option<String>,
) -> Result<(), AppError> {
    // 保存 profile 文件
    service::profile::save_profile(&app_state.app_data_dir, &profile_id, &config)
        .map_err(AppError::from)?;

    // 保存数据库密码到 Keychain
    if let Some(password) = db_password
        && !password.is_empty() {
            service::keychain::set_db_password(&profile_id, &password).map_err(AppError::from)?;
        }

    // 保存加密密钥到 Keychain
    if let Some(key) = encryption_key
        && !key.is_empty() {
            service::keychain::set_encryption_key(&profile_id, &key).map_err(AppError::from)?;
        }

    // 保存服务器认证信息到 Keychain
    if let Some(token) = server_token
        && !token.is_empty() {
            service::keychain::set_server_token(&profile_id, &token).map_err(AppError::from)?;
        }
    if let Some(password) = server_password
        && !password.is_empty() {
            service::keychain::set_server_password(&profile_id, &password).map_err(AppError::from)?;
        }
    if let Some(secret) = server_client_secret
        && !secret.is_empty() {
            service::keychain::set_server_client_secret(&profile_id, &secret).map_err(AppError::from)?;
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
    config: service::profile::ProfileConfig,
    db_password: Option<String>,
) -> Result<bool, AppError> {
    match crate::config::database_connection_from_profile(&config, db_password.as_deref()).await {
        Ok(_) => Ok(true),
        Err(e) => Err(AppError::code_with_args("DB_CONNECTION_FAILED", vec![e.to_string()])),
    }
}

/// 测试服务器连接
#[tauri::command]
pub async fn test_server_connection(
    config: ServerConfig,
    server_token: Option<String>,
    server_password: Option<String>,
    server_header_value: Option<String>,
) -> Result<bool, AppError> {
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
    app_state: tauri::State<'_, Arc<AppState>>,
    profile_id: String,
) -> Result<(), AppError> {
    info!("Hot-switching profile: {}", profile_id);

    // 1. 设置活跃 profile
    service::profile::set_active_profile(&app_state.app_data_dir, &profile_id)
        .map_err(AppError::from)?;

    // 2. 读取新 profile 配置
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

        let client = EnoteServerClient::new(server_config, secrets)
            .map_err(|e| AppError::code_with_args("SERVER_CONNECTION_FAILED", vec![e.to_string()]))?;

        // 替换 AppState
        {
            let mut db_guard = app_state.database_connection.write().await;
            *db_guard = None; // Server 后端不使用数据库连接
        }
        {
            let mut backend_guard = app_state.backend.write().await;
            *backend_guard = ProfileBackend::Server(client);
        }
        {
            let mut key_guard = app_state.encryption_key.write().await;
            *key_guard = None; // Server 后端的加密由服务端处理
        }
    } else {
        // ======== Database 后端路径（原有逻辑不变） ========

        // 3. 从 Keychain 获取数据库密码
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

        // 4. 创建新数据库连接
        let new_db =
            crate::config::database_connection_from_profile(&profile_config, db_password.as_deref())
                .await
                .map_err(|e| AppError::code_with_args("DB_CONNECTION_FAILED", vec![e.to_string()]))?;

        // 5. 运行数据库迁移
        use sea_orm_migration::MigratorTrait;
        crate::migration::Migrator::up(&new_db, None)
            .await
            .map_err(|e| AppError::code_with_args("DB_MIGRATION_FAILED", vec![e.to_string()]))?;

        // 6. 从 Keychain 获取加密密钥
        #[cfg(any(feature = "desktop", feature = "db-full"))]
        let new_encryption_key = if profile_config.security.content_encryption {
            service::keychain::get_encryption_key(&profile_id).map_err(AppError::from)?
        } else {
            None
        };
        #[cfg(not(any(feature = "desktop", feature = "db-full")))]
        let new_encryption_key: Option<String> = None;

        // 7. 替换 AppState 中的连接和相关状态
        {
            let mut db_guard = app_state.database_connection.write().await;
            *db_guard = Some(new_db);
        }
        {
            let mut backend_guard = app_state.backend.write().await;
            *backend_guard = ProfileBackend::Database;
        }
        {
            let mut key_guard = app_state.encryption_key.write().await;
            *key_guard = new_encryption_key;
        }
    }

    // 公共状态更新
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
