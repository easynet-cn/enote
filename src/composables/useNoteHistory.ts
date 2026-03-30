import { useAppStore } from '../stores/app'
import { noteApi } from '../api/note'
import { parseId } from '../utils/validation'
import { withNotification } from '../utils/errorHandler'
import i18n from '../i18n'
import { LRUCache } from '../utils/lruCache'
import { HISTORY_CACHE_MAX_SIZE, HISTORY_CACHE_TTL } from '../config/constants'
import type { NoteHistory, PageResult } from '../types'

// 历史记录缓存项
interface HistoryCacheEntry {
  data: NoteHistory[]
  total: number
}

// LRU 缓存：缓存历史记录分页结果
const historyCache = new LRUCache<string, HistoryCacheEntry>(
  HISTORY_CACHE_MAX_SIZE,
  HISTORY_CACHE_TTL,
)

// 生成缓存 key
const getCacheKey = (noteId: string, pageIndex: number, pageSize: number): string => {
  return `${noteId}-${pageIndex}-${pageSize}`
}

export function useNoteHistory() {
  const store = useAppStore()

  // 搜索历史记录（带缓存）
  const searchNoteHistories = async (skipCache = false) => {
    const noteId = store.activeNote ?? ''
    const cacheKey = getCacheKey(noteId, store.historyPageIndex, store.historyPageSize)

    // 检查缓存
    if (!skipCache) {
      const cached = historyCache.get(cacheKey)
      if (cached) {
        store.histories = cached.data
        store.historyTotal = cached.total
        return
      }
    }

    store.historyLoading = true
    try {
      await withNotification(
        async () => {
          const pageResult: PageResult<NoteHistory> = await noteApi.searchPageNoteHistories({
            pageIndex: store.historyPageIndex,
            pageSize: store.historyPageSize,
            noteId: parseId(noteId),
          })

          store.histories = pageResult.data
          store.historyTotal = pageResult.total

          // 写入缓存
          historyCache.set(cacheKey, {
            data: pageResult.data,
            total: pageResult.total,
          })
        },
        {
          error: i18n.global.t('composable.loadHistoryFailed'),
        },
      )
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

  // 清除历史缓存（笔记编辑/删除后调用）
  const clearHistoryCache = () => {
    historyCache.clear()
  }

  return {
    searchNoteHistories,
    openHistoryDialog,
    handleNoteHistorySizeChange,
    handleNoteHistoryCurrentChange,
    clearHistoryCache,
  }
}
