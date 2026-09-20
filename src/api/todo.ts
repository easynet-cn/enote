import { invoke } from '@tauri-apps/api/core'
import type { Todo, TodoList, TodoSearchParam, TodoStats } from '../types'

export const todoApi = {
  /** 查询待办列表 */
  async search(param: TodoSearchParam): Promise<Todo[]> {
    return await invoke('search_todos', { param })
  },

  /** 根据 ID 查询待办 */
  async findById(id: number): Promise<Todo | null> {
    return await invoke('find_todo', { id })
  },

  /** 创建待办 */
  async create(todo: Todo): Promise<Todo> {
    return await invoke('create_todo', { todo })
  },

  /** 更新待办 */
  async update(todo: Todo): Promise<Todo | null> {
    return await invoke('update_todo', { todo })
  },

  /** 切换待办完成状态 */
  async toggle(id: number): Promise<Todo | null> {
    return await invoke('toggle_todo', { id })
  },

  /** 软删除待办（移入回收站） */
  async remove(id: number): Promise<void> {
    return await invoke('delete_todo', { id })
  },

  /** 恢复回收站中的待办 */
  async restore(id: number): Promise<void> {
    return await invoke('restore_todo', { id })
  },

  /** 彻底删除待办 */
  async permanentDelete(id: number): Promise<void> {
    return await invoke('permanent_delete_todo', { id })
  },

  /** 清空待办回收站 */
  async emptyTrash(): Promise<number> {
    return await invoke('empty_todo_trash')
  },

  /** 查询所有待办清单 */
  async findAllLists(): Promise<TodoList[]> {
    return await invoke('find_all_todo_lists')
  },

  /** 创建待办清单 */
  async createList(list: TodoList): Promise<TodoList> {
    return await invoke('create_todo_list', { list })
  },

  /** 更新待办清单 */
  async updateList(list: TodoList): Promise<TodoList | null> {
    return await invoke('update_todo_list', { list })
  },

  /** 删除待办清单（清单下的待办仅解除关联） */
  async deleteList(id: number): Promise<void> {
    return await invoke('delete_todo_list', { id })
  },

  /** 批量更新排序（拖拽排序后持久化） */
  async reorder(orders: [number, number][]): Promise<void> {
    return await invoke('reorder_todos', { orders })
  },

  /** 查询待办关联的标签 ID */
  async findTags(todoId: number): Promise<number[]> {
    return await invoke('find_todo_tags', { todoId })
  },

  /** 设置待办关联的标签（全量替换） */
  async setTags(todoId: number, tagIds: number[]): Promise<void> {
    return await invoke('set_todo_tags', { todoId, tagIds })
  },

  /** 批量切换完成状态 */
  async batchToggle(ids: number[]): Promise<number> {
    return await invoke('batch_toggle_todos', { ids })
  },

  /** 批量软删除（移入回收站） */
  async batchDelete(ids: number[]): Promise<number> {
    return await invoke('batch_delete_todos', { ids })
  },

  /** 批量移动到清单（listId 为 null 表示移出清单） */
  async batchMove(ids: number[], listId: number | null): Promise<number> {
    return await invoke('batch_move_todos', { ids, listId })
  },

  /** 获取统计信息 */
  async stats(): Promise<TodoStats> {
    return await invoke('todo_stats')
  },
}
