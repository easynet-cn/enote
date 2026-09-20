<template>
  <div
    class="group flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-surface-dim transition-colors"
  >
    <!-- 批量选择框 -->
    <input
      v-if="selectable"
      type="checkbox"
      class="w-3.5 h-3.5 shrink-0 cursor-pointer"
      :checked="selected"
      @change="emit('toggleSelect')"
    />

    <!-- 拖拽手柄（仅顶级任务，回收站中禁用拖拽） -->
    <span
      v-if="!isSubtask && !deleted"
      class="todo-drag-handle p-0.5 text-content-secondary hover:text-content cursor-grab active:cursor-grabbing shrink-0"
      :title="t('common.sort')"
    >
      <GripVertical class="w-3.5 h-3.5" />
    </span>

    <!-- 展开箭头（仅顶级任务占位，有子任务时可点击） -->
    <button
      v-if="!isSubtask"
      class="p-0.5 text-content-secondary hover:text-content rounded cursor-pointer shrink-0"
      :class="hasChildren ? '' : 'invisible'"
      @click="emit('expand')"
    >
      <ChevronRight class="w-3.5 h-3.5 transition-transform" :class="expanded ? 'rotate-90' : ''" />
    </button>

    <input
      v-if="!deleted"
      type="checkbox"
      class="w-4 h-4 shrink-0 cursor-pointer"
      :checked="todo.isCompleted === 1"
      @change="emit('toggle')"
    />

    <span
      class="flex-1 min-w-0 truncate text-sm cursor-pointer"
      :class="[
        todo.isCompleted === 1 ? 'line-through text-content-secondary' : 'text-content',
        active ? 'font-medium' : '',
      ]"
      @click="emit('select')"
    >
      {{ todo.title }}
    </span>

    <span v-if="todo.recurrenceType > 0" class="shrink-0 text-content-secondary">
      <Repeat class="w-3 h-3" />
    </span>

    <span v-if="todo.priority" class="text-xs shrink-0" :class="priorityClass">
      {{ priorityLabel }}
    </span>

    <span
      v-if="todo.dueDate"
      class="text-xs shrink-0"
      :class="isOverdue ? 'text-red-500' : 'text-content-secondary'"
    >
      {{ todo.dueDate.substring(0, 10) }}
    </span>

    <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
      <button
        v-if="!deleted"
        class="p-1 text-content-secondary hover:text-content rounded cursor-pointer"
        :title="t('todo.edit')"
        @click="emit('edit')"
      >
        <Pencil class="w-3.5 h-3.5" />
      </button>
      <button
        v-if="deleted"
        class="p-1 text-content-secondary hover:text-content rounded cursor-pointer"
        :title="t('todo.restore')"
        @click="emit('restore')"
      >
        <RotateCcw class="w-3.5 h-3.5" />
      </button>
      <button
        class="p-1 text-content-secondary hover:text-red-500 rounded cursor-pointer"
        :title="deleted ? t('todo.permanentDelete') : t('todo.delete')"
        @click="emit('remove')"
      >
        <Trash2 class="w-3.5 h-3.5" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { ChevronRight, GripVertical, Pencil, Repeat, RotateCcw, Trash2 } from '@lucide/vue'
import type { Todo } from '../types'
import { TodoPriority } from '../types'

const props = defineProps<{
  todo: Todo
  isSubtask?: boolean
  expanded?: boolean
  hasChildren?: boolean
  deleted?: boolean
  /** 是否处于批量选择模式 */
  selectable?: boolean
  /** 是否被选中（批量模式） */
  selected?: boolean
  /** 是否为右侧详情当前展示项 */
  active?: boolean
}>()

const emit = defineEmits<{
  toggle: []
  edit: []
  select: []
  remove: []
  restore: []
  expand: []
  toggleSelect: []
}>()

const { t } = useI18n()

const priorityLabel = computed(() => {
  if (props.todo.priority === TodoPriority.Low) return t('todo.priorityLow')
  if (props.todo.priority === TodoPriority.Medium) return t('todo.priorityMedium')
  if (props.todo.priority === TodoPriority.High) return t('todo.priorityHigh')
  return ''
})

const priorityClass = computed(() => {
  if (props.todo.priority === TodoPriority.High) return 'text-red-500'
  if (props.todo.priority === TodoPriority.Medium) return 'text-amber-500'
  if (props.todo.priority === TodoPriority.Low) return 'text-sky-500'
  return 'text-content-secondary'
})

const nowString = () => {
  const d = new Date()
  const pad = (n: number) => String(n).padStart(2, '0')
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}`
}

const isOverdue = computed(() => {
  const todo = props.todo
  if (!todo.dueDate || todo.isCompleted === 1) return false
  return todo.dueDate < nowString()
})
</script>
