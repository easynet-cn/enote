//! 笔记加密/解密服务
//!
//! 使用 AES-256-GCM 加密笔记内容
//! 密钥通过 Argon2id 从用户密码派生

use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
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
        .map_err(|e| anyhow::anyhow!("Argon2 key derivation failed: {}", e))?;
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
    let cipher = Aes256Gcm::new_from_slice(&key).context("Failed to create cipher")?;

    // 生成随机 nonce (96 bits)
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, content.as_bytes())
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    // 组合 salt + nonce + ciphertext
    let mut combined = Vec::with_capacity(16 + 12 + ciphertext.len());
    combined.extend_from_slice(&salt);
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    Ok(format!(
        "{}{}",
        ENCRYPTED_PREFIX,
        STANDARD.encode(&combined)
    ))
}

/// 解密文本内容
pub fn decrypt(encrypted: &str, password: &str) -> Result<String> {
    let data = encrypted
        .strip_prefix(ENCRYPTED_PREFIX)
        .context("Not a valid encrypted content")?;

    let combined = STANDARD.decode(data).context("Failed to decode Base64")?;

    // salt(16) + nonce(12) + ciphertext(>=1)
    if combined.len() < 29 {
        anyhow::bail!("Invalid encrypted data format");
    }

    let salt = &combined[..16];
    let nonce_bytes = &combined[16..28];
    let ciphertext = &combined[28..];

    let key = derive_key(password, salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key).context("Failed to create decipher")?;

    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow::anyhow!("Decryption failed: wrong password"))?;

    String::from_utf8(plaintext).context("Decrypted content is not valid UTF-8")
}

/// 检测内容是否已加密
pub fn is_encrypted(content: &str) -> bool {
    content.starts_with(ENCRYPTED_PREFIX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let content = "Hello, 世界! 🌍";
        let password = "test-password-123";
        let encrypted = encrypt(content, password).unwrap();
        let decrypted = decrypt(&encrypted, password).unwrap();
        assert_eq!(decrypted, content);
    }

    #[test]
    fn encrypted_content_has_prefix() {
        let encrypted = encrypt("test", "pwd").unwrap();
        assert!(encrypted.starts_with(ENCRYPTED_PREFIX));
        assert!(is_encrypted(&encrypted));
    }

    #[test]
    fn plain_text_not_detected_as_encrypted() {
        assert!(!is_encrypted("just plain text"));
        assert!(!is_encrypted(""));
        assert!(!is_encrypted("ENOTE_ENC_V1:old_format"));
    }

    #[test]
    fn wrong_password_fails_decrypt() {
        let encrypted = encrypt("secret", "correct-password").unwrap();
        let result = decrypt(&encrypted, "wrong-password");
        assert!(result.is_err());
    }

    #[test]
    fn each_encryption_produces_different_output() {
        let content = "same content";
        let password = "same-password";
        let enc1 = encrypt(content, password).unwrap();
        let enc2 = encrypt(content, password).unwrap();
        // 不同的随机 salt + nonce 产生不同密文
        assert_ne!(enc1, enc2);
        // 但都能正确解密
        assert_eq!(decrypt(&enc1, password).unwrap(), content);
        assert_eq!(decrypt(&enc2, password).unwrap(), content);
    }

    #[test]
    fn empty_content_roundtrip() {
        let encrypted = encrypt("", "pwd").unwrap();
        let decrypted = decrypt(&encrypted, "pwd").unwrap();
        assert_eq!(decrypted, "");
    }

    #[test]
    fn large_content_roundtrip() {
        let content = "x".repeat(1_000_000); // 1MB
        let encrypted = encrypt(&content, "pwd").unwrap();
        let decrypted = decrypt(&encrypted, "pwd").unwrap();
        assert_eq!(decrypted, content);
    }

    #[test]
    fn invalid_encrypted_data_fails() {
        // 无效前缀
        assert!(decrypt("not_encrypted", "pwd").is_err());
        // 正确前缀但无效 base64
        assert!(decrypt(&format!("{}!!invalid!!", ENCRYPTED_PREFIX), "pwd").is_err());
        // 正确前缀但数据太短
        assert!(decrypt(&format!("{}AAAA", ENCRYPTED_PREFIX), "pwd").is_err());
    }

    #[test]
    fn tampered_ciphertext_fails() {
        let mut encrypted = encrypt("secret", "pwd").unwrap();
        // 篡改密文最后一个字符
        let len = encrypted.len();
        let last = encrypted.as_bytes()[len - 1];
        let tampered = if last == b'A' { b'B' } else { b'A' };
        unsafe {
            encrypted.as_bytes_mut()[len - 1] = tampered;
        }
        assert!(decrypt(&encrypted, "pwd").is_err());
    }

    #[test]
    fn unicode_password_works() {
        let content = "test content";
        let password = "密码🔑パスワード";
        let encrypted = encrypt(content, password).unwrap();
        let decrypted = decrypt(&encrypted, password).unwrap();
        assert_eq!(decrypted, content);
    }
}
