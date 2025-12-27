import { histories, state } from './store'
import { noteApi } from '../api/note'
import { parseId } from '../utils/validation'
import { showError } from '../utils/errorHandler'

export function useNoteHistory() {
  // 搜索历史记录
  const searchNoteHistories = async () => {
    state.historyLoading = true
    try {
      const pageResult = await noteApi.searchPageNoteHistories({
        pageIndex: state.historyPageIndex,
        pageSize: state.historyPageSize,
        noteId: parseId(state.activeNote ?? ''),
      })

      histories.value = pageResult.data
      state.historyTotal = pageResult.total
    } catch (error) {
      showError(error, '加载历史记录失败')
    } finally {
      state.historyLoading = false
    }
  }

  // 打开历史对话框
  const openHistoryDialog = async () => {
    await searchNoteHistories()
  }

  // 历史记录分页大小变化
  const handleNoteHistorySizeChange = async (pageSize: number) => {
    state.historyPageSize = pageSize
    await searchNoteHistories()
  }

  // 历史记录页码变化
  const handleNoteHistoryCurrentChange = async (pageIndex: number) => {
    state.historyPageIndex = pageIndex
    await searchNoteHistories()
  }

  return {
    histories,
    searchNoteHistories,
    openHistoryDialog,
    handleNoteHistorySizeChange,
    handleNoteHistoryCurrentChange,
  }
}
