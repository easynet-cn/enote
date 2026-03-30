//! 应用日志服务模块
//!
//! 提供操作审计日志的 CRUD、搜索、脱敏功能。
//! - 加密笔记内容不记录
//! - 敏感信息自动脱敏（密码、密钥等）

use chrono::Local;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use tracing::warn;

use crate::{
    entity,
    model::{AppLog, AppLogSearchParam, PageResult},
    service::crypto,
};

// ============================================================================
// 脱敏工具
// ============================================================================

/// 脱敏敏感字段值
///
/// 保留前2个和后2个字符，中间用 **** 替代
pub fn sanitize_value(value: &str) -> String {
    let len = value.chars().count();
    if len <= 4 {
        "****".to_string()
    } else {
        let chars: Vec<char> = value.chars().collect();
        let prefix: String = chars[..2].iter().collect();
        let suffix: String = chars[len - 2..].iter().collect();
        format!("{}****{}", prefix, suffix)
    }
}

/// 检查内容是否已加密，加密内容不记录到日志
pub fn should_skip_content(content: &str) -> bool {
    crypto::is_encrypted(content)
}

/// 脱敏数据库连接字符串中的密码
///
/// 匹配 `://user:password@host` 格式，将密码部分替换为 ****
/// 使用最后一个 `@`（在路径 `/` 之前）作为凭据分隔符，正确处理密码中含 `@` 的情况
fn sanitize_db_url(message: &str) -> String {
    let mut result = message.to_string();
    for prefix in ["mysql://", "postgres://", "postgresql://", "sqlite://"] {
        if let Some(proto_pos) = result.find(prefix) {
            let after_proto = proto_pos + prefix.len();
            let rest = &result[after_proto..];

            // 确定 authority 部分的范围（到第一个 / 或字符串结尾）
            let authority_end = rest.find('/').unwrap_or(rest.len());
            let authority = &rest[..authority_end];

            // 使用最后一个 @ 作为凭据与主机的分隔符
            if let Some(at_in_auth) = authority.rfind('@') {
                let credentials = &authority[..at_in_auth];
                // 查找 user:password 中的第一个 :
                if let Some(colon) = credentials.find(':') {
                    let pwd_start = after_proto + colon + 1;
                    let pwd_end = after_proto + at_in_auth;
                    if pwd_end > pwd_start {
                        result = format!(
                            "{}****{}",
                            &result[..pwd_start],
                            &result[pwd_end..]
                        );
                    }
                }
            }
        }
    }
    result
}

/// 脱敏日志消息中可能存在的敏感信息
///
/// 对 password、key、secret、token 等关键词后的值进行掩码，
/// 同时脱敏数据库连接字符串中的密码
pub fn sanitize_message(message: &str) -> String {
    let sensitive_patterns = ["password", "密码", "key", "密钥", "secret", "token"];
    let mut result = sanitize_db_url(message);

    for pattern in &sensitive_patterns {
        // 查找 pattern=xxx 或 pattern: xxx 或 pattern xxx 形式
        if let Some(pos) = result.to_lowercase().find(&pattern.to_lowercase()) {
            let after = pos + pattern.len();
            if after < result.len() {
                let rest = &result[after..];
                // 跳过分隔符（=、:、空格）
                let skip = rest
                    .chars()
                    .take_while(|c| *c == '=' || *c == ':' || *c == ' ')
                    .count();
                let value_start = after + skip;
                if value_start < result.len() {
                    // 获取值（到下一个空格或结尾）
                    let value_end = result[value_start..]
                        .find(|c: char| c.is_whitespace() || c == ',' || c == ';')
                        .map(|p| value_start + p)
                        .unwrap_or(result.len());
                    if value_end > value_start {
                        let value = &result[value_start..value_end];
                        let sanitized = sanitize_value(value);
                        result = format!(
                            "{}{}{}",
                            &result[..value_start],
                            sanitized,
                            &result[value_end..]
                        );
                    }
                }
            }
        }
    }

    result
}

// ============================================================================
// CRUD 操作
// ============================================================================

