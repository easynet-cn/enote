//! 应用程序错误处理模块
//!
//! 定义了统一的错误类型和错误处理机制，
//! 使前端能够获取结构化的错误信息。

use serde::Serialize;
use thiserror::Error;

use crate::i18n::t_simple;

/// 应用程序错误类型
#[derive(Error, Debug)]
pub enum AppError {
    /// 数据库操作错误
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    /// 业务逻辑错误（保留兼容，逐步迁移到 BusinessCode）
    #[error("{0}")]
    Business(String),

    /// 业务逻辑错误（编码化，支持国际化）
    ///
    /// code: 错误码（如 "DB_NOT_CONNECTED"），前端通过 i18n 的 errorCodes.{code} 查翻译
    /// args: 插值参数（如错误详情），前端通过 {0}, {1} 等占位符替换
    #[error("Business error: {code}")]
    BusinessCode { code: String, args: Vec<String> },

    /// 内部错误
    #[error("Internal error: {0}")]
    Internal(anyhow::Error),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        // 如果 anyhow 内部包裹的是 AppError，则直接提取（支持 service 层抛出 BusinessCode）
        match err.downcast::<AppError>() {
            Ok(app_error) => app_error,
            Err(err) => AppError::Internal(err),
        }
    }
}

/// 错误响应结构，用于序列化到前端
#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    /// 错误代码
    pub code: String,
    /// 错误消息（BusinessCode 时为空，由前端 i18n 处理）
    pub message: String,
    /// 插值参数（用于前端 i18n 占位符替换）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// 详细信息（可选，仅 debug 模式暴露内部错误）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl From<AppError> for ErrorResponse {
    fn from(error: AppError) -> Self {
        // 安全处理：不向前端暴露敏感的内部错误细节
        match &error {
            AppError::Database(_e) => {
                // 数据库错误不暴露具体SQL或结构信息
                #[cfg(debug_assertions)]
                let detail = Some(_e.to_string());
                #[cfg(not(debug_assertions))]
                let detail: Option<String> = None;

                ErrorResponse {
                    code: "DATABASE_ERROR".to_string(),
                    message: t_simple("error.databaseError"),
                    args: None,
                    details: detail,
                }
            }
            AppError::Business(msg) => ErrorResponse {
                code: "BUSINESS_ERROR".to_string(),
                message: msg.clone(),
                args: None,
                details: None,
            },
            AppError::BusinessCode { code, args } => ErrorResponse {
                code: code.clone(),
                message: String::new(),
                args: if args.is_empty() { None } else { Some(args.clone()) },
                details: None,
            },
            AppError::Internal(_e) => {
                // 内部错误不暴露堆栈或实现细节
                #[cfg(debug_assertions)]
                let detail = Some(_e.to_string());
                #[cfg(not(debug_assertions))]
                let detail: Option<String> = None;

                ErrorResponse {
                    code: "INTERNAL_ERROR".to_string(),
                    message: t_simple("error.internalError"),
                    args: None,
                    details: detail,
                }
            }
        }
    }
}

// 实现 Serialize 以便 Tauri 可以将错误发送到前端
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let response: ErrorResponse = self.clone_to_response();
        response.serialize(serializer)
    }
}

impl AppError {
    /// 创建编码化业务错误（无参数）
    pub fn code(code: &str) -> Self {
        AppError::BusinessCode {
            code: code.to_string(),
            args: Vec::new(),
        }
    }

    /// 创建编码化业务错误（带参数）
    pub fn code_with_args(code: &str, args: Vec<String>) -> Self {
        AppError::BusinessCode {
            code: code.to_string(),
            args,
        }
    }

    /// 将错误转换为 ErrorResponse（用于序列化）
    fn clone_to_response(&self) -> ErrorResponse {
        ErrorResponse::from(self.clone_error())
    }

    /// 克隆错误（用于序列化）
    fn clone_error(&self) -> AppError {
        match self {
            AppError::Database(e) => AppError::Business(e.to_string()),
            AppError::Business(msg) => AppError::Business(msg.clone()),
            AppError::BusinessCode { code, args } => AppError::BusinessCode {
                code: code.clone(),
                args: args.clone(),
            },
            AppError::Internal(e) => AppError::Business(e.to_string()),
        }
    }
}
