export interface PageResult<T> {
  total: number
  totalPages: number
  data: T[]
}

export interface Notebook {
  id: number
  parentId?: number
  name: string
  description?: string
  icon?: string
  cls?: string
  sortOrder?: number
  /** MCP 访问控制：0=继承, 1=读写, 2=只读, 3=禁止 */
  mcpAccess?: McpAccess
  count?: number
  createTime?: string | null
  updateTime?: string | null
}

export interface ShowNotebook {
  id: string
  parentId?: number
  name: string
  description?: string
  icon?: string
  cls?: string
  sortOrder?: number
  mcpAccess?: McpAccess
  count?: number
  createTime?: string | null
  updateTime?: string | null
}

export interface NotebookTreeNode extends ShowNotebook {
  children: NotebookTreeNode[]
  expanded: boolean
}

export interface Tag {
  id: number
  name: string
  icon?: string
  cls?: string
  sortOrder?: number
  /** MCP 访问控制：0=继承, 1=读写, 2=只读, 3=禁止 */
  mcpAccess?: McpAccess
  createTime?: string | null
  updateTime?: string | null
}

export interface ShowTag {
  id: string
  name: string
  icon?: string
  cls?: string
  sortOrder?: number
  mcpAccess?: McpAccess
  createTime?: string
  updateTime?: string
}

/** MCP 访问控制枚举 */
export enum McpAccess {
  /** 从上层继承（默认） */
  Inherit = 0,
  /** AI 可读可写 */
  ReadWrite = 1,
  /** AI 只能读取 */
  ReadOnly = 2,
  /** AI 完全不可访问 */
  Deny = 3,
}

/** 内容类型枚举 */
export enum ContentType {
  /** HTML 格式 */
  Html = 0,
  /** Markdown 格式 */
  Markdown = 1,
}

/** Markdown 布局类型 */
export enum MarkdownLayout {
  /** 无布局（仅源码或预览） */
  None = 'none',
  /** 上下布局（上源码，下预览） */
  Vertical = 'vertical',
  /** 左右布局（左源码，右预览） */
  Horizontal = 'horizontal',
}

export interface Note {
  id: number
  notebookId: number
  notebookName?: string
  title: string
  content: string
  /** 内容类型：0 = HTML，1 = Markdown */
  contentType: ContentType
  /** 是否置顶：0 = 否，1 = 是 */
  isPinned: number
  /** 是否收藏/星标：0 = 否，1 = 是 */
  isStarred: number
  /** MCP 访问控制：0=继承, 1=读写, 2=只读, 3=禁止 */
  mcpAccess?: McpAccess
  tags: Tag[]
  createTime: string | null
  updateTime: string | null
  /** 软删除时间 */
  deletedAt: string | null
}

export interface ShowNote {
  id: string
  notebookId: string
  notebookName?: string
  title: string
  content: string
  /** 内容类型：0 = HTML，1 = Markdown */
  contentType: ContentType
  /** 是否置顶 */
  isPinned: number
  /** 是否收藏/星标 */
  isStarred: number
  /** MCP 访问控制 */
  mcpAccess?: McpAccess
  tags: ShowTag[]
  createTime: string | null
  updateTime: string | null
  deletedAt?: string | null
}

/** 新建笔记时的部分类型（某些字段可以为空） */
export interface PartialShowNote {
  id: string
  notebookId?: string
  notebookName?: string
  title: string
  content: string
  contentType?: ContentType
  isPinned?: number
  isStarred?: number
  mcpAccess?: McpAccess
  tags?: ShowTag[]
  createTime?: string | null
  updateTime?: string | null
  deletedAt?: string | null
}

export interface NoteStatsResult {
  total: number
  notebookCounts: Map<number, number>
}

export interface NoteHistoryExtra {
  notebookId: number
  notebookName: string
  noteId: number
  title: string
  contentType: ContentType
  tags: Tag[]
}

export interface NoteHistory {
  id: string
  noteId: number
  oldContent: string
  newContent: string
  extra: NoteHistoryExtra
  operateType: number
  /** 操作来源：0=用户操作, 1=MCP操作 */
  operateSource: number
  operateTime: string
  createTime: string
}

export interface NoteSearchPageParam {
  pageIndex: number
  pageSize: number
  notebookId: number
  tagId: number
  keyword: string
  sortField: string
  sortOrder: string
  isStarred?: boolean
}

