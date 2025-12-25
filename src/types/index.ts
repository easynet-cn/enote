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
  count?: number
  createTime?: string | null
  updateTime?: string | null
}

export interface ShowNotebook {
  id: string
  parentId?: number
  name?: string
  description?: string
  icon?: string
  cls?: string
  sortOrder?: number
  count?: number
  createTime?: string | null
  updateTime?: string | null
}

export interface Tag {
  id: number
  name: string
  icon?: string
  cls?: string
  sortOrder?: number
  createTime?: string | null
  updateTime?: string | null
}

export interface ShowTag {
  id: string
  name: string
  icon?: string
  cls?: string
  sortOrder?: number
  createTime?: string
  updateTime?: string
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
  tags: Tag[]
  createTime: string | null
  updateTime: string | null
}

export interface ShowNote {
  id: string
  notebookId?: string
  notebookName?: string
  title: string
  content: string
  /** 内容类型：0 = HTML，1 = Markdown */
  contentType?: ContentType
  tags?: ShowTag[]
  createTime?: string | null
  updateTime?: string | null
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
  operateTime: string
  createTime: string
}

export interface NoteSearchPageParam {
  pageIndex: number
  pageSize: number
  notebookId: number
  tagId: number
  keyword: string
}

export interface NoteHistorySearchPageParam {
  pageIndex: number
  pageSize: number
  noteId: number
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
}
