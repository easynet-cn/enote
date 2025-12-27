import { ref, reactive, shallowRef } from 'vue'
import { ContentType } from '../types'
import type { AppState, ShowNotebook, ShowTag, ShowNote, NoteHistory } from '../types'
import {
  DEFAULT_NOTE_PAGE_SIZE,
  DEFAULT_SEARCH_PAGE_SIZE,
  DEFAULT_HISTORY_PAGE_SIZE,
} from '../config/constants'

// 全局共享状态 - 单例模式
// 使用 shallowRef 优化大数组性能（避免深层响应式追踪）
export const notebooks = shallowRef<ShowNotebook[]>([
  { id: '0', name: '全部', count: 0, icon: 'BookOpen' },
])
export const tags = shallowRef<ShowTag[]>([{ id: '0', name: '全部', icon: 'Tags' }])
export const notes = shallowRef<ShowNote[]>([])
export const query = ref<string>('')
export const histories = shallowRef<NoteHistory[]>([])

// 编辑缓冲区：编辑模式下使用独立的响应式对象，避免频繁替换 notes 数组
// 使用 reactive 实现深层响应式，编辑时只触发缓冲区更新
export const editingNote = ref<ShowNote | null>(null)

export const state = reactive<AppState>({
  notePageIndex: 1,
  notePageSize: DEFAULT_NOTE_PAGE_SIZE,
  noteTotal: 0,
  activeNotebook: '',
  activeTag: '',
  activeNote: null,
  noteSearchPageParam: {
    pageIndex: 1,
    pageSize: DEFAULT_SEARCH_PAGE_SIZE,
    notebookId: 0,
    tagId: 0,
    keyword: '',
  },
  editMode: false,
  loading: false,
  historyPageIndex: 1,
  historyPageSize: DEFAULT_HISTORY_PAGE_SIZE,
  historyTotal: 0,
  historyLoading: false,
})

// 默认笔记模板
export const createDefaultNote = (notebookId: string, timestamp: number): ShowNote => {
  const timeStr = new Date(timestamp).toISOString().replace('T', ' ').slice(0, 19)
  return {
    id: '0-' + timestamp,
    notebookId,
    notebookName: undefined,
    title: '',
    content: '',
    contentType: ContentType.Html,
    tags: [],
    createTime: timeStr,
    updateTime: timeStr,
  }
}
