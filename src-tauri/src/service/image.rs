//! 图片存储服务
//!
//! 将图片保存为本地文件，替代 Base64 内联存储。
//! 使用 Tauri asset protocol 在前端访问本地图片文件。

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use tracing::info;
use uuid::Uuid;

/// 获取图片存储目录
pub fn images_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("images")
}

/// 确保图片存储目录存在
fn ensure_images_dir(app_data_dir: &Path) -> Result<PathBuf> {
    let dir = images_dir(app_data_dir);
    if !dir.exists() {
        std::fs::create_dir_all(&dir).context("Failed to create images directory")?;
    }
    Ok(dir)
}

/// 保存图片到本地文件
///
/// # 参数
/// - `app_data_dir`: 应用数据目录
/// - `base64_data`: Base64 编码的图片数据（可带 data URI 前缀）
///
/// # 返回
/// 图片文件的绝对路径
pub fn save_image(app_data_dir: &Path, base64_data: &str) -> Result<String> {
    let dir = ensure_images_dir(app_data_dir)?;

    // 解析 data URI（如 "data:image/png;base64,iVBOR..."）
    let (ext, data) = parse_data_uri(base64_data)?;

    // 生成唯一文件名
    let filename = format!("{}.{}", Uuid::new_v4(), ext);
    let file_path = dir.join(&filename);

    // 写入文件
    std::fs::write(&file_path, data).context("Failed to write image file")?;

    info!("Image saved: {}", file_path.display());

    Ok(file_path.to_string_lossy().to_string())
}

/// 删除图片文件
///
/// 安全校验：仅允许删除 images 目录内的文件，防止路径穿越攻击
pub fn delete_image(app_data_dir: &Path, path: &str) -> Result<()> {
    let target = Path::new(path);
    let images = images_dir(app_data_dir);

    // 规范化路径并检查是否在 images 目录内
    let canonical_target = target
        .canonicalize()
        .context("Image file not found or path invalid")?;
    // images 目录可能不存在，如果不存在则说明没有图片可删
    let canonical_images = match images.canonicalize() {
        Ok(p) => p,
        Err(_) => return Ok(()), // images 目录不存在，无需删除
    };

    if !canonical_target.starts_with(&canonical_images) {
        anyhow::bail!("Path traversal detected: image path must be within images directory");
    }

    if canonical_target.exists() {
        std::fs::remove_file(&canonical_target).context("Failed to delete image file")?;
    }
    Ok(())
}

/// 解析 data URI，返回 (扩展名, 二进制数据)
fn parse_data_uri(data_uri: &str) -> Result<(String, Vec<u8>)> {
    if let Some(rest) = data_uri.strip_prefix("data:") {
        // data:image/png;base64,iVBOR...
        if let Some((mime_part, base64_part)) = rest.split_once(";base64,") {
            let ext = mime_to_ext(mime_part);
            let bytes = STANDARD.decode(base64_part).context("Failed to decode Base64")?;
            return Ok((ext, bytes));
        }
    }

    // 纯 Base64 数据（无 data URI 前缀），默认 png
    let bytes = STANDARD.decode(data_uri).context("Failed to decode Base64")?;
    Ok(("png".to_string(), bytes))
}

/// MIME 类型转文件扩展名
fn mime_to_ext(mime: &str) -> String {
    match mime {
        "image/png" => "png",
        "image/jpeg" | "image/jpg" => "jpg",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "image/svg+xml" => "svg",
        "image/bmp" => "bmp",
        "image/x-icon" | "image/vnd.microsoft.icon" => "ico",
        _ => "png",
    }
    .to_string()
}
