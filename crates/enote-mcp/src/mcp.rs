//! ENote MCP Server 实现
//!
//! 通过 rmcp 提供笔记操作工具给 AI 客户端。
//! 所有数据操作统一走 [`AccessStore`] trait，因此 Database / Git / Server
//! 三种后端对 AI 工具呈现完全相同的行为。

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::tool::{ToolCallContext, ToolRouter},
    model::{
        CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, Implementation,
        InitializeRequestParams, InitializeResult, ListToolsResult, PaginatedRequestParams,
        ProtocolVersion, ServerCapabilities, ServerInfo, Tool,
    },
    service::RequestContext,
    tool, tool_router,
};
use serde::Serialize;

use enote_lib::{
    model::{
        McpPermission, Note, NoteSearchPageParam, Notebook, OperateSource, PageParam, Tag, Todo,
        TodoList, TodoSearchParam,
    },
    store::AccessStore,
};

/// 全部内置工具名称
///
/// 用途：
/// - 用户未配置 `mcpEnabledTools` 时作为默认启用集合
/// - 与 `mcpKnownTools` 比较，识别「本次升级新增的工具」
const DEFAULT_TOOLS: &[&str] = &[
    "search_notes",
    "get_note",
    "create_note",
    "update_note",
    "delete_note",
    "list_notebooks",
    "create_notebook",
    "list_tags",
    "create_tag",
    "note_stats",
    "search_todos",
    "get_todo",
    "create_todo",
    "update_todo",
    "complete_todo",
    "delete_todo",
    "list_todo_lists",
    "create_todo_list",
    "todo_stats",
];

/// 获取已启用的工具名称集合
///
/// 升级兼容：用户可能保存过不含新工具的 `mcpEnabledTools`。
/// 通过 `mcpKnownTools`（前端保存时写入的"当时已知工具全集"）识别新增工具并自动启用，
/// 避免升级后新工具默认不可见。此逻辑与 settings 存储位置无关，原样保留。
fn get_enabled_tools(settings: &HashMap<String, String>) -> HashSet<String> {
    // MCP 总开关关闭 → 无可用工具
    if settings.get("mcpEnabled").map(String::as_str) != Some("1") {
        return HashSet::new();
    }

    let default_tools: HashSet<String> = DEFAULT_TOOLS.iter().map(|s| s.to_string()).collect();

    let Some(enabled_tools) = settings.get("mcpEnabledTools") else {
        // 没有配置 → 默认全部启用
        return default_tools;
    };

    let enabled: HashSet<String> = enabled_tools
        .split(',')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    // 新增工具 = 默认全集 - 上次保存时已知的工具
    let known: HashSet<String> = settings
        .get("mcpKnownTools")
        .map(|v| {
            v.split(',')
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    let new_tools: HashSet<String> = default_tools.difference(&known).cloned().collect();

    enabled.union(&new_tools).cloned().collect()
}

/// 检查指定 MCP 工具是否已启用
async fn check_tool_enabled(
    settings: &HashMap<String, String>,
    tool_name: &str,
) -> Result<(), McpError> {
    let enabled = get_enabled_tools(settings);
    if !enabled.contains(tool_name) {
        return Err(McpError::invalid_request(
            format!("Tool '{}' is disabled. Enable it in Settings.", tool_name),
            None,
        ));
    }
    Ok(())
}

/// 解析 "YYYY-MM-DD HH:MM:SS" 格式的时间字符串，失败时返回 None
fn parse_dt_opt(value: &str) -> Option<chrono::NaiveDateTime> {
    chrono::NaiveDateTime::parse_from_str(value.trim(), "%Y-%m-%d %H:%M:%S").ok()
}

/// ENote MCP Server
#[derive(Clone)]
pub struct ENoteMcpServer {
    store: Arc<dyn AccessStore>,
    settings: Arc<HashMap<String, String>>,
    tool_router: ToolRouter<Self>,
}

// ============================================================================
// Tool 参数定义
// ============================================================================

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "搜索笔记参数")]
pub struct SearchNotesParams {
    #[schemars(description = "搜索关键词（匹配标题和内容）")]
    keyword: Option<String>,
    #[schemars(description = "按笔记本 ID 过滤（0 或不传表示不过滤）")]
    notebook_id: Option<i64>,
    #[schemars(description = "按标签 ID 过滤（0 或不传表示不过滤）")]
    tag_id: Option<i64>,
    #[schemars(description = "页码，从 1 开始（默认 1）")]
    page: Option<i64>,
    #[schemars(description = "每页数量（默认 20，最大 50）")]
    page_size: Option<i64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "获取笔记参数")]
