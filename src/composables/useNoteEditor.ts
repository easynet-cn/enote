import { computed, watch } from 'vue'
import { notes, state, createDefaultNote, editingNote } from './store'
import { noteApi } from '../api/note'
import { parseId, isTemporaryId, validateNoteTitle, validateNoteContent } from '../utils/validation'
import { showError, withNotification } from '../utils/errorHandler'
import { showTagsToTags } from '../utils/converters'
import { ContentType } from '../types'

export function useNoteEditor() {
  // 当前活动笔记数据
  // 编辑模式下返回编辑缓冲区，避免频繁触发 notes 数组替换
  const activeNoteData = computed(() => {
    if (state.editMode && editingNote.value) {
      return editingNote.value
    }
    return notes.value.find((note) => note.id === state.activeNote) || null
  })

  // 监听活动笔记变化，同步到编辑缓冲区
  watch(
    () => state.activeNote,
    (noteId) => {
      if (noteId) {
        const note = notes.value.find((n) => n.id === noteId)
        if (note) {
          // 深拷贝到编辑缓冲区
          editingNote.value = JSON.parse(JSON.stringify(note))
        }
      } else {
        editingNote.value = null
      }
    },
    { immediate: true },
  )

  // 设置活动笔记
  const setActiveNote = (noteId: string) => {
    state.activeNote = noteId
    state.editMode = false
  }

  // 创建新笔记
  const createNewNote = () => {
    const timestamp = Date.now()
    const newNote = createDefaultNote(state.activeNotebook, timestamp)

    // shallowRef 需要整体替换数组
    notes.value = [newNote, ...notes.value]
    state.activeNote = newNote.id
    // 同步到编辑缓冲区
    editingNote.value = JSON.parse(JSON.stringify(newNote))
    state.editMode = true
  }

  // 保存笔记
  const saveNote = async (refreshNotes: () => Promise<void>) => {
    if (!state.activeNote || !activeNoteData.value) return

    // 验证标题
    const titleValidation = validateNoteTitle(activeNoteData.value.title)
    if (!titleValidation.valid) {
      showError(null, titleValidation.error)
      return
    }

    // 验证内容大小
    const contentValidation = validateNoteContent(activeNoteData.value.content)
    if (!contentValidation.valid) {
      showError(null, contentValidation.error)
      return
    }

    const noteId = state.activeNote
    let newNoteId = noteId

    await withNotification(
      async () => {
        const tagList = showTagsToTags(activeNoteData.value?.tags ?? [])

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
      // shallowRef 需要整体替换数组
      notes.value = notes.value.filter((note) => note.id !== state.activeNote)
      state.activeNote = null
    }
  }

  // 删除笔记
  const deleteNote = async (refreshNotes: () => Promise<void>) => {
    if (!state.activeNote) return

    const noteId = state.activeNote

    if (isTemporaryId(noteId)) {
      // shallowRef 需要整体替换数组
      notes.value = notes.value.filter((note) => note.id !== state.activeNote)
      state.activeNote = null
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
    if (!state.activeNote || !editingNote.value) return

    // 只更新编辑缓冲区，避免触发 notes 数组替换
    editingNote.value.title = title
  }

  // 更新笔记内容
  const updateNoteContent = (content: string) => {
    if (!state.activeNote || !editingNote.value) return

    // 只更新编辑缓冲区，避免触发 notes 数组替换
    editingNote.value.content = content
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
