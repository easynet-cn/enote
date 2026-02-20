//! 国际化模块
//!
//! 提供基于系统语言的国际化支持，用于启动时的错误提示等场景

use sys_locale::get_locale;
use std::collections::HashMap;

/// 支持的语言代码
const SUPPORTED_LOCALES: &[&str] = &["zh-CN", "en-US"];

/// 获取当前系统语言
///
/// 优先返回系统语言，如果不支持则返回英文
pub fn get_system_locale() -> &'static str {
    if let Some(locale) = get_locale() {
        // 提取语言代码（如 zh-CN -> zh, en-US -> en）
        let lang_code = locale.split('-').next().unwrap_or("");
        
        // 匹配支持的语言
        for supported in SUPPORTED_LOCALES {
            if supported.starts_with(lang_code) || *supported == locale {
                return supported;
            }
        }
    }
    // 默认返回英文
    "en-US"
}

/// 中文语言包
fn zh_cn_messages() -> HashMap<String, String> {
    let mut messages = HashMap::new();

    // 数据库相关错误
    messages.insert(
        "database.connection.failed.title".to_string(),
        "数据库连接失败".to_string()
    );
    messages.insert(
        "database.connection.failed.message".to_string(),
        "无法连接到数据库，请检查数据库服务是否已启动。\n\n错误详情：{}".to_string()
    );
    messages.insert(
        "database.migration.failed.title".to_string(),
        "数据库迁移失败".to_string()
    );
    messages.insert(
        "database.migration.failed.message".to_string(),
        "数据库迁移失败，请检查数据库配置。\n\n错误详情：{}".to_string()
    );

    // 配置相关错误
    messages.insert(
        "config.load.failed.title".to_string(),
        "配置加载失败".to_string()
    );
    messages.insert(
        "config.load.failed.message".to_string(),
        "应用配置加载失败，请检查应用数据目录权限。\n\n错误详情：{}".to_string()
    );

    // 配置文件相关错误
    messages.insert(
        "config.get_app_data_dir.failed".to_string(),
        "无法获取应用数据目录".to_string()
    );
    messages.insert(
        "config.create_app_data_dir.failed".to_string(),
        "无法创建应用数据目录".to_string()
    );
    messages.insert(
        "config.invalid_config_path".to_string(),
        "配置文件路径包含无效字符".to_string()
    );
    messages.insert(
        "config.load_file.failed".to_string(),
        "无法加载配置文件，请确保 application.yml 存在且格式正确".to_string()
    );
    messages.insert(
        "config.invalid_db_path".to_string(),
        "数据库路径包含无效字符".to_string()
    );
    messages.insert(
        "config.write_file.failed".to_string(),
        "无法写入配置文件".to_string()
    );
    messages.insert(
        "config.missing_datasource_url".to_string(),
        "配置文件中缺少 datasource.url".to_string()
    );
    messages.insert(
        "config.database.connect.failed".to_string(),
        "无法连接数据库，请检查数据库配置和网络连接".to_string()
    );

    // 通用错误
    messages.insert(
        "error.common.title".to_string(),
        "错误".to_string()
    );

    messages
}

/// 英文语言包
fn en_us_messages() -> HashMap<String, String> {
    let mut messages = HashMap::new();

    // Database related errors
    messages.insert(
        "database.connection.failed.title".to_string(),
        "Database Connection Failed".to_string()
    );
    messages.insert(
        "database.connection.failed.message".to_string(),
        "Failed to connect to the database. Please check if the database service is running.\n\nError details: {}".to_string()
    );
    messages.insert(
        "database.migration.failed.title".to_string(),
        "Database Migration Failed".to_string()
    );
    messages.insert(
        "database.migration.failed.message".to_string(),
        "Database migration failed. Please check the database configuration.\n\nError details: {}".to_string()
    );

    // Configuration related errors
    messages.insert(
        "config.load.failed.title".to_string(),
        "Configuration Load Failed".to_string()
    );
    messages.insert(
        "config.load.failed.message".to_string(),
        "Failed to load application configuration. Please check app data directory permissions.\n\nError details: {}".to_string()
    );

    // Configuration file related errors
    messages.insert(
        "config.get_app_data_dir.failed".to_string(),
        "Failed to get app data directory".to_string()
    );
    messages.insert(
        "config.create_app_data_dir.failed".to_string(),
        "Failed to create app data directory".to_string()
    );
    messages.insert(
        "config.invalid_config_path".to_string(),
        "Config file path contains invalid characters".to_string()
    );
    messages.insert(
        "config.load_file.failed".to_string(),
        "Failed to load configuration file".to_string()
    );
    messages.insert(
        "config.invalid_db_path".to_string(),
        "Database path contains invalid characters".to_string()
    );
    messages.insert(
        "config.write_file.failed".to_string(),
        "Failed to write configuration file".to_string()
    );
    messages.insert(
        "config.missing_datasource_url".to_string(),
        "Missing datasource.url in configuration".to_string()
    );
    messages.insert(
        "config.database.connect.failed".to_string(),
        "Failed to connect to database".to_string()
    );

    // Common errors
    messages.insert(
        "error.common.title".to_string(),
        "Error".to_string()
    );

    messages
}

/// 获取指定语言的消息映射
fn get_language_pack(locale: &str) -> HashMap<String, String> {
    match locale {
        "zh-CN" => zh_cn_messages(),
        "en-US" | _ => en_us_messages(),
    }
}

/// 翻译文本
///
/// # Arguments
/// * `key` - 翻译键
/// * `args` - 格式化参数（使用 {} 占位符）
///
/// # Examples
/// ```
/// let text = t("database.connection.failed.title", &[]);
/// let text_with_args = t("database.connection.failed.message", &["Connection timeout"]);
/// ```
pub fn t(key: &str, args: &[&str]) -> String {
    let locale = get_system_locale();
    let messages = get_language_pack(locale);
    
    if let Some(mut message) = messages.get(key).cloned() {
        for arg in args {
            message = message.replacen("{}", arg, 1);
        }
        message
    } else {
        // 如果找不到翻译，返回 key 本身
        key.to_string()
    }
}

/// 翻译文本（无参数版本）
pub fn t_simple(key: &str) -> String {
    t(key, &[])
}