pub struct GetNoteParams {
    #[schemars(description = "笔记 ID")]
    note_id: i64,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "创建笔记参数")]
pub struct CreateNoteParams {
    #[schemars(description = "笔记标题")]
    title: String,
    #[schemars(description = "笔记内容（HTML 格式）")]
    content: Option<String>,
    #[schemars(description = "笔记本 ID（0 表示不归属任何笔记本）")]
    notebook_id: Option<i64>,
    #[schemars(description = "标签 ID 列表")]
    tag_ids: Option<Vec<i64>>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "更新笔记参数")]
pub struct UpdateNoteParams {
    #[schemars(description = "笔记 ID")]
    note_id: i64,
    #[schemars(description = "新标题（不传则不修改）")]
    title: Option<String>,
    #[schemars(description = "新内容（HTML 格式，不传则不修改）")]
    content: Option<String>,
    #[schemars(description = "新笔记本 ID（不传则不修改）")]
    notebook_id: Option<i64>,
    #[schemars(description = "新标签 ID 列表（不传则不修改）")]
    tag_ids: Option<Vec<i64>>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "删除笔记参数")]
pub struct DeleteNoteParams {
    #[schemars(description = "笔记 ID")]
    note_id: i64,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "创建笔记本参数")]
pub struct CreateNotebookParams {
    #[schemars(description = "笔记本名称")]
    name: String,
    #[schemars(description = "笔记本描述")]
    description: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "创建标签参数")]
pub struct CreateTagParams {
    #[schemars(description = "标签名称")]
    name: String,
}

// ============================================================================
// 待办 Tool 参数定义
// ============================================================================

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "搜索待办参数")]
pub struct SearchTodosParams {
    #[schemars(description = "搜索关键词（匹配标题和备注）")]
    keyword: Option<String>,
    #[schemars(description = "按清单 ID 过滤")]
    list_id: Option<i64>,
    #[schemars(description = "按标签 ID 过滤")]
    tag_id: Option<i64>,
    #[schemars(description = "完成状态过滤：true=已完成，false=未完成，不传=全部")]
    completed: Option<bool>,
    #[schemars(description = "视图：all / today / planned / overdue，默认 all")]
    view: Option<String>,
    #[schemars(description = "页码，从 1 开始（默认 1）")]
    page: Option<i64>,
    #[schemars(description = "每页数量（默认 20，最大 50）")]
    page_size: Option<i64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "获取待办参数")]
pub struct GetTodoParams {
    #[schemars(description = "待办 ID")]
    todo_id: i64,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "创建待办参数")]
pub struct CreateTodoParams {
    #[schemars(description = "待办标题")]
    title: String,
    #[schemars(description = "备注")]
    description: Option<String>,
    #[schemars(description = "优先级：0=无, 1=低, 2=中, 3=高（默认 0）")]
    priority: Option<i32>,
    #[schemars(description = "截止日期，格式 YYYY-MM-DD HH:MM:SS")]
    due_date: Option<String>,
    #[schemars(description = "所属清单 ID（不传表示未归类）")]
    list_id: Option<i64>,
    #[schemars(description = "标签 ID 列表")]
    tag_ids: Option<Vec<i64>>,
    #[schemars(description = "关联笔记 ID")]
    note_id: Option<i64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "更新待办参数")]
pub struct UpdateTodoParams {
    #[schemars(description = "待办 ID")]
    todo_id: i64,
    #[schemars(description = "新标题（不传则不修改）")]
    title: Option<String>,
    #[schemars(description = "新备注（不传则不修改）")]
    description: Option<String>,
    #[schemars(description = "优先级：0=无, 1=低, 2=中, 3=高（不传则不修改）")]
    priority: Option<i32>,
    #[schemars(description = "截止日期，格式 YYYY-MM-DD HH:MM:SS（不传则不修改）")]
    due_date: Option<String>,
    #[schemars(description = "所属清单 ID（不传则不修改）")]
    list_id: Option<i64>,
    #[schemars(description = "标签 ID 列表（不传则不修改）")]
    tag_ids: Option<Vec<i64>>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "切换待办完成状态参数")]
