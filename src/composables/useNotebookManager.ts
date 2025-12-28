import { useAppStore } from '../stores/app'
import { noteApi } from '../api/note'
import { parseId, validateNotebookName } from '../utils/validation'
import { showError, withNotification } from '../utils/errorHandler'
import type { ShowNotebook } from '../types'

export function useNotebookManager() {
  const store = useAppStore()

  // 获取笔记本列表
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
      store.setNotebooks(result)
    }
  }

  // 保存笔记本（创建或更新）
  const saveNotebook = async (showNotebook: ShowNotebook, refreshNotes: () => Promise<void>) => {
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

        await Promise.all([getNotebooks(), refreshNotes()])
      },
      {
        loading: '正在保存笔记本',
        success: '笔记本保存成功',
        error: '保存笔记本失败',
      },
    )
  }

  // 删除笔记本
  const deleteNotebook = async (id: string, refreshNotes: () => Promise<void>) => {
    const notebookId = parseId(id)
    if (notebookId === 0) return

    await withNotification(
      async () => {
        await noteApi.deleteNotebook(notebookId)
        await Promise.all([getNotebooks(), refreshNotes()])
      },
      {
        loading: '正在删除笔记本',
        success: '笔记本已删除',
        error: '删除笔记本失败',
      },
    )
  }

  // 设置活动笔记本
  const setActiveNotebook = async (notebookId: string, searchNotes: () => Promise<void>) => {
    store.activeNotebook = notebookId
    store.activeNote = null
    store.noteSearchPageParam.notebookId = parseId(notebookId)
    await searchNotes()
  }

  // 更新笔记本统计数量
  const updateNotebookCounts = (countMap: Map<number, number>) => {
    let totalCount = 0
    countMap.forEach((v) => (totalCount += v))

    // 更新每个笔记本的计数
    for (const notebook of store.notebooks) {
      const count = notebook.id === '0' ? totalCount : countMap.get(parseId(notebook.id)) || 0
      store.updateNotebook({ ...notebook, count })
    }
  }

  return {
    getNotebooks,
    saveNotebook,
    deleteNotebook,
    setActiveNotebook,
    updateNotebookCounts,
  }
}
