//! 系统 Keychain 服务模块
//!
//! 使用 OS 原生安全存储（macOS Keychain / Windows Credential Store / Linux Secret Service）
//! 存储敏感数据，如数据库密码和内容加密密钥
//!
//! Keychain 条目命名规则：
//! - service: 使用 Tauri identifier (net.easycloudcn.enote)
//! - user: `{profile_name}.{key_name}` (如 "local-sqlite.encryption_key")

use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::RngCore;

/// Tauri 应用 identifier，用作 Keychain service name
const SERVICE_NAME: &str = "net.easycloudcn.enote";

/// 存储字符串到 Keychain
pub fn set_secret(key: &str, value: &str) -> Result<()> {
    let entry = keyring::Entry::new(SERVICE_NAME, key)
        .context("创建 Keychain 条目失败")?;
    entry
        .set_password(value)
        .context("写入 Keychain 失败")?;
    Ok(())
}

/// 从 Keychain 读取字符串
pub fn get_secret(key: &str) -> Result<Option<String>> {
    let entry = keyring::Entry::new(SERVICE_NAME, key)
        .context("创建 Keychain 条目失败")?;
    match entry.get_password() {
        Ok(value) => Ok(Some(value)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(anyhow::anyhow!("读取 Keychain 失败: {}", e)),
    }
}

/// 从 Keychain 删除条目
pub fn delete_secret(key: &str) -> Result<()> {
    let entry = keyring::Entry::new(SERVICE_NAME, key)
        .context("创建 Keychain 条目失败")?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()), // 不存在也算成功
        Err(e) => Err(anyhow::anyhow!("删除 Keychain 条目失败: {}", e)),
    }
}

/// 生成 256-bit 随机加密密钥并返回 Base64 编码
pub fn generate_encryption_key() -> String {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    STANDARD.encode(key)
}

// ============================================================================
// Profile 相关的便捷方法
// ============================================================================

/// 获取 profile 的加密密钥 Keychain key
fn encryption_key_name(profile_name: &str) -> String {
    format!("{}.encryption_key", profile_name)
}

/// 获取 profile 的数据库密码 Keychain key
fn db_password_key_name(profile_name: &str) -> String {
    format!("{}.db_password", profile_name)
}

/// 存储 profile 的内容加密密钥
pub fn set_encryption_key(profile_name: &str, key: &str) -> Result<()> {
    set_secret(&encryption_key_name(profile_name), key)
}

/// 读取 profile 的内容加密密钥
pub fn get_encryption_key(profile_name: &str) -> Result<Option<String>> {
    get_secret(&encryption_key_name(profile_name))
}

/// 存储 profile 的数据库密码
pub fn set_db_password(profile_name: &str, password: &str) -> Result<()> {
    set_secret(&db_password_key_name(profile_name), password)
}

/// 读取 profile 的数据库密码
pub fn get_db_password(profile_name: &str) -> Result<Option<String>> {
    get_secret(&db_password_key_name(profile_name))
}

/// 删除 profile 的所有 Keychain 条目
pub fn delete_profile_secrets(profile_name: &str) -> Result<()> {
    delete_secret(&encryption_key_name(profile_name))?;
    delete_secret(&db_password_key_name(profile_name))?;
    Ok(())
}