export interface NoteHistorySearchPageParam {
  pageIndex: number
  pageSize: number
  noteId: number
}

export interface NoteLink {
  id: number
  noteId: number
  noteTitle: string
  createTime: string | null
}

export interface NoteTemplate {
  id: number
  name: string
  content: string
  /** 内容类型：0 = HTML，1 = Markdown */
  contentType: ContentType
  sortOrder: number
  createTime: string | null
  updateTime: string | null
}

// ============================================================================
// 笔记附件相关类型
// ============================================================================

export interface NoteAttachment {
  id: number
  noteId: number
  fileName: string
  filePath: string
  fileSize: number
  mimeType: string
  fileHash: string
  refCount: number
  createTime: string | null
}

export interface AttachmentStats {
  totalCount: number
  uniqueFileCount: number
  totalSize: number
  orphanCount: number
}

// ============================================================================
// 待办相关类型
// ============================================================================

/** 待办优先级 */
export enum TodoPriority {
  /** 无 */
  None = 0,
  /** 低 */
  Low = 1,
  /** 中 */
  Medium = 2,
  /** 高 */
  High = 3,
}

/**
 * 待办视图类型
 *
 * `quadrant` 为前端四象限分组展示，后端不做过滤（等同于 all）
 */
export type TodoView = 'all' | 'today' | 'planned' | 'overdue' | 'quadrant'

/** 待办重复类型 */
export enum TodoRecurrence {
  /** 不重复 */
  None = 0,
  /** 每天 */
  Daily = 1,
  /** 每周 */
  Weekly = 2,
  /** 每月 */
  Monthly = 3,
  /** 每年 */
  Yearly = 4,
}

/** 待办 */
export interface Todo {
  id: number
  title: string
  description: string
  /** 是否已完成：0 = 未完成，1 = 已完成 */
  isCompleted: number
  /** 优先级：0 = 无，1 = 低，2 = 中，3 = 高 */
  priority: number
  dueDate: string | null
  completedAt: string | null
  /** 所属清单 ID，null 表示未归类 */
  listId: number | null
  sortOrder: number
  createTime: string | null
  updateTime: string | null
  /** 软删除时间，null 表示未删除 */
  deletedAt: string | null
  // ↓ P1 字段
  startDate: string | null
  remindAt: string | null
  /** 是否已提醒：0 = 未提醒，1 = 已提醒 */
  isReminded: number
  /** 父待办 ID，0 表示无父待办 */
  parentId: number
  noteId: number | null
  /** 重复类型：0 = 不重复，1 = 每天，2 = 每周，3 = 每月，4 = 每年 */
  recurrenceType: number
  recurrenceInterval: number
  recurrenceEndDate: string | null
  /** MCP 访问控制：0=继承, 1=读写, 2=只读, 3=禁止 */
  mcpAccess: number
}

/** 待办清单 */
export interface TodoList {
  id: number
  name: string
  icon: string
  color: string
  sortOrder: number
  /** MCP 访问控制：0=继承, 1=读写, 2=只读, 3=禁止 */
  mcpAccess: number
  createTime: string | null
  updateTime: string | null
}

/** 待办查询参数 */
export interface TodoSearchParam {
  keyword: string
  listId: number | null
  /** 标签 ID 过滤（通过 todo_tags 关联表） */
  tagId: number | null
  /** 完成状态过滤：null = 全部，true = 已完成，false = 未完成 */
  completed: boolean | null
  view: TodoView
  priority: number | null
  /** 是否只查询回收站（已软删除） */
  deleted: boolean
  sortField: string
  sortOrder: string
}

/** 待办统计信息 */
export interface TodoStats {
  /** 总数（不含回收站） */
  totalCount: number
  /** 已完成数 */
  completedCount: number
  /** 未完成数 */
  pendingCount: number
  /** 今日到期（未完成） */
  todayCount: number
  /** 已过期（未完成） */
  overdueCount: number
  /** 完成率（0-100） */
  completionRate: number
}

// ============================================================================
// Profile / Setup 相关类型
// ============================================================================

/** Profile 索引信息 */
export interface ProfileIndex {
  active: string
  autoConnect: boolean
}

/** SSL 配置 */
export interface SslConfig {
  mode: string
  caCert: string
  clientCert: string
  clientKey: string
}

