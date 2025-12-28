import { useAppStore } from '../stores/app'
import { noteApi } from '../api/note'
import { parseId } from '../utils/validation'
import { showError } from '../utils/errorHandler'

export function useNoteHistory() {
  const store = useAppStore()

  // 搜索历史记录
  const searchNoteHistories = async () => {
    store.historyLoading = true
    try {
      const pageResult = await noteApi.searchPageNoteHistories({
        pageIndex: store.historyPageIndex,
        pageSize: store.historyPageSize,
        noteId: parseId(store.activeNote ?? ''),
      })

      store.histories = pageResult.data
      store.historyTotal = pageResult.total
    } catch (error) {
      showError(error, '加载历史记录失败')
    } finally {
      store.historyLoading = false
    }
  }

  // 打开历史对话框
  const openHistoryDialog = async () => {
    await searchNoteHistories()
  }

  // 历史记录分页大小变化
  const handleNoteHistorySizeChange = async (pageSize: number) => {
    store.historyPageSize = pageSize
    await searchNoteHistories()
  }

  // 历史记录页码变化
  const handleNoteHistoryCurrentChange = async (pageIndex: number) => {
    store.historyPageIndex = pageIndex
    await searchNoteHistories()
  }

  return {
    searchNoteHistories,
    openHistoryDialog,
    handleNoteHistorySizeChange,
    handleNoteHistoryCurrentChange,
  }
}
