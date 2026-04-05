//! 服务器认证模块
//!
//! 支持多种认证方式：Bearer Token、Basic Auth、JWT、自定义 Header、OAuth2

use serde::{Deserialize, Serialize};
use tokio::time::Instant;

/// 服务器认证方式（存储在 profile YAML 中）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ServerAuthMethod {
    /// 无认证
    #[default]
    None,
    /// Bearer Token（token 存 Keychain）
    Bearer,
    /// Basic Auth（username 在 YAML，password 存 Keychain）
    Basic {
        #[serde(default)]
        username: String,
    },
    /// JWT 自动刷新（token 存 Keychain）
    Jwt {
        /// Token 刷新端点
        #[serde(default, rename = "refresh-url")]
        refresh_url: String,
        /// 登录用户名
        #[serde(default)]
        username: String,
    },
    /// 自定义 Header（header value 存 Keychain）
    CustomHeader {
        /// Header 名称
        #[serde(default, rename = "header-name")]
        header_name: String,
    },
    /// OAuth 2.0
    OAuth2 {
        /// Token 端点
        #[serde(default, rename = "token-url")]
        token_url: String,
        /// 客户端 ID
        #[serde(default, rename = "client-id")]
        client_id: String,
        /// 权限范围
        #[serde(default)]
        scopes: String,
    },
}

/// 服务器配置（存储在 profile YAML 中）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerConfig {
    /// 服务器基础 URL（如 "https://notes.example.com"）
    #[serde(default)]
    pub url: String,
    /// 认证方式
    #[serde(default, rename = "auth-method")]
    pub auth_method: ServerAuthMethod,
    /// 请求超时（秒）
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

fn default_timeout() -> u64 {
    30
}

/// 服务器敏感信息（从 Keychain 加载，不存储在 YAML 中）
#[derive(Debug, Clone, Default)]
pub struct ServerSecrets {
    /// Bearer token 或 JWT access token
    pub token: Option<String>,
    /// JWT refresh token
    pub refresh_token: Option<String>,
    /// Basic auth 或 JWT 密码
    pub password: Option<String>,
    /// OAuth2 client secret
    pub client_secret: Option<String>,
    /// Custom header value
    pub header_value: Option<String>,
}

/// 运行时认证状态（持有当前有效的 token 等）
pub struct AuthState {
    /// 认证方式配置
    pub method: ServerAuthMethod,
    /// 当前 access token（Bearer / JWT / OAuth2）
    pub access_token: Option<String>,
    /// Refresh token（JWT / OAuth2）
    pub refresh_token: Option<String>,
    /// Basic auth 凭据 (username, password)
    pub basic_credentials: Option<(String, String)>,
    /// 自定义 header (name, value)
    pub custom_header: Option<(String, String)>,
    /// Token 过期时间
    pub token_expiry: Option<Instant>,
    /// 服务器基础 URL（用于 token 刷新）
    pub base_url: String,
}

impl AuthState {
    /// 从配置和密钥创建认证状态
    pub fn new(config: &ServerConfig, secrets: &ServerSecrets) -> Self {
        let method = config.auth_method.clone();
        let mut state = AuthState {
            method: method.clone(),
            access_token: None,
            refresh_token: None,
            basic_credentials: None,
            custom_header: None,
            token_expiry: None,
            base_url: config.url.clone(),
        };

        match &method {
            ServerAuthMethod::None => {}
            ServerAuthMethod::Bearer => {
                state.access_token = secrets.token.clone();
            }
            ServerAuthMethod::Basic { username } => {
                if let Some(password) = &secrets.password {
                    state.basic_credentials = Some((username.clone(), password.clone()));
                }
            }
            ServerAuthMethod::Jwt { .. } => {
                state.access_token = secrets.token.clone();
                state.refresh_token = secrets.refresh_token.clone();
            }
            ServerAuthMethod::CustomHeader { header_name } => {
                if let Some(value) = &secrets.header_value {
                    state.custom_header = Some((header_name.clone(), value.clone()));
                }
            }
            ServerAuthMethod::OAuth2 { .. } => {
                state.access_token = secrets.token.clone();
                state.refresh_token = secrets.refresh_token.clone();
            }
        }

        state
    }