/// 记录一条应用日志
///
/// 会自动对 message 和 detail 进行脱敏处理。
/// 写入失败不会向调用方传播错误，仅通过 tracing 记录。
pub async fn log_action(
    db: &DatabaseConnection,
    module: &str,
    action: &str,
    target_id: Option<&str>,
    target_name: Option<&str>,
    message: &str,
    detail: Option<&str>,
) -> anyhow::Result<()> {
    let now = Local::now().naive_local();

    let sanitized_message = sanitize_message(message);
    let sanitized_detail = detail.map(sanitize_message);

    let active_model = entity::app_log::ActiveModel {
        id: NotSet,
        level: Set("INFO".to_string()),
        module: Set(module.to_string()),
        action: Set(action.to_string()),
        target_id: Set(target_id.map(|s| s.to_string())),
        target_name: Set(target_name.map(|s| s.to_string())),
        message: Set(sanitized_message),
        detail: Set(sanitized_detail),
        create_time: Set(now),
    };

    if let Err(e) = active_model.insert(db).await {
        warn!("Failed to write app log [module={}, action={}]: {:#}", module, action, e);
        return Err(e.into());
    }
    Ok(())
}

/// 记录一条错误日志
pub async fn log_error(
    db: &DatabaseConnection,
    module: &str,
    action: &str,
    target_id: Option<&str>,
    target_name: Option<&str>,
    message: &str,
    detail: Option<&str>,
) -> anyhow::Result<()> {
    let now = Local::now().naive_local();

    let sanitized_message = sanitize_message(message);
    let sanitized_detail = detail.map(sanitize_message);

    let active_model = entity::app_log::ActiveModel {
        id: NotSet,
        level: Set("ERROR".to_string()),
        module: Set(module.to_string()),
        action: Set(action.to_string()),
        target_id: Set(target_id.map(|s| s.to_string())),
        target_name: Set(target_name.map(|s| s.to_string())),
        message: Set(sanitized_message),
        detail: Set(sanitized_detail),
        create_time: Set(now),
    };

    if let Err(e) = active_model.insert(db).await {
        warn!("Failed to write app error log [module={}, action={}]: {:#}", module, action, e);
        return Err(e.into());
    }
    Ok(())
}

/// 记录一条警告日志
pub async fn log_warn(
    db: &DatabaseConnection,
    module: &str,
    action: &str,
    message: &str,
) -> anyhow::Result<()> {
    let now = Local::now().naive_local();

    let active_model = entity::app_log::ActiveModel {
        id: NotSet,
        level: Set("WARN".to_string()),
        module: Set(module.to_string()),
        action: Set(action.to_string()),
        target_id: Set(None),
        target_name: Set(None),
        message: Set(sanitize_message(message)),
        detail: Set(None),
        create_time: Set(now),
    };

    active_model.insert(db).await?;
    Ok(())
}

/// 从前端记录日志（统一入口）
pub async fn log_from_frontend(
    db: &DatabaseConnection,
    log: &AppLog,
) -> anyhow::Result<()> {
    let now = Local::now().naive_local();

    let active_model = entity::app_log::ActiveModel {
        id: NotSet,
        level: Set(log.level.clone()),
        module: Set(log.module.clone()),
        action: Set(log.action.clone()),
        target_id: Set(log.target_id.clone()),
        target_name: Set(log.target_name.clone()),
        message: Set(sanitize_message(&log.message)),
        detail: Set(log.detail.as_deref().map(sanitize_message)),
        create_time: Set(now),
    };

    if let Err(e) = active_model.insert(db).await {
        warn!("Failed to write frontend log [module={}, action={}]: {:#}", log.module, log.action, e);
        return Err(e.into());
    }
    Ok(())
}

