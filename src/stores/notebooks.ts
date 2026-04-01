import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import i18n from '../i18n'
import type { ShowNotebook, NotebookTreeNode } from '../types'

export const useNotebookStore = defineStore('notebooks', () => {
  const t = i18n.global.t

  const notebooksMap = ref<Map<string, ShowNotebook>>(new Map())
  const expandedNotebooks = ref<Set<string>>(new Set())

  const defaultNotebook = computed<ShowNotebook>(() => ({
    id: '0',
    name: t('notebook.all'),
    count: 0,
    icon: 'BookOpen',
  }))

  // 初始化默认项
  notebooksMap.value.set('0', defaultNotebook.value)

  // ==================== Getters ====================
  const notebooks = computed<ShowNotebook[]>(() => {
    return Array.from(notebooksMap.value.values())
  })

  const notebookTree = computed<NotebookTreeNode[]>(() => {
    const allNotebooks = Array.from(notebooksMap.value.values()).filter((n) => n.id !== '0')
    const expanded = expandedNotebooks.value

    const buildChildren = (parentId: number): NotebookTreeNode[] => {
      return allNotebooks
        .filter((n) => (n.parentId ?? 0) === parentId)
        .sort((a, b) => (b.sortOrder ?? 0) - (a.sortOrder ?? 0))
        .map((n) => ({
          ...n,
          expanded: expanded.has(n.id),
          children: buildChildren(Number(n.id)),
        }))
    }

    return buildChildren(0)
  })

  // ==================== Actions ====================
  const setNotebooks = (items: ShowNotebook[]) => {
    const isFirstLoad = notebooksMap.value.size <= 1
    notebooksMap.value.clear()
    notebooksMap.value.set('0', defaultNotebook.value)

    for (const item of items) {
      notebooksMap.value.set(item.id, item)
    }

    if (isFirstLoad) {
      initExpandedNotebooks()
    }
  }

  const updateNotebook = (notebook: ShowNotebook) => {
    notebooksMap.value.set(notebook.id, notebook)
  }

  const removeNotebook = (id: string) => {
    notebooksMap.value.delete(id)
  }

  const getNotebookById = (id: string): ShowNotebook | undefined => {
    return notebooksMap.value.get(id)
  }

  const toggleNotebookExpand = (id: string) => {
    const newSet = new Set(expandedNotebooks.value)
    if (newSet.has(id)) {
      newSet.delete(id)
    } else {
      newSet.add(id)
    }
    expandedNotebooks.value = newSet
  }

  const initExpandedNotebooks = () => {
    const rootIds = Array.from(notebooksMap.value.values())
      .filter((n) => n.id !== '0' && (!n.parentId || n.parentId === 0))
      .map((n) => n.id)
    expandedNotebooks.value = new Set(rootIds)
  }

  const updateDefaultNotebook = () => {
    notebooksMap.value.set('0', defaultNotebook.value)
  }

  return {
    notebooks,
    notebookTree,
    expandedNotebooks,
    setNotebooks,
    updateNotebook,
    removeNotebook,
    getNotebookById,
    toggleNotebookExpand,
    initExpandedNotebooks,
    updateDefaultNotebook,
  }
})