pub struct CompleteTodoParams {
    #[schemars(description = "待办 ID")]
    todo_id: i64,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "删除待办参数")]
pub struct DeleteTodoParams {
    #[schemars(description = "待办 ID")]
    todo_id: i64,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "创建待办清单参数")]
pub struct CreateTodoListParams {
    #[schemars(description = "清单名称")]
    name: String,
    #[schemars(description = "图标标识")]
    icon: Option<String>,
    #[schemars(description = "主题色")]
    color: Option<String>,
}

// ============================================================================
// Tool 实现
// ============================================================================

/// 去除 HTML 标签，生成纯文本摘要
fn strip_html(html: &str, max_len: usize) -> String {
    let mut text = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => text.push(ch),
            _ => {}
        }
    }
    let text = text.trim().to_string();
    if text.len() > max_len {
        format!("{}...", &text[..max_len])
    } else {
        text
    }
}

/// 笔记摘要（搜索结果用）
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NoteSummary {
    id: i64,
    title: String,
    snippet: String,
    notebook_name: String,
    tags: Vec<String>,
    update_time: Option<String>,
}

impl From<Note> for NoteSummary {
    fn from(note: Note) -> Self {
        Self {
            id: note.id,
            title: note.title,
            snippet: strip_html(&note.content, 200),
            notebook_name: note.notebook_name,
            tags: note.tags.iter().map(|t| t.name.clone()).collect(),
            update_time: note
                .update_time
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

/// 搜索结果
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchResult {
    total: i64,
    total_pages: i64,
    notes: Vec<NoteSummary>,
}

/// 笔记详情
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NoteDetail {
    id: i64,
    title: String,
    content: String,
    notebook_id: i64,
    notebook_name: String,
    content_type: i32,
    is_pinned: i32,
    tags: Vec<TagInfo>,
    create_time: Option<String>,
    update_time: Option<String>,
}

#[derive(Serialize)]
struct TagInfo {
    id: i64,
    name: String,
}

impl From<Note> for NoteDetail {
    fn from(note: Note) -> Self {
        Self {
            id: note.id,
            title: note.title,
            content: note.content,
            notebook_id: note.notebook_id,
            notebook_name: note.notebook_name,
            content_type: note.content_type,
            is_pinned: note.is_pinned,
            tags: note
                .tags
                .iter()
                .map(|t| TagInfo {
                    id: t.id,
                    name: t.name.clone(),
                })
                .collect(),
            create_time: note
                .create_time
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: note
                .update_time
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

#[tool_router]
impl ENoteMcpServer {
    pub fn new(store: Arc<dyn AccessStore>, settings: HashMap<String, String>) -> Self {
        Self {
            store,
            settings: Arc::new(settings),
            tool_router: Self::tool_router(),
        }
    }

    // ---- 笔记操作 ----

    #[tool(
        description = "搜索笔记。支持关键词搜索、按笔记本或标签过滤，返回分页结果（标题+摘要）。注意：设置了 AI 访问禁止的笔记不会出现在结果中"
    )]
    async fn search_notes(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<SearchNotesParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "search_notes").await?;
        let mut search_param = NoteSearchPageParam {
            page_param: PageParam {
                page_index: params.page.unwrap_or(1),
                page_size: params.page_size.unwrap_or(20).min(50),
            },
            notebook_id: params.notebook_id.unwrap_or(0),
            tag_id: params.tag_id.unwrap_or(0),
            keyword: params.keyword.unwrap_or_default(),
            sort_field: String::new(),
            sort_order: String::new(),
            is_starred: false,
        };
        search_param.normalize();

        let result = self
            .store
            .search_page_notes(&search_param)
            .await
            .map_err(|e| McpError::internal_error(format!("搜索失败: {}", e), None))?;

        // 后置过滤：根据 MCP 访问控制过滤掉 Deny 的笔记
        let access_map = self
            .store
            .resolve_notes_access(&result.data)
            .await
            .map_err(|e| McpError::internal_error(format!("权限解析失败: {}", e), None))?;

        let filtered_notes: Vec<NoteSummary> = result
            .data
            .into_iter()
            .filter(|note| {
                access_map
                    .get(&note.id)
                    .map(|p| *p != McpPermission::Deny)
                    .unwrap_or(true)
            })
            .map(NoteSummary::from)
            .collect();

        let search_result = SearchResult {
            total: result.total,
            total_pages: result.total_pages,
            notes: filtered_notes,
        };

        let content = ContentBlock::json(search_result)
            .map_err(|e| McpError::internal_error(format!("JSON 序列化失败: {}", e), None))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "获取笔记详情，返回完整的笔记内容（包括 HTML 内容、标签、笔记本信息）")]
    async fn get_note(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<GetNoteParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "get_note").await?;

        // 检查读取权限
        self.store
            .check_note_read(params.note_id)
            .await
            .map_err(|e| McpError::invalid_request(format!("{}", e), None))?;

        let note = self
            .store
            .find_note_by_id(params.note_id)
            .await
            .map_err(|e| McpError::internal_error(format!("查询失败: {}", e), None))?;

        match note {
            Some(note) => {
                let detail = NoteDetail::from(note);
                let content = ContentBlock::json(detail).map_err(|e| {
                    McpError::internal_error(format!("JSON 序列化失败: {}", e), None)
                })?;
                Ok(CallToolResult::success(vec![content]))
            }
            None => Ok(CallToolResult::success(vec![ContentBlock::text(format!(
                "笔记 ID {} 不存在",
                params.note_id
            ))])),
        }
    }

    #[tool(description = "创建新笔记。需要标题，可选内容（HTML）、笔记本 ID 和标签 ID 列表")]
    async fn create_note(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<CreateNoteParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "create_note").await?;

        // 检查目标笔记本写入权限
        let notebook_id = params.notebook_id.unwrap_or(0);
        self.store
            .check_notebook_write(notebook_id)
            .await
            .map_err(|e| McpError::invalid_request(format!("{}", e), None))?;

        let tags: Vec<Tag> = params
            .tag_ids
            .unwrap_or_default()
            .into_iter()
            .map(|id| Tag {
                id,
                ..Default::default()
            })
            .collect();

        let note = Note {
            title: params.title,
            content: params.content.unwrap_or_default(),
            notebook_id,
            tags,
            ..Default::default()
        };

        // 通过带 source 的写方法记录 AI 操作来源（Database 后端写入历史）
        let result = self
            .store
            .create_note_with_source(&note, OperateSource::Mcp)
            .await
            .map_err(|e| McpError::internal_error(format!("创建失败: {}", e), None))?;

        match result {
            Some(note) => {
                let msg = format!("笔记创建成功，ID: {}，标题: {}", note.id, note.title);
                Ok(CallToolResult::success(vec![ContentBlock::text(msg)]))
            }
            None => Ok(CallToolResult::success(vec![ContentBlock::text(
                "创建后未找到笔记",
            )])),
        }
    }

