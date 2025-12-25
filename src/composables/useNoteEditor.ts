import { computed } from 'vue'
import { notes, state, createDefaultNote } from './store'
import { noteApi } from '../api/note'
import { parseId, isTemporaryId, validateNoteTitle } from '../utils/validation'
import { showError, withNotification } from '../utils/errorHandler'
import { ContentType } from '../types'

export function useNoteEditor() {
  // 当前活动笔记数据
  const activeNoteData = computed(() => {
    return notes.value.find((note) => note.id === state.activeNote) || null
  })

  // 设置活动笔记
  const setActiveNote = (noteId: string) => {
    state.activeNote = noteId
    state.editMode = false
  }

  // 创建新笔记
  const createNewNote = () => {
    const timestamp = Date.now()
    const newNote = createDefaultNote(state.activeNotebook, timestamp)

    notes.value.unshift(newNote)
    state.activeNote = newNote.id
    state.editMode = true
  }

  // 保存笔记
  const saveNote = async (refreshNotes: () => Promise<void>) => {
    if (!state.activeNote || !activeNoteData.value) return

    const validation = validateNoteTitle(activeNoteData.value.title)
    if (!validation.valid) {
      showError(null, validation.error)
      return
    }

    const noteId = state.activeNote
    let newNoteId = noteId

    await withNotification(
      async () => {
        const tagList =
          activeNoteData.value?.tags?.map((e) => ({
            id: parseId(e.id),
            name: e.name,
            icon: e.icon,
            cls: e.cls,
            sortOrder: e.sortOrder,
          })) ?? []

        if (!isTemporaryId(noteId)) {
          await noteApi.updateNote(
            parseId(noteId),
            parseId(activeNoteData.value?.notebookId),
            activeNoteData.value?.title ?? '',
            activeNoteData.value?.content ?? '',
            activeNoteData.value?.contentType ?? ContentType.Html,
            tagList,
          )
        } else {
          const newNote = await noteApi.createNote(
            parseId(activeNoteData.value?.notebookId),
            activeNoteData.value?.title ?? '',
            activeNoteData.value?.content ?? '',
            activeNoteData.value?.contentType ?? ContentType.Html,
            tagList,
          )
          newNoteId = String(newNote.id)
        }

        await refreshNotes()
        setActiveNote(newNoteId)
      },
      {
        loading: '正在保存笔记',
        success: '笔记保存成功',
        error: '保存笔记失败',
      },
    )
  }

  // 取消编辑
  const cancelEdit = () => {
    state.editMode = false

    if (!state.activeNote) return

    if (isTemporaryId(state.activeNote)) {
      const noteIndex = notes.value.findIndex((note) => note.id === state.activeNote)
      if (noteIndex !== -1) {
        notes.value.splice(noteIndex, 1)
        state.activeNote = null
      }
    }
  }

  // 删除笔记
  const deleteNote = async (refreshNotes: () => Promise<void>) => {
    if (!state.activeNote) return

    const noteId = state.activeNote

    if (isTemporaryId(noteId)) {
      const noteIndex = notes.value.findIndex((note) => note.id === state.activeNote)
      if (noteIndex !== -1) {
        notes.value.splice(noteIndex, 1)
        state.activeNote = null
      }
    } else {
      await withNotification(
        async () => {
          await noteApi.deleteNote(parseId(noteId))
          state.noteSearchPageParam.pageIndex = 1
          await refreshNotes()
          state.activeNote = null
        },
        {
          loading: '正在删除笔记',
          success: '笔记已删除',
          error: '删除笔记失败',
        },
      )
    }
  }

  // 更新笔记标题
  const updateNoteTitle = (title: string) => {
    if (!state.activeNote) return

    const noteIndex = notes.value.findIndex((note) => note.id === state.activeNote)
    if (noteIndex !== -1) {
      notes.value[noteIndex].title = title
    }
  }

  // 更新笔记内容
  const updateNoteContent = (content: string) => {
    if (!state.activeNote) return

    const noteIndex = notes.value.findIndex((note) => note.id === state.activeNote)
    if (noteIndex !== -1) {
      notes.value[noteIndex].content = content
    }
  }

  return {
    activeNoteData,
    setActiveNote,
    createNewNote,
    saveNote,
    cancelEdit,
    deleteNote,
    updateNoteTitle,
    updateNoteContent,
  }
}
