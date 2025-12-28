import { computed } from 'vue'
import { useAppStore } from '../stores/app'
import type { AppState, ShowNotebook, ShowTag, ShowNote, NoteHistory } from '../types'

/**
 * 向后兼容层
 * 现有代码可以继续使用这些导出，内部已切换到 Pinia
 */

// 创建一个延迟获取 store 的函数
let _store: ReturnType<typeof useAppStore> | null = null
const getStore = () => {
  if (!_store) {
    _store = useAppStore()
  }
  return _store
}

// 导出兼容的响应式引用
// 使用 computed 包装以保持响应式
export const notebooks = computed<ShowNotebook[]>(() => getStore().notebooks)
export const tags = computed<ShowTag[]>(() => getStore().tags)
export const notes = computed<ShowNote[]>(() => getStore().notes)
export const query = computed({
  get: () => getStore().query,
  set: (val: string) => {
    getStore().query = val
  },
})
export const histories = computed<NoteHistory[]>(() => getStore().histories)
export const editingNote = computed({
  get: () => getStore().editingNote,
  set: (val: ShowNote | null) => {
    getStore().editingNote = val
  },
})

// 兼容的 state 对象
// 使用 Proxy 实现双向绑定到 Pinia store
export const state = new Proxy({} as AppState, {
  get(_, prop: keyof AppState) {
    const store = getStore()
    switch (prop) {
      case 'notePageIndex':
        return store.notePageIndex
      case 'notePageSize':
        return store.notePageSize
      case 'noteTotal':
        return store.noteTotal
      case 'activeNotebook':
        return store.activeNotebook
      case 'activeTag':
        return store.activeTag
      case 'activeNote':
        return store.activeNote
      case 'noteSearchPageParam':
        return store.noteSearchPageParam
      case 'editMode':
        return store.editMode
      case 'loading':
        return store.loading
      case 'historyPageIndex':
        return store.historyPageIndex
      case 'historyPageSize':
        return store.historyPageSize
      case 'historyTotal':
        return store.historyTotal
      case 'historyLoading':
        return store.historyLoading
      default:
        return undefined
    }
  },
  set(_, prop: keyof AppState, value) {
    const store = getStore()
    switch (prop) {
      case 'notePageIndex':
        store.notePageIndex = value as number
        break
      case 'notePageSize':
        store.notePageSize = value as number
        break
      case 'noteTotal':
        store.noteTotal = value as number
        break
      case 'activeNotebook':
        store.activeNotebook = value as string
        break
      case 'activeTag':
        store.activeTag = value as string
        break
      case 'activeNote':
        store.activeNote = value as string | null
        break
      case 'noteSearchPageParam':
        Object.assign(store.noteSearchPageParam, value)
        break
      case 'editMode':
        store.editMode = value as boolean
        break
      case 'loading':
        store.loading = value as boolean
        break
      case 'historyPageIndex':
        store.historyPageIndex = value as number
        break
      case 'historyPageSize':
        store.historyPageSize = value as number
        break
      case 'historyTotal':
        store.historyTotal = value as number
        break
      case 'historyLoading':
        store.historyLoading = value as boolean
        break
    }
    return true
  },
})

// 默认笔记模板 - 直接使用 store 的方法
export const createDefaultNote = (notebookId: string, timestamp: number): ShowNote => {
  return getStore().createDefaultNote(notebookId, timestamp)
}

// 导出 Pinia store 供需要直接使用的地方
export { useAppStore } from '../stores/app'
