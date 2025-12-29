import { onBeforeMount } from 'vue'
import { storeToRefs } from 'pinia'
import { useAppStore } from '../stores/app'
import { useNotebookManager } from './useNotebookManager'
import { useTagManager } from './useTagManager'
import { useNoteSearch } from './useNoteSearch'
import { useNoteEditor } from './useNoteEditor'
import { useNoteHistory } from './useNoteHistory'
import { showNotification } from '../components/ui/notification'
import { showError } from '../utils/errorHandler'
import type { ShowNotebook, ShowTag } from '../types'

export function useNotes() {
  const store = useAppStore()

  // 使用 storeToRefs 保持响应式
  const {
    notebooks,
    tags,
    notes,
    query,
    histories,
    activeNotebook,
    activeTag,
    activeNote,
    editMode,
    loading,
    notePageIndex,
    notePageSize,
    noteTotal,
    historyPageIndex,
    historyPageSize,
    historyTotal,
    historyLoading,
  } = storeToRefs(store)

  const notebookManager = useNotebookManager()
  const tagManager = useTagManager()
  const noteSearch = useNoteSearch()
  const noteEditor = useNoteEditor()
  const noteHistory = useNoteHistory()

  // 刷新所有数据
  const refreshAllData = async () => {
    await Promise.all([
      notebookManager.getNotebooks(),
      tagManager.getTags(),
      noteSearch.refreshNotes(),
    ])
  }

  // 更新统计数据
  const updateStats = async () => {
    const countMap = await noteSearch.stats()
    notebookManager.updateNotebookCounts(countMap)
  }

  // 包装的保存笔记本方法
  const saveNotebook = async (showNotebook: ShowNotebook) => {
    await notebookManager.saveNotebook(showNotebook, noteSearch.refreshNotes)
  }

  // 包装的删除笔记本方法
  const deleteNotebook = async (id: string) => {
    await notebookManager.deleteNotebook(id, noteSearch.refreshNotes)
  }

  // 包装的保存标签方法
  const saveTag = async (showTag: ShowTag) => {
    await tagManager.saveTag(showTag, refreshAllData)
  }

  // 包装的删除标签方法
  const deleteTag = async (id: string) => {
    await tagManager.deleteTag(id, refreshAllData)
  }

  // 包装的设置活动笔记本方法
  const setActiveNotebook = async (notebookId: string) => {
    await notebookManager.setActiveNotebook(notebookId, noteSearch.refreshNotes)
  }

  // 包装的设置活动标签方法
  const setActiveTag = async (tagId: string) => {
    await tagManager.setActiveTag(tagId, noteSearch.refreshNotes)
  }

  // 包装的保存笔记方法
  const saveNote = async () => {
    await noteEditor.saveNote(noteSearch.refreshNotes)
  }

  // 包装的删除笔记方法
  const deleteNote = async () => {
    await noteEditor.deleteNote(noteSearch.refreshNotes)
  }

  // 包装的搜索查询更新方法
  const handleUpdateSearchQuery = async () => {
    await noteSearch.handleUpdateSearchQuery(updateStats)
  }

  // 初始化 - 使用 Promise.all 并行化
  const initialize = async () => {
    store.loading = true

    const notification = showNotification({
      message: '正在加载',
      type: 'success',
      duration: 0,
    })

    try {
      // 第一阶段：并行加载笔记本和标签
      await Promise.all([notebookManager.getNotebooks(), tagManager.getTags()])

      // 第二阶段：设置默认选中（需要等第一阶段完成）
      store.activeNotebook = store.notebooks[0].id
      store.activeTag = store.tags[0].id

      // 第三阶段：并行加载笔记和统计
      await Promise.all([noteSearch.refreshNotes(), updateStats()])
    } catch (error) {
      showError(error, '初始化失败，请刷新页面重试')
    } finally {
      store.loading = false
      notification.close()
    }
  }

  onBeforeMount(() => {
    initialize()
  })

  return {
    // 数据状态
    notebooks,
    tags,
    notes,
    query,
    histories,

    // UI 状态 refs（直接导出，不再使用 Proxy 兼容层）
    activeNotebook,
    activeTag,
    activeNote,
    editMode,
    loading,
    notePageIndex,
    notePageSize,
    noteTotal,
    historyPageIndex,
    historyPageSize,
    historyTotal,
    historyLoading,

    // 笔记数据
    activeNoteData: noteEditor.activeNoteData,

    // 笔记本操作
    saveNotebook,
    deleteNotebook,
    setActiveNotebook,

    // 标签操作
    saveTag,
    deleteTag,
    setActiveTag,
    getTagById: tagManager.getTagById,

    // 笔记操作
    setActiveNote: noteEditor.setActiveNote,
    createNewNote: noteEditor.createNewNote,
    saveNote,
    cancelEdit: noteEditor.cancelEdit,
    deleteNote,
    updateNoteTitle: noteEditor.updateNoteTitle,
    updateNoteContent: noteEditor.updateNoteContent,
    updateNoteContentType: noteEditor.updateNoteContentType,
    updateNoteSetting: noteEditor.updateNoteSetting,

    // 搜索操作
    handleUpdateSearchQuery,
    handleNoteSizeChange: noteSearch.handleNoteSizeChange,
    handleNoteCurrentChange: noteSearch.handleNoteCurrentChange,

    // 历史记录操作
    openHistoryDialog: noteHistory.openHistoryDialog,
    handleNoteHistorySizeChange: noteHistory.handleNoteHistorySizeChange,
    handleNoteHistoryCurrentChange: noteHistory.handleNoteHistoryCurrentChange,
  }
}
