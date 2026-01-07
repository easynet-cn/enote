//! 数据传输对象（DTO）模块
//!
//! 本模块定义了前后端交互使用的数据结构，包括：
//! - 分页参数和结果
//! - 笔记本、笔记、标签等业务对象
//! - 搜索参数
//! - 日期时间序列化/反序列化工具函数
//!
//! 所有结构体都实现了 Serialize 和 Deserialize，
//! 使用 camelCase 命名约定与前端 JavaScript 保持一致。

use std::collections::HashMap;

use chrono::NaiveDateTime;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnNull, serde_as};
use tracing::warn;

use crate::entity;

// ============================================================================
// 操作类型枚举
// ============================================================================

/// 笔记历史操作类型
///
/// 用于记录笔记的创建、更新、删除等操作
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum OperationType {
    /// 创建操作
    Create = 1,
    /// 更新操作
    Update = 2,
    /// 删除操作
    Delete = 3,
}

impl OperationType {
    /// 转换为 i32 值（用于数据库存储）
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

impl From<OperationType> for i32 {
    fn from(op: OperationType) -> Self {
        op as i32
    }
}

// ============================================================================
// 分页相关结构体
// ============================================================================

/// 分页查询参数
///
/// 用于接收前端传入的分页信息
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageParam {
    /// 页码，从 1 开始（默认: 1）
    #[serde(default = "PageParam::default_page_index")]
    pub page_index: i64,
    /// 每页数量（默认: 50）
    #[serde(default = "PageParam::default_page_size")]
    pub page_size: i64,
}

impl PageParam {
    /// 最大每页数量限制
    const MAX_PAGE_SIZE: i64 = 1000;

    /// 计算数据库查询的起始偏移量
    ///
    /// # 返回
    /// (page_index - 1) * page_size
    pub fn start(&self) -> i64 {
        (self.page_index - 1) * self.page_size
    }

    /// 验证分页参数
    ///
    /// # 返回
    /// - `Ok(())`: 参数有效
    /// - `Err`: 参数无效，包含错误信息
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.page_index < 1 {
            anyhow::bail!("页码必须大于等于 1")
        }
        if self.page_size < 1 {
            anyhow::bail!("每页数量必须大于等于 1")
        }
        if self.page_size > Self::MAX_PAGE_SIZE {
            anyhow::bail!("每页数量不能超过 {}", Self::MAX_PAGE_SIZE)
        }
        Ok(())
    }

    /// 规范化分页参数（自动修正无效值）
    #[allow(dead_code)]
    pub fn normalize(&mut self) {
        if self.page_index < 1 {
            self.page_index = 1;
        }
        if self.page_size < 1 {
            self.page_size = Self::default_page_size();
        }
        if self.page_size > Self::MAX_PAGE_SIZE {
            self.page_size = Self::MAX_PAGE_SIZE;
        }
    }

    fn default_page_index() -> i64 {
        1
    }

    fn default_page_size() -> i64 {
        50
    }
}

/// 分页查询结果
///
/// 通用的分页结果包装器，可用于任何类型的数据列表
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    /// 总记录数
    #[serde(default = "PageResult::<T>::default_total")]
    pub total: i64,
    /// 总页数
    #[serde(default = "PageResult::<T>::default_total_pages")]
    pub total_pages: i64,
    /// 当前页数据列表
    #[serde(default = "PageResult::<T>::default_data")]
    pub data: Vec<T>,
}

impl<T> PageResult<T> {
    /// 创建新的分页结果
    #[allow(dead_code)]
    pub fn new(total: i64, total_pages: i64, data: Vec<T>) -> Self {
        Self {
            total,
            total_pages,
            data,
        }
    }

    /// 使用总数和数据创建分页结果（总页数稍后计算）
    pub fn with_data(total: i64, data: Vec<T>) -> Self {
        Self {
            total,
            total_pages: 0,
            data,
        }
    }

