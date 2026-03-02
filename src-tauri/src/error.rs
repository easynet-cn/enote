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

    /// 业务逻辑错误
    #[error("{0}")]
    Business(String),

    /// 内部错误
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

/// 错误响应结构，用于序列化到前端
#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    /// 错误代码
    pub code: String,
    /// 错误消息
    pub message: String,
    /// 详细信息（可选）
    pub details: Option<String>,
}

impl From<AppError> for ErrorResponse {
    fn from(error: AppError) -> Self {
        // 安全处理：不向前端暴露敏感的内部错误细节
        let (code, message, details) = match &error {
            AppError::Database(_e) => {
                // 数据库错误不暴露具体SQL或结构信息
                #[cfg(debug_assertions)]
                let detail = Some(_e.to_string());
                #[cfg(not(debug_assertions))]
                let detail: Option<String> = None;

                (
                    "DATABASE_ERROR".to_string(),
                    t_simple("error.databaseError"),
                    detail,
                )
            }
            AppError::Business(msg) => (
                "BUSINESS_ERROR".to_string(),
                msg.clone(),
                None,
            ),
            AppError::Internal(_e) => {
                // 内部错误不暴露堆栈或实现细节
                #[cfg(debug_assertions)]
                let detail = Some(_e.to_string());
                #[cfg(not(debug_assertions))]
                let detail: Option<String> = None;

                (
                    "INTERNAL_ERROR".to_string(),
                    t_simple("error.internalError"),
                    detail,
                )
            }
        };

        ErrorResponse {
            code,
            message,
            details,
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
    /// 将错误转换为 ErrorResponse（用于序列化）
    fn clone_to_response(&self) -> ErrorResponse {
        ErrorResponse::from(self.clone_error())
    }

    /// 克隆错误（用于序列化）
    fn clone_error(&self) -> AppError {
        match self {
            AppError::Database(e) => AppError::Business(e.to_string()),
            AppError::Business(msg) => AppError::Business(msg.clone()),
            AppError::Internal(e) => AppError::Business(e.to_string()),
        }
    }

}
