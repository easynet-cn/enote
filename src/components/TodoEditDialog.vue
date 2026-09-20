<template>
  <Dialog v-model="visible" :title="t('todo.edit')" :width="520">
    <div v-if="draft" class="space-y-3">
      <div>
        <label class="block text-xs text-content-secondary mb-1">{{ t('todo.title') }}</label>
        <input
          v-model="draft.title"
          type="text"
          class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
        />
      </div>
      <div>
        <label class="block text-xs text-content-secondary mb-1">{{ t('todo.description') }}</label>
        <textarea
          v-model="draft.description"
          rows="3"
          class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
        />
      </div>
      <div class="flex gap-3">
        <div class="flex-1">
          <label class="block text-xs text-content-secondary mb-1">{{ t('todo.dueDate') }}</label>
          <input
            type="date"
            :value="dueDateValue"
            class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
            @change="onDueChange"
          />
        </div>
        <div class="flex-1">
          <label class="block text-xs text-content-secondary mb-1">{{ t('todo.priority') }}</label>
          <select
            v-model.number="draft.priority"
            class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
          >
            <option :value="0">{{ t('todo.priorityNone') }}</option>
            <option :value="1">{{ t('todo.priorityLow') }}</option>
            <option :value="2">{{ t('todo.priorityMedium') }}</option>
            <option :value="3">{{ t('todo.priorityHigh') }}</option>
          </select>
        </div>
      </div>
      <div>
        <label class="block text-xs text-content-secondary mb-1">{{ t('todo.remindAt') }}</label>
        <input
          type="datetime-local"
          :value="remindAtValue"
          class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
          @change="onRemindChange"
        />
      </div>
      <div class="flex gap-3">
        <div class="flex-1">
          <label class="block text-xs text-content-secondary mb-1">
            {{ t('todo.recurrence') }}
          </label>
          <select
            v-model.number="draft.recurrenceType"
            class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
          >
            <option :value="0">{{ t('todo.recurrenceNone') }}</option>
            <option :value="1">{{ t('todo.recurrenceDaily') }}</option>
            <option :value="2">{{ t('todo.recurrenceWeekly') }}</option>
            <option :value="3">{{ t('todo.recurrenceMonthly') }}</option>
            <option :value="4">{{ t('todo.recurrenceYearly') }}</option>
          </select>
        </div>
        <div v-if="draft.recurrenceType > 0" class="flex-1">
          <label class="block text-xs text-content-secondary mb-1">
            {{ t('todo.recurrenceInterval') }}
          </label>
          <input
            v-model.number="draft.recurrenceInterval"
            type="number"
            min="1"
            class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
          />
        </div>
      </div>
      <div v-if="draft.recurrenceType > 0">
        <label class="block text-xs text-content-secondary mb-1">
          {{ t('todo.recurrenceEndDate') }}
        </label>
        <input
          type="date"
          :value="recurrenceEndValue"
          class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
          @change="onRecurrenceEndChange"
        />
      </div>
      <div>
        <label class="block text-xs text-content-secondary mb-1">
          {{ t('todo.relatedNote') }}
        </label>
        <select
          v-model="noteIdValue"
          class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
        >
          <option :value="0">{{ t('todo.noRelatedNote') }}</option>
          <option v-for="n in notes" :key="n.id" :value="n.id">{{ n.title }}</option>
        </select>
      </div>
      <div>
        <label class="block text-xs text-content-secondary mb-1">
          {{ t('todo.tags') }}
        </label>
        <select
          v-model="selectedTagIds"
          multiple
          size="4"
          class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
        >
          <option v-for="tag in tags" :key="tag.id" :value="tag.id">{{ tag.name }}</option>
        </select>
      </div>
      <div>
        <label class="block text-xs text-content-secondary mb-1">
          {{ t('settings.mcpAccess') }}
        </label>
        <select
          v-model.number="draft.mcpAccess"
          class="w-full px-3 py-2 text-sm bg-surface-alt border border-edge rounded-lg text-content outline-none focus:border-blue-500"
        >
          <option :value="0">{{ t('settings.mcpAccessInherit') }}</option>
          <option :value="1">{{ t('settings.mcpAccessReadWrite') }}</option>
          <option :value="2">{{ t('settings.mcpAccessReadOnly') }}</option>
          <option :value="3">{{ t('settings.mcpAccessDeny') }}</option>
        </select>
      </div>
      <div class="flex justify-end gap-2 pt-2">
        <button
          class="px-3 py-1.5 text-sm rounded-lg border border-edge text-content-secondary hover:bg-surface-dim transition-colors cursor-pointer"
          @click="visible = false"
        >
          {{ t('common.cancel') }}
        </button>
        <button
          class="px-3 py-1.5 text-sm rounded-lg bg-blue-500 text-white hover:bg-blue-600 transition-colors cursor-pointer"
          @click="save"
        >
          {{ t('common.save') }}
        </button>
      </div>
    </div>
  </Dialog>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Dialog, showNotification } from './ui'
