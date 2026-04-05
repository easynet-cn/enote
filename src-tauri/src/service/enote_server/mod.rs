//! ENote Server 远程后端模块
//!
//! 提供通过 HTTP API 连接远程 ENote 服务器的能力，
//! 支持多种认证方式（Bearer、Basic、JWT、自定义 Header、OAuth2）。

pub mod auth;
pub mod client;
pub mod endpoints;
pub mod error;

pub use auth::{AuthState, ServerAuthMethod, ServerConfig, ServerSecrets};
pub use client::EnoteServerClient;
