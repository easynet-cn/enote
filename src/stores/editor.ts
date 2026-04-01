import { defineStore } from 'pinia'
import { ref, computed, toRaw } from 'vue'
import { ContentType } from '../types'
import type { ShowNote, NoteHistory } from '../types'
import { DEFAULT_HISTORY_PAGE_SIZE } from '../config/constants'
import { useNoteStore } from './notes'

export const useEditorStore = defineStore('editor', () => {
  const noteStore = useNoteStore()

  const editingNote = ref<ShowNote | null>(null)
  const editMode = ref<boolean>(false)
  const activeNote = ref<string | null>(null)

  // 历史记录
  const histories = ref<NoteHistory[]>([])
  const historyPageIndex = ref<number>(1)
  const historyPageSize = ref<number>(DEFAULT_HISTORY_PAGE_SIZE)
  const historyTotal = ref<number>(0)
  const historyLoading = ref<boolean>(false)

  // ==================== Getters ====================
  const activeNoteData = computed<ShowNote | null>(() => {
    if (editMode.value && editingNote.value) {
      return editingNote.value
    }
    if (!activeNote.value) return null
    return noteStore.getNoteById(activeNote.value) || null
  })

  const isDirty = computed<boolean>(() => {
    if (!editMode.value || !editingNote.value || !activeNote.value) return false
    const original = noteStore.getNoteById(activeNote.value)
    if (!original) return false
    return (
      editingNote.value.title !== original.title ||
      editingNote.value.content !== original.content ||
      editingNote.value.contentType !== original.contentType
    )
  })

  // ==================== Actions ====================
  const setActiveNote = (noteId: string | null) => {
    activeNote.value = noteId
    editMode.value = false

    if (noteId) {
      const note = noteStore.getNoteById(noteId)
      if (note) {
        editingNote.value = structuredClone(toRaw(note))
      }
    } else {
      editingNote.value = null
    }
  }

  const createDefaultNote = (
    notebookId: string,
    timestamp: number,
    contentType: ContentType = ContentType.Html,
  ): ShowNote => {
    const timeStr = new Date(timestamp).toISOString().replace('T', ' ').slice(0, 19)
    return {
      id: '0-' + timestamp,
      notebookId,
      notebookName: undefined,
      title: '',
      content: '',
      contentType,
      isPinned: 0,
      isStarred: 0,
      tags: [],
      createTime: timeStr,
      updateTime: timeStr,
    }
  }

  const startEdit = () => {
    editMode.value = true
  }

  const cancelEdit = () => {
    editMode.value = false
    if (activeNote.value) {
      const note = noteStore.getNoteById(activeNote.value)
      if (note) {
        editingNote.value = structuredClone(toRaw(note))
      }
    }
  }

  return {
    editingNote,
    editMode,
    activeNote,
    activeNoteData,
    isDirty,

    // 历史
    histories,
    historyPageIndex,
    historyPageSize,
    historyTotal,
    historyLoading,

    // Actions
    setActiveNote,
    createDefaultNote,
    startEdit,
    cancelEdit,
  }
})
