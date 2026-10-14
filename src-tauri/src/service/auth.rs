//! 锁屏认证服务模块
//!
//! 提供应用锁屏密码的设置、验证和清除功能
//! 使用 Argon2id 进行密码哈希，防止暴力破解

use std::collections::HashMap;

use anyhow::{Context, Result};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use sea_orm::DatabaseConnection;

use super::settings;

const KEY_LOCK_PASSWORD_HASH: &str = "lockPasswordHash";
const KEY_LOCK_MODE: &str = "lockMode";

/// 设置锁屏密码
///
/// 使用 Argon2id 哈希密码并存入 settings 表
pub async fn set_password(db: &DatabaseConnection, password: &str) -> Result<()> {
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes())
        .map_err(|e| {
            anyhow::Error::from(crate::error::AppError::code_with_args(
                "PASSWORD_HASH_FAILED",
                vec![e.to_string()],
            ))
        })?
        .to_string();

    let mut map = HashMap::new();
    map.insert(KEY_LOCK_PASSWORD_HASH.to_string(), hash);
    settings::save(db, map)
        .await
        .context("Failed to save password hash")
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

    let parsed = PasswordHash::new(hash_str).map_err(|e| {
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
    settings::save(db, map)
        .await
        .context("Failed to clear password")
}

// ============================================================================
// 文件存储版（Git 后端）
// ============================================================================

/// 设置锁屏密码（文件存储版，Git 后端）
pub fn set_password_file(
    app_data_dir: &std::path::Path,
    profile_id: &str,
    password: &str,
) -> Result<()> {
    let hash = hash_password(password)?;
    let mut map = HashMap::new();
    map.insert(KEY_LOCK_PASSWORD_HASH.to_string(), hash);
    crate::service::settings_file::save(app_data_dir, profile_id, &map)
}

/// 验证锁屏密码（文件存储版，Git 后端）
pub fn verify_password_file(
    app_data_dir: &std::path::Path,
    profile_id: &str,
    password: &str,
) -> Result<bool> {
    let all = crate::service::settings_file::get_all(app_data_dir, profile_id)?;
    let Some(hash_str) = all.get(KEY_LOCK_PASSWORD_HASH) else {
        return Ok(false);
    };
    if hash_str.is_empty() {
        return Ok(false);
    }
    verify_hash(password, hash_str)
}

/// 清除锁屏密码（文件存储版，Git 后端）
pub fn clear_password_file(app_data_dir: &std::path::Path, profile_id: &str) -> Result<()> {
    let mut map = HashMap::new();
    map.insert(KEY_LOCK_PASSWORD_HASH.to_string(), String::new());
    map.insert(KEY_LOCK_MODE.to_string(), "none".to_string());
    crate::service::settings_file::save(app_data_dir, profile_id, &map)
}

/// 生成 Argon2id 密码哈希
fn hash_password(password: &str) -> Result<String> {
    Argon2::default()
        .hash_password(password.as_bytes())
        .map_err(|e| {
            anyhow::Error::from(crate::error::AppError::code_with_args(
                "PASSWORD_HASH_FAILED",
                vec![e.to_string()],
            ))
        })
        .map(|h| h.to_string())
}

/// 验证密码与哈希是否匹配
fn verify_hash(password: &str, hash_str: &str) -> Result<bool> {
    let parsed = PasswordHash::new(hash_str).map_err(|e| {
        anyhow::Error::from(crate::error::AppError::code_with_args(
            "PASSWORD_HASH_INVALID",
            vec![e.to_string()],
        ))
    })?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn temp_dir(name: &str) -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "enote-auth-test-{}-{}-{n}",
            std::process::id(),
            name
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn file_password_set_verify_clear_roundtrip() {
        let dir = temp_dir("roundtrip");

        // 初始状态：未设置密码
        assert!(!verify_password_file(&dir, "p1", "any").unwrap());

        // 设置密码后验证通过
        set_password_file(&dir, "p1", "correct_pw").unwrap();
        assert!(verify_password_file(&dir, "p1", "correct_pw").unwrap());

        // 错误密码验证失败
        assert!(!verify_password_file(&dir, "p1", "wrong_pw").unwrap());

        // 清除后验证失败
        clear_password_file(&dir, "p1").unwrap();
        assert!(!verify_password_file(&dir, "p1", "correct_pw").unwrap());
    }

    #[test]
    fn file_password_hash_format_is_argon2() {
        let dir = temp_dir("hash");
        set_password_file(&dir, "p1", "test").unwrap();

        let all = crate::service::settings_file::get_all(&dir, "p1").unwrap();
        let hash = all.get("lockPasswordHash").unwrap();
        assert!(hash.starts_with("$argon2id$"));
    }

    #[test]
    fn file_password_clear_resets_mode() {
        let dir = temp_dir("clear");
        set_password_file(&dir, "p1", "test").unwrap();
        clear_password_file(&dir, "p1").unwrap();

        let all = crate::service::settings_file::get_all(&dir, "p1").unwrap();
        assert_eq!(all.get("lockPasswordHash"), Some(&String::new()));
        assert_eq!(all.get("lockMode"), Some(&"none".to_string()));
    }

    #[test]
    fn file_password_profile_isolation() {
        let dir = temp_dir("isolation");
        set_password_file(&dir, "p1", "pw1").unwrap();
        set_password_file(&dir, "p2", "pw2").unwrap();

        assert!(verify_password_file(&dir, "p1", "pw1").unwrap());
        assert!(verify_password_file(&dir, "p2", "pw2").unwrap());
        assert!(!verify_password_file(&dir, "p1", "pw2").unwrap());
        assert!(!verify_password_file(&dir, "p2", "pw1").unwrap());
    }
}
