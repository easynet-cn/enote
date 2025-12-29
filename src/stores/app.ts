import { defineStore } from 'pinia'
import { ref, computed, toRaw } from 'vue'
import { ContentType } from '../types'
import type { ShowNotebook, ShowTag, ShowNote, NoteHistory, NoteSearchPageParam } from '../types'
import {
  DEFAULT_NOTE_PAGE_SIZE,
  DEFAULT_SEARCH_PAGE_SIZE,
  DEFAULT_HISTORY_PAGE_SIZE,
} from '../config/constants'

export const useAppStore = defineStore('app', () => {
  // ==================== 数据状态 ====================
  // 使用 Map 优化查询性能 O(1)
  const notebooksMap = ref<Map<string, ShowNotebook>>(new Map())
  const tagsMap = ref<Map<string, ShowTag>>(new Map())
  const notesMap = ref<Map<string, ShowNote>>(new Map())

  // 排序后的 ID 列表（保持顺序）
  const notebookIds = ref<string[]>(['0'])
  const tagIds = ref<string[]>(['0'])
  const noteIds = ref<string[]>([])

  // 默认项
  const defaultNotebook: ShowNotebook = { id: '0', name: '全部', count: 0, icon: 'BookOpen' }
  const defaultTag: ShowTag = { id: '0', name: '全部', icon: 'Tags' }

  // 初始化默认项
  notebooksMap.value.set('0', defaultNotebook)
  tagsMap.value.set('0', defaultTag)

  // 历史记录
  const histories = ref<NoteHistory[]>([])

  // 搜索关键词
  const query = ref<string>('')

  // 编辑缓冲区
  const editingNote = ref<ShowNote | null>(null)

  // ==================== UI 状态 ====================
  const activeNotebook = ref<string>('')
  const activeTag = ref<string>('')
  const activeNote = ref<string | null>(null)
  const editMode = ref<boolean>(false)
  const loading = ref<boolean>(false)

  // 笔记分页
  const notePageIndex = ref<number>(1)
  const notePageSize = ref<number>(DEFAULT_NOTE_PAGE_SIZE)
  const noteTotal = ref<number>(0)

  // 搜索参数
  const noteSearchPageParam = ref<NoteSearchPageParam>({
    pageIndex: 1,
    pageSize: DEFAULT_SEARCH_PAGE_SIZE,
    notebookId: 0,
    tagId: 0,
    keyword: '',
  })

  // 历史分页
  const historyPageIndex = ref<number>(1)
  const historyPageSize = ref<number>(DEFAULT_HISTORY_PAGE_SIZE)
  const historyTotal = ref<number>(0)
  const historyLoading = ref<boolean>(false)

  // ==================== Getters ====================
  // 计算属性：数组形式（兼容现有组件）
  const notebooks = computed<ShowNotebook[]>(() => {
    return notebookIds.value.map((id) => notebooksMap.value.get(id)!).filter(Boolean)
  })

  const tags = computed<ShowTag[]>(() => {
    return tagIds.value.map((id) => tagsMap.value.get(id)!).filter(Boolean)
  })

  const notes = computed<ShowNote[]>(() => {
    return noteIds.value.map((id) => notesMap.value.get(id)!).filter(Boolean)
  })

  // 当前活动笔记数据
  const activeNoteData = computed<ShowNote | null>(() => {
    if (editMode.value && editingNote.value) {
      return editingNote.value
    }
    if (!activeNote.value) return null
    return notesMap.value.get(activeNote.value) || null
  })

  // ==================== Actions ====================
  // 设置笔记本列表
  const setNotebooks = (items: ShowNotebook[]) => {
    notebooksMap.value.clear()
    notebooksMap.value.set('0', defaultNotebook)
    notebookIds.value = ['0']

    for (const item of items) {
      notebooksMap.value.set(item.id, item)
      notebookIds.value.push(item.id)
    }
  }

  // 更新笔记本
  const updateNotebook = (notebook: ShowNotebook) => {
    notebooksMap.value.set(notebook.id, notebook)
  }

  // 删除笔记本
  const removeNotebook = (id: string) => {
    notebooksMap.value.delete(id)
    notebookIds.value = notebookIds.value.filter((nid) => nid !== id)
  }

  // 设置标签列表
  const setTags = (items: ShowTag[]) => {
    tagsMap.value.clear()
    tagsMap.value.set('0', defaultTag)
    tagIds.value = ['0']

    for (const item of items) {
      tagsMap.value.set(item.id, item)
      tagIds.value.push(item.id)
    }
  }

  // 更新标签
  const updateTag = (tag: ShowTag) => {
    tagsMap.value.set(tag.id, tag)
  }

  // 删除标签
  const removeTag = (id: string) => {
    tagsMap.value.delete(id)
    tagIds.value = tagIds.value.filter((tid) => tid !== id)
  }

  // 设置笔记列表
  const setNotes = (items: ShowNote[]) => {
    notesMap.value.clear()
    noteIds.value = []

    for (const item of items) {
      notesMap.value.set(item.id, item)
      noteIds.value.push(item.id)
    }
  }

  // 添加笔记到开头
  const prependNote = (note: ShowNote) => {
    notesMap.value.set(note.id, note)
    noteIds.value = [note.id, ...noteIds.value]
  }

  // 更新笔记
  const updateNote = (note: ShowNote) => {
    notesMap.value.set(note.id, note)
  }

  // 删除笔记
  const removeNote = (id: string) => {
    notesMap.value.delete(id)
    noteIds.value = noteIds.value.filter((nid) => nid !== id)
  }

  // 根据 ID 获取笔记
  const getNoteById = (id: string): ShowNote | undefined => {
    return notesMap.value.get(id)
  }

  // 根据 ID 获取笔记本
  const getNotebookById = (id: string): ShowNotebook | undefined => {
    return notebooksMap.value.get(id)
  }

  // 根据 ID 获取标签
  const getTagById = (id: string): ShowTag | undefined => {
    return tagsMap.value.get(id)
  }

  // 设置活动笔记
  const setActiveNote = (noteId: string | null) => {
    activeNote.value = noteId
    editMode.value = false

    if (noteId) {
      const note = notesMap.value.get(noteId)
      if (note) {
        // 使用 toRaw 获取原始对象，避免 structuredClone 无法克隆响应式代理
        editingNote.value = structuredClone(toRaw(note))
      }
    } else {
      editingNote.value = null
    }
  }

  // 创建默认笔记
  const createDefaultNote = (notebookId: string, timestamp: number): ShowNote => {
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

  // 开始编辑
  const startEdit = () => {
    editMode.value = true
  }

  // 取消编辑
  const cancelEdit = () => {
    editMode.value = false
    if (activeNote.value) {
      const note = notesMap.value.get(activeNote.value)
      if (note) {
        // 使用 toRaw 获取原始对象，避免 structuredClone 无法克隆响应式代理
        editingNote.value = structuredClone(toRaw(note))
      }
    }
  }

  // 重置搜索参数
  const resetSearchParam = () => {
    noteSearchPageParam.value = {
      pageIndex: 1,
      pageSize: DEFAULT_SEARCH_PAGE_SIZE,
      notebookId: 0,
      tagId: 0,
      keyword: '',
    }
  }

  return {
    // 数据状态
    notebooks,
    tags,
    notes,
    histories,
    query,
    editingNote,

    // UI 状态
    activeNotebook,
    activeTag,
    activeNote,
    editMode,
    loading,

    // 分页状态
    notePageIndex,
    notePageSize,
    noteTotal,
    noteSearchPageParam,
    historyPageIndex,
    historyPageSize,
    historyTotal,
    historyLoading,

    // Getters
    activeNoteData,

    // Actions - 笔记本
    setNotebooks,
    updateNotebook,
    removeNotebook,
    getNotebookById,

    // Actions - 标签
    setTags,
    updateTag,
    removeTag,
    getTagById,

    // Actions - 笔记
    setNotes,
    prependNote,
    updateNote,
    removeNote,
    getNoteById,
    setActiveNote,
    createDefaultNote,

    // Actions - 编辑
    startEdit,
    cancelEdit,
    resetSearchParam,
  }
})