    #[tool(description = "更新已有笔记的标题、内容、笔记本或标签。只需传入要修改的字段")]
    async fn update_note(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<UpdateNoteParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "update_note").await?;

        // 检查写入权限
        self.store
            .check_note_write(params.note_id)
            .await
            .map_err(|e| McpError::invalid_request(format!("{}", e), None))?;

        // 先获取现有笔记
        let existing = self
            .store
            .find_note_by_id(params.note_id)
            .await
            .map_err(|e| McpError::internal_error(format!("查询失败: {}", e), None))?;

        let Some(existing) = existing else {
            return Ok(CallToolResult::success(vec![ContentBlock::text(format!(
                "笔记 ID {} 不存在",
                params.note_id
            ))]));
        };

        let tags = if let Some(tag_ids) = params.tag_ids {
            tag_ids
                .into_iter()
                .map(|id| Tag {
                    id,
                    ..Default::default()
                })
                .collect()
        } else {
            existing.tags
        };

        let note = Note {
            id: params.note_id,
            title: params.title.unwrap_or(existing.title),
            content: params.content.unwrap_or(existing.content),
            notebook_id: params.notebook_id.unwrap_or(existing.notebook_id),
            content_type: existing.content_type,
            is_pinned: existing.is_pinned,
            tags,
            ..Default::default()
        };

        let result = self
            .store
            .update_note_with_source(&note, OperateSource::Mcp)
            .await
            .map_err(|e| McpError::internal_error(format!("更新失败: {}", e), None))?;

