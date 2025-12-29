import { computed, watch, toRaw } from 'vue'
import { useAppStore } from '../stores/app'
import { noteApi } from '../api/note'
import { parseId, isTemporaryId, validateNoteTitle, validateNoteContent } from '../utils/validation'
import { showError, withNotification } from '../utils/errorHandler'
import { showTagsToTags } from '../utils/converters'
import { ContentType } from '../types'

export function useNoteEditor() {
  const store = useAppStore()

  // 当前活动笔记数据
  const activeNoteData = computed(() => store.activeNoteData)

  // 监听活动笔记变化，同步到编辑缓冲区
  watch(
    () => store.activeNote,
    (noteId) => {
      if (noteId) {
        const note = store.getNoteById(noteId)
        if (note) {
          // 深拷贝到编辑缓冲区（使用 toRaw 获取原始对象，避免克隆响应式代理）
          store.editingNote = structuredClone(toRaw(note))
        }
      } else {
        store.editingNote = null
      }
    },
    { immediate: true },
  )

  // 设置活动笔记
  const setActiveNote = (noteId: string) => {
    store.setActiveNote(noteId)
  }

  // 创建新笔记
  const createNewNote = () => {
    const timestamp = Date.now()
    const newNote = store.createDefaultNote(store.activeNotebook, timestamp)

    // 添加到笔记列表开头
    store.prependNote(newNote)
    store.activeNote = newNote.id
    // 同步到编辑缓冲区（新笔记是普通对象，无需 toRaw）
    store.editingNote = structuredClone(newNote)
    store.editMode = true
  }

  // 保存笔记
  const saveNote = async (refreshNotes: () => Promise<void>) => {
    if (!store.activeNote || !activeNoteData.value) return

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

    const noteId = store.activeNote
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
    store.editMode = false

    if (!store.activeNote) return

    if (isTemporaryId(store.activeNote)) {
      // 移除临时笔记
      store.removeNote(store.activeNote)
      store.activeNote = null
    }
  }

  // 删除笔记
  const deleteNote = async (refreshNotes: () => Promise<void>) => {
    if (!store.activeNote) return

    const noteId = store.activeNote

    if (isTemporaryId(noteId)) {
      // 移除临时笔记
      store.removeNote(noteId)
      store.activeNote = null
    } else {
      await withNotification(
        async () => {
          await noteApi.deleteNote(parseId(noteId))
          store.noteSearchPageParam.pageIndex = 1
          await refreshNotes()
          store.activeNote = null
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
    if (!store.activeNote || !store.editingNote) return

    // 只更新编辑缓冲区，避免触发 notes 数组替换
    store.editingNote.title = title
  }

  // 更新笔记内容
  const updateNoteContent = (content: string) => {
    if (!store.activeNote || !store.editingNote) return

    // 只更新编辑缓冲区，避免触发 notes 数组替换
    store.editingNote.content = content
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
