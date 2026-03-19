//! 笔记加密/解密服务
//!
//! 使用 AES-256-GCM 加密笔记内容
//! 密钥通过 Argon2id 从用户密码派生

use aes_gcm::{
    Aes256Gcm, KeyInit, Nonce,
    aead::Aead,
};
use anyhow::{Context, Result};
use argon2::Argon2;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::RngCore;

/// 加密前缀标识（用于检测内容是否已加密）
const ENCRYPTED_PREFIX: &str = "ENOTE_ENC_V2:";

/// 从密码派生 AES-256 密钥（Argon2id + 随机盐）
fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow::anyhow!("Argon2 密钥派生失败: {}", e))?;
    Ok(key)
}

/// 加密文本内容
///
/// 返回格式：`ENOTE_ENC_V2:<base64(salt + nonce + ciphertext)>`
/// salt: 16 bytes, nonce: 12 bytes
pub fn encrypt(content: &str, password: &str) -> Result<String> {
    // 生成随机盐 (128 bits)
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);

    let key = derive_key(password, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .context("创建加密器失败")?;

    // 生成随机 nonce (96 bits)
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, content.as_bytes())
        .map_err(|e| anyhow::anyhow!("加密失败: {}", e))?;

    // 组合 salt + nonce + ciphertext
    let mut combined = Vec::with_capacity(16 + 12 + ciphertext.len());
    combined.extend_from_slice(&salt);
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    Ok(format!("{}{}", ENCRYPTED_PREFIX, STANDARD.encode(&combined)))
}

/// 解密文本内容
pub fn decrypt(encrypted: &str, password: &str) -> Result<String> {
    let data = encrypted
        .strip_prefix(ENCRYPTED_PREFIX)
        .context("不是有效的加密内容")?;

    let combined = STANDARD
        .decode(data)
        .context("Base64 解码失败")?;

    // salt(16) + nonce(12) + ciphertext(>=1)
    if combined.len() < 29 {
        anyhow::bail!("加密数据格式无效");
    }

    let salt = &combined[..16];
    let nonce_bytes = &combined[16..28];
    let ciphertext = &combined[28..];

    let key = derive_key(password, salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .context("创建解密器失败")?;

    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow::anyhow!("解密失败：密码错误"))?;

    String::from_utf8(plaintext).context("解密后的内容不是有效的 UTF-8 文本")
}

/// 检测内容是否已加密
pub fn is_encrypted(content: &str) -> bool {
    content.starts_with(ENCRYPTED_PREFIX)
}
