import { ref, reactive, computed, onBeforeMount } from 'vue'
import { useDateFormat, useNow } from '@vueuse/core'
import { ContentType } from '../types'
import type {
  AppState,
  ShowNotebook,
  ShowTag,
  ShowNote,
  NotePageResult,
  NoteHistory,
} from '../types'
import { showNotification } from '../components/ui/notification'
import { noteApi } from '../api/note'
import {
  parseId,
  isTemporaryId,
  validateNotebookName,
  validateTagName,
  validateNoteTitle,
} from '../utils/validation'
import { showError, withNotification } from '../utils/errorHandler'

const notebooks = ref<ShowNotebook[]>([{ id: '0', name: '全部', count: 0, icon: 'BookOpen' }])
const tags = ref<ShowTag[]>([{ id: '0', name: '全部', icon: 'Tags' }])
const notes = ref<ShowNote[]>([])
const query = ref<string>('')
const histories = ref<NoteHistory[]>([])

// 状态管理
const state = reactive<AppState>({
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

export function useNotes() {
  // 获取笔记本
  const getNotebooks = async () => {
    const result = await withNotification(
      async () => {
        const data = (await noteApi.getNotebooks()).map(
          (notebook): ShowNotebook => ({
            id: String(notebook.id),
            parentId: notebook.parentId,
            name: notebook.name,
            description: notebook.description,
            icon: notebook.icon,
            cls: notebook.cls,
            count: notebook.count,
            createTime: notebook.createTime,
            updateTime: notebook.updateTime,
          }),
        )
        return data
      },
      { loading: '正在加载笔记本', error: '加载笔记本失败' },
    )

    if (result) {
      notebooks.value = [notebooks.value[0], ...result]
    }
  }

  // 获取笔记
  const searchNotes = async (): Promise<ShowNote[]> => {
    const result = await withNotification(
      async () => {
        state.noteSearchPageParam.pageIndex = state.notePageIndex
        state.noteSearchPageParam.pageSize = state.notePageSize

        const pageResult: NotePageResult = await noteApi.searchPageNotes(state.noteSearchPageParam)

        state.noteTotal = pageResult.total

        const countMap = new Map<number, number>()
        let totalCount = 0

        Object.entries(pageResult.notebookCounts).forEach(([k, v]) => {
          const id = parseId(k)
          countMap.set(id, v)
          totalCount += v
        })

        notebooks.value.forEach((e) => {
          if (e.id === '0') {
            e.count = totalCount
          } else {
            const id = parseId(e.id)
            e.count = countMap.get(id) || 0
          }
        })

        return pageResult.data.map(
          (note): ShowNote => ({
            id: String(note.id),
            notebookId: String(note.notebookId),
            notebookName: note.notebookName,
            title: note.title,
            content: note.content,
            contentType: note.contentType,
            tags: note.tags.map((e) => ({
              id: String(e.id),
              name: e.name,
              icon: e.icon,
              cls: e.cls,
              sortOrder: e.sortOrder,
            })),
            createTime: note.createTime,
            updateTime: note.updateTime,
          }),
        )
      },
      { loading: '正在加载笔记', error: '加载笔记失败' },
    )

    return result || []
  }

  // 获取标签
  const getTags = async () => {
    const result = await withNotification(
      async () => {
        const data = (await noteApi.geTags()).map(
          (tag): ShowTag => ({
            id: String(tag.id),
            name: tag.name,
            icon: tag.icon,
            cls: tag.cls,
            createTime: tag.createTime ?? '',
            updateTime: tag.updateTime ?? '',
          }),
        )
        return data
      },
      { loading: '正在加载标签', error: '加载标签失败' },
    )

    if (result) {
      tags.value = [tags.value[0], ...result]
    }
  }

  const activeNoteData = computed(() => {
    return notes.value.find((note) => note.id === state.activeNote) || null
  })

  // 保存笔记本
  const saveNotebook = async (showNotebook: ShowNotebook) => {
    // 验证
    const validation = validateNotebookName(showNotebook.name)
    if (!validation.valid) {
      showError(null, validation.error)
      return
    }

    const id = parseId(showNotebook.id)

    await withNotification(
      async () => {
        const notebookData = {
          id,
          parentId: showNotebook.parentId,
          name: showNotebook.name ?? '',
          description: showNotebook.description,
          icon: showNotebook.icon,
          cls: showNotebook.cls,
        }

        if (id === 0) {
          await noteApi.createNotebook(notebookData)
        } else {
          await noteApi.updateNotebook(notebookData)
        }

        // 并行刷新数据
        await Promise.all([getNotebooks(), searchNotes().then((data) => (notes.value = data))])
      },
      {
        loading: '正在保存笔记本',
        success: '笔记本保存成功',
        error: '保存笔记本失败',
      },
    )
  }

  // 删除笔记本
  const deleteNotebook = async (id: string) => {
    const notebookId = parseId(id)
    if (notebookId === 0) return

    await withNotification(
      async () => {
        await noteApi.deleteNotebook(notebookId)
        await Promise.all([getNotebooks(), searchNotes().then((data) => (notes.value = data))])
      },
      {
        loading: '正在删除笔记本',
        success: '笔记本已删除',
        error: '删除笔记本失败',
      },
    )
  }

  // 保存标签
  const saveTag = async (showTag: ShowTag) => {
    // 验证
    const validation = validateTagName(showTag.name)
    if (!validation.valid) {
      showError(null, validation.error)
      return
    }

    const id = parseId(showTag.id)

    await withNotification(
      async () => {
        const tagData = {
          id,
          name: showTag.name ?? '',
          icon: showTag.icon,
          cls: showTag.cls,
        }

        if (id === 0) {
          await noteApi.createTag(tagData)
        } else {
          await noteApi.updateTag(tagData)
        }

        // 并行刷新数据
        await Promise.all([
          getNotebooks(),
          getTags(),
          searchNotes().then((data) => (notes.value = data)),
        ])
      },
      {
        loading: '正在保存标签',
        success: '标签保存成功',
        error: '保存标签失败',
      },
    )
  }

  // 删除标签
  const deleteTag = async (id: string) => {
    const tagId = parseId(id)
    if (tagId === 0) return

    await withNotification(
      async () => {
        await noteApi.deleteTag(tagId)
        await Promise.all([getNotebooks(), searchNotes().then((data) => (notes.value = data))])
      },
      {
        loading: '正在删除标签',
        success: '标签已删除',
        error: '删除标签失败',
      },
    )
  }

  const setActiveNotebook = async (notebookId: string) => {
    state.activeNotebook = notebookId
    state.activeNote = null
    state.noteSearchPageParam.notebookId = parseId(notebookId)

    notes.value = await searchNotes()
  }

  const setActiveTag = async (tagId: string) => {
    state.activeTag = tagId
    state.activeNote = null
    state.noteSearchPageParam.tagId = parseId(tagId)

    notes.value = await searchNotes()
  }

  const setActiveNote = (noteId: string) => {
    state.activeNote = noteId
    state.editMode = false
  }

  const createNewNote = () => {
    const now = useNow()
    const nowStr = useDateFormat(now, 'YYYY-MM-DD HH:mm:ss').value

    const newNote: ShowNote = {
      id: '0-' + now.value.getTime(),
      notebookId: state.activeNotebook,
      title: '',
      content: '',
      tags: [],
      createTime: nowStr,
      updateTime: nowStr,
    }

    notes.value.unshift(newNote)
    state.activeNote = newNote.id
    state.editMode = true
  }

  // 保存笔记
  const saveNote = async () => {
    if (!state.activeNote || !activeNoteData.value) return

    // 验证标题
    const validation = validateNoteTitle(activeNoteData.value.title)
    if (!validation.valid) {
      showError(null, validation.error)
      return
    }

    const noteId = state.activeNote
    let newNoteId = noteId

    await withNotification(
      async () => {
        const tags =
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
            tags,
          )
        } else {
          const newNote = await noteApi.createNote(
            parseId(activeNoteData.value?.notebookId),
            activeNoteData.value?.title ?? '',
            activeNoteData.value?.content ?? '',
            activeNoteData.value?.contentType ?? ContentType.Html,
            tags,
          )
          newNoteId = String(newNote.id)
        }

        notes.value = await searchNotes()
        setActiveNote(newNoteId)
      },
      {
        loading: '正在保存笔记',
        success: '笔记保存成功',
        error: '保存笔记失败',
      },
    )
  }

  const cancelEdit = async () => {
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
  const deleteNote = async () => {
    if (!state.activeNote) return

    const noteId = state.activeNote

    if (isTemporaryId(noteId)) {
      // 临时笔记直接从列表移除
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
          notes.value = await searchNotes()
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

  const updateNoteTitle = (title: string) => {
    if (!state.activeNote) return

    const noteIndex = notes.value.findIndex((note) => note.id === state.activeNote)
    if (noteIndex !== -1) {
      notes.value[noteIndex].title = title
    }
  }

  const updateNoteContent = (content: string) => {
    if (!state.activeNote) return

    const noteIndex = notes.value.findIndex((note) => note.id === state.activeNote)
    if (noteIndex !== -1) {
      notes.value[noteIndex].content = content
    }
  }

  const getTagById = (tagId: string) => {
    return tags.value.find((tag) => tag.id === tagId)
  }

  const handleUpdateSearchQuery = async () => {
    state.noteSearchPageParam.keyword = query.value
    notes.value = await searchNotes()
  }

  const handleNoteSizeChange = async (pageSize: number) => {
    state.notePageSize = pageSize
    notes.value = await searchNotes()
  }

  const handleNoteCurrentChange = async (pageIndex: number) => {
    state.notePageIndex = pageIndex
    notes.value = await searchNotes()
  }

  // 搜索历史记录
  const searchNoteHistories = async () => {
    await withNotification(
      async () => {
        const pageResult = await noteApi.searchPageNoteHistories({
          pageIndex: state.historyPageIndex,
          pageSize: state.historyPageSize,
          noteId: parseId(state.activeNote ?? ''),
        })

        histories.value = pageResult.data
        state.historyTotal = pageResult.total
      },
      { loading: '正在加载历史记录', error: '加载历史记录失败' },
    )
  }

  const openHistoryDialog = async () => {
    await searchNoteHistories()
  }

  const handleNoteHistorySizeChange = async (pageSize: number) => {
    state.historyPageSize = pageSize
    await searchNoteHistories()
  }

  const handleNoteHistoryCurrentChange = async (pageIndex: number) => {
    state.historyPageIndex = pageIndex
    await searchNoteHistories()
  }

  // 初始化
  const initialize = async () => {
    state.loading = true

    const notification = showNotification({
      message: '正在加载',
      type: 'success',
      duration: 0,
    })

    try {
      await getNotebooks()
      await setActiveNotebook(notebooks.value[0].id)
      await getTags()
      await setActiveTag(tags.value[0].id)
      notes.value = await searchNotes()
    } catch (error) {
      showError(error, '初始化失败，请刷新页面重试')
    } finally {
      state.loading = false
      notification.close()
    }
  }

  onBeforeMount(() => {
    initialize()
  })

  return {
    notebooks,
    tags,
    notes,
    query,
    histories,
    state,
    activeNoteData,
    saveNotebook,
    deleteNotebook,
    saveTag,
    deleteTag,
    setActiveNotebook,
    setActiveTag,
    setActiveNote,
    createNewNote,
    saveNote,
    cancelEdit,
    deleteNote,
    updateNoteTitle,
    updateNoteContent,
    getTagById,
    handleUpdateSearchQuery,
    handleNoteSizeChange,
    handleNoteCurrentChange,
    openHistoryDialog,
    handleNoteHistorySizeChange,
    handleNoteHistoryCurrentChange,
  }
}
