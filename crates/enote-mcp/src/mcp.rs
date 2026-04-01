//! ENote MCP Server 实现
//!
//! 通过 rmcp 提供笔记操作工具给 AI 客户端

use rmcp::{
    ErrorData as McpError,
    ServerHandler,
    handler::server::tool::{ToolCallContext, ToolRouter},
    model::{
        CallToolRequestParams, CallToolResult, Content, Implementation, InitializeRequestParams,
        InitializeResult, ListToolsResult, PaginatedRequestParams, ProtocolVersion,
        ServerCapabilities, ServerInfo, Tool,
    },
    service::RequestContext,
    tool, tool_router, RoleServer,
};
use sea_orm::DatabaseConnection;
use serde::Serialize;

use enote_lib::{
    model::{McpPermission, Note, NoteSearchPageParam, Notebook, OperateSource, PageParam, Tag},
    service,
};

/// 获取已启用的工具名称集合
async fn get_enabled_tools(
    db: &DatabaseConnection,
) -> Result<std::collections::HashSet<String>, McpError> {
    let settings = service::settings::get_all(db)
        .await
        .map_err(|e| McpError::internal_error(format!("Failed to load settings: {}", e), None))?;

    // MCP 总开关关闭 → 无可用工具
    if settings.get("mcpEnabled").map(|v| v.as_str()) != Some("1") {
        return Ok(std::collections::HashSet::new());
    }

    // 返回已启用的工具集合
    if let Some(enabled_tools) = settings.get("mcpEnabledTools") {
        Ok(enabled_tools.split(',').filter(|s| !s.is_empty()).map(String::from).collect())
    } else {
        // 没有配置 → 默认全部启用
        Ok(["search_notes", "get_note", "create_note", "update_note", "delete_note",
            "list_notebooks", "create_notebook", "list_tags", "create_tag", "note_stats"]
            .iter().map(|s| s.to_string()).collect())
    }
}

/// 检查指定 MCP 工具是否已启用
async fn check_tool_enabled(db: &DatabaseConnection, tool_name: &str) -> Result<(), McpError> {
    let enabled = get_enabled_tools(db).await?;
    if !enabled.contains(tool_name) {
        return Err(McpError::invalid_request(
            format!("Tool '{}' is disabled. Enable it in Settings.", tool_name),
            None,
        ));
    }
    Ok(())
}