/// 分页搜索应用日志
pub async fn search_page(
    db: &DatabaseConnection,
    param: &AppLogSearchParam,
) -> anyhow::Result<PageResult<AppLog>> {
    let mut query = entity::app_log::Entity::find();

    // 按级别筛选
    if let Some(ref level) = param.level
        && !level.is_empty() {
            query = query.filter(entity::app_log::Column::Level.eq(level.as_str()));
        }

    // 按模块筛选
    if let Some(ref module) = param.module
        && !module.is_empty() {
            query = query.filter(entity::app_log::Column::Module.eq(module.as_str()));
        }

    // 按操作筛选
    if let Some(ref action) = param.action
        && !action.is_empty() {
            query = query.filter(entity::app_log::Column::Action.eq(action.as_str()));
        }

    // 关键词模糊搜索
    if !param.keyword.is_empty() {
        let keyword = format!("%{}%", param.keyword);
        query = query.filter(entity::app_log::Column::Message.like(&keyword));
    }

    // 时间范围
    if let Some(start) = param.start_time {
        query = query.filter(entity::app_log::Column::CreateTime.gte(start));
    }
    if let Some(end) = param.end_time {
        query = query.filter(entity::app_log::Column::CreateTime.lte(end));
    }

    // 按时间倒序
    let query = query.order_by_desc(entity::app_log::Column::CreateTime);

    // 查询总数
    let total = query.clone().count(db).await? as i64;

    // 分页查询
    let data: Vec<AppLog> = query
        .offset(param.page_param.start() as u64)
        .limit(param.page_param.page_size as u64)
        .all(db)
        .await?
        .into_iter()
        .map(AppLog::from)
        .collect();

    let mut result = PageResult::with_data(total, data);
    result.total_pages(param.page_param.page_size);
    Ok(result)
}

/// 根据 ID 删除日志
pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
    entity::app_log::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}

/// 批量删除日志
pub async fn delete_batch(db: &DatabaseConnection, ids: Vec<i64>) -> anyhow::Result<u64> {
    if ids.is_empty() {
        return Ok(0);
    }
    let result = entity::app_log::Entity::delete_many()
        .filter(entity::app_log::Column::Id.is_in(ids))
        .exec(db)
        .await?;
    Ok(result.rows_affected)
}

/// 删除指定时间之前的日志
pub async fn delete_before(
    db: &DatabaseConnection,
    before: chrono::NaiveDateTime,
) -> anyhow::Result<u64> {
    let result = entity::app_log::Entity::delete_many()
        .filter(entity::app_log::Column::CreateTime.lt(before))
        .exec(db)
        .await?;
    Ok(result.rows_affected)
}

/// 清空所有日志
pub async fn clear_all(db: &DatabaseConnection) -> anyhow::Result<u64> {
    let result = entity::app_log::Entity::delete_many().exec(db).await?;
    Ok(result.rows_affected)
}

/// 获取日志统计信息（各级别数量）
pub async fn get_stats(
    db: &DatabaseConnection,
) -> anyhow::Result<std::collections::HashMap<String, i64>> {
    use sea_orm::{FromQueryResult, Statement};
    use std::collections::HashMap;

    #[derive(Debug, FromQueryResult)]
    struct LevelCount {
        level: String,
        count: i64,
    }

    let backend = db.get_database_backend();
    let results: Vec<LevelCount> = LevelCount::find_by_statement(Statement::from_string(
        backend,
        "SELECT level, COUNT(*) as count FROM app_log GROUP BY level".to_string(),
    ))
    .all(db)
    .await?;

    let mut stats = HashMap::new();
    for item in results {
        stats.insert(item.level, item.count);
    }
    Ok(stats)
}

// ============================================================================
// 文件日志管理
// ============================================================================

use crate::model::LogFileInfo;
use std::path::Path;

/// 列出日志文件
pub fn list_log_files(log_dir: &Path) -> anyhow::Result<Vec<LogFileInfo>> {
    let mut files = Vec::new();

    if !log_dir.exists() {
        return Ok(files);
    }

    for entry in std::fs::read_dir(log_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            // 只列出 enote 相关日志文件
            if !name.starts_with("enote") {
                continue;
            }

            let metadata = entry.metadata()?;
            let modified = metadata
                .modified()
                .ok()
                .and_then(|t| {
                    let duration = t.duration_since(std::time::UNIX_EPOCH).ok()?;
                    chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                        .map(|dt| dt.naive_local())
                });

            let is_error = name.contains("error");

            files.push(LogFileInfo {
                name,
                size: metadata.len(),
                is_error,
                modified_time: modified,
            });
        }
    }

    // 按修改时间倒序
    files.sort_by(|a, b| b.modified_time.cmp(&a.modified_time));
    Ok(files)
}

