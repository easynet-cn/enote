<template>
  <div class="h-full flex flex-col bg-surface-alt">
    <!-- 主区 -->
    <div class="flex-1 flex flex-col min-w-0">
      <header class="h-12 px-4 flex items-center justify-between border-b border-edge bg-surface">
        <h2 class="text-base font-medium text-content">{{ currentTitle }}</h2>
        <div class="flex items-center gap-1">
          <button
            v-if="!store.deleted"
            class="px-2 py-1 text-xs rounded-lg transition-colors cursor-pointer"
            :class="
              batchMode ? 'bg-blue-500 text-white' : 'text-content-secondary hover:bg-surface-dim'
            "
            @click="batchMode ? exitBatch() : (batchMode = true)"
          >
            {{ batchMode ? t('todo.exitBatch') : t('todo.batchMode') }}
          </button>
          <button
            class="p-1.5 text-content-secondary hover:bg-surface-dim rounded-lg transition-colors cursor-pointer"
            @click="emit('close')"
          >
            <X class="w-4 h-4" />
          </button>
        </div>
      </header>

      <div v-if="!store.deleted" class="p-4 border-b border-edge bg-surface">
        <input
          v-model="newTitle"
          type="text"
          class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
          :placeholder="t('todo.addPlaceholder')"
          @keyup.enter="addTodo"
        />
      </div>
      <div v-else class="p-3 border-b border-edge bg-surface flex justify-end">
        <button
          class="px-3 py-1.5 text-sm rounded-lg bg-red-500 text-white hover:bg-red-600 transition-colors cursor-pointer"
          @click="emptyTrash"
        >
          {{ t('todo.emptyTrash') }}
        </button>
      </div>

      <!-- 批量操作工具栏 -->
      <div
        v-if="batchMode && !store.deleted"
        class="px-4 py-2 border-b border-edge bg-surface flex items-center gap-2 flex-wrap"
      >
        <span class="text-xs text-content-secondary">
          {{ t('todo.selectedCount', { count: selectedIds.size }) }}
        </span>
        <button
          class="px-2 py-1 text-xs rounded border border-edge text-content-secondary hover:bg-surface-dim cursor-pointer"
          @click="selectAll"
        >
          {{ t('todo.selectAll') }}
        </button>
        <button
          class="px-2 py-1 text-xs rounded border border-edge text-content-secondary hover:bg-surface-dim cursor-pointer"
          @click="runBatchComplete"
        >
          {{ t('todo.batchComplete') }}
        </button>
        <select
          class="px-2 py-1 text-xs rounded border border-edge bg-surface-alt text-content cursor-pointer"
          @change="onBatchMoveSelect"
        >
          <option value="">{{ t('todo.batchMove') }}</option>
          <option v-for="l in store.lists" :key="l.id" :value="l.id">{{ l.name }}</option>
        </select>
        <button
          class="px-2 py-1 text-xs rounded bg-red-500 text-white hover:bg-red-600 cursor-pointer"
          @click="runBatchDelete"
        >
          {{ t('todo.batchDelete') }}
        </button>
      </div>

      <!-- 统计概览 -->
      <div
        v-if="store.stats && !store.deleted"
        class="px-4 py-2 border-b border-edge bg-surface flex items-center gap-4 text-xs text-content-secondary flex-wrap"
      >
        <span>{{ t('todo.totalCount') }}: {{ store.stats.totalCount }}</span>
        <span>{{ t('todo.pendingCount') }}: {{ store.stats.pendingCount }}</span>
        <span>{{ t('todo.completedCount') }}: {{ store.stats.completedCount }}</span>
        <span>{{ t('todo.todayCount') }}: {{ store.stats.todayCount }}</span>
        <span>{{ t('todo.overdueCount') }}: {{ store.stats.overdueCount }}</span>
        <span> {{ t('todo.completionRate') }}: {{ store.stats.completionRate.toFixed(0) }}% </span>
      </div>

      <!-- 番茄钟 -->
      <div
        v-if="!store.deleted"
        class="px-4 py-2 border-b border-edge bg-surface flex items-center gap-2"
      >
        <Timer class="w-3.5 h-3.5 text-content-secondary shrink-0" />
        <span class="text-xs text-content-secondary">{{ t('todo.pomodoro') }}</span>
        <span class="text-sm text-content tabular-nums">{{ pomoLabel }}</span>
        <button
          class="px-2 py-1 text-xs rounded border border-edge text-content-secondary hover:bg-surface-dim cursor-pointer"
          @click="pomoRunning ? pausePomo() : startPomo()"
        >
          {{ pomoRunning ? t('todo.pomodoroPause') : t('todo.pomodoroStart') }}
        </button>
        <button
          class="px-2 py-1 text-xs rounded border border-edge text-content-secondary hover:bg-surface-dim cursor-pointer"
          @click="resetPomo"
        >
          {{ t('todo.pomodoroReset') }}
        </button>
      </div>

      <div class="flex-1 overflow-y-auto p-4">
        <!-- 四象限视图 -->
        <div v-if="isQuadrantView" class="grid grid-cols-2 gap-3">
          <div v-for="q in quadrants" :key="q.key" class="border border-edge rounded-lg p-2">
            <div class="text-xs font-medium text-content-secondary mb-1">{{ q.label }}</div>
            <div v-if="q.items.length === 0" class="text-xs text-content-secondary py-2">—</div>
            <div v-else class="space-y-0.5">
              <TodoItem
                v-for="item in q.items"
                :key="item.id"
                :todo="item"
                :deleted="store.deleted"
                :selectable="batchMode"
                :selected="selectedIds.has(item.id)"
                :active="store.activeTodoId === item.id"
                @toggle="toggle(item)"
                @edit="openEdit(item)"
                @select="store.setActiveTodo(item.id)"
                @remove="remove(item.id)"
                @toggle-select="toggleSelect(item.id)"
              />
            </div>
          </div>
        </div>

        <div v-else-if="store.loading" class="text-sm text-content-secondary py-8 text-center">
          {{ t('common.loading') }}
        </div>
        <div
          v-else-if="store.todos.length === 0"
          class="text-sm text-content-secondary py-8 text-center"
        >
          {{ t('todo.empty') }}
        </div>
        <ul v-else ref="todoListRef" class="space-y-1">
          <li v-for="todo in topLevelTodos" :key="todo.id">
            <TodoItem
              :todo="todo"
              :expanded="isExpanded(todo.id)"
              :has-children="childrenOf(todo.id).length > 0"
              :deleted="store.deleted"
              :selectable="batchMode"
              :selected="selectedIds.has(todo.id)"
              :active="store.activeTodoId === todo.id"
              @toggle="toggle(todo)"
              @edit="openEdit(todo)"
              @select="store.setActiveTodo(todo.id)"
              @remove="store.deleted ? permanentDelete(todo.id) : remove(todo.id)"
              @restore="restore(todo.id)"
              @expand="toggleExpand(todo.id)"
              @toggle-select="toggleSelect(todo.id)"
            />
            <ul v-if="isExpanded(todo.id)" class="ml-6 mt-0.5 space-y-0.5">
              <li v-for="child in childrenOf(todo.id)" :key="child.id">
                <TodoItem
                  :todo="child"
                  is-subtask
                  :deleted="store.deleted"
                  :active="store.activeTodoId === child.id"
                  @toggle="toggle(child)"
                  @edit="openEdit(child)"
                  @select="store.setActiveTodo(child.id)"
                  @remove="store.deleted ? permanentDelete(child.id) : remove(child.id)"
                  @restore="restore(child.id)"
                />
              </li>
              <li v-if="!store.deleted" class="px-3 py-1">
                <input
                  v-model="subtaskDrafts[todo.id]"
                  type="text"
                  class="w-full px-2 py-1 text-xs bg-surface-alt border border-edge rounded text-content outline-none focus:border-blue-500"
                  :placeholder="t('todo.addSubtask')"
                  @keyup.enter="addSubtask(todo.id)"
                />
              </li>
            </ul>
          </li>
        </ul>
      </div>
    </div>

    <TodoEditDialog v-model="editDialog" :todo="editing" @saved="reload" />
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { AlertCircle, Calendar, CalendarClock, LayoutGrid, ListTodo, Timer, X } from '@lucide/vue'
import { showNotification } from './ui'
import TodoEditDialog from './TodoEditDialog.vue'
import TodoItem from './TodoItem.vue'
import { useTodoStore } from '../stores/todos'
import type { Todo, TodoView } from '../types'
import { TodoPriority } from '../types'
import Sortable from 'sortablejs'