        match result {
            Some(note) => {
                let msg = format!("笔记更新成功，ID: {}，标题: {}", note.id, note.title);
                Ok(CallToolResult::success(vec![ContentBlock::text(msg)]))
            }
            None => Ok(CallToolResult::success(vec![ContentBlock::text(
                "更新后未找到笔记",
            )])),
        }
    }

    #[tool(description = "删除笔记（移入回收站，可恢复）")]
    async fn delete_note(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<DeleteNoteParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "delete_note").await?;

        // 检查写入权限
        self.store
            .check_note_write(params.note_id)
            .await
            .map_err(|e| McpError::invalid_request(format!("{}", e), None))?;

        self.store
            .delete_note_by_id(params.note_id)
            .await
            .map_err(|e| McpError::internal_error(format!("删除失败: {}", e), None))?;

        Ok(CallToolResult::success(vec![ContentBlock::text(format!(
            "笔记 ID {} 已移入回收站",
            params.note_id
        ))]))
    }

    // ---- 笔记本操作 ----

    #[tool(description = "列出所有笔记本（包含 AI 访问权限设置）")]
    async fn list_notebooks(&self) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "list_notebooks").await?;
        let notebooks = self
            .store
            .find_all_notebooks()
            .await
            .map_err(|e| McpError::internal_error(format!("查询失败: {}", e), None))?;

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct NotebookInfo {
            id: i64,
            name: String,
            description: String,
            mcp_access: i32,
        }

        let list: Vec<NotebookInfo> = notebooks
            .into_iter()
            .map(|n| NotebookInfo {
                id: n.id,
                name: n.name,
                description: n.description,
                mcp_access: n.mcp_access,
            })
            .collect();

        let content = ContentBlock::json(list)
            .map_err(|e| McpError::internal_error(format!("JSON 序列化失败: {}", e), None))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "创建新笔记本")]
    async fn create_notebook(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<CreateNotebookParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "create_notebook").await?;
        let notebook = Notebook {
            name: params.name,
            description: params.description.unwrap_or_default(),
            ..Default::default()
        };

        let result = self
            .store
            .create_notebook(&notebook)
            .await
            .map_err(|e| McpError::internal_error(format!("创建失败: {}", e), None))?;

        match result {
            Some(nb) => {
                let msg = format!("笔记本创建成功，ID: {}，名称: {}", nb.id, nb.name);
                Ok(CallToolResult::success(vec![ContentBlock::text(msg)]))
            }
            None => Ok(CallToolResult::success(vec![ContentBlock::text(
                "创建后未找到笔记本",
            )])),
        }
    }

    // ---- 标签操作 ----

    #[tool(description = "列出所有标签（包含 AI 访问权限设置）")]
    async fn list_tags(&self) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "list_tags").await?;
        let tags = self
            .store
            .find_all_tags()
            .await
            .map_err(|e| McpError::internal_error(format!("查询失败: {}", e), None))?;

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct TagAccessInfo {
            id: i64,
            name: String,
            mcp_access: i32,
        }

        let list: Vec<TagAccessInfo> = tags
            .into_iter()
            .map(|t| TagAccessInfo {
                id: t.id,
                name: t.name,
                mcp_access: t.mcp_access,
            })
            .collect();

        let content = ContentBlock::json(list)
            .map_err(|e| McpError::internal_error(format!("JSON 序列化失败: {}", e), None))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "创建新标签")]
    async fn create_tag(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<CreateTagParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "create_tag").await?;
        let tag = Tag {
            name: params.name,
            ..Default::default()
        };

        let result = self
            .store
            .create_tag(&tag)
            .await
            .map_err(|e| McpError::internal_error(format!("创建失败: {}", e), None))?;

        match result {
            Some(t) => {
                let msg = format!("标签创建成功，ID: {}，名称: {}", t.id, t.name);
                Ok(CallToolResult::success(vec![ContentBlock::text(msg)]))
            }
            None => Ok(CallToolResult::success(vec![ContentBlock::text(
                "创建后未找到标签",
            )])),
        }
    }

    // ---- 统计 ----

    #[tool(description = "获取笔记统计信息（总数和各笔记本的笔记数量）")]
    async fn note_stats(&self) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "note_stats").await?;
        let search_param = NoteSearchPageParam::default();
        let stats = self
            .store
            .note_stats(&search_param)
            .await
            .map_err(|e| McpError::internal_error(format!("统计失败: {}", e), None))?;

        let content = ContentBlock::json(stats)
            .map_err(|e| McpError::internal_error(format!("JSON 序列化失败: {}", e), None))?;
        Ok(CallToolResult::success(vec![content]))
    }

    // ========================================================================
    // 待办工具
    // ========================================================================

    #[tool(
        description = "搜索待办。支持关键词、清单、标签、完成状态与视图（今天/计划/已过期）过滤，返回分页结果。注意：AI 访问被禁止的待办不会出现在结果中"
    )]
    async fn search_todos(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<SearchTodosParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "search_todos").await?;

        let search_param = TodoSearchParam {
            keyword: params.keyword.unwrap_or_default(),
            list_id: params.list_id,
            tag_id: params.tag_id,
            completed: params.completed,
            view: params.view.unwrap_or_else(|| "all".to_string()),
            priority: None,
            deleted: false,
            sort_field: "manual".to_string(),
            sort_order: "desc".to_string(),
            page_index: params.page.unwrap_or(1) as i32,
            page_size: params.page_size.unwrap_or(20).min(50) as i32,
        };

        let todos = self
            .store
            .search_todos(&search_param)
            .await
            .map_err(|e| McpError::internal_error(format!("搜索待办失败: {}", e), None))?;

        // 后置过滤：剔除 AI 禁止访问的待办
        let access_map = self
            .store
            .resolve_todos_access(&todos)
            .await
            .map_err(|e| McpError::internal_error(format!("权限解析失败: {}", e), None))?;

        let filtered: Vec<Todo> = todos
            .into_iter()
            .filter(|todo| {
                access_map
                    .get(&todo.id)
                    .map(|p| *p != McpPermission::Deny)
                    .unwrap_or(true)
            })
            .collect();

        let content = ContentBlock::json(filtered)
            .map_err(|e| McpError::internal_error(format!("JSON 序列化失败: {}", e), None))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "获取待办详情。注意：AI 访问被禁止的待办无法获取")]
    async fn get_todo(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<GetTodoParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "get_todo").await?;
        self.store
            .check_todo_read(params.todo_id)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let todo = self
            .store
            .find_todo(params.todo_id)
            .await
            .map_err(|e| McpError::internal_error(format!("查询失败: {}", e), None))?
            .ok_or_else(|| McpError::invalid_request("待办不存在", None))?;

        let content = ContentBlock::json(todo)
            .map_err(|e| McpError::internal_error(format!("JSON 序列化失败: {}", e), None))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "创建待办。可指定优先级、截止日期、清单、标签与关联笔记")]
    async fn create_todo(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<CreateTodoParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "create_todo").await?;

        // 检查目标清单是否可写
        if let Some(list_id) = params.list_id {
            self.store
                .check_todo_list_write(Some(list_id))
                .await
                .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        }

        let todo = Todo {
            title: params.title,
            description: params.description.unwrap_or_default(),
            priority: params.priority.unwrap_or(0),
            due_date: params.due_date.as_deref().and_then(parse_dt_opt),
            list_id: params.list_id,
            note_id: params.note_id,
            ..Default::default()
        };

        let created = self
            .store
            .create_todo(&todo)
            .await
            .map_err(|e| McpError::internal_error(format!("创建失败: {}", e), None))?;

        // 关联标签
        if let Some(tag_ids) = params.tag_ids {
            if !tag_ids.is_empty() {
                self.store
                    .set_todo_tags(created.id, tag_ids)
                    .await
                    .map_err(|e| McpError::internal_error(format!("设置标签失败: {}", e), None))?;
            }
        }

        Ok(CallToolResult::success(vec![ContentBlock::text(format!(
            "待办创建成功，ID: {}，标题: {}",
            created.id, created.title
        ))]))
    }

    #[tool(description = "更新待办。仅更新传入的字段，未传字段保持不变")]
    async fn update_todo(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<UpdateTodoParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "update_todo").await?;
        self.store
            .check_todo_write(params.todo_id)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let mut todo = self
            .store
            .find_todo(params.todo_id)
            .await
            .map_err(|e| McpError::internal_error(format!("查询失败: {}", e), None))?
            .ok_or_else(|| McpError::invalid_request("待办不存在", None))?;

        if let Some(title) = params.title {
            todo.title = title;
        }
        if let Some(description) = params.description {
            todo.description = description;
        }
        if let Some(priority) = params.priority {
            todo.priority = priority;
        }
        if let Some(due_date) = params.due_date {
            todo.due_date = parse_dt_opt(&due_date);
        }
        if let Some(list_id) = params.list_id {
            todo.list_id = Some(list_id);
        }

        let updated = self
            .store
            .update_todo(&todo)
            .await
            .map_err(|e| McpError::internal_error(format!("更新失败: {}", e), None))?;

        // 关联标签（传入则全量替换）
        if let Some(tag_ids) = params.tag_ids {
            self.store
                .set_todo_tags(params.todo_id, tag_ids)
                .await
                .map_err(|e| McpError::internal_error(format!("设置标签失败: {}", e), None))?;
        }

        match updated {
            Some(t) => Ok(CallToolResult::success(vec![ContentBlock::text(format!(
                "待办更新成功，ID: {}，标题: {}",
                t.id, t.title
            ))])),
            None => Ok(CallToolResult::success(vec![ContentBlock::text(
                "更新后未找到待办",
            )])),
        }
    }

    #[tool(
        description = "切换待办完成状态。若待办设置了重复规则，完成时会顺延到下一个周期而非真正完成"
    )]
    async fn complete_todo(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<CompleteTodoParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "complete_todo").await?;
        self.store
            .check_todo_write(params.todo_id)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let updated = self
            .store
            .toggle_todo(params.todo_id)
            .await
            .map_err(|e| McpError::internal_error(format!("切换失败: {}", e), None))?;

        match updated {
            Some(t) => Ok(CallToolResult::success(vec![ContentBlock::text(format!(
                "待办 ID {} 当前状态：{}",
                t.id,
                if t.is_completed == 1 {
                    "已完成"
                } else {
                    "未完成"
                }
            ))])),
            None => Ok(CallToolResult::success(vec![ContentBlock::text(
                "待办不存在",
            )])),
        }
    }

    #[tool(description = "删除待办（移入回收站，可在应用内恢复）")]
    async fn delete_todo(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<DeleteTodoParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "delete_todo").await?;
        self.store
            .check_todo_write(params.todo_id)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        self.store
            .delete_todo(params.todo_id)
            .await
            .map_err(|e| McpError::internal_error(format!("删除失败: {}", e), None))?;

        Ok(CallToolResult::success(vec![ContentBlock::text(format!(
            "待办 ID {} 已移入回收站",
            params.todo_id
        ))]))
    }

    #[tool(description = "列出所有待办清单")]
    async fn list_todo_lists(&self) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "list_todo_lists").await?;

        let lists = self
            .store
            .find_all_todo_lists()
            .await
            .map_err(|e| McpError::internal_error(format!("查询失败: {}", e), None))?;

        let content = ContentBlock::json(lists)
            .map_err(|e| McpError::internal_error(format!("JSON 序列化失败: {}", e), None))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "创建待办清单")]
    async fn create_todo_list(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<CreateTodoListParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "create_todo_list").await?;

        let list = TodoList {
            name: params.name,
            icon: params.icon.unwrap_or_default(),
            color: params.color.unwrap_or_default(),
            ..Default::default()
        };

        let created = self
            .store
            .create_todo_list(&list)
            .await
            .map_err(|e| McpError::internal_error(format!("创建失败: {}", e), None))?;

        Ok(CallToolResult::success(vec![ContentBlock::text(format!(
            "待办清单创建成功，ID: {}，名称: {}",
            created.id, created.name
        ))]))
    }

    #[tool(description = "待办统计（总数、已完成、未完成、今日到期、已过期、完成率）")]
    async fn todo_stats(&self) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.settings, "todo_stats").await?;

        let stats = self
            .store
            .todo_stats()
            .await
            .map_err(|e| McpError::internal_error(format!("统计失败: {}", e), None))?;

        let content = ContentBlock::json(stats)
            .map_err(|e| McpError::internal_error(format!("JSON 序列化失败: {}", e), None))?;
        Ok(CallToolResult::success(vec![content]))
    }
}

