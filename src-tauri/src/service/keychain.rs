//! 系统 Keychain 服务模块
//!
//! 桌面端使用 OS 原生安全存储（macOS Keychain / Windows Credential Store / Linux Secret Service）
//! 移动端提供 stub 实现（返回 None / Ok）

use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::RngCore;

/// 生成 256-bit 随机加密密钥并返回 Base64 编码
pub fn generate_encryption_key() -> String {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    STANDARD.encode(key)
}

// ============================================================================
// 桌面端：使用 keyring crate 访问 OS Keychain
// ============================================================================

#[cfg(any(feature = "desktop", feature = "db-full"))]
mod platform {
    use anyhow::{Context, Result};

    const SERVICE_NAME: &str = "net.easycloudcn.enote";

    pub fn set_secret(key: &str, value: &str) -> Result<()> {
        let entry = keyring::Entry::new(SERVICE_NAME, key).context("Failed to create Keychain entry")?;
        entry.set_password(value).context("Failed to write to Keychain")?;
        Ok(())
    }

    pub fn get_secret(key: &str) -> Result<Option<String>> {
        let entry = keyring::Entry::new(SERVICE_NAME, key).context("Failed to create Keychain entry")?;
        match entry.get_password() {
            Ok(value) => Ok(Some(value)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("Failed to read Keychain: {}", e)),
        }
    }

    pub fn delete_secret(key: &str) -> Result<()> {
        let entry = keyring::Entry::new(SERVICE_NAME, key).context("Failed to create Keychain entry")?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("Failed to delete Keychain entry: {}", e)),
        }
    }
}

// ============================================================================
// 移动端：Stub 实现（不访问 Keychain）
// ============================================================================

#[cfg(not(any(feature = "desktop", feature = "db-full")))]
mod platform {
    use anyhow::Result;

    pub fn set_secret(_key: &str, _value: &str) -> Result<()> {
        Ok(())
    }

    pub fn get_secret(_key: &str) -> Result<Option<String>> {
        Ok(None)
    }

    pub fn delete_secret(_key: &str) -> Result<()> {
        Ok(())
    }
}

// ============================================================================
// 公共 API（桌面/移动端统一接口）
// ============================================================================

pub fn set_secret(key: &str, value: &str) -> Result<()> {
    platform::set_secret(key, value)
}

pub fn get_secret(key: &str) -> Result<Option<String>> {
    platform::get_secret(key)
}

pub fn delete_secret(key: &str) -> Result<()> {
    platform::delete_secret(key)
}

fn encryption_key_name(profile_name: &str) -> String {
    format!("{}.encryption_key", profile_name)
}

fn db_password_key_name(profile_name: &str) -> String {
    format!("{}.db_password", profile_name)
}

pub fn set_encryption_key(profile_name: &str, key: &str) -> Result<()> {
    set_secret(&encryption_key_name(profile_name), key)
}

pub fn get_encryption_key(profile_name: &str) -> Result<Option<String>> {
    get_secret(&encryption_key_name(profile_name))
}

pub fn set_db_password(profile_name: &str, password: &str) -> Result<()> {
    set_secret(&db_password_key_name(profile_name), password)
}

pub fn get_db_password(profile_name: &str) -> Result<Option<String>> {
    get_secret(&db_password_key_name(profile_name))
}

pub fn delete_profile_secrets(profile_name: &str) -> Result<()> {
    delete_secret(&encryption_key_name(profile_name))?;
    delete_secret(&db_password_key_name(profile_name))?;
    Ok(())
}