const emit = defineEmits<{ close: [] }>()
const { t } = useI18n()
const store = useTodoStore()

const newTitle = ref('')
const editDialog = ref(false)
const editing = ref<Todo | null>(null)

const views = computed(() => [
  { key: 'all' as TodoView, label: t('todo.viewAll'), icon: ListTodo },
  { key: 'today' as TodoView, label: t('todo.viewToday'), icon: Calendar },
  { key: 'planned' as TodoView, label: t('todo.viewPlanned'), icon: CalendarClock },
  { key: 'overdue' as TodoView, label: t('todo.viewOverdue'), icon: AlertCircle },
  { key: 'quadrant' as TodoView, label: t('todo.quadrant'), icon: LayoutGrid },
])

const currentTitle = computed(() => {
  if (store.deleted) return t('todo.viewTrash')
  return views.value.find((item) => item.key === store.view)?.label ?? t('todo.title')
})

// -------------------- 批量操作 --------------------
const batchMode = ref(false)
const selectedIds = ref<Set<number>>(new Set())

const toggleSelect = (id: number) => {
  const next = new Set(selectedIds.value)
  if (next.has(id)) {
    next.delete(id)
  } else {
    next.add(id)
  }
  selectedIds.value = next
}

const selectAll = () => {
  selectedIds.value = new Set(topLevelTodos.value.map((item) => item.id))
}

