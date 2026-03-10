<template>
  <Dialog v-model="visible" :title="t('backup.title')" :width="480">
    <!-- Tab 切换 -->
    <div class="flex border-b border-edge mb-4">
      <button
        v-for="tab in tabs"
        :key="tab.value"
        :class="[
          'flex-1 py-2.5 text-sm font-medium transition-colors relative',
          activeTab === tab.value ? 'text-indigo-600' : 'text-content-secondary hover:text-content',
        ]"
        @click="activeTab = tab.value"
      >
        {{ tab.label }}
        <div
          v-if="activeTab === tab.value"
          class="absolute bottom-0 left-0 right-0 h-0.5 bg-indigo-600"
        />
      </button>
    </div>

    <!-- 导出 Tab -->
    <div v-if="activeTab === 'export'">
      <div class="text-sm text-content-secondary mb-3">{{ t('backup.selectFormat') }}</div>
      <div class="space-y-2">
        <label
          v-for="fmt in formats"
          :key="fmt.value"
          class="backup-format-item"
          :class="{ active: selectedFormat === fmt.value }"
        >
          <input type="radio" :value="fmt.value" v-model="selectedFormat" class="sr-only" />
          <div class="flex items-center gap-3">
            <div class="format-icon">
              <FileText v-if="fmt.value === 'sql'" class="w-5 h-5" />
              <Sheet v-else-if="fmt.value === 'excel'" class="w-5 h-5" />
              <FileSpreadsheet v-else class="w-5 h-5" />
            </div>
            <div class="flex-1">
              <div class="font-medium text-content">{{ fmt.label }}</div>
              <div class="text-xs text-content-secondary">{{ fmt.desc }}</div>
            </div>
            <div class="format-check">
              <Check class="w-4 h-4" />
            </div>
          </div>
        </label>
      </div>
    </div>

    <!-- 导入 Tab -->
    <div v-if="activeTab === 'import'">
      <div class="text-sm text-content-secondary mb-3">{{ t('backup.selectFormat') }}</div>
      <div class="space-y-2">
        <label
          v-for="fmt in formats"
          :key="fmt.value"
          class="backup-format-item"
          :class="{ active: selectedFormat === fmt.value }"
        >
          <input type="radio" :value="fmt.value" v-model="selectedFormat" class="sr-only" />
          <div class="flex items-center gap-3">
            <div class="format-icon">
              <FileText v-if="fmt.value === 'sql'" class="w-5 h-5" />
              <Sheet v-else-if="fmt.value === 'excel'" class="w-5 h-5" />
              <FileSpreadsheet v-else class="w-5 h-5" />
            </div>
            <div class="flex-1">
              <div class="font-medium text-content">{{ fmt.label }}</div>
              <div class="text-xs text-content-secondary">{{ fmt.desc }}</div>
            </div>
            <div class="format-check">
              <Check class="w-4 h-4" />
            </div>
          </div>
        </label>
      </div>
      <div class="mt-4 p-3 bg-amber-50 border border-amber-200 rounded-lg">
        <div class="flex items-start gap-2">
          <AlertTriangle class="w-4 h-4 text-amber-500 mt-0.5 shrink-0" />
          <div class="text-xs text-amber-700">{{ t('backup.importWarning') }}</div>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-end gap-3">
        <Button type="secondary" @click="visible = false">{{ t('common.cancel') }}</Button>
        <Button
          v-if="activeTab === 'export'"
          type="primary"
          :loading="processing"
          @click="handleExport"
        >
          <template #icon>
            <Download class="w-4 h-4" />
          </template>
          {{ t('backup.exportButton') }}
        </Button>
        <Button v-else type="primary" :loading="processing" @click="handleImport">
          <template #icon>
            <Upload class="w-4 h-4" />
          </template>
          {{ t('backup.importButton') }}
        </Button>
      </div>
    </template>
  </Dialog>

  <ConfirmDialog
    v-model="confirmVisible"
    :title="t('backup.importConfirm.title')"
    :message="t('backup.importConfirm.message')"
    type="danger"
    :confirm-text="t('backup.importConfirm.confirmText')"
    @confirm="doImport"
  />
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { save, open } from '@tauri-apps/plugin-dialog'
import {
  FileText,
  Sheet,
  FileSpreadsheet,
  Check,
  Download,
  Upload,
  AlertTriangle,
} from 'lucide-vue-next'
import { Dialog, Button, ConfirmDialog } from './ui'
import { showNotification } from './ui/notification'
import { backupApi } from '../api/note'

const { t } = useI18n()

const visible = defineModel<boolean>({ default: false })
const emit = defineEmits<{ imported: [] }>()

const activeTab = ref<'export' | 'import'>('export')
const selectedFormat = ref('sql')
const processing = ref(false)
const confirmVisible = ref(false)
let pendingImportPath = ''

const tabs = computed(() => [
  { value: 'export' as const, label: t('backup.tabExport') },
  { value: 'import' as const, label: t('backup.tabImport') },
])

const formats = computed(() => [
  { value: 'sql', label: 'SQL', desc: t('backup.sqlDesc') },
  { value: 'excel', label: 'Excel (.xlsx)', desc: t('backup.excelDesc') },
  { value: 'csv', label: 'CSV (.zip)', desc: t('backup.csvDesc') },
])

const formatExtMap: Record<string, { name: string; extensions: string[] }> = {
  sql: { name: 'SQL', extensions: ['sql'] },
  excel: { name: 'Excel', extensions: ['xlsx'] },
  csv: { name: 'CSV ZIP', extensions: ['zip'] },
}

const defaultFileMap: Record<string, string> = {
  sql: 'enote_backup.sql',
  excel: 'enote_backup.xlsx',
  csv: 'enote_backup.zip',
}

const handleExport = async () => {
  const filter = formatExtMap[selectedFormat.value]
  const filePath = await save({
    filters: [filter],
    defaultPath: defaultFileMap[selectedFormat.value],
  })

  if (!filePath) return

  processing.value = true
  try {
    await backupApi.exportBackup(selectedFormat.value, filePath)
    showNotification({ type: 'success', message: t('backup.exportSuccess') })
    visible.value = false
  } catch {
    showNotification({ type: 'error', message: t('backup.exportError') })
  } finally {
    processing.value = false
  }
}

const handleImport = async () => {
  const filter = formatExtMap[selectedFormat.value]
  const filePath = await open({
    filters: [filter],
    multiple: false,
  })

  if (!filePath) return

  pendingImportPath = filePath as string
  confirmVisible.value = true
}

const doImport = async () => {
  processing.value = true
  try {
    await backupApi.importBackup(selectedFormat.value, pendingImportPath)
    showNotification({ type: 'success', message: t('backup.importSuccess') })
    visible.value = false
    emit('imported')
  } catch {
    showNotification({ type: 'error', message: t('backup.importError') })
  } finally {
    processing.value = false
  }
}
</script>

<style scoped>
.backup-format-item {
  display: block;
  padding: 12px 16px;
  border: 2px solid var(--color-border);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s ease;
  background: var(--color-bg-primary);
}

.backup-format-item:hover {
  border-color: var(--color-text-disabled);
  background: var(--color-bg-secondary);
}

.backup-format-item.active {
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

.backup-format-item.active .format-icon {
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

.backup-format-item.active .format-check {
  border-color: var(--color-primary);
  background: var(--color-primary);
  color: white;
}
</style>
