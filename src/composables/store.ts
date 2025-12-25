import { ref, reactive, shallowRef } from 'vue'
import { ContentType } from '../types'
import type { AppState, ShowNotebook, ShowTag, ShowNote, NoteHistory } from '../types'

// 全局共享状态 - 单例模式
// 使用 shallowRef 优化大数组性能（避免深层响应式追踪）
export const notebooks = shallowRef<ShowNotebook[]>([
  { id: '0', name: '全部', count: 0, icon: 'BookOpen' },
])
export const tags = shallowRef<ShowTag[]>([{ id: '0', name: '全部', icon: 'Tags' }])
export const notes = shallowRef<ShowNote[]>([])
export const query = ref<string>('')
export const histories = shallowRef<NoteHistory[]>([])

export const state = reactive<AppState>({
  notePageIndex: 1,
  notePageSize: 10,
  noteTotal: 0,
  activeNotebook: '',
  activeTag: '',
  activeNote: null,
  noteSearchPageParam: {
    pageIndex: 1,
    pageSize: 50,
    notebookId: 0,
    tagId: 0,
    keyword: '',
  },
  editMode: false,
  loading: false,
  historyPageIndex: 1,
  historyPageSize: 50,
  historyTotal: 0,
})

// 默认笔记模板
export const createDefaultNote = (notebookId: string, timestamp: number): ShowNote => ({
  id: '0-' + timestamp,
  notebookId,
  title: '',
  content: '',
  contentType: ContentType.Html,
  tags: [],
  createTime: new Date(timestamp).toISOString().replace('T', ' ').slice(0, 19),
  updateTime: new Date(timestamp).toISOString().replace('T', ' ').slice(0, 19),
})
