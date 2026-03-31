//! 笔记服务模块
//!
//! 本模块提供笔记相关的所有业务逻辑，是应用程序中最复杂的服务模块。
//!
//! # 功能概述
//! - 笔记的 CRUD 操作
//! - 笔记与标签的关联管理
//! - 笔记历史记录的自动生成
//! - 分页搜索功能（支持多条件过滤）
//!
//! # 事务处理
//! 创建、更新、删除操作都在事务中执行，确保数据一致性：
//! - 笔记数据与标签关联的原子操作
//! - 历史记录的同步写入
//!
//! # 历史记录类型
//! - OperationType::Create (1): 创建
//! - OperationType::Update (2): 更新
//! - OperationType::Delete (3): 删除

mod crypto_helper;
mod crud;
mod search;

pub use crud::*;
pub use search::*;
