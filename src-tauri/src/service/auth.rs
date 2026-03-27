//! 锁屏认证服务模块
//!
//! 提供应用锁屏密码的设置、验证和清除功能
//! 使用 Argon2id 进行密码哈希，防止暴力破解

use std::collections::HashMap;

use anyhow::{Context, Result};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use sea_orm::DatabaseConnection;

use super::settings;

const KEY_LOCK_PASSWORD_HASH: &str = "lockPasswordHash";
const KEY_LOCK_MODE: &str = "lockMode";

/// 设置锁屏密码
///
/// 使用 Argon2id 哈希密码并存入 settings 表
pub async fn set_password(db: &DatabaseConnection, password: &str) -> Result<()> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            anyhow::Error::from(crate::error::AppError::code_with_args(
                "PASSWORD_HASH_FAILED",
                vec![e.to_string()],
            ))
        })?
        .to_string();

    let mut map = HashMap::new();
    map.insert(KEY_LOCK_PASSWORD_HASH.to_string(), hash);
    settings::save(db, map).await.context("Failed to save password hash")
}

/// 验证锁屏密码
///
/// 返回 true 表示密码正确，false 表示密码错误
pub async fn verify_password(db: &DatabaseConnection, password: &str) -> Result<bool> {
    let all = settings::get_all(db).await?;
    let Some(hash_str) = all.get(KEY_LOCK_PASSWORD_HASH) else {
        return Ok(false);
    };
    if hash_str.is_empty() {
        return Ok(false);
    }

    let parsed =
        PasswordHash::new(hash_str).map_err(|e| {
            anyhow::Error::from(crate::error::AppError::code_with_args(
                "PASSWORD_HASH_INVALID",
                vec![e.to_string()],
            ))
        })?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

/// 清除锁屏密码并将锁定模式设为 none
pub async fn clear_password(db: &DatabaseConnection) -> Result<()> {
    let mut map = HashMap::new();
    map.insert(KEY_LOCK_PASSWORD_HASH.to_string(), String::new());
    map.insert(KEY_LOCK_MODE.to_string(), "none".to_string());
    settings::save(db, map).await.context("Failed to clear password")
}
