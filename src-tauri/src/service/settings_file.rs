//! 应用设置的文件存储（Git 后端专用）
//!
//! Git 后端没有数据库连接，设置写入 `{app_data_dir}/settings/{profile_id}.yml`，
//! 采用 YAML mapping 格式，与数据库 [`super::settings`] 行为一致。
//! 锁屏密码哈希也作为 settings 的 key 存储，自动跟随迁移。

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

/// 设置文件目录：`{app_data_dir}/settings/`
fn settings_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("settings")
}

/// 单个 profile 的设置文件路径
fn settings_file(app_data_dir: &Path, profile_id: &str) -> PathBuf {
    settings_dir(app_data_dir).join(format!("{}.yml", profile_id))
}

/// 读取全部设置（文件不存在返回空 map）
pub fn get_all(app_data_dir: &Path, profile_id: &str) -> Result<HashMap<String, String>> {
    let path = settings_file(app_data_dir, profile_id);
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let content =
        std::fs::read_to_string(&path).with_context(|| format!("读取设置文件失败: {:?}", path))?;
    let settings: HashMap<String, String> =
        yaml_serde::from_str(&content).with_context(|| format!("解析设置文件失败: {:?}", path))?;
    Ok(settings)
}

/// 保存设置（upsert，全量合并写回，原子替换）
pub fn save(
    app_data_dir: &Path,
    profile_id: &str,
    updates: &HashMap<String, String>,
) -> Result<()> {
    let path = settings_file(app_data_dir, profile_id);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // 合并已有设置（upsert 语义）
    let mut current = if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        yaml_serde::from_str::<HashMap<String, String>>(&content).unwrap_or_default()
    } else {
        HashMap::new()
    };
    for (k, v) in updates {
        current.insert(k.clone(), v.clone());
    }

    // 原子写入：临时文件 + rename
    let tmp = path.with_extension("yml.tmp");
    let content = yaml_serde::to_string(&current)?;
    std::fs::write(&tmp, &content)?;
    std::fs::rename(&tmp, &path)?;

    Ok(())
}

/// 从数据库迁移设置到文件（仅 Database 后端可迁移）
///
/// 兼容旧版本：设置原本存于 settings 表，首次读取时自动迁移到文件。
pub async fn migrate_from_db(
    app_data_dir: &Path,
    profile_id: &str,
    db: &sea_orm::DatabaseConnection,
) -> Result<HashMap<String, String>> {
    let settings = super::settings::get_all(db).await?;
    if !settings.is_empty() {
        let _ = save(app_data_dir, profile_id, &settings);
    }
    Ok(settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn temp_dir(name: &str) -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "enote-settings-file-test-{}-{}-{n}",
            std::process::id(),
            name
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn save_and_get_roundtrip() {
        let dir = temp_dir("basic");
        let mut settings = HashMap::new();
        settings.insert("theme".to_string(), "dark".to_string());
        settings.insert("language".to_string(), "zh-CN".to_string());
        save(&dir, "profile1", &settings).unwrap();

        let result = get_all(&dir, "profile1").unwrap();
        assert_eq!(result.get("theme"), Some(&"dark".to_string()));
        assert_eq!(result.get("language"), Some(&"zh-CN".to_string()));
    }

    #[test]
    fn save_upsert_merges() {
        let dir = temp_dir("upsert");
        let mut s1 = HashMap::new();
        s1.insert("theme".to_string(), "dark".to_string());
        s1.insert("fontSize".to_string(), "14".to_string());
        save(&dir, "p1", &s1).unwrap();

        // 只更新一个 key，另一个应保留
        let mut s2 = HashMap::new();
        s2.insert("theme".to_string(), "light".to_string());
        save(&dir, "p1", &s2).unwrap();

        let result = get_all(&dir, "p1").unwrap();
        assert_eq!(result.get("theme"), Some(&"light".to_string()));
        assert_eq!(result.get("fontSize"), Some(&"14".to_string()));
    }

    #[test]
    fn profiles_are_isolated() {
        let dir = temp_dir("isolated");
        let mut s1 = HashMap::new();
        s1.insert("theme".to_string(), "dark".to_string());
        save(&dir, "p1", &s1).unwrap();

        let mut s2 = HashMap::new();
        s2.insert("theme".to_string(), "light".to_string());
        save(&dir, "p2", &s2).unwrap();

        assert_eq!(
            get_all(&dir, "p1").unwrap().get("theme"),
            Some(&"dark".to_string())
        );
        assert_eq!(
            get_all(&dir, "p2").unwrap().get("theme"),
            Some(&"light".to_string())
        );
    }

    #[test]
    fn missing_file_returns_empty() {
        let dir = temp_dir("missing");
        let result = get_all(&dir, "nonexistent").unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn lock_password_hash_stored_via_settings() {
        // 验证锁屏密码哈希作为 settings key 存储正常
        let dir = temp_dir("lockpw");
        let mut settings = HashMap::new();
        settings.insert(
            "lockPasswordHash".to_string(),
            "$argon2id$v=19...".to_string(),
        );
        settings.insert("lockMode".to_string(), "password".to_string());
        save(&dir, "p1", &settings).unwrap();

        let result = get_all(&dir, "p1").unwrap();
        assert!(
            result
                .get("lockPasswordHash")
                .unwrap()
                .starts_with("$argon2id$")
        );
        assert_eq!(result.get("lockMode"), Some(&"password".to_string()));
    }
}