    /// 根据每页数量计算总页数
    pub fn total_pages(&mut self, page_size: i64) {
        if self.total > 0 && page_size > 0 {
            self.total_pages = ((self.total as f64) / (page_size as f64)).ceil() as i64
        }
    }

    fn default_total() -> i64 {
        0
    }

    fn default_total_pages() -> i64 {
        0
    }

    fn default_data() -> Vec<T> {
        vec![]
    }
}

impl<T> Default for PageResult<T> {
    fn default() -> Self {
        Self {
            total: 0,
            total_pages: 0,
            data: vec![],
        }
    }
}

// ============================================================================
// 业务对象
// ============================================================================

/// 笔记本数据传输对象
///
/// 用于前后端交互的笔记本数据结构
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Notebook {
    /// 笔记本 ID
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: i64,
    /// 父笔记本 ID（用于层级结构，0 表示顶级）
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub parent_id: i64,
    /// 笔记本名称
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// 笔记本描述
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub description: String,
    /// 图标（Lucide 图标名称）
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub icon: String,
    /// CSS 类名（用于自定义样式）
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub cls: String,
    /// 排序顺序
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub sort_order: i32,
    /// 创建时间
    #[serde(
        serialize_with = "serialize_option_dt",
        deserialize_with = "deserialize_option_dt"
    )]
    pub create_time: Option<NaiveDateTime>,
    /// 更新时间
    #[serde(
        serialize_with = "serialize_option_dt",
        deserialize_with = "deserialize_option_dt"
    )]
    pub update_time: Option<NaiveDateTime>,
}

/// 从数据库实体引用转换为 DTO
impl From<&entity::notebook::Model> for Notebook {
    fn from(value: &entity::notebook::Model) -> Self {
        Self {
            id: value.id,
            parent_id: value.parent_id,
            name: value.name.clone(),
            description: value.description.clone(),
            icon: value.icon.clone(),
            cls: value.cls.clone(),
            sort_order: value.sort_order,
            create_time: Some(value.create_time),
            update_time: Some(value.update_time),
        }
    }
}

/// 从数据库实体转换为 DTO（消耗所有权）
impl From<entity::notebook::Model> for Notebook {
    fn from(value: entity::notebook::Model) -> Self {
        Self {
            id: value.id,
            parent_id: value.parent_id,
            name: value.name,
            description: value.description,
            icon: value.icon,
            cls: value.cls,
            sort_order: value.sort_order,
            create_time: Some(value.create_time),
            update_time: Some(value.update_time),
        }
    }
}

/// 笔记数据传输对象
///
/// 用于前后端交互的笔记数据结构，包含关联的标签信息
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Note {
    /// 笔记 ID
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: i64,
    /// 所属笔记本 ID
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub notebook_id: i64,
    /// 所属笔记本名称（用于显示）
    pub notebook_name: String,
    /// 笔记标题
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub title: String,
    /// 笔记内容（HTML 或 Markdown 格式）
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub content: String,
    /// 内容类型：0 = HTML（默认），1 = Markdown
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub content_type: i32,
    /// 创建时间
    #[serde(
        serialize_with = "serialize_option_dt",
        deserialize_with = "deserialize_option_dt"
    )]
    pub create_time: Option<NaiveDateTime>,
    /// 更新时间
    #[serde(
        serialize_with = "serialize_option_dt",
        deserialize_with = "deserialize_option_dt"
    )]
    pub update_time: Option<NaiveDateTime>,
    /// 关联的标签列表
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tags: Vec<Tag>,
}

/// 从数据库实体转换为 DTO（消耗所有权）
impl From<entity::note::Model> for Note {
    fn from(value: entity::note::Model) -> Self {
        Self {
            id: value.id,
            notebook_id: value.notebook_id,
            title: value.title,
            content: value.content,
            content_type: value.content_type,
            create_time: Some(value.create_time),
            update_time: Some(value.update_time),
            ..Default::default()
        }
    }
}

