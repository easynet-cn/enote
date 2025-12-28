<template>
  <Dialog v-model="visible" title="导出笔记" :width="400">
    <div class="space-y-4">
      <div class="text-sm text-slate-600 mb-4">选择导出格式，将笔记保存到本地文件</div>

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
              <div class="font-medium text-slate-900">{{ format.label }}</div>
              <div class="text-xs text-slate-500">{{ format.description }}</div>
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
        <Button type="secondary" @click="visible = false">取消</Button>
        <Button type="primary" :loading="exporting" @click="handleExport">
          <template #icon>
            <Download class="w-4 h-4" />
          </template>
          导出
        </Button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { FileText, FileJson, FileCode, Check, Download } from 'lucide-vue-next'
import { Dialog, Button } from './ui'
import { showNotification } from './ui/notification'
import { exportNote, getExportFormats, type ExportFormat } from '../utils/export'
import type { ShowNote } from '../types'

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
    showNotification({ type: 'error', message: '没有可导出的笔记' })
    return
  }

  exporting.value = true

  try {
    const success = await exportNote({
      note: props.note,
      format: selectedFormat.value,
    })

    if (success) {
      showNotification({ type: 'success', message: '笔记导出成功' })
      visible.value = false
    }
  } catch {
    showNotification({ type: 'error', message: '导出失败，请重试' })
  } finally {
    exporting.value = false
  }
}
</script>

<style scoped>
.export-format-item {
  display: block;
  padding: 12px 16px;
  border: 2px solid #e2e8f0;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s ease;
  background: white;
}

.export-format-item:hover {
  border-color: #cbd5e1;
  background: #f8fafc;
}

.export-format-item.active {
  border-color: #4f46e5;
  background: linear-gradient(135deg, #eef2ff 0%, #e0e7ff 100%);
}

.format-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: 10px;
  background: #f1f5f9;
  color: #64748b;
  transition: all 0.2s ease;
}

.export-format-item.active .format-icon {
  background: #4f46e5;
  color: white;
}

.format-check {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: 2px solid #cbd5e1;
  background: white;
  color: transparent;
  transition: all 0.2s ease;
}

.export-format-item.active .format-check {
  border-color: #4f46e5;
  background: #4f46e5;
  color: white;
}
</style>