/** 数据源配置 */
export interface DatasourceConfig {
  type: string
  path: string
  host: string
  port: number
  database: string
  username: string
  authMethod: string
  ssl: SslConfig
}

/** 安全配置 */
export interface SecurityConfig {
  contentEncryption: boolean
}

/** 服务器认证方式 */
export type ServerAuthMethod =
  | { type: 'none' }
  | { type: 'bearer' }
  | { type: 'basic'; username: string }
  | { type: 'jwt'; 'refresh-url': string; username: string }
  | { type: 'custom-header'; 'header-name': string }
  | { type: 'oauth2'; 'token-url': string; 'client-id': string; scopes: string }

/** 服务器配置 */
export interface ServerConfig {
  url: string
  'auth-method': ServerAuthMethod
  timeout: number
}

/** Profile 配置 */
export interface ProfileConfig {
  name: string
  icon: string
  /** 后端类型："database"（默认）或 "server" */
  backend?: string
  datasource: DatasourceConfig
  /** 远程服务器配置（backend = "server" 时使用） */
  server?: ServerConfig
  security: SecurityConfig
}

/** Profile 摘要信息（列表展示） */
export interface ProfileSummary {
  id: string
  name: string
  icon: string
  /** 后端类型："database" 或 "server" */
  backendType: string
  dbType: string
  connectionInfo: string
  contentEncryption: boolean
  isActive: boolean
}

// ============================================================================
// 跨 Profile 同步相关类型
// ============================================================================

/** 同步模式 */
export type SyncMode = 'append' | 'overwrite'

/** 同步范围 */
export interface SyncScope {
  notebooks: boolean
  tags: boolean
  notes: boolean
  noteHistories: boolean
  templates: boolean
  todos: boolean
  settings: boolean
}

/** 同步请求参数 */
export interface SyncOptions {
  targetProfileId: string
  mode: SyncMode
  scope: SyncScope
  autoBackup: boolean
  backupFormat?: string
}

/** 同步日志 */
export interface SyncLog {
  id: number
  sourceProfile: string
  targetProfile: string
  sourceDbType: string
  targetDbType: string
  syncMode: string
  syncScope: string
  backupFormat: string | null
  sourceBackup: string | null
  targetBackup: string | null
  status: string
  totalCount: number
  successCount: number
  failedCount: number
  errorMessage: string | null
  startedAt: string | null
  finishedAt: string | null
  createTime: string | null
}

/** 同步明细 */
export interface SyncLogDetail {
  id: number
  syncLogId: number
  tableName: string
  sourceId: number
  targetId: number | null
  recordName: string
  status: string
  errorMessage: string | null
  syncedAt: string | null
  createTime: string | null
}

/** 同步预览 */
export interface SyncPreview {
  source: DataStats
  target: DataStats
}

/** 数据统计 */
export interface DataStats {
  profileName: string
  dbType: string
  notebookCount: number
  tagCount: number
  noteCount: number
  noteHistoryCount: number
  templateCount: number
  settingsCount: number
}

/** 同步进度事件 */
export interface SyncProgress {
  syncLogId: number
  stage: string
  tableName: string
  current: number
  total: number
  recordName: string
}

// ============================================================================
// 应用日志相关类型
// ============================================================================

/** 应用日志 */
export interface AppLog {
  id: number
  level: string
  module: string
  action: string
  targetId: string | null
  targetName: string | null
  message: string
  detail: string | null
  createTime: string | null
}

/** 应用日志搜索参数 */
export interface AppLogSearchParam {
  pageIndex: number
  pageSize: number
  level?: string
  module?: string
  action?: string
  keyword: string
  startTime?: string | null
  endTime?: string | null
}

/** 日志文件信息 */
export interface LogFileInfo {
  name: string
  size: number
  isError: boolean
  modifiedTime: string | null
}

export interface CloudStorageConfig {
  provider: string
  endpoint: string
  bucket: string
  region: string
  accessKeyId: string
  secretAccessKey: string
  prefix: string
  username: string
  password: string
}

export interface CloudBackupEntry {
  name: string
  size: number
  lastModified: string
}

export interface AppState {
  notePageIndex: number
  notePageSize: number
  noteTotal: number
  activeNotebook: string
  activeTag: string
  activeNote: string | null
  noteSearchPageParam: NoteSearchPageParam
  editMode: boolean
  loading: boolean
  historyPageIndex: number
  historyPageSize: number
  historyTotal: number
  historyLoading: boolean
}
