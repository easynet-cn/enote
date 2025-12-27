/**
 * 应用全局配置常量
 * 集中管理各种配置值，便于统一调整和维护
 */

// ============= 内容限制 =============

/** 笔记内容最大大小（5MB） */
export const MAX_NOTE_CONTENT_SIZE = 5 * 1024 * 1024

/** 笔记标题最大长度 */
export const MAX_NOTE_TITLE_LENGTH = 200

/** 笔记本名称最大长度 */
export const MAX_NOTEBOOK_NAME_LENGTH = 50

/** 标签名称最大长度 */
export const MAX_TAG_NAME_LENGTH = 30

// ============= 缓存配置 =============

/** 预览文本缓存最大条目数 */
export const PREVIEW_CACHE_MAX_SIZE = 200

/** 搜索结果缓存有效期（毫秒） */
export const SEARCH_CACHE_TTL = 5000

// ============= 防抖/节流延迟 =============

/** 搜索防抖延迟（毫秒） */
export const SEARCH_DEBOUNCE_DELAY = 300

/** Markdown 预览更新节流延迟（毫秒） */
export const MARKDOWN_PREVIEW_THROTTLE = 100

/** 滚动同步节流延迟（毫秒，约 60fps） */
export const SCROLL_SYNC_THROTTLE = 16

// ============= 分页默认值 =============

/** 笔记列表默认每页条数 */
export const DEFAULT_NOTE_PAGE_SIZE = 10

/** 笔记搜索默认每页条数 */
export const DEFAULT_SEARCH_PAGE_SIZE = 50

/** 历史记录默认每页条数 */
export const DEFAULT_HISTORY_PAGE_SIZE = 50

/** 可选的分页大小选项 */
export const PAGE_SIZE_OPTIONS = [10, 20, 50, 100]

/** 历史记录分页大小选项 */
export const HISTORY_PAGE_SIZE_OPTIONS = [20, 50, 100, 200]

// ============= UI 配置 =============

/** NoteList 默认宽度（像素） */
export const DEFAULT_NOTE_LIST_WIDTH = 320

/** NoteList 最小宽度（像素） */
export const MIN_NOTE_LIST_WIDTH = 200

/** NoteList 最大宽度（像素） */
export const MAX_NOTE_LIST_WIDTH = 600

/** 预览文本最大长度（字符） */
export const PREVIEW_TEXT_MAX_LENGTH = 80

/** Markdown 分割面板最小比例（%） */
export const SPLIT_PANEL_MIN_RATIO = 20

/** Markdown 分割面板最大比例（%） */
export const SPLIT_PANEL_MAX_RATIO = 80

/** Markdown 分割面板默认比例（%） */
export const SPLIT_PANEL_DEFAULT_RATIO = 50

// ============= 动画配置 =============

/** 列表过渡动画持续时间（毫秒） */
export const LIST_TRANSITION_DURATION = 200

/** 折叠面板过渡持续时间（毫秒） */
export const COLLAPSE_TRANSITION_DURATION = 300

// ============= 通知配置 =============

/** 成功通知显示时长（毫秒） */
export const NOTIFICATION_SUCCESS_DURATION = 3000

/** 错误通知显示时长（毫秒） */
export const NOTIFICATION_ERROR_DURATION = 5000

/** 加载通知持续显示（0 表示不自动关闭） */
export const NOTIFICATION_LOADING_DURATION = 0