// ============================================================================
// ServerHandler 实现
// ============================================================================

impl ServerHandler for ENoteMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(ProtocolVersion::V_2024_11_05)
            .with_server_info(
                Implementation::new("enote-mcp", env!("CARGO_PKG_VERSION"))
                    .with_title("ENote MCP Server")
                    .with_description("笔记管理 MCP 服务器"),
            )
            .with_instructions(
                "ENote MCP Server - 笔记管理工具。\n\
                 支持的操作：\n\
                 - search_notes: 搜索笔记\n\
                 - get_note: 获取笔记详情\n\
                 - create_note: 创建笔记\n\
                 - update_note: 更新笔记\n\
                 - delete_note: 删除笔记（移入回收站）\n\
                 - list_notebooks: 列出笔记本\n\
                 - create_notebook: 创建笔记本\n\
                 - list_tags: 列出标签\n\
                 - create_tag: 创建标签\n\
                 - note_stats: 笔记统计\n\
                 - search_todos: 搜索待办\n\
                 - get_todo: 获取待办详情\n\
                 - create_todo: 创建待办\n\
                 - update_todo: 更新待办\n\
                 - complete_todo: 切换待办完成状态\n\
                 - delete_todo: 删除待办（移入回收站）\n\
                 - list_todo_lists: 列出待办清单\n\
                 - create_todo_list: 创建待办清单\n\
                 - todo_stats: 待办统计\n\
                 \n\
                 访问控制说明：\n\
                 笔记本、标签和笔记都有 mcp_access 权限设置：\n\
                 0=继承上层, 1=读写, 2=只读, 3=禁止。\n\
                 加密笔记始终禁止 AI 访问。\n\
                 待办访问控制：\n\
                 待办、待办清单、标签也都有 mcp_access 权限设置（取值同上）。\n\
                 若待办关联的笔记被禁止，则该待办连带禁止（安全优先）。"
                    .to_string(),
            )
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        tracing::info!("MCP client connected");
        Ok(self.get_info())
    }

    /// 返回可用工具列表，根据设置动态过滤
    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let enabled = get_enabled_tools(&self.settings);
        let tools: Vec<Tool> = self
            .tool_router
            .list_all()
            .into_iter()
            .filter(|t| enabled.contains(t.name.as_ref()))
            .collect();
        Ok(ListToolsResult::with_all_items(tools))
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        self.tool_router.get(name).cloned()
    }

    /// 调用工具（委托给 tool_router）
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, McpError> {
        let ctx = ToolCallContext::new(self, request, context);
        self.tool_router.call(ctx).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings(enabled: &str, tools: Option<&str>, known: Option<&str>) -> HashMap<String, String> {
        let mut s = HashMap::new();
        s.insert("mcpEnabled".to_string(), enabled.to_string());
        if let Some(v) = tools {
            s.insert("mcpEnabledTools".to_string(), v.to_string());
        }
        if let Some(v) = known {
            s.insert("mcpKnownTools".to_string(), v.to_string());
        }
        s
    }

    #[test]
    fn disabled_by_switch_returns_no_tools() {
        assert!(get_enabled_tools(&settings("0", None, None)).is_empty());
        assert!(get_enabled_tools(&settings("", Some("search_notes"), None)).is_empty());
    }

    #[test]
    fn no_saved_selection_enables_all_defaults() {
        let enabled = get_enabled_tools(&settings("1", None, None));
        assert_eq!(enabled.len(), DEFAULT_TOOLS.len());
        assert!(DEFAULT_TOOLS.iter().all(|t| enabled.contains(*t)));
    }

    #[test]
    fn upgrade_enables_new_tools_automatically() {
        // 模拟旧版本保存的设置：当时只有 5 个工具，且 known 也是那 5 个
        let old = "search_notes,get_note,create_note,update_note,delete_note";
        let enabled = get_enabled_tools(&settings("1", Some(old), Some(old)));
        // 旧工具保留，其余 14 个新增工具自动启用
        assert_eq!(enabled.len(), DEFAULT_TOOLS.len());
        assert!(DEFAULT_TOOLS.iter().all(|t| enabled.contains(*t)));
    }

    #[test]
    fn explicit_disable_survives_upgrade() {
        // known 含全部工具 → 没有"新增工具"，用户显式关闭的应保持关闭
        let all = DEFAULT_TOOLS.join(",");
        let selected = DEFAULT_TOOLS
            .iter()
            .filter(|t| **t != "create_note")
            .copied()
            .collect::<Vec<_>>()
            .join(",");
        let enabled = get_enabled_tools(&settings("1", Some(&selected), Some(&all)));
        assert!(!enabled.contains("create_note"));
        assert_eq!(enabled.len(), DEFAULT_TOOLS.len() - 1);
    }

    #[test]
    fn missing_known_tools_defaults_to_all_new() {
        // 老版本没写 mcpKnownTools → 全部视为新增，不能让老用户升级后无工具可用
        let enabled = get_enabled_tools(&settings(
            "1",
            Some("search_notes"),
            None,
        ));
        assert_eq!(enabled.len(), DEFAULT_TOOLS.len());
    }
}
