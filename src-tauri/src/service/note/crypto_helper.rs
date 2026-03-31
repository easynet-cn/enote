use crate::model::Note;
use crate::service::crypto;

/// 加密内容（如果提供了密钥）
pub(super) fn encrypt_content(content: &str, encryption_key: Option<&str>) -> anyhow::Result<String> {
    match encryption_key {
        Some(key) if !key.is_empty() => crypto::encrypt(content, key),
        _ => Ok(content.to_string()),
    }
}

/// 解密内容（如果内容已加密且提供了密钥）
///
/// 解密失败时返回占位文本，避免将加密密文泄漏给前端
pub(super) fn decrypt_content(content: &str, encryption_key: Option<&str>) -> String {
    match encryption_key {
        Some(key) if !key.is_empty() && crypto::is_encrypted(content) => {
            crypto::decrypt(content, key).unwrap_or_else(|e| {
                tracing::warn!("Failed to decrypt note content: {}", e);
                "[内容无法解密]".to_string()
            })
        }
        _ => content.to_string(),
    }
}

/// 解密 Note 的内容字段
pub(super) fn decrypt_note(note: &mut Note, encryption_key: Option<&str>) {
    note.content = decrypt_content(&note.content, encryption_key);
}
