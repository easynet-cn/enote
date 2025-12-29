//! 应用程序错误处理模块
//!
//! 定义了统一的错误类型和错误处理机制，
//! 使前端能够获取结构化的错误信息。

use serde::Serialize;
use thiserror::Error;

/// 应用程序错误类型
#[derive(Error, Debug)]
pub enum AppError {
    /// 数据库操作错误
    #[error("数据库错误: {0}")]
    Database(#[from] sea_orm::DbErr),

    /// 资源未找到
    #[error("{resource}不存在: ID {id}")]
    NotFound { resource: String, id: i64 },

    /// 验证错误
    #[error("验证失败: {0}")]
    Validation(String),

    /// 业务逻辑错误
    #[error("{0}")]
    Business(String),

    /// 配置错误
    #[error("配置错误: {0}")]
    Config(String),

    /// 内部错误
    #[error("内部错误: {0}")]
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
                    "数据库操作失败，请稍后重试".to_string(),
                    detail,
                )
            }
            AppError::NotFound { resource, id } => (
                "NOT_FOUND".to_string(),
                format!("{}不存在", resource),
                Some(format!("ID: {}", id)),
            ),
            AppError::Validation(msg) => (
                "VALIDATION_ERROR".to_string(),
                msg.clone(),
                None,
            ),
            AppError::Business(msg) => (
                "BUSINESS_ERROR".to_string(),
                msg.clone(),
                None,
            ),
            AppError::Config(_msg) => {
                // 配置错误不暴露具体配置细节
                #[cfg(debug_assertions)]
                let detail = Some(_msg.clone());
                #[cfg(not(debug_assertions))]
                let detail: Option<String> = None;

                (
                    "CONFIG_ERROR".to_string(),
                    "配置错误，请检查应用配置".to_string(),
                    detail,
                )
            }
            AppError::Internal(_e) => {
                // 内部错误不暴露堆栈或实现细节
                #[cfg(debug_assertions)]
                let detail = Some(_e.to_string());
                #[cfg(not(debug_assertions))]
                let detail: Option<String> = None;

                (
                    "INTERNAL_ERROR".to_string(),
                    "系统内部错误，请稍后重试".to_string(),
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
            AppError::NotFound { resource, id } => AppError::NotFound {
                resource: resource.clone(),
                id: *id,
            },
            AppError::Validation(msg) => AppError::Validation(msg.clone()),
            AppError::Business(msg) => AppError::Business(msg.clone()),
            AppError::Config(msg) => AppError::Config(msg.clone()),
            AppError::Internal(e) => AppError::Business(e.to_string()),
        }
    }

    /// 创建验证错误
    pub fn validation(message: impl Into<String>) -> Self {
        AppError::Validation(message.into())
    }

    /// 创建业务错误
    pub fn business(message: impl Into<String>) -> Self {
        AppError::Business(message.into())
    }

    /// 创建资源未找到错误
    pub fn not_found(resource: impl Into<String>, id: i64) -> Self {
        AppError::NotFound {
            resource: resource.into(),
            id,
        }
    }
}

/// 结果类型别名
pub type AppResult<T> = Result<T, AppError>;