import { noteApi } from '../api/note'
import { todoApi } from '../api/todo'
import type { Note, Tag, Todo } from '../types'

const props = defineProps<{ modelValue: boolean; todo: Todo | null }>()
const emit = defineEmits<{ 'update:modelValue': [value: boolean]; saved: [] }>()
const { t } = useI18n()

const visible = computed({
  get: () => props.modelValue,
  set: (value: boolean) => emit('update:modelValue', value),
})

const draft = ref<Todo | null>(null)

const tags = ref<Tag[]>([])
const selectedTagIds = ref<number[]>([])

/** 加载全部标签（复用笔记标签体系） */
const loadTags = async () => {
  try {
    tags.value = await invoke<Tag[]>('find_all_tags')
  } catch {
    // 标签加载失败不阻塞编辑
  }
}

/** 加载待办已关联的标签 */
const loadTodoTags = async (todoId: number) => {
  try {
    selectedTagIds.value = await todoApi.findTags(todoId)
  } catch {
    selectedTagIds.value = []
  }
}

watch(
  () => props.todo,
  (val) => {
    draft.value = val ? { ...val } : null
    selectedTagIds.value = []
    if (val && val.id > 0) {
      void loadTodoTags(val.id)
    }
  },
  { immediate: true },
)

const notes = ref<Note[]>([])

/** 加载笔记列表用于关联下拉（个人笔记量有限，一次性取前 200 条） */
const loadNotes = async () => {
  try {
    const res = await noteApi.searchPageNotes({
      pageIndex: 1,
      pageSize: 200,
      notebookId: 0,
      tagId: 0,
      keyword: '',
      sortField: 'updateTime',
      sortOrder: 'desc',
    })
    notes.value = res.data
  } catch {
    // 笔记列表加载失败不阻塞待办编辑
  }
}

// 打开对话框时刷新笔记与标签列表
watch(visible, (value) => {
  if (value) {
    void loadNotes()
    void loadTags()
  }
})

/** 关联笔记：0 表示不关联，对应数据库 NULL */
const noteIdValue = computed({
  get: () => draft.value?.noteId ?? 0,
  set: (value: number) => {
    if (draft.value) {
      draft.value.noteId = value > 0 ? value : null
    }
  },
})

const dueDateValue = computed(() =>
  draft.value?.dueDate ? draft.value.dueDate.substring(0, 10) : '',
)

const onDueChange = (event: Event) => {
  if (!draft.value) return
  const value = (event.target as HTMLInputElement).value
  draft.value.dueDate = value ? `${value} 23:59:59` : null
}

const remindAtValue = computed(() => {
  if (!draft.value?.remindAt) return ''
  const value = draft.value.remindAt
  return value.length >= 16 ? `${value.substring(0, 10)}T${value.substring(11, 16)}` : ''
})

const onRemindChange = (event: Event) => {
  if (!draft.value) return
  const value = (event.target as HTMLInputElement).value
  draft.value.remindAt = value ? `${value.replace('T', ' ')}:00` : null
}

const recurrenceEndValue = computed(() =>
  draft.value?.recurrenceEndDate ? draft.value.recurrenceEndDate.substring(0, 10) : '',
)

const onRecurrenceEndChange = (event: Event) => {
  if (!draft.value) return
  const value = (event.target as HTMLInputElement).value
  draft.value.recurrenceEndDate = value ? `${value} 23:59:59` : null
}

const save = async () => {
  if (!draft.value) return
  if (!draft.value.title.trim()) {
    showNotification({ type: 'error', message: t('todo.titleRequired') })
    return
  }
  try {
    await todoApi.update(draft.value)
    // 保存标签关联（全量替换）
    if (draft.value.id > 0) {
      await todoApi.setTags(draft.value.id, selectedTagIds.value)
    }
    visible.value = false
    emit('saved')
  } catch {
    showNotification({ type: 'error', message: t('todo.updateFailed') })
  }
}
</script>
