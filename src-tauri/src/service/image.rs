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
        std::fs::create_dir_all(&dir).context("创建图片存储目录失败")?;
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
    std::fs::write(&file_path, data).context("写入图片文件失败")?;

    info!("图片已保存: {}", file_path.display());

    Ok(file_path.to_string_lossy().to_string())
}

/// 删除图片文件
pub fn delete_image(path: &str) -> Result<()> {
    let path = Path::new(path);
    if path.exists() {
        std::fs::remove_file(path).context("删除图片文件失败")?;
    }
    Ok(())
}

/// 解析 data URI，返回 (扩展名, 二进制数据)
fn parse_data_uri(data_uri: &str) -> Result<(String, Vec<u8>)> {
    if let Some(rest) = data_uri.strip_prefix("data:") {
        // data:image/png;base64,iVBOR...
        if let Some((mime_part, base64_part)) = rest.split_once(";base64,") {
            let ext = mime_to_ext(mime_part);
            let bytes = STANDARD
                .decode(base64_part)
                .context("Base64 解码失败")?;
            return Ok((ext, bytes));
        }
    }

    // 纯 Base64 数据（无 data URI 前缀），默认 png
    let bytes = STANDARD
        .decode(data_uri)
        .context("Base64 解码失败")?;
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
