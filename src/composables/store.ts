import { ref, reactive } from 'vue'
import { ContentType } from '../types'
import type { AppState, ShowNotebook, ShowTag, ShowNote, NoteHistory } from '../types'

// 全局共享状态 - 单例模式
export const notebooks = ref<ShowNotebook[]>([
  { id: '0', name: '全部', count: 0, icon: 'BookOpen' },
])
export const tags = ref<ShowTag[]>([{ id: '0', name: '全部', icon: 'Tags' }])
export const notes = ref<ShowNote[]>([])
export const query = ref<string>('')
export const histories = ref<NoteHistory[]>([])

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
