import { useAppStore } from '../stores/app'
import { noteApi } from '../api/note'
import { parseId } from '../utils/validation'
import { withNotification } from '../utils/errorHandler'
import i18n from '../i18n'
import { debounce } from '../utils/debounce'
import { noteToShowNote } from '../utils/converters'
import { LRUCache } from '../utils/lruCache'
import { SEARCH_DEBOUNCE_DELAY, SEARCH_CACHE_TTL, SEARCH_CACHE_MAX_SIZE } from '../config/constants'
import type { ShowNote, Note, PageResult, NoteSearchPageParam } from '../types'

// 搜索结果缓存项
interface SearchCacheEntry {
  result: ShowNote[]
  total: number
}

// LRU 缓存：缓存搜索结果，自动淘汰最久未使用的项
const searchCache = new LRUCache<string, SearchCacheEntry>(SEARCH_CACHE_MAX_SIZE, SEARCH_CACHE_TTL)

// 生成缓存 key
const getSearchCacheKey = (param: NoteSearchPageParam): string => {
  return `${param.pageIndex}-${param.pageSize}-${param.notebookId}-${param.tagId}-${param.keyword}`
}

export function useNoteSearch() {
  const store = useAppStore()

  // 搜索笔记（带 LRU 缓存）
  const searchNotes = async (skipCache = false): Promise<ShowNote[]> => {
    // 更新搜索参数
    store.noteSearchPageParam.pageIndex = store.notePageIndex
    store.noteSearchPageParam.pageSize = store.notePageSize

    // 检查缓存
    const cacheKey = getSearchCacheKey(store.noteSearchPageParam)

    if (!skipCache) {
      const cached = searchCache.get(cacheKey)
      if (cached) {
        store.noteTotal = cached.total
        return cached.result
      }
    }

    const result = await withNotification(
      async () => {
        const pageResult: PageResult<Note> = await noteApi.searchPageNotes(
          store.noteSearchPageParam,
        )

        store.noteTotal = pageResult.total

        const notes = pageResult.data.map(noteToShowNote)

        // 更新 LRU 缓存
        searchCache.set(cacheKey, {
          result: notes,
          total: pageResult.total,
        })

        return notes
      },
      {
        loading: i18n.global.t('composable.loadingNotes'),
        error: i18n.global.t('composable.loadNotesFailed'),
      },
    )

    return result || []
  }

  // 清除搜索缓存（在数据变更后调用）
  const clearSearchCache = () => {
    searchCache.clear()
  }

  // 刷新笔记列表（跳过缓存）
  const refreshNotes = async () => {
    clearSearchCache()
    const notes = await searchNotes(true)
    store.setNotes(notes)
  }

  // 统计
  const stats = async (): Promise<Map<number, number>> => {
    const countMap = new Map<number, number>()

    await withNotification(
      async () => {
        const result = await noteApi.noteStats(store.noteSearchPageParam)

        Object.entries(result.notebookCounts).forEach(([k, v]) => {
          const id = parseId(k)
          countMap.set(id, v)
        })

        return result
      },
      {
        loading: i18n.global.t('composable.countingNotes'),
        error: i18n.global.t('composable.countNotesFailed'),
      },
    )

    return countMap
  }

  // 执行搜索的核心函数
  const executeSearch = async (updateStats: () => Promise<void>) => {
    store.noteSearchPageParam.keyword = store.query
    const notes = await searchNotes()
    store.setNotes(notes)
    await updateStats()
  }

  // 防抖版本的搜索
  const debouncedSearch = debounce(executeSearch, SEARCH_DEBOUNCE_DELAY)

  // 更新搜索关键词（带防抖）
  const handleUpdateSearchQuery = (updateStats: () => Promise<void>) => {
    debouncedSearch(updateStats)
  }

  // 分页大小变化
  const handleNoteSizeChange = async (pageSize: number) => {
    store.notePageSize = pageSize
    const notes = await searchNotes()
    store.setNotes(notes)
  }

  // 页码变化
  const handleNoteCurrentChange = async (pageIndex: number) => {
    store.notePageIndex = pageIndex
    const notes = await searchNotes()
    store.setNotes(notes)
  }

  return {
    searchNotes,
    refreshNotes,
    clearSearchCache,
    stats,
    handleUpdateSearchQuery,
    handleNoteSizeChange,
    handleNoteCurrentChange,
  }
}
