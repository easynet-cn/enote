<template>
  <Dialog v-model="visible" :title="t('editorSettings.title')" :width="500">
    <div class="space-y-4" role="form" :aria-label="t('editorSettings.ariaLabel')">
      <div>
        <label class="block text-sm font-medium text-content-secondary mb-2">{{
          t('editorSettings.notebook')
        }}</label>
        <Select
          v-model="form.notebookId"
          :options="notebookOptions"
          :placeholder="t('editorSettings.selectNotebook')"
          clearable
          filterable
        />
      </div>
      <div>
        <span class="block text-sm font-medium text-content-secondary mb-2" id="tags-label">{{
          t('editorSettings.tags')
        }}</span>
        <div class="flex flex-wrap gap-2" role="group" aria-labelledby="tags-label">
          <label
            v-for="tag in availableTags"
            :key="tag.id"
            class="tag-select-item"
            :class="{ 'tag-select-item-active': form.tagIds.includes(tag.id) }"
          >
            <input
              type="checkbox"
              :value="tag.id"
              v-model="form.tagIds"
              class="sr-only"
              :aria-label="tag.name"
            />
            <span class="tag-select-check">
              <Check class="w-3 h-3" />
            </span>
            <span :class="tag.cls" aria-hidden="true">●</span>
            <span class="text-sm">{{ tag.name }}</span>
          </label>
        </div>
      </div>
      <div>
        <label class="block text-sm font-medium text-content-secondary mb-2">{{
          t('settings.mcpAccess')
        }}</label>
        <select
          v-model.number="form.mcpAccess"
          class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
        >
          <option :value="0">{{ t('settings.mcpAccessInherit') }}</option>
          <option :value="1">{{ t('settings.mcpAccessReadWrite') }}</option>
          <option :value="2">{{ t('settings.mcpAccessReadOnly') }}</option>
          <option :value="3">{{ t('settings.mcpAccessDeny') }}</option>
        </select>
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end gap-3">
        <Button type="secondary" @click="visible = false">{{ t('common.cancel') }}</Button>
        <Button type="primary" @click="handleSubmit">{{ t('common.save') }}</Button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { computed, reactive, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Check } from 'lucide-vue-next'
import { Button, Select, Dialog } from './ui'
import type { SelectOption } from './ui'
import type { ShowNotebook, ShowTag } from '../types'
import { McpAccess } from '../types'

const { t } = useI18n()

interface Props {
  notebooks: ShowNotebook[]
  tags: ShowTag[]
  notebookId: string
  selectedTagIds: string[]
  mcpAccess?: McpAccess
}

const props = defineProps<Props>()

const visible = defineModel<boolean>({ default: false })

const emit = defineEmits<{
  save: [notebookId: string, tagIds: string[], mcpAccess: McpAccess]
}>()

interface SettingForm {
  notebookId: string
  tagIds: string[]
  mcpAccess: McpAccess
}

const form = reactive<SettingForm>({
  notebookId: '',
  tagIds: [],
  mcpAccess: McpAccess.Inherit,
})

// 笔记本选项（过滤掉"全部"）
const notebookOptions = computed<SelectOption[]>(() => {
  return props.notebooks
    .filter((n) => n.id !== '0')
    .map((n) => ({
      label: n.name || '',
      value: n.id,
    }))
})

// 可选标签（过滤掉"全部"）
const availableTags = computed(() => {
  return props.tags.filter((t) => t.id !== '0')
})

// 监听 visible 变化，初始化表单
watch(visible, (newVal) => {
  if (newVal) {
    form.notebookId = props.notebookId
    form.tagIds = [...props.selectedTagIds]
    form.mcpAccess = props.mcpAccess ?? McpAccess.Inherit
  }
})

const handleSubmit = () => {
  emit('save', form.notebookId, form.tagIds, form.mcpAccess)
  visible.value = false
}
</script>

<style scoped>
/* 标签选择样式 */
.tag-select-item {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border: 2px solid var(--color-border);
  border-radius: 20px;
  cursor: pointer;
  transition: all 0.2s ease;
  background: var(--color-bg-primary);
  user-select: none;
}

.tag-select-item:hover {
  border-color: var(--color-text-disabled);
  background: var(--color-bg-secondary);
}

.tag-select-item-active {
  border-color: var(--color-primary);
  background: linear-gradient(
    135deg,
    var(--color-primary-lighter) 0%,
    var(--color-primary-light) 100%
  );
  box-shadow: var(--shadow-primary);
}

.tag-select-item-active:hover {
  border-color: var(--color-primary-hover);
  background: linear-gradient(135deg, var(--color-primary-light) 0%, #c7d2fe 100%);
}

.tag-select-check {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  border: 2px solid var(--color-text-disabled);
  background: var(--color-bg-primary);
  color: transparent;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.tag-select-item-active .tag-select-check {
  border-color: var(--color-primary);
  background: var(--color-primary);
  color: white;
}
</style>