/// ENote MCP Server
#[derive(Clone)]
pub struct ENoteMcpServer {
    db: DatabaseConnection,
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
            update_time: note.update_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
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
            tags: note.tags.iter().map(|t| TagInfo { id: t.id, name: t.name.clone() }).collect(),
            create_time: note.create_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: note.update_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

#[tool_router]
impl ENoteMcpServer {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db: db.clone(),
            tool_router: Self::tool_router(),
        }
    }

    // ---- 笔记操作 ----

    #[tool(description = "搜索笔记。支持关键词搜索、按笔记本或标签过滤，返回分页结果（标题+摘要）。注意：设置了 AI 访问禁止的笔记不会出现在结果中")]
    async fn search_notes(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<SearchNotesParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.db, "search_notes").await?;
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

        let result = service::note::search_page(&self.db, &search_param)
            .await
            .map_err(|e| McpError::internal_error(format!("搜索失败: {}", e), None))?;

        // 后置过滤：根据 MCP 访问控制过滤掉 Deny 的笔记
        let access_map = service::mcp_access::resolve_notes_access(&self.db, &result.data)
            .await
            .map_err(|e| McpError::internal_error(format!("权限解析失败: {}", e), None))?;

        let filtered_notes: Vec<NoteSummary> = result.data
            .into_iter()
            .filter(|note| {
                access_map.get(&note.id)
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

        let content = Content::json(search_result)
            .map_err(|e| McpError::internal_error(format!("JSON 序列化失败: {}", e), None))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "获取笔记详情，返回完整的笔记内容（包括 HTML 内容、标签、笔记本信息）")]
    async fn get_note(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<GetNoteParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.db, "get_note").await?;

        // 检查读取权限
        service::mcp_access::check_read(&self.db, params.note_id)
            .await
            .map_err(|e| McpError::invalid_request(format!("{}", e), None))?;

        let note = service::note::find_by_id(&self.db, params.note_id)
            .await
            .map_err(|e| McpError::internal_error(format!("查询失败: {}", e), None))?;

        match note {
            Some(note) => {
                let detail = NoteDetail::from(note);
                let content = Content::json(detail)
                    .map_err(|e| McpError::internal_error(format!("JSON 序列化失败: {}", e), None))?;
                Ok(CallToolResult::success(vec![content]))
            }
            None => Ok(CallToolResult::success(vec![Content::text(
                format!("笔记 ID {} 不存在", params.note_id),
            )])),
        }
    }

    #[tool(description = "创建新笔记。需要标题，可选内容（HTML）、笔记本 ID 和标签 ID 列表")]
    async fn create_note(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<CreateNoteParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.db, "create_note").await?;

        // 检查目标笔记本写入权限
        let notebook_id = params.notebook_id.unwrap_or(0);
        service::mcp_access::check_notebook_write(&self.db, notebook_id)
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

        let result = service::note::create(&self.db, &note, OperateSource::Mcp)
            .await
            .map_err(|e| McpError::internal_error(format!("创建失败: {}", e), None))?;

        match result {
            Some(note) => {
                let msg = format!("笔记创建成功，ID: {}，标题: {}", note.id, note.title);
                Ok(CallToolResult::success(vec![Content::text(msg)]))
            }
            None => Ok(CallToolResult::success(vec![Content::text("创建后未找到笔记")])),
        }
    }

    #[tool(description = "更新已有笔记的标题、内容、笔记本或标签。只需传入要修改的字段")]
    async fn update_note(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<UpdateNoteParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.db, "update_note").await?;

        // 检查写入权限
        service::mcp_access::check_write(&self.db, params.note_id)
            .await
            .map_err(|e| McpError::invalid_request(format!("{}", e), None))?;

        // 先获取现有笔记
        let existing = service::note::find_by_id(&self.db, params.note_id)
            .await
            .map_err(|e| McpError::internal_error(format!("查询失败: {}", e), None))?;

        let Some(existing) = existing else {
            return Ok(CallToolResult::success(vec![Content::text(
                format!("笔记 ID {} 不存在", params.note_id),
            )]));
        };

        let tags = if let Some(tag_ids) = params.tag_ids {
            tag_ids
                .into_iter()
                .map(|id| Tag { id, ..Default::default() })
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

        let result = service::note::update(&self.db, &note, OperateSource::Mcp)
            .await
            .map_err(|e| McpError::internal_error(format!("更新失败: {}", e), None))?;

        match result {
            Some(note) => {
                let msg = format!("笔记更新成功，ID: {}，标题: {}", note.id, note.title);
                Ok(CallToolResult::success(vec![Content::text(msg)]))
            }
            None => Ok(CallToolResult::success(vec![Content::text("更新后未找到笔记")])),
        }
    }

    #[tool(description = "删除笔记（移入回收站，可恢复）")]
    async fn delete_note(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<DeleteNoteParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.db, "delete_note").await?;

        // 检查写入权限
        service::mcp_access::check_write(&self.db, params.note_id)
            .await
            .map_err(|e| McpError::invalid_request(format!("{}", e), None))?;

        service::note::delete_by_id(&self.db, params.note_id)
            .await
            .map_err(|e| McpError::internal_error(format!("删除失败: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            format!("笔记 ID {} 已移入回收站", params.note_id),
        )]))
    }

    // ---- 笔记本操作 ----

    #[tool(description = "列出所有笔记本（包含 AI 访问权限设置）")]
    async fn list_notebooks(&self) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.db, "list_notebooks").await?;
        let notebooks = service::notebook::find_all(&self.db)
            .await
            .map_err(|e| McpError::internal_error(format!("查询失败: {}", e), None))?;

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct NotebookInfo { id: i64, name: String, description: String, mcp_access: i32 }

        let list: Vec<NotebookInfo> = notebooks
            .into_iter()
            .map(|n| NotebookInfo { id: n.id, name: n.name, description: n.description, mcp_access: n.mcp_access })
            .collect();

        let content = Content::json(list)
            .map_err(|e| McpError::internal_error(format!("JSON 序列化失败: {}", e), None))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "创建新笔记本")]
    async fn create_notebook(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<CreateNotebookParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.db, "create_notebook").await?;
        let notebook = Notebook {
            name: params.name,
            description: params.description.unwrap_or_default(),
            ..Default::default()
        };

        let result = service::notebook::create(&self.db, &notebook)
            .await
            .map_err(|e| McpError::internal_error(format!("创建失败: {}", e), None))?;

        match result {
            Some(nb) => {
                let msg = format!("笔记本创建成功，ID: {}，名称: {}", nb.id, nb.name);
                Ok(CallToolResult::success(vec![Content::text(msg)]))
            }
            None => Ok(CallToolResult::success(vec![Content::text("创建后未找到笔记本")])),
        }
    }

    // ---- 标签操作 ----

    #[tool(description = "列出所有标签（包含 AI 访问权限设置）")]
    async fn list_tags(&self) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.db, "list_tags").await?;
        let tags = service::tag::find_all(&self.db)
            .await
            .map_err(|e| McpError::internal_error(format!("查询失败: {}", e), None))?;

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct TagInfo { id: i64, name: String, mcp_access: i32 }

        let list: Vec<TagInfo> = tags
            .into_iter()
            .map(|t| TagInfo { id: t.id, name: t.name, mcp_access: t.mcp_access })
            .collect();

        let content = Content::json(list)
            .map_err(|e| McpError::internal_error(format!("JSON 序列化失败: {}", e), None))?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "创建新标签")]
    async fn create_tag(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<CreateTagParams>,
    ) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.db, "create_tag").await?;
        let tag = Tag {
            name: params.name,
            ..Default::default()
        };

        let result = service::tag::create(&self.db, &tag)
            .await
            .map_err(|e| McpError::internal_error(format!("创建失败: {}", e), None))?;

        match result {
            Some(t) => {
                let msg = format!("标签创建成功，ID: {}，名称: {}", t.id, t.name);
                Ok(CallToolResult::success(vec![Content::text(msg)]))
            }
            None => Ok(CallToolResult::success(vec![Content::text("创建后未找到标签")])),
        }
    }

    // ---- 统计 ----

    #[tool(description = "获取笔记统计信息（总数和各笔记本的笔记数量）")]
    async fn note_stats(&self) -> Result<CallToolResult, McpError> {
        check_tool_enabled(&self.db, "note_stats").await?;
        let search_param = NoteSearchPageParam::default();
        let stats = service::note::stats(&self.db, &search_param)
            .await
            .map_err(|e| McpError::internal_error(format!("统计失败: {}", e), None))?;

        let content = Content::json(stats)
            .map_err(|e| McpError::internal_error(format!("JSON 序列化失败: {}", e), None))?;
        Ok(CallToolResult::success(vec![content]))
    }
}

// ============================================================================
// ServerHandler 实现
// ============================================================================

impl ServerHandler for ENoteMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "enote-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                title: Some("ENote MCP Server".to_string()),
                description: Some("笔记管理 MCP 服务器".to_string()),
                icons: None,
                website_url: None,
            },
            instructions: Some(
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
                 \n\
                 访问控制说明：\n\
                 笔记本、标签和笔记都有 mcp_access 权限设置：\n\
                 0=继承上层, 1=读写, 2=只读, 3=禁止。\n\
                 加密笔记始终禁止 AI 访问。"
                    .to_string(),
            ),
        }
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
        let enabled = get_enabled_tools(&self.db).await?;
        let tools: Vec<Tool> = self
            .tool_router
            .list_all()
            .into_iter()
            .filter(|t| enabled.contains(t.name.as_ref()))
            .collect();
        Ok(ListToolsResult { tools, next_cursor: None, meta: None })
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        self.tool_router.get(name).cloned()
    }

    /// 调用工具（委托给 tool_router）
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let ctx = ToolCallContext::new(self, request, context);
        self.tool_router.call(ctx).await
    }
}
