<template>
  <Dialog v-model="visible" title="设置" :width="500">
    <div class="space-y-4" role="form" aria-label="笔记设置">
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-2">笔记本</label>
        <Select
          v-model="form.notebookId"
          :options="notebookOptions"
          placeholder="请选择笔记本"
          clearable
          filterable
        />
      </div>
      <div>
        <span class="block text-sm font-medium text-slate-700 mb-2" id="tags-label">标签</span>
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
    </div>
    <template #footer>
      <div class="flex justify-end gap-3">
        <Button type="secondary" @click="visible = false">取消</Button>
        <Button type="primary" @click="handleSubmit">保存</Button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { computed, reactive, watch } from 'vue'
import { Check } from 'lucide-vue-next'
import { Button, Select, Dialog } from './ui'
import type { SelectOption } from './ui'
import type { ShowNotebook, ShowTag } from '../types'

interface Props {
  notebooks: ShowNotebook[]
  tags: ShowTag[]
  notebookId: string
  selectedTagIds: string[]
}

const props = defineProps<Props>()

const visible = defineModel<boolean>({ default: false })

const emit = defineEmits<{
  save: [notebookId: string, tagIds: string[]]
}>()

interface SettingForm {
  notebookId: string
  tagIds: string[]
}

const form = reactive<SettingForm>({
  notebookId: '',
  tagIds: [],
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
  }
})

const handleSubmit = () => {
  emit('save', form.notebookId, form.tagIds)
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
  border: 2px solid #e2e8f0;
  border-radius: 20px;
  cursor: pointer;
  transition: all 0.2s ease;
  background: white;
  user-select: none;
}

.tag-select-item:hover {
  border-color: #cbd5e1;
  background: #f8fafc;
}

.tag-select-item-active {
  border-color: #4f46e5;
  background: linear-gradient(135deg, #eef2ff 0%, #e0e7ff 100%);
  box-shadow: 0 2px 8px rgba(79, 70, 229, 0.25);
}

.tag-select-item-active:hover {
  border-color: #4338ca;
  background: linear-gradient(135deg, #e0e7ff 0%, #c7d2fe 100%);
}

.tag-select-check {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  border: 2px solid #cbd5e1;
  background: white;
  color: transparent;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.tag-select-item-active .tag-select-check {
  border-color: #4f46e5;
  background: #4f46e5;
  color: white;
}
</style>
