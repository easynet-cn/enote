import { notes, query, state } from './store'
import { noteApi } from '../api/note'
import { parseId } from '../utils/validation'
import { withNotification } from '../utils/errorHandler'
import { debounce } from '../utils/debounce'
import { noteToShowNote } from '../utils/converters'
import { SEARCH_DEBOUNCE_DELAY, SEARCH_CACHE_TTL } from '../config/constants'
import type { ShowNote, Note, PageResult, NoteSearchPageParam } from '../types'

// 搜索结果缓存
interface SearchCache {
  key: string
  result: ShowNote[]
  total: number
  timestamp: number
}

let searchCache: SearchCache | null = null

// 生成缓存 key
const getSearchCacheKey = (param: NoteSearchPageParam): string => {
  return `${param.pageIndex}-${param.pageSize}-${param.notebookId}-${param.tagId}-${param.keyword}`
}

// 检查缓存是否有效
const isCacheValid = (key: string): boolean => {
  if (!searchCache) return false
  if (searchCache.key !== key) return false
  if (Date.now() - searchCache.timestamp > SEARCH_CACHE_TTL) return false
  return true
}

export function useNoteSearch() {
  // 搜索笔记（带缓存）
  const searchNotes = async (skipCache = false): Promise<ShowNote[]> => {
    // 更新搜索参数
    state.noteSearchPageParam.pageIndex = state.notePageIndex
    state.noteSearchPageParam.pageSize = state.notePageSize

    // 检查缓存
    const cacheKey = getSearchCacheKey(state.noteSearchPageParam)
    if (!skipCache && isCacheValid(cacheKey)) {
      state.noteTotal = searchCache!.total
      return searchCache!.result
    }

    const result = await withNotification(
      async () => {
        const pageResult: PageResult<Note> = await noteApi.searchPageNotes(
          state.noteSearchPageParam,
        )

        state.noteTotal = pageResult.total

        const notes = pageResult.data.map(noteToShowNote)

        // 更新缓存
        searchCache = {
          key: cacheKey,
          result: notes,
          total: pageResult.total,
          timestamp: Date.now(),
        }

        return notes
      },
      { loading: '正在加载笔记', error: '加载笔记失败' },
    )

    return result || []
  }

  // 清除搜索缓存（在数据变更后调用）
  const clearSearchCache = () => {
    searchCache = null
  }

  // 刷新笔记列表（跳过缓存）
  const refreshNotes = async () => {
    clearSearchCache()
    notes.value = await searchNotes(true)
  }

  // 统计
  const stats = async (): Promise<Map<number, number>> => {
    const countMap = new Map<number, number>()

    await withNotification(
      async () => {
        const result = await noteApi.noteStats(state.noteSearchPageParam)

        Object.entries(result.notebookCounts).forEach(([k, v]) => {
          const id = parseId(k)
          countMap.set(id, v)
        })

        return result
      },
      { loading: '正在统计笔记', error: '统计笔记失败' },
    )

    return countMap
  }

  // 执行搜索的核心函数
  const executeSearch = async (updateStats: () => Promise<void>) => {
    state.noteSearchPageParam.keyword = query.value
    notes.value = await searchNotes()
    await updateStats()
  }

  // 防抖版本的搜索
  const debouncedSearch = debounce(executeSearch, SEARCH_DEBOUNCE_DELAY)

  // 更新搜索关键词（带防抖）
  const handleUpdateSearchQuery = (updateStats: () => Promise<void>) => {
    debouncedSearch(updateStats)
  }

  // 立即搜索（不带防抖，用于分页等需要即时响应的场景）
  const handleImmediateSearch = async (updateStats: () => Promise<void>) => {
    await executeSearch(updateStats)
  }

  // 分页大小变化
  const handleNoteSizeChange = async (pageSize: number) => {
    state.notePageSize = pageSize
    notes.value = await searchNotes()
  }

  // 页码变化
  const handleNoteCurrentChange = async (pageIndex: number) => {
    state.notePageIndex = pageIndex
    notes.value = await searchNotes()
  }

  return {
    notes,
    query,
    searchNotes,
    refreshNotes,
    clearSearchCache,
    stats,
    handleUpdateSearchQuery,
    handleImmediateSearch,
    handleNoteSizeChange,
    handleNoteCurrentChange,
  }
}