/// 从数据库实体引用转换为 DTO
impl From<&entity::note::Model> for Note {
    fn from(value: &entity::note::Model) -> Self {
        Self {
            id: value.id,
            notebook_id: value.notebook_id,
            title: value.title.clone(),
            content: value.content.clone(),
            content_type: value.content_type,
            create_time: Some(value.create_time),
            update_time: Some(value.update_time),
            ..Default::default()
        }
    }
}

/// 从 DTO 转换为数据库活动模型（用于插入/更新）
impl From<Note> for entity::note::ActiveModel {
    fn from(note: Note) -> Self {
        Self {
            id: Set(note.id),
            notebook_id: Set(note.notebook_id),
            title: Set(note.title),
            content: Set(note.content),
            content_type: Set(note.content_type),
            create_time: Set(note.create_time.unwrap_or_default()),
            update_time: Set(note.update_time.unwrap_or_default()),
        }
    }
}

/// 从 DTO 引用转换为数据库活动模型
impl From<&Note> for entity::note::ActiveModel {
    fn from(note: &Note) -> Self {
        Self {
            id: Set(note.id),
            notebook_id: Set(note.notebook_id),
            title: Set(note.title.clone()),
            content: Set(note.content.clone()),
            content_type: Set(note.content_type),
            create_time: Set(note.create_time.unwrap_or_default()),
            update_time: Set(note.update_time.unwrap_or_default()),
        }
    }
}

/// 笔记统计结果
///
/// 笔记本的笔记数量统计
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteStatsResult {
    pub total: i64,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub notebook_counts: HashMap<i64, i64>,
}

/// 标签数据传输对象
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Tag {
    /// 标签 ID
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: i64,
    /// 标签名称
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// 图标（Lucide 图标名称）
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub icon: String,
    /// CSS 类名（用于颜色等样式）
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub cls: String,
    /// 排序顺序
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub sort_order: i32,
    /// 创建时间
    #[serde(
        serialize_with = "serialize_option_dt",
        deserialize_with = "deserialize_option_dt"
    )]
    pub create_time: Option<NaiveDateTime>,
    /// 更新时间
    #[serde(
        serialize_with = "serialize_option_dt",
        deserialize_with = "deserialize_option_dt"
    )]
    pub update_time: Option<NaiveDateTime>,
}

impl From<entity::tag::Model> for Tag {
    fn from(value: entity::tag::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            icon: value.icon,
            cls: value.cls,
            sort_order: value.sort_order,
            create_time: Some(value.create_time),
            update_time: Some(value.update_time),
        }
    }
}

impl From<&entity::tag::Model> for Tag {
    fn from(value: &entity::tag::Model) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            icon: value.icon.clone(),
            cls: value.cls.clone(),
            sort_order: value.sort_order,
            create_time: Some(value.create_time),
            update_time: Some(value.update_time),
        }
    }
}

// ============================================================================
// 搜索参数
// ============================================================================

/// 笔记搜索分页参数
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteSearchPageParam {
    /// 分页参数（展开到同一层级）
    #[serde(flatten)]
    pub page_param: PageParam,
    /// 筛选笔记本 ID（0 表示不筛选）
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub notebook_id: i64,
    /// 筛选标签 ID（0 表示不筛选）
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tag_id: i64,
    /// 搜索关键词（搜索标题和内容）
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub keyword: String,
}

impl NoteSearchPageParam {
    /// 最大关键词长度
    #[allow(dead_code)]
    const MAX_KEYWORD_LENGTH: usize = 500;

    /// 验证搜索参数
    #[allow(dead_code)]
    pub fn validate(&self) -> anyhow::Result<()> {
        self.page_param.validate()?;

        if self.keyword.len() > Self::MAX_KEYWORD_LENGTH {
            anyhow::bail!("搜索关键词不能超过 {} 个字符", Self::MAX_KEYWORD_LENGTH)
        }

        Ok(())
    }

