import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { todoApi } from '../api/todo'
import type { Todo, TodoList, TodoSearchParam, TodoStats, TodoView } from '../types'

/** 创建空白待办对象（用于新增） */
export function createEmptyTodo(): Todo {
  return {
    id: 0,
    title: '',
    description: '',
    isCompleted: 0,
    priority: 0,
    dueDate: null,
    completedAt: null,
    listId: null,
    sortOrder: 0,
    createTime: null,
    updateTime: null,
    deletedAt: null,
    startDate: null,
    remindAt: null,
    isReminded: 0,
    parentId: 0,
    noteId: null,
    recurrenceType: 0,
    recurrenceInterval: 1,
    recurrenceEndDate: null,
    mcpAccess: 0,
  }
}

export const useTodoStore = defineStore('todos', () => {
  const todosMap = ref<Map<string, Todo>>(new Map())
  const lists = ref<TodoList[]>([])
  const loading = ref<boolean>(false)

  // 查询条件
  const view = ref<TodoView>('all')
  const activeListId = ref<number | null>(null)
  const activeTagId = ref<number | null>(null)
  const keyword = ref<string>('')
  const showCompleted = ref<boolean>(false)
  const deleted = ref<boolean>(false)
  const stats = ref<TodoStats | null>(null)
  /** 右侧详情当前选中的待办 ID */
  const activeTodoId = ref<number | null>(null)

  // ==================== Getters ====================
  const todos = computed<Todo[]>(() => Array.from(todosMap.value.values()))

  const activeList = computed<TodoList | null>(
    () => lists.value.find((item) => item.id === activeListId.value) ?? null,
  )

  const activeTodo = computed<Todo | null>(() =>
    activeTodoId.value === null ? null : (todosMap.value.get(String(activeTodoId.value)) ?? null),
  )

  // ==================== Actions ====================
  const buildParam = (): TodoSearchParam => ({
    keyword: keyword.value,
    listId: activeListId.value,
    tagId: activeTagId.value,
    completed: showCompleted.value ? null : false,
    view: view.value,
    priority: null,
    deleted: deleted.value,
    sortField: 'manual',
    sortOrder: 'desc',
  })

  const setTodos = (items: Todo[]) => {
    const map = new Map<string, Todo>()
    for (const item of items) {
      map.set(String(item.id), item)
    }
    todosMap.value = map
  }

  const loadTodos = async () => {
    loading.value = true
    try {
      const items = await todoApi.search(buildParam())
      setTodos(items)
    } finally {
      loading.value = false
    }
  }

  const loadLists = async () => {
    lists.value = await todoApi.findAllLists()
  }

  const createTodo = async (title: string, listId?: number | null, parentId = 0): Promise<Todo> => {
    const draft = createEmptyTodo()
    draft.title = title
    draft.listId = listId ?? activeListId.value
    draft.parentId = parentId
    const created = await todoApi.create(draft)
    const map = new Map(todosMap.value)
    map.set(String(created.id), created)
    todosMap.value = map
    return created
  }

  const updateTodo = async (todo: Todo) => {
    const updated = await todoApi.update(todo)
    if (updated) {
      const map = new Map(todosMap.value)
      map.set(String(updated.id), updated)
      todosMap.value = map
    }
  }

  const toggleTodo = async (id: number) => {
    const updated = await todoApi.toggle(id)
    if (updated) {
      const map = new Map(todosMap.value)
      map.set(String(updated.id), updated)
      todosMap.value = map
    }
  }

  const removeTodo = async (id: number) => {
    await todoApi.remove(id)
    const map = new Map(todosMap.value)
    map.delete(String(id))
    todosMap.value = map
  }

  const restoreTodo = async (id: number) => {
    await todoApi.restore(id)
    await loadTodos()
  }

  const permanentDeleteTodo = async (id: number) => {
    await todoApi.permanentDelete(id)
    const map = new Map(todosMap.value)
    map.delete(String(id))
    todosMap.value = map
  }

  const emptyTrash = async () => {
    await todoApi.emptyTrash()
    todosMap.value = new Map()
  }

  // -------------------- 待办清单 --------------------
  const createList = async (name: string, mcpAccess = 0): Promise<TodoList> => {
    const draft: TodoList = {
      id: 0,
      name,
      icon: '',
      color: '',
      sortOrder: 0,
      mcpAccess,
      createTime: null,
      updateTime: null,
    }
    const created = await todoApi.createList(draft)
    lists.value = [...lists.value, created]
    return created
  }

  const removeList = async (id: number) => {
    await todoApi.deleteList(id)
    lists.value = lists.value.filter((item) => item.id !== id)
    if (activeListId.value === id) {
      activeListId.value = null
    }
  }

  /** 批量更新排序：orders 为 [待办 ID, 新排序值] 列表 */
  const reorderTodos = async (orders: [number, number][]) => {
    await todoApi.reorder(orders)
  }

  /** 批量切换完成状态 */
  const batchToggleTodos = async (ids: number[]) => {
    await todoApi.batchToggle(ids)
    await loadTodos()
  }

  /** 批量软删除（移入回收站） */
  const batchDeleteTodos = async (ids: number[]) => {
    await todoApi.batchDelete(ids)
    await loadTodos()
  }

  /** 批量移动到清单（listId 为 null 表示移出清单） */
  const batchMoveTodos = async (ids: number[], listId: number | null) => {
    await todoApi.batchMove(ids, listId)
    await loadTodos()
  }

  /** 加载统计信息 */
  const loadStats = async () => {
    stats.value = await todoApi.stats()
  }

  const setView = (next: TodoView) => {
    view.value = next
  }

  const setActiveTag = (id: number | null) => {
    activeTagId.value = id
  }

  const setActiveList = (id: number | null) => {
    activeListId.value = id
  }

  const setActiveTodo = (id: number | null) => {
    activeTodoId.value = id
  }

  return {
    todos,
    lists,
    loading,
    view,
    activeListId,
    activeTagId,
    keyword,
    showCompleted,
    deleted,
    activeList,
    activeTodo,
    activeTodoId,
    stats,
    loadTodos,
    loadLists,
    createTodo,
    updateTodo,
    toggleTodo,
    removeTodo,
    restoreTodo,
    permanentDeleteTodo,
    emptyTrash,
    createList,
    removeList,
    reorderTodos,
    batchToggleTodos,
    batchDeleteTodos,
    batchMoveTodos,
    loadStats,
    setView,
    setActiveList,
    setActiveTag,
    setActiveTodo,
  }
})