/// 读取日志文件内容
pub fn read_log_file(log_dir: &Path, filename: &str) -> anyhow::Result<String> {
    // 安全检查：防止路径遍历
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        anyhow::bail!("Invalid log filename");
    }

    let path = log_dir.join(filename);
    if !path.exists() {
        anyhow::bail!("Log file not found: {}", filename);
    }

    // 限制读取大小（最多 5MB）
    let metadata = std::fs::metadata(&path)?;
    if metadata.len() > 5 * 1024 * 1024 {
        // 读取最后 5MB
        use std::io::{Read, Seek, SeekFrom};
        let mut file = std::fs::File::open(&path)?;
        let start = metadata.len() - 5 * 1024 * 1024;
        file.seek(SeekFrom::Start(start))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        // 跳过第一个不完整的行
        if let Some(pos) = content.find('\n') {
            content = content[pos + 1..].to_string();
        }
        Ok(format!("... (truncated, showing last 5MB) ...\n{}", content))
    } else {
        Ok(std::fs::read_to_string(&path)?)
    }
}

/// 删除日志文件
pub fn delete_log_files(log_dir: &Path, filenames: Vec<String>) -> anyhow::Result<u32> {
    let mut deleted = 0;
    for filename in &filenames {
        // 安全检查
        if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
            warn!("Skipping invalid log filename: {}", filename);
            continue;
        }
        let path = log_dir.join(filename);
        if path.exists() {
            std::fs::remove_file(&path)?;
            deleted += 1;
        }
    }
    Ok(deleted)
}

/// 清理指定天数之前的日志文件
pub fn cleanup_old_log_files(log_dir: &Path, retention_days: u32) -> anyhow::Result<u32> {
    if !log_dir.exists() {
        return Ok(0);
    }

    let cutoff = Local::now() - chrono::Duration::days(retention_days as i64);
    let cutoff_time = cutoff.naive_local();
    let mut deleted = 0;

    for entry in std::fs::read_dir(log_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            if !name.starts_with("enote") {
                continue;
            }

            if let Ok(metadata) = entry.metadata()
                && let Ok(modified) = metadata.modified() {
                    let duration = modified
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();
                    if let Some(dt) =
                        chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                        && dt.naive_local() < cutoff_time
                            && std::fs::remove_file(&path).is_ok() {
                                deleted += 1;
                            }
                }
        }
    }

    Ok(deleted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_value_short() {
        assert_eq!(sanitize_value("ab"), "****");
        assert_eq!(sanitize_value("abcd"), "****");
    }

    #[test]
    fn sanitize_value_long() {
        assert_eq!(sanitize_value("password123"), "pa****23");
    }

    #[test]
    fn sanitize_db_url_mysql() {
        let input = "mysql://root:secret123@localhost:3306/enote";
        let result = sanitize_db_url(input);
        assert_eq!(result, "mysql://root:****@localhost:3306/enote");
        assert!(!result.contains("secret123"));
    }

    #[test]
    fn sanitize_db_url_postgres() {
        let input = "postgres://admin:p@ssw0rd@db.example.com/enote";
        let result = sanitize_db_url(input);
        assert_eq!(result, "postgres://admin:****@db.example.com/enote");
    }

    #[test]
    fn sanitize_db_url_no_password() {
        let input = "sqlite:///path/to/db.sqlite";
        let result = sanitize_db_url(input);
        assert_eq!(result, input);
    }

    #[test]
    fn sanitize_message_with_password_keyword() {
        let input = "Connection failed, password=abc123 retrying";
        let result = sanitize_message(input);
        assert!(!result.contains("abc123"));
    }

    #[test]
    fn sanitize_message_with_db_url() {
        let input = "Connecting to mysql://user:mypass@host/db";
        let result = sanitize_message(input);
        assert!(!result.contains("mypass"));
        assert!(result.contains("****"));
    }

    #[test]
    fn sanitize_message_no_sensitive() {
        let input = "Normal log message, nothing special";
        let result = sanitize_message(input);
        assert_eq!(result, input);
    }
}