const clearSelection = () => {
  selectedIds.value = new Set()
}

const exitBatch = () => {
  batchMode.value = false
  clearSelection()
}

const runBatchComplete = async () => {
  if (selectedIds.value.size === 0) return
  await store.batchToggleTodos([...selectedIds.value])
  clearSelection()
}

const runBatchDelete = async () => {
  if (selectedIds.value.size === 0) return
  await store.batchDeleteTodos([...selectedIds.value])
  clearSelection()
}

const runBatchMove = async (listId: number | null) => {
  if (selectedIds.value.size === 0) return
  await store.batchMoveTodos([...selectedIds.value], listId)
  clearSelection()
}

const onBatchMoveSelect = (event: Event) => {
  const value = (event.target as HTMLSelectElement).value
  void runBatchMove(value ? Number(value) : null)
}

// -------------------- 四象限 --------------------
/** 紧急：未完成且截止日期在今天或之前 */
const isUrgent = (todo: Todo) => {
  if (!todo.dueDate || todo.isCompleted === 1) return false
  const d = new Date()
  const pad = (n: number) => String(n).padStart(2, '0')
  const endOfToday = `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} 23:59:59`
  return todo.dueDate <= endOfToday
}

/** 重要：优先级为高 */
const isImportant = (todo: Todo) => todo.priority >= TodoPriority.High

const quadrants = computed(() => {
  const list = topLevelTodos.value
  return [
    { key: 'q1', label: t('todo.q1'), items: list.filter((i) => isImportant(i) && isUrgent(i)) },
    { key: 'q2', label: t('todo.q2'), items: list.filter((i) => isImportant(i) && !isUrgent(i)) },
    { key: 'q3', label: t('todo.q3'), items: list.filter((i) => !isImportant(i) && isUrgent(i)) },
    { key: 'q4', label: t('todo.q4'), items: list.filter((i) => !isImportant(i) && !isUrgent(i)) },
  ]
})

const isQuadrantView = computed(() => store.view === 'quadrant')

// -------------------- 番茄钟 --------------------
const POMODORO_SECONDS = 25 * 60
const pomoSeconds = ref(POMODORO_SECONDS)
const pomoRunning = ref(false)
let pomoTimer: ReturnType<typeof setInterval> | null = null

const pomoLabel = computed(() => {
  const pad = (n: number) => String(n).padStart(2, '0')
  return `${pad(Math.floor(pomoSeconds.value / 60))}:${pad(pomoSeconds.value % 60)}`
})

const stopPomo = () => {
  if (pomoTimer) {
    clearInterval(pomoTimer)
    pomoTimer = null
  }
}

