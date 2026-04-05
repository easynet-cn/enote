//! ENote Server HTTP 客户端
//!
//! 封装 reqwest::Client，提供统一的认证、错误处理和请求方法

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::RwLock;
use tracing::{debug, warn};

use super::auth::{AuthState, ServerConfig, ServerSecrets};
use super::error::{map_http_error, map_reqwest_error};
use crate::error::AppError;

/// ENote Server 客户端
#[derive(Clone)]
pub struct EnoteServerClient {
    /// HTTP 客户端
    http: reqwest::Client,
    /// 服务器基础 URL
    base_url: String,
    /// 运行时认证状态
    auth: Arc<RwLock<AuthState>>,
}

impl EnoteServerClient {
    /// 创建新的客户端实例
    pub fn new(config: &ServerConfig, secrets: ServerSecrets) -> Result<Self> {
        let url = config.url.trim_end_matches('/').to_string();
        if url.is_empty() {
            anyhow::bail!("Server URL is empty");
        }

        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .user_agent("enote-desktop")
            .build()?;

        let auth_state = AuthState::new(config, &secrets);

        Ok(Self {
            http,
            base_url: url,
            auth: Arc::new(RwLock::new(auth_state)),
        })
    }

    /// 测试连接
    pub async fn test_connection(&self) -> Result<(), AppError> {
        self.get::<serde_json::Value>("/api/health")
            .await
            .map(|_| ())
    }

    /// GET 请求
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, AppError> {
        self.request(Method::GET, path, Option::<&()>::None).await
    }

    /// POST 请求
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, AppError> {
        self.request(Method::POST, path, Some(body)).await
    }

    /// POST 请求（无返回值）
    pub async fn post_void<B: Serialize>(&self, path: &str, body: &B) -> Result<(), AppError> {
        self.request_void(Method::POST, path, Some(body)).await
    }

    /// PUT 请求
    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, AppError> {
        self.request(Method::PUT, path, Some(body)).await
    }

    /// PUT 请求（无返回值）
    pub async fn put_void<B: Serialize>(&self, path: &str, body: &B) -> Result<(), AppError> {
        self.request_void(Method::PUT, path, Some(body)).await
    }

    /// DELETE 请求
    pub async fn delete(&self, path: &str) -> Result<(), AppError> {
        self.request_void(Method::DELETE, path, Option::<&()>::None)
            .await
    }

    /// DELETE 请求（有返回值）
    pub async fn delete_with_response<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, AppError> {
        self.request(Method::DELETE, path, Option::<&()>::None)
            .await
    }

    /// 通用请求方法（有返回值）
    async fn request<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, AppError> {
        let response = self.send_request(method.clone(), path, body).await?;
        let status = response.status();

        if status.is_success() {
            response.json::<T>().await.map_err(|e| {
                AppError::code_with_args("SERVER_PARSE_ERROR", vec![e.to_string()])
            })
        } else {
            let body_text = response.text().await.unwrap_or_default();
            Err(map_http_error(status, &body_text))
        }
    }

    /// 通用请求方法（无返回值）
    async fn request_void<B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<(), AppError> {
        let response = self.send_request(method, path, body).await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let body_text = response.text().await.unwrap_or_default();
            Err(map_http_error(status, &body_text))
        }
    }

    /// 发送请求（含认证和 401 重试）
    async fn send_request<B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<reqwest::Response, AppError> {
        let url = format!("{}{}", self.base_url, path);
        debug!("HTTP {} {}", method, url);

        // 第一次请求
        let response = {
            let auth = self.auth.read().await;
            let mut builder = self.http.request(method.clone(), &url);
            builder = auth.apply_to_request(builder);
            if let Some(body) = body {
                builder = builder.json(body);
            }
            builder.send().await.map_err(map_reqwest_error)?
        };

        // 401 时尝试刷新 token 并重试
        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            let can_refresh = self.auth.read().await.can_refresh();
            if can_refresh {
                warn!("Got 401, attempting token refresh...");
                let mut auth = self.auth.write().await;
                if let Err(e) = auth.refresh(&self.http).await {
                    warn!("Token refresh failed: {}", e);
                    return Ok(response);
                }
                drop(auth);

                // 用新 token 重试
                let auth = self.auth.read().await;
                let mut builder = self.http.request(method, &url);
                builder = auth.apply_to_request(builder);
                if let Some(body) = body {
                    builder = builder.json(body);
                }
                return builder.send().await.map_err(map_reqwest_error);
            }
        }

        Ok(response)
    }

    /// 获取基础 URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// 上传附件（multipart form）
    pub async fn upload_multipart(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::Response, AppError> {
        let url = format!("{}{}", self.base_url, path);
        let auth = self.auth.read().await;
        let builder = self.http.post(&url);
        let builder = auth.apply_to_request(builder);
        let response = builder.multipart(form).send().await.map_err(map_reqwest_error)?;
        Ok(response)
    }
}
