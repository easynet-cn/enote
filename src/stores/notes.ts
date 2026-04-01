import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { ShowNote, NoteSearchPageParam } from '../types'
import { DEFAULT_NOTE_PAGE_SIZE } from '../config/constants'

export const useNoteStore = defineStore('notes', () => {
  const notesMap = ref<Map<string, ShowNote>>(new Map())

  // 笔记分页（通过 computed 代理 noteSearchPageParam，避免双重状态）
  const noteSearchPageParam = ref<NoteSearchPageParam>({
    pageIndex: 1,
    pageSize: DEFAULT_NOTE_PAGE_SIZE,
    notebookId: 0,
    tagId: 0,
    keyword: '',
    sortField: 'update_time',
    sortOrder: 'desc',
    isStarred: false,
  })

  const notePageIndex = computed({
    get: () => noteSearchPageParam.value.pageIndex,
    set: (v: number) => {
      noteSearchPageParam.value.pageIndex = v
    },
  })
  const notePageSize = computed({
    get: () => noteSearchPageParam.value.pageSize,
    set: (v: number) => {
      noteSearchPageParam.value.pageSize = v
    },
  })
  const noteTotal = ref<number>(0)
  const noteSortField = computed({
    get: () => noteSearchPageParam.value.sortField,
    set: (v: string) => {
      noteSearchPageParam.value.sortField = v
    },
  })
  const noteSortOrder = computed({
    get: () => noteSearchPageParam.value.sortOrder,
    set: (v: string) => {
      noteSearchPageParam.value.sortOrder = v
    },
  })

  // 搜索关键词
  const query = ref<string>('')

  // 多选模式
  const isSelectMode = ref<boolean>(false)
  const selectedNotes = ref<Set<string>>(new Set())

  // ==================== Getters ====================
  const notes = computed<ShowNote[]>(() => {
    return Array.from(notesMap.value.values())
  })

  // ==================== Actions ====================
  const setNotes = (items: ShowNote[]) => {
    notesMap.value.clear()
    for (const item of items) {
      notesMap.value.set(item.id, item)
    }
  }

  const prependNote = (note: ShowNote) => {
    const newMap = new Map<string, ShowNote>()
    newMap.set(note.id, note)
    for (const [id, n] of notesMap.value) {
      newMap.set(id, n)
    }
    notesMap.value = newMap
  }

  const updateNote = (note: ShowNote) => {
    notesMap.value.set(note.id, note)
  }

  const removeNote = (id: string) => {
    notesMap.value.delete(id)
  }

  const getNoteById = (id: string): ShowNote | undefined => {
    return notesMap.value.get(id)
  }

  const toggleSelectMode = () => {
    isSelectMode.value = !isSelectMode.value
    if (!isSelectMode.value) {
      selectedNotes.value = new Set()
    }
  }

  const toggleNoteSelection = (noteId: string) => {
    const newSet = new Set(selectedNotes.value)
    if (newSet.has(noteId)) {
      newSet.delete(noteId)
    } else {
      newSet.add(noteId)
    }
    selectedNotes.value = newSet
  }

  const selectAllNotes = () => {
    const newSet = new Set<string>()
    for (const id of notesMap.value.keys()) {
      newSet.add(id)
    }
    selectedNotes.value = newSet
  }

  const clearSelection = () => {
    selectedNotes.value = new Set()
    isSelectMode.value = false
  }

  const resetSearchParam = () => {
    noteSearchPageParam.value = {
      pageIndex: 1,
      pageSize: DEFAULT_NOTE_PAGE_SIZE,
      notebookId: 0,
      tagId: 0,
      keyword: '',
      sortField: 'update_time',
      sortOrder: 'desc',
      isStarred: false,
    }
  }

  return {
    notes,
    query,
    notePageIndex,
    notePageSize,
    noteTotal,
    noteSortField,
    noteSortOrder,
    noteSearchPageParam,
    notesLoading: ref<boolean>(false),

    // 多选
    isSelectMode,
    selectedNotes,
    toggleSelectMode,
    toggleNoteSelection,
    selectAllNotes,
    clearSelection,

    // CRUD
    setNotes,
    prependNote,
    updateNote,
    removeNote,
    getNoteById,
    resetSearchParam,
  }
})