const startPomo = () => {
  if (pomoRunning.value) return
  pomoRunning.value = true
  pomoTimer = setInterval(() => {
    if (pomoSeconds.value > 0) {
      pomoSeconds.value -= 1
      return
    }
    // 时间到：停止计时并提示
    stopPomo()
    pomoRunning.value = false
    pomoSeconds.value = POMODORO_SECONDS
    showNotification({ type: 'success', message: t('todo.pomodoroDone') })
  }, 1000)
}

const pausePomo = () => {
  pomoRunning.value = false
  stopPomo()
}

const resetPomo = () => {
  pausePomo()
  pomoSeconds.value = POMODORO_SECONDS
}

// -------------------- 拖拽排序 --------------------
const todoListRef = ref<HTMLElement | null>(null)
let todoSortable: Sortable | null = null

/** 按新顺序重新分配 sort_order（列表按 sort_order DESC，越靠前值越大） */
const applyNewOrder = async (oldIndex: number, newIndex: number) => {
  if (oldIndex === newIndex) return

  const items = [...topLevelTodos.value]
  const [moved] = items.splice(oldIndex, 1)
  if (!moved) return
  items.splice(newIndex, 0, moved)

  const total = items.length
  const orders: [number, number][] = items.map((item, index) => [item.id, total - index])

  try {
    await store.reorderTodos(orders)
  } catch {
    showNotification({ type: 'error', message: t('todo.updateFailed') })
  }
  await reload()
}

const initSortable = () => {
  if (!todoListRef.value) return
  todoSortable?.destroy()
  todoSortable = Sortable.create(todoListRef.value, {
    handle: '.todo-drag-handle',
    animation: 150,
    onEnd: (evt) => {
      if (evt.oldIndex === undefined || evt.newIndex === undefined) return
      void applyNewOrder(evt.oldIndex, evt.newIndex)
    },
  })
}

// 列表重建后重新绑定拖拽
watch(
  () => store.todos.length,
  async () => {
    await nextTick()
    initSortable()
  },
)

onMounted(async () => {
  await store.loadLists()
  await store.loadTodos()
  await store.loadStats()
  await nextTick()
  initSortable()
})

onUnmounted(() => {
  stopPomo()
  todoSortable?.destroy()
  todoSortable = null
})

const reload = async () => {
  await store.loadTodos()
  await store.loadStats()
}

const addTodo = async () => {
  const title = newTitle.value.trim()
  if (!title) return
  try {
    await store.createTodo(title)
    newTitle.value = ''
    await reload()
  } catch {
    showNotification({ type: 'error', message: t('todo.createFailed') })
  }
}

const toggle = async (todo: Todo) => {
  await store.toggleTodo(todo.id)
  await reload()
}

const remove = async (id: number) => {
  await store.removeTodo(id)
}

const restore = async (id: number) => {
  await store.restoreTodo(id)
}

const permanentDelete = async (id: number) => {
  await store.permanentDeleteTodo(id)
}

const emptyTrash = async () => {
  await store.emptyTrash()
}

const openEdit = (todo: Todo) => {
  editing.value = todo
  editDialog.value = true
}

// -------------------- 子任务 --------------------
const expandedIds = ref<Set<number>>(new Set())
const subtaskDrafts = ref<Record<number, string>>({})

// 顶级任务：parentId 为 0，或父任务不在当前结果中（避免子任务因父任务被过滤而丢失）
const topLevelTodos = computed(() => {
  const ids = new Set(store.todos.map((item) => item.id))
  return store.todos.filter((item) => item.parentId === 0 || !ids.has(item.parentId))
})

const childrenOf = (id: number) => store.todos.filter((item) => item.parentId === id)

const isExpanded = (id: number) => expandedIds.value.has(id)

const toggleExpand = (id: number) => {
  const next = new Set(expandedIds.value)
  if (next.has(id)) {
    next.delete(id)
  } else {
    next.add(id)
  }
  expandedIds.value = next
}

const addSubtask = async (parentId: number) => {
  const title = (subtaskDrafts.value[parentId] ?? '').trim()
  if (!title) return
  try {
    await store.createTodo(title, undefined, parentId)
    subtaskDrafts.value = { ...subtaskDrafts.value, [parentId]: '' }
    if (!isExpanded(parentId)) {
      toggleExpand(parentId)
    }
    await reload()
  } catch {
    showNotification({ type: 'error', message: t('todo.createFailed') })
  }
}
</script>