    /// 将认证信息应用到请求上
    pub fn apply_to_request(
        &self,
        builder: reqwest::RequestBuilder,
    ) -> reqwest::RequestBuilder {
        match &self.method {
            ServerAuthMethod::None => builder,
            ServerAuthMethod::Bearer | ServerAuthMethod::Jwt { .. } | ServerAuthMethod::OAuth2 { .. } => {
                if let Some(token) = &self.access_token {
                    builder.bearer_auth(token)
                } else {
                    builder
                }
            }
            ServerAuthMethod::Basic { .. } => {
                if let Some((username, password)) = &self.basic_credentials {
                    builder.basic_auth(username, Some(password))
                } else {
                    builder
                }
            }
            ServerAuthMethod::CustomHeader { .. } => {
                if let Some((name, value)) = &self.custom_header {
                    builder.header(name.as_str(), value.as_str())
                } else {
                    builder
                }
            }
        }
    }

    /// 检查 token 是否过期
    pub fn is_expired(&self) -> bool {
        self.token_expiry
            .is_some_and(|expiry| Instant::now() >= expiry)
    }

    /// 是否支持 token 刷新
    pub fn can_refresh(&self) -> bool {
        matches!(
            self.method,
            ServerAuthMethod::Jwt { .. } | ServerAuthMethod::OAuth2 { .. }
        ) && self.refresh_token.is_some()
    }

    /// 刷新 token（JWT / OAuth2）
    pub async fn refresh(&mut self, http: &reqwest::Client) -> anyhow::Result<()> {
        let refresh_token_value = self
            .refresh_token
            .clone()
            .ok_or_else(|| anyhow::anyhow!("No refresh token available"))?;

        // 克隆 method 以避免借用冲突
        let method = self.method.clone();

        match &method {
            ServerAuthMethod::Jwt { refresh_url, .. } => {
                let url = format!("{}{}", self.base_url, refresh_url);
                let resp = http
                    .post(&url)
                    .json(&serde_json::json!({ "refreshToken": refresh_token_value }))
                    .send()
                    .await?;

                if !resp.status().is_success() {
                    anyhow::bail!("Token refresh failed: {}", resp.status());
                }

                let body: TokenResponse = resp.json().await?;
                self.access_token = Some(body.access_token);
                if let Some(rt) = body.refresh_token {
                    self.refresh_token = Some(rt);
                }
                if let Some(expires_in) = body.expires_in {
                    self.token_expiry =
                        Some(Instant::now() + std::time::Duration::from_secs(expires_in));
                }
            }
            ServerAuthMethod::OAuth2 {
                token_url,
                client_id,
                ..
            } => {
                let url = if token_url.starts_with("http") {
                    token_url.clone()
                } else {
                    format!("{}{}", self.base_url, token_url)
                };

                let params = vec![
                    ("grant_type", "refresh_token"),
                    ("refresh_token", refresh_token_value.as_str()),
                    ("client_id", client_id.as_str()),
                ];

                let resp = http.post(&url).form(&params).send().await?;

                if !resp.status().is_success() {
                    anyhow::bail!("OAuth2 token refresh failed: {}", resp.status());
                }

                let body: TokenResponse = resp.json().await?;
                self.access_token = Some(body.access_token);
                if let Some(rt) = body.refresh_token {
                    self.refresh_token = Some(rt);
                }
                if let Some(expires_in) = body.expires_in {
                    self.token_expiry =
                        Some(Instant::now() + std::time::Duration::from_secs(expires_in));
                }
            }
            _ => anyhow::bail!("Auth method does not support refresh"),
        }

        Ok(())
    }
}

/// Token 响应结构
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<u64>,
}
