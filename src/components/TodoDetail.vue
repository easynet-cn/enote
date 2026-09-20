<template>
  <div class="h-full flex flex-col bg-surface">
    <!-- 空状态 -->
    <div
      v-if="!todo"
      class="flex-1 flex items-center justify-center px-6 text-sm text-content-secondary text-center"
    >
      {{ t('todo.noSelection') }}
    </div>

    <template v-else>
      <header class="h-12 px-4 flex items-center justify-between border-b border-edge shrink-0">
        <span class="text-sm font-medium text-content truncate">
          {{ form.title || t('todo.title') }}
        </span>
        <div class="flex items-center gap-2 shrink-0">
          <Button type="primary" @click="save">{{ t('common.save') }}</Button>
          <button
            class="p-1.5 text-content-secondary hover:text-red-500 hover:bg-surface-dim rounded-lg transition-colors cursor-pointer"
            :title="t('todo.delete')"
            @click="remove"
          >
            <Trash2 class="w-4 h-4" />
          </button>
        </div>
      </header>

      <div class="flex-1 overflow-y-auto p-4 space-y-4">
        <label class="flex items-center gap-2 text-sm text-content cursor-pointer">
          <input
            v-model="completed"
            type="checkbox"
            class="w-4 h-4 cursor-pointer"
            @change="save"
          />
          <span>{{ t('todo.completed') }}</span>
        </label>

        <div>
          <label class="block text-xs text-content-secondary mb-1">{{ t('todo.title') }}</label>
          <input
            v-model="form.title"
            type="text"
            class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
          />
        </div>

        <div>
          <label class="block text-xs text-content-secondary mb-1">
            {{ t('todo.description') }}
          </label>
          <textarea
            v-model="form.description"
            rows="6"
            class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500 resize-none"
          />
        </div>

        <div>
          <label class="block text-xs text-content-secondary mb-1">{{ t('todo.dueDate') }}</label>
          <input
            v-model="dateOnly"
            type="date"
            class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
          />
        </div>

        <div>
          <label class="block text-xs text-content-secondary mb-1">{{ t('todo.priority') }}</label>
          <select
            v-model.number="form.priority"
            class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500 cursor-pointer"
          >
            <option :value="0">{{ t('todo.priorityNone') }}</option>
            <option :value="1">{{ t('todo.priorityLow') }}</option>
            <option :value="2">{{ t('todo.priorityMedium') }}</option>
            <option :value="3">{{ t('todo.priorityHigh') }}</option>
          </select>
        </div>

        <div>
          <label class="block text-xs text-content-secondary mb-1">
            {{ t('todo.listTitle') }}
          </label>
          <select
            v-model="listIdValue"
            class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500 cursor-pointer"
          >
            <option value="">{{ t('todo.unassigned') }}</option>
            <option v-for="item in store.lists" :key="item.id" :value="String(item.id)">
              {{ item.name }}
            </option>
          </select>
        </div>

        <div>
          <label class="block text-xs text-content-secondary mb-1">
            {{ t('todo.recurrence') }}
          </label>
          <select
            v-model.number="form.recurrenceType"
            class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500 cursor-pointer"
          >
            <option :value="0">{{ t('todo.recurrenceNone') }}</option>
            <option :value="1">{{ t('todo.recurrenceDaily') }}</option>
            <option :value="2">{{ t('todo.recurrenceWeekly') }}</option>
            <option :value="3">{{ t('todo.recurrenceMonthly') }}</option>
            <option :value="4">{{ t('todo.recurrenceYearly') }}</option>
          </select>
        </div>

        <div v-if="form.recurrenceType > 0">
          <label class="block text-xs text-content-secondary mb-1">
            {{ t('todo.recurrenceInterval') }}
          </label>
          <input
            v-model.number="form.recurrenceInterval"
            type="number"
            min="1"
            class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
          />
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Trash2 } from '@lucide/vue'
import { Button, showNotification } from './ui'
import { createEmptyTodo, useTodoStore } from '../stores/todos'
import type { Todo } from '../types'

const { t } = useI18n()
const store = useTodoStore()

const todo = computed(() => store.activeTodo)
/** 始终持有可编辑副本（未选中时为空对象），避免模板中的空值判断 */
const form = ref<Todo>(createEmptyTodo())
const completed = ref(false)

// 选中项变化时重置表单
watch(
  todo,
  (value) => {
    form.value = value ? { ...value } : createEmptyTodo()
    completed.value = value?.isCompleted === 1
  },
  { immediate: true },
)

const listIdValue = computed({
  get: () => (form.value.listId == null ? '' : String(form.value.listId)),
  set: (value: string) => {
    if (form.value) {
      form.value.listId = value ? Number(value) : null
    }
  },
})

/** 日期输入框只处理日期部分，时间部分沿用原值 */
const dateOnly = computed({
  get: () => (form.value.dueDate ? form.value.dueDate.substring(0, 10) : ''),
  set: (value: string) => {
    if (!form.value) return
    if (!value) {
      form.value.dueDate = null
      return
    }
    const time = todo.value?.dueDate?.substring(11) || '23:59:59'
    form.value.dueDate = `${value} ${time}`
  },
})

const save = async () => {
  if (!form.value) return
  form.value.isCompleted = completed.value ? 1 : 0
  try {
    await store.updateTodo(form.value)
    await store.loadStats()
  } catch {
    showNotification({ type: 'error', message: t('todo.updateFailed') })
  }
}

const remove = async () => {
  if (!form.value) return
  const id = form.value.id
  await store.removeTodo(id)
  if (store.activeTodoId === id) {
    store.setActiveTodo(null)
  }
}
</script>
