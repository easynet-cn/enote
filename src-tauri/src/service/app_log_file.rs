//! 前端日志的文件存储（Git 后端专用）
//!
//! Git 后端没有数据库连接，前端日志写入 `{app_data_dir}/logs/frontend.jsonl`，
//! 每行一条 JSON（camelCase，与 [`AppLog`] 序列化格式一致）。
//! 查询、删除、清理与数据库实现（[`super::app_log`]）行为保持一致。

use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{Local, NaiveDateTime, Utc};

use crate::model::{AppLog, AppLogSearchParam, PageParam, PageResult};

/// 前端日志文件路径：`{app_data_dir}/logs/frontend.jsonl`
fn frontend_log_path(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("logs").join("frontend.jsonl")
}

/// 追加一条前端日志
///
/// ID 使用时间戳微秒值，create_time 使用本地时间，message/detail 脱敏，
/// 与数据库写入行为保持一致。
pub fn append_frontend_log(app_data_dir: &Path, log: &AppLog) -> Result<()> {
    let path = frontend_log_path(app_data_dir);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("创建日志目录失败: {:?}", parent))?;
    }

    let entry = AppLog {
        id: Utc::now().timestamp_micros(),
        level: log.level.clone(),
        module: log.module.clone(),
        action: log.action.clone(),
        target_id: log.target_id.clone(),
        target_name: log.target_name.clone(),
        message: super::app_log::sanitize_message(&log.message),
        detail: log.detail.as_deref().map(super::app_log::sanitize_message),
        create_time: Some(Local::now().naive_local()),
    };

    let line = serde_json::to_string(&entry)?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .with_context(|| format!("打开日志文件失败: {:?}", path))?;
    writeln!(file, "{}", line)?;
    Ok(())
}

/// 读取全部日志条目（跳过无法解析的行）
fn read_all(app_data_dir: &Path) -> Result<Vec<AppLog>> {
    let path = frontend_log_path(app_data_dir);
    if !path.exists() {
        return Ok(vec![]);
    }
    let file =
        std::fs::File::open(&path).with_context(|| format!("打开日志文件失败: {:?}", path))?;
    let reader = std::io::BufReader::new(file);
    let mut logs = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        if let Ok(log) = serde_json::from_str::<AppLog>(&line) {
            logs.push(log);
        }
    }
    Ok(logs)
}

/// 原子重写日志文件（临时文件 + rename），返回写入条数
fn rewrite(app_data_dir: &Path, logs: &[AppLog]) -> Result<()> {
    let path = frontend_log_path(app_data_dir);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("jsonl.tmp");
    {
        let mut file = std::fs::File::create(&tmp)
            .with_context(|| format!("创建临时日志文件失败: {:?}", tmp))?;
        for log in logs {
            writeln!(file, "{}", serde_json::to_string(log)?)?;
        }
        file.flush()?;
    }
    std::fs::rename(&tmp, &path)
        .with_context(|| format!("替换日志文件失败: {:?} -> {:?}", tmp, path))?;
    Ok(())
}

/// 分页搜索前端日志（level/module/action/keyword/时间范围过滤，按时间倒序）
pub fn search_page(app_data_dir: &Path, param: &AppLogSearchParam) -> Result<PageResult<AppLog>> {
    let mut logs = read_all(app_data_dir)?;

    // 按级别筛选
    if let Some(ref level) = param.level
        && !level.is_empty()
    {
        logs.retain(|l| l.level.eq_ignore_ascii_case(level));
    }
    // 按模块筛选
    if let Some(ref module) = param.module
        && !module.is_empty()
    {
        logs.retain(|l| l.module == *module);
    }
    // 按操作筛选
    if let Some(ref action) = param.action
        && !action.is_empty()
    {
        logs.retain(|l| l.action == *action);
    }
    // 关键词模糊匹配 message
    if !param.keyword.is_empty() {
        let kw = param.keyword.to_lowercase();
        logs.retain(|l| l.message.to_lowercase().contains(&kw));
    }
    // 时间范围
    if let Some(start) = param.start_time {
        logs.retain(|l| l.create_time.is_some_and(|t| t >= start));
    }
    if let Some(end) = param.end_time {
        logs.retain(|l| l.create_time.is_some_and(|t| t <= end));
    }
    // 时间倒序
    logs.sort_by(|a, b| b.create_time.cmp(&a.create_time));

    let total = logs.len() as i64;
    let page: PageParam = param.page_param.clone();
    let start = page.start().max(0) as usize;
    let data: Vec<AppLog> = logs
        .into_iter()
        .skip(start)
        .take(page.page_size.max(0) as usize)
        .collect();

    let mut result = PageResult::with_data(total, data);
    result.total_pages(page.page_size);
    Ok(result)
}

