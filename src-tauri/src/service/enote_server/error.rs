//! HTTP 错误映射模块
//!
//! 将 reqwest HTTP 错误和服务端错误响应映射为 AppError

use crate::error::AppError;

/// 服务端错误响应格式（与前端 ErrorResponse 对齐）
#[derive(Debug, serde::Deserialize)]
pub struct ServerErrorResponse {
    pub code: String,
    #[serde(default)]
    pub args: Option<Vec<String>>,
    #[serde(default)]
    pub message: Option<String>,
}

/// 将 HTTP 状态码和响应体映射为 AppError
pub fn map_http_error(status: reqwest::StatusCode, body: &str) -> AppError {
    match status.as_u16() {
        401 => AppError::code("SERVER_AUTH_FAILED"),
        403 => AppError::code("SERVER_FORBIDDEN"),
        404 => AppError::code("SERVER_NOT_FOUND"),
        422 => {
            // 尝试解析服务端业务错误
            if let Ok(err) = serde_json::from_str::<ServerErrorResponse>(body) {
                AppError::BusinessCode {
                    code: err.code,
                    args: err.args.unwrap_or_default(),
                }
            } else {
                AppError::code("SERVER_VALIDATION_ERROR")
            }
        }
        429 => AppError::code("SERVER_RATE_LIMITED"),
        500..=599 => AppError::code_with_args("SERVER_ERROR", vec![status.to_string()]),
        _ => AppError::code_with_args("SERVER_UNEXPECTED", vec![status.to_string()]),
    }
}

/// 将 reqwest 错误映射为 AppError
pub fn map_reqwest_error(err: reqwest::Error) -> AppError {
    if err.is_timeout() {
        AppError::code("SERVER_TIMEOUT")
    } else if err.is_connect() {
        AppError::code_with_args("SERVER_CONNECTION_FAILED", vec![err.to_string()])
    } else {
        AppError::code_with_args("SERVER_REQUEST_FAILED", vec![err.to_string()])
    }
}
