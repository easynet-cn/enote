<template>
  <Dialog v-model="visible" :title="t('sync.historyTitle')" :width="700">
    <div class="space-y-4">
      <!-- 列表 -->
      <div v-if="!selectedLog">
        <div
          v-if="logs.length === 0 && !loading"
          class="text-center py-8 text-content-tertiary text-sm"
        >
          {{ t('sync.noHistory') }}
        </div>
        <div v-else class="space-y-2 max-h-96 overflow-y-auto">
          <div
            v-for="log in logs"
            :key="log.id"
            class="flex items-center justify-between p-3 rounded-lg border border-edge hover:bg-surface-alt transition-colors"
          >
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2 text-sm">
                <span class="font-medium text-content">
                  {{ log.sourceProfile }} → {{ log.targetProfile }}
                </span>
                <span
                  class="text-[10px] px-1.5 py-0.5 rounded font-medium"
                  :class="
                    log.failedCount > 0
                      ? 'bg-yellow-50 text-yellow-700'
                      : log.status === 'failed'
                        ? 'bg-red-50 text-red-600'
                        : 'bg-green-50 text-green-600'
                  "
                >
                  {{ log.status === 'failed' ? t('sync.failed') : t('sync.completed') }}
                  {{ log.failedCount > 0 ? `${log.successCount}/${log.failedCount}` : '' }}
                </span>
              </div>
              <div class="flex items-center gap-3 text-xs text-content-tertiary mt-0.5">
                <span>{{ log.startedAt }}</span>
                <span>{{
                  log.syncMode === 'append' ? t('sync.modeAppend') : t('sync.modeOverwrite')
                }}</span>
                <span>{{ log.sourceDbType }} → {{ log.targetDbType }}</span>
                <span>{{ log.totalCount }} {{ t('sync.total') }}</span>
              </div>
            </div>
            <div class="flex items-center gap-1 ml-3 shrink-0">
              <button
                @click="selectedLog = log"
                class="px-2 py-1 text-xs text-indigo-600 hover:bg-indigo-50 rounded transition-colors"
              >
                {{ t('sync.viewDetails') }}
              </button>
              <button
                @click="handleExport(log)"
                class="px-2 py-1 text-xs text-content-secondary hover:bg-surface-dim rounded transition-colors"
              >
                {{ t('sync.exportLog') }}
              </button>
              <button
                @click="handleDelete(log)"
                class="px-2 py-1 text-xs text-red-600 hover:bg-red-50 rounded transition-colors"
              >
                {{ t('common.delete') }}
              </button>
            </div>
          </div>
        </div>

        <!-- 分页 -->
        <Pagination
          v-if="total > pageSize"
          :current-page="pageIndex"
          :total="total"
          :page-size="pageSize"
          @current-change="
            (p: number) => {
              pageIndex = p
              loadLogs()
            }
          "
          class="mt-3"
        />
      </div>

      <!-- 详情视图 -->
      <div v-else>
        <button
          @click="selectedLog = null"
          class="flex items-center gap-1 text-sm text-content-secondary hover:text-content transition-colors mb-3"
        >
          <ArrowLeft class="w-4 h-4" />
          {{ t('sync.history') }}
        </button>
        <SyncResultDialog :sync-log="selectedLog" :embedded="true" @close="selectedLog = null" />
      </div>
    </div>

    <template #footer v-if="!selectedLog">
      <div class="flex justify-between">
        <Button v-if="logs.length > 0" @click="handleClearAll" class="text-red-600 hover:bg-red-50">
          {{ t('sync.clearAll') }}
        </Button>
        <div class="flex-1"></div>
        <Button @click="visible = false">{{ t('common.close') }}</Button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { ArrowLeft } from 'lucide-vue-next'
import { Dialog, Button, Pagination } from './ui'
import { showNotification } from './ui/notification'
import { showError } from '../utils/errorHandler'
import SyncResultDialog from './SyncResultDialog.vue'
import { useSyncHistory } from '../composables/useSync'
import { save } from '@tauri-apps/plugin-dialog'
import { syncApi } from '../api/note'
import type { SyncLog } from '../types'

const props = defineProps<{
  modelValue: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
}>()

const { t } = useI18n()

const { loading, logs, total, pageIndex, pageSize, loadLogs, deleteLog, clearAll } =
  useSyncHistory()

const visible = computed({
  get: () => props.modelValue,
  set: (v) => emit('update:modelValue', v),
})

const selectedLog = ref<SyncLog | null>(null)

watch(visible, (v) => {
  if (v) {
    selectedLog.value = null
    pageIndex.value = 1
    loadLogs()
  }
})

async function handleDelete(log: SyncLog) {
  if (!confirm(t('sync.confirmDelete'))) return
  await deleteLog(log.id)
}

async function handleClearAll() {
  if (!confirm(t('sync.confirmClearAll'))) return
  await clearAll()
}

async function handleExport(log: SyncLog) {
  try {
    const path = await save({
      defaultPath: `sync_log_${log.id}.json`,
      filters: [{ name: 'JSON', extensions: ['json'] }],
    })
    if (path) {
      await syncApi.exportSyncLog(log.id, path)
      showNotification({ type: 'success', message: t('sync.exportLog') + ' OK' })
    }
  } catch (e: unknown) {
    showError(e)
  }
}
</script>