    /// 规范化搜索参数
    #[allow(dead_code)]
    pub fn normalize(&mut self) {
        self.page_param.normalize();

        // 截断过长的关键词
        if self.keyword.len() > Self::MAX_KEYWORD_LENGTH {
            self.keyword = self.keyword.chars().take(Self::MAX_KEYWORD_LENGTH).collect();
        }

        // 去除关键词首尾空格
        self.keyword = self.keyword.trim().to_string();
    }
}

/// 笔记历史记录额外信息
///
/// 存储在历史记录中的快照信息，用于恢复或查看历史状态
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteHistoryExtra {
    /// 当时的笔记本 ID
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub notebook_id: i64,
    /// 当时的笔记本名称
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub notebook_name: String,
    /// 当时的笔记内容类型
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub content_type: i32,
    /// 当时的笔记标题
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub title: String,
    /// 当时的标签列表
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tags: Vec<Tag>,
}

/// 笔记历史记录
///
/// 记录笔记的每次修改，用于版本追溯
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteHistory {
    /// 历史记录 ID
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: i64,
    /// 关联的笔记 ID
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub note_id: i64,
    /// 修改前的内容
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub old_content: String,
    /// 修改后的内容
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub new_content: String,
    /// 额外信息（JSON 解析）
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub extra: NoteHistoryExtra,
    /// 操作类型：1=创建, 2=更新, 3=删除
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub operate_type: i32,
    /// 操作时间
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub operate_time: NaiveDateTime,
    /// 创建时间
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub create_time: NaiveDateTime,
}

impl From<entity::note_history::Model> for NoteHistory {
    fn from(value: entity::note_history::Model) -> Self {
        Self {
            id: value.id,
            note_id: value.note_id,
            old_content: value.old_content,
            new_content: value.new_content,
            extra: serde_json::from_str(&value.extra).unwrap_or_else(|e| {
                warn!("笔记历史记录 extra 反序列化失败: {}", e);
                NoteHistoryExtra::default()
            }),
            operate_type: value.operate_type,
            operate_time: value.operate_time,
            create_time: value.create_time,
        }
    }
}

impl From<&entity::note_history::Model> for NoteHistory {
    fn from(value: &entity::note_history::Model) -> Self {
        Self {
            id: value.id,
            note_id: value.note_id,
            old_content: value.old_content.clone(),
            new_content: value.new_content.clone(),
            extra: serde_json::from_str(&value.extra).unwrap_or_else(|e| {
                warn!("笔记历史记录 extra 反序列化失败: {}", e);
                NoteHistoryExtra::default()
            }),
            operate_type: value.operate_type,
            operate_time: value.operate_time,
            create_time: value.create_time,
        }
    }
}

/// 笔记历史搜索分页参数
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteHistorySearchPageParam {
    /// 分页参数
    #[serde(flatten)]
    pub page_param: PageParam,
    /// 笔记 ID（必需）
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub note_id: i64,
}

// ============================================================================
// 日期时间序列化工具函数
// ============================================================================

/// 序列化日期时间为字符串
///
/// 格式：YYYY-MM-DD HH:mm:ss
pub fn serialize_dt<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&dt.format("%Y-%m-%d %H:%M:%S").to_string())
}

/// 反序列化字符串为日期时间
///
/// 格式：YYYY-MM-DD HH:mm:ss
pub fn deserialize_dt<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
}

/// 序列化可选日期时间为字符串
///
/// None 序列化为 null，Some 序列化为格式化字符串
pub fn serialize_option_dt<S>(
    value: &Option<NaiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(dt) => serializer.serialize_some(&dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        None => serializer.serialize_none(),
    }
}

/// 反序列化可选日期时间
///
/// null 反序列化为 None，字符串反序列化为 Some(NaiveDateTime)
pub fn deserialize_option_dt<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(str) => NaiveDateTime::parse_from_str(&str, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)
            .map(Some),
        None => Ok(None),
    }
}
