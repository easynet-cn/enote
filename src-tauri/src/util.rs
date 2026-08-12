//! 工具函数模块
//!
//! 提供不依赖外部 crate 的轻量工具函数，
//! 替代 `dirs` 和 `hex` 的功能。

use std::path::PathBuf;

/// 获取用户数据目录（替代 `dirs::data_dir`）
///
/// 平台行为与 `dirs::data_dir()` 完全一致：
/// - macOS: `~/Library/Application Support`
/// - Windows: `%APPDATA%`（即 `C:\Users\<user>\AppData\Roaming`）
/// - Linux/Unix: `$XDG_DATA_HOME`，未设置时回退到 `~/.local/share`
pub fn data_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        std::env::home_dir().map(|h| h.join("Library").join("Application Support"))
    }
    #[cfg(target_os = "windows")]
    {
        std::env::var_os("APPDATA").map(PathBuf::from)
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::home_dir().map(|h| h.join(".local").join("share")))
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", unix)))]
    {
        None
    }
}

/// 将字节数组编码为十六进制字符串（替代 `hex::encode`）
///
/// 示例: `[0xde, 0xad]` → `"dead"`
pub fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}
