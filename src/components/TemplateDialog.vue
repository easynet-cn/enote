<template>
  <Dialog v-model="visible" :title="t('template.title')" :width="600">
    <div class="space-y-4">
      <!-- Create new template -->
      <div class="flex gap-2">
        <input
          v-model="newTemplateName"
          :placeholder="t('template.namePlaceholder')"
          class="flex-1 px-3 py-2 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
          @keyup.enter="handleCreate"
        />
        <button
          @click="handleCreate"
          :disabled="!newTemplateName.trim()"
          class="px-4 py-2 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50"
        >
          {{ t('template.create') }}
        </button>
      </div>

      <!-- Template list -->
      <div v-if="templates.length === 0" class="text-center py-8 text-content-tertiary text-sm">
        {{ t('template.noTemplates') }}
      </div>
      <div v-else class="space-y-2 max-h-80 overflow-y-auto">
        <div
          v-for="tpl in templates"
          :key="tpl.id"
          class="flex items-center justify-between p-3 rounded-lg border border-edge hover:bg-surface-alt transition-colors"
        >
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-content truncate">{{ tpl.name }}</div>
            <div class="text-xs text-content-tertiary mt-0.5">{{ tpl.updateTime }}</div>
          </div>
          <div class="flex items-center gap-2 ml-3">
            <button
              @click="$emit('use-template', tpl)"
              class="px-3 py-1 text-xs bg-indigo-50 text-indigo-600 rounded-md hover:bg-indigo-100 transition-colors"
            >
              {{ t('template.useTemplate') }}
            </button>
            <button
              @click="handleDelete(tpl)"
              class="px-3 py-1 text-xs text-red-600 hover:bg-red-50 rounded-md transition-colors"
            >
              {{ t('common.delete') }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-end">
        <Button type="secondary" @click="visible = false">{{ t('common.close') }}</Button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Dialog, Button } from './ui'
import { templateApi } from '../api/note'
import { showNotification } from './ui/notification'
import type { NoteTemplate } from '../types'

const { t } = useI18n()

const visible = defineModel<boolean>({ default: false })
defineEmits<{
  'use-template': [template: NoteTemplate]
}>()

const templates = ref<NoteTemplate[]>([])
const newTemplateName = ref('')

const loadTemplates = async () => {
  try {
    templates.value = await templateApi.findAll()
  } catch {
    // ignore
  }
}

const handleCreate = async () => {
  const name = newTemplateName.value.trim()
  if (!name) return
  try {
    await templateApi.create({
      id: 0,
      name,
      content: '',
      sortOrder: 0,
      createTime: null,
      updateTime: null,
    })
    newTemplateName.value = ''
    await loadTemplates()
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : String(e)
    showNotification({ type: 'error', message: msg })
  }
}

const handleDelete = async (tpl: NoteTemplate) => {
  try {
    await templateApi.delete(tpl.id)
    await loadTemplates()
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : String(e)
    showNotification({ type: 'error', message: msg })
  }
}

watch(visible, (val) => {
  if (val) loadTemplates()
})
</script>
