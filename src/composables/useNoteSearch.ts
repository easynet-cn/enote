import { notes, query, state } from './store'
import { noteApi } from '../api/note'
import { parseId } from '../utils/validation'
import { withNotification } from '../utils/errorHandler'
import { debounce } from '../utils/debounce'
import type { ShowNote, Note, PageResult } from '../types'

// 搜索防抖延迟（毫秒）
const SEARCH_DEBOUNCE_DELAY = 300

export function useNoteSearch() {
  // 搜索笔记
  const searchNotes = async (): Promise<ShowNote[]> => {
    const result = await withNotification(
      async () => {
        state.noteSearchPageParam.pageIndex = state.notePageIndex
        state.noteSearchPageParam.pageSize = state.notePageSize

        const pageResult: PageResult<Note> = await noteApi.searchPageNotes(
          state.noteSearchPageParam,
        )

        state.noteTotal = pageResult.total

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

  // 刷新笔记列表
  const refreshNotes = async () => {
    notes.value = await searchNotes()
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
    stats,
    handleUpdateSearchQuery,
    handleImmediateSearch,
    handleNoteSizeChange,
    handleNoteCurrentChange,
  }
}
