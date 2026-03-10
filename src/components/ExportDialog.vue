<template>
  <Dialog v-model="visible" :title="t('export.title')" :width="400">
    <div class="space-y-4">
      <div class="text-sm text-content-secondary mb-4">{{ t('export.format') }}</div>

      <div class="space-y-2">
        <label
          v-for="format in formats"
          :key="format.value"
          class="export-format-item"
          :class="{ active: selectedFormat === format.value }"
        >
          <input type="radio" :value="format.value" v-model="selectedFormat" class="sr-only" />
          <div class="flex items-center gap-3">
            <div class="format-icon">
              <FileText v-if="format.value === 'word'" class="w-5 h-5" />
              <FileJson v-else-if="format.value === 'json'" class="w-5 h-5" />
              <FileCode v-else class="w-5 h-5" />
            </div>
            <div class="flex-1">
              <div class="font-medium text-content">{{ format.label }}</div>
              <div class="text-xs text-content-secondary">{{ format.description }}</div>
            </div>
            <div class="format-check">
              <Check class="w-4 h-4" />
            </div>
          </div>
        </label>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-end gap-3">
        <Button type="secondary" @click="visible = false">{{ t('common.cancel') }}</Button>
        <Button type="primary" :loading="exporting" @click="handleExport">
          <template #icon>
            <Download class="w-4 h-4" />
          </template>
          {{ t('export.exportButton') }}
        </Button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { FileText, FileJson, FileCode, Check, Download } from 'lucide-vue-next'
import { Dialog, Button } from './ui'
import { showNotification } from './ui/notification'
import { exportNote, getExportFormats, type ExportFormat } from '../utils/export'
import type { ShowNote } from '../types'

const { t } = useI18n()

interface Props {
  note: ShowNote | null
}

const props = defineProps<Props>()

const visible = defineModel<boolean>({ default: false })
const selectedFormat = ref<ExportFormat>('word')
const exporting = ref(false)
const formats = getExportFormats()

const handleExport = async () => {
  if (!props.note) {
    showNotification({ type: 'error', message: t('notification.error') })
    return
  }

  exporting.value = true

  try {
    const success = await exportNote({
      note: props.note,
      format: selectedFormat.value,
    })

    if (success) {
      showNotification({ type: 'success', message: t('export.success') })
      visible.value = false
    }
  } catch {
    showNotification({ type: 'error', message: t('export.error') })
  } finally {
    exporting.value = false
  }
}
</script>

<style scoped>
.export-format-item {
  display: block;
  padding: 12px 16px;
  border: 2px solid var(--color-border);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s ease;
  background: var(--color-bg-primary);
}

.export-format-item:hover {
  border-color: var(--color-text-disabled);
  background: var(--color-bg-secondary);
}

.export-format-item.active {
  border-color: var(--color-primary);
  background: linear-gradient(
    135deg,
    var(--color-primary-lighter) 0%,
    var(--color-primary-light) 100%
  );
}

.format-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: 10px;
  background: var(--color-bg-tertiary);
  color: var(--color-text-secondary);
  transition: all 0.2s ease;
}

.export-format-item.active .format-icon {
  background: var(--color-primary);
  color: white;
}

.format-check {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: 2px solid var(--color-text-disabled);
  background: var(--color-bg-primary);
  color: transparent;
  transition: all 0.2s ease;
}

.export-format-item.active .format-check {
  border-color: var(--color-primary);
  background: var(--color-primary);
  color: white;
}
</style>