/// 删除单条日志
pub fn delete_by_id(app_data_dir: &Path, id: i64) -> Result<()> {
    let logs: Vec<AppLog> = read_all(app_data_dir)?
        .into_iter()
        .filter(|l| l.id != id)
        .collect();
    rewrite(app_data_dir, &logs)?;
    Ok(())
}

/// 批量删除日志，返回删除条数
pub fn delete_batch(app_data_dir: &Path, ids: &[i64]) -> Result<u64> {
    let id_set: std::collections::HashSet<i64> = ids.iter().copied().collect();
    let mut removed = 0u64;
    let mut kept = Vec::new();
    for log in read_all(app_data_dir)? {
        if id_set.contains(&log.id) {
            removed += 1;
        } else {
            kept.push(log);
        }
    }
    rewrite(app_data_dir, &kept)?;
    Ok(removed)
}

/// 清空全部日志，返回删除条数
pub fn clear_all(app_data_dir: &Path) -> Result<u64> {
    let count = read_all(app_data_dir)?.len() as u64;
    let path = frontend_log_path(app_data_dir);
    if path.exists() {
        std::fs::remove_file(&path).with_context(|| format!("删除日志文件失败: {:?}", path))?;
    }
    Ok(count)
}

/// 删除指定时间之前的日志，返回删除条数
pub fn delete_before(app_data_dir: &Path, before: NaiveDateTime) -> Result<u64> {
    let mut removed = 0u64;
    let mut kept = Vec::new();
    for log in read_all(app_data_dir)? {
        if log.create_time.is_some_and(|t| t < before) {
            removed += 1;
        } else {
            kept.push(log);
        }
    }
    rewrite(app_data_dir, &kept)?;
    Ok(removed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::PageParam;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn temp_dir(name: &str) -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "enote-applog-file-test-{}-{}-{n}",
            std::process::id(),
            name
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn sample(id: i64, level: &str, module: &str, message: &str, ts: &str) -> AppLog {
        AppLog {
            id,
            level: level.to_string(),
            module: module.to_string(),
            action: "create".to_string(),
            target_id: None,
            target_name: None,
            message: message.to_string(),
            detail: None,
            create_time: NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S").ok(),
        }
    }

    #[test]
    fn append_and_search_roundtrip() {
        let dir = temp_dir("append");
        let log = sample(
            0,
            "INFO",
            "note",
            "创建笔记：密码 abc123 已写入",
            "2026-10-13 10:00:00",
        );
        append_frontend_log(&dir, &log).unwrap();
        append_frontend_log(
            &dir,
            &sample(0, "ERROR", "system", "系统异常", "2026-10-13 11:00:00"),
        )
        .unwrap();

        // 全量查询
        let param = AppLogSearchParam {
            page_param: PageParam {
                page_index: 1,
                page_size: 10,
            },
            level: None,
            module: None,
            action: None,
            keyword: String::new(),
            start_time: None,
            end_time: None,
        };
        let result = search_page(&dir, &param).unwrap();
        assert_eq!(result.total, 2);

        // message 脱敏生效（值保留首尾、中间打码），不依赖排序位置
        let sanitized = result
            .data
            .iter()
            .find(|l| l.message.contains("已写入"))
            .unwrap();
        assert_eq!(sanitized.message, "创建笔记：密码 ab****23 已写入");
        assert_eq!(sanitized.level, "INFO");

        // create_time 由服务端时间填充
        assert!(result.data.iter().all(|l| l.create_time.is_some()));

        // 级别过滤（大小写不敏感）
        let param_level = AppLogSearchParam {
            page_param: PageParam {
                page_index: 1,
                page_size: 10,
            },
            level: Some("info".to_string()),
            module: None,
            action: None,
            keyword: String::new(),
            start_time: None,
            end_time: None,
        };
        let result = search_page(&dir, &param_level).unwrap();
        assert_eq!(result.total, 1);
        assert_eq!(result.data[0].level, "INFO");
    }

    #[test]
    fn search_sorts_by_time_desc_and_filters() {
        let dir = temp_dir("sort");
        let path = frontend_log_path(&dir);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let mut content = String::new();
        for (ts, msg) in [
            ("2026-10-01 08:00:00", "旧日志"),
            ("2026-10-03 08:00:00", "新日志"),
            ("2026-10-02 08:00:00", "中日志"),
        ] {
            let log = sample(0, "INFO", "note", msg, ts);
            content.push_str(&serde_json::to_string(&log).unwrap());
            content.push('\n');
        }
        std::fs::write(&path, content).unwrap();

        let param = AppLogSearchParam {
            page_param: PageParam {
                page_index: 1,
                page_size: 10,
            },
            level: None,
            module: Some("note".to_string()),
            action: None,
            keyword: String::new(),
            start_time: Some(
                NaiveDateTime::parse_from_str("2026-10-02 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
            end_time: None,
        };
        let result = search_page(&dir, &param).unwrap();
        assert_eq!(result.total, 2);
        // 时间倒序
        assert_eq!(result.data[0].message, "新日志");
        assert_eq!(result.data[1].message, "中日志");
    }

    #[test]
    fn delete_clear_and_cleanup() {
        let dir = temp_dir("delete");
        append_frontend_log(&dir, &sample(0, "INFO", "note", "a", "2026-10-01 08:00:00")).unwrap();
        append_frontend_log(&dir, &sample(0, "WARN", "sync", "b", "2026-10-02 09:00:00")).unwrap();
        append_frontend_log(
            &dir,
            &sample(0, "ERROR", "note", "c", "2026-10-03 10:00:00"),
        )
        .unwrap();

        // 记录第一条的真实 id（时间戳微秒）
        let all = read_all(&dir).unwrap();
        assert_eq!(all.len(), 3);
        let first_id = all[0].id;

        // 单删
        delete_by_id(&dir, first_id).unwrap();
        assert_eq!(read_all(&dir).unwrap().len(), 2);

        // 按时间清理（删 10-02 之前的，实际已无可删）
        let dt = NaiveDateTime::parse_from_str("2026-10-02 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let removed = delete_before(&dir, dt).unwrap();
        assert_eq!(removed, 0);

        // 清空
        let cleared = clear_all(&dir).unwrap();
        assert_eq!(cleared, 2);
        assert_eq!(
            search_page(
                &dir,
                &AppLogSearchParam {
                    page_param: PageParam {
                        page_index: 1,
                        page_size: 10
                    },
                    level: None,
                    module: None,
                    action: None,
                    keyword: String::new(),
                    start_time: None,
                    end_time: None,
                }
            )
            .unwrap()
            .total,
            0
        );
    }

    #[test]
    fn empty_dir_returns_empty_result() {
        let dir = temp_dir("empty");
        let param = AppLogSearchParam {
            page_param: PageParam {
                page_index: 1,
                page_size: 10,
            },
            level: None,
            module: None,
            action: None,
            keyword: String::new(),
            start_time: None,
            end_time: None,
        };
        let result = search_page(&dir, &param).unwrap();
        assert_eq!(result.total, 0);
        assert!(result.data.is_empty());
    }
}
