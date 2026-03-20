import { ref, onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { syncApi, profileApi } from '../api/note'
import type {
  SyncLog,
  SyncLogDetail,
  SyncOptions,
  SyncPreview,
  SyncProgress,
  ProfileSummary,
} from '../types'
import { showNotification } from '../components/ui/notification'
import { parseError } from '../utils/errorHandler'

export function useSync() {
  const loading = ref(false)
  const syncing = ref(false)
  const preview = ref<SyncPreview | null>(null)
  const progress = ref<SyncProgress | null>(null)
  const syncResult = ref<SyncLog | null>(null)
  const profiles = ref<ProfileSummary[]>([])

  let unlistenProgress: UnlistenFn | null = null

  async function loadProfiles() {
    try {
      const all = await profileApi.listProfiles()
      profiles.value = all.filter((p) => !p.isActive)
    } catch (e: unknown) {
      showNotification({ type: 'error', message: parseError(e) })
    }
  }

  async function loadPreview(targetProfileId: string, targetDbPassword?: string) {
    loading.value = true
    try {
      preview.value = await syncApi.getPreview(targetProfileId, targetDbPassword)
    } catch (e: unknown) {
      showNotification({ type: 'error', message: parseError(e) })
      preview.value = null
    } finally {
      loading.value = false
    }
  }

  async function startSync(
    options: SyncOptions,
    targetDbPassword?: string,
    targetEncryptionKey?: string,
  ) {
    syncing.value = true
    progress.value = null
    syncResult.value = null

    try {
      const result = await syncApi.startSync(options, targetDbPassword, targetEncryptionKey)
      syncResult.value = result
      if (result.failedCount > 0) {
        showNotification({
          type: 'warning',
          message: `同步完成: ${result.successCount} 成功, ${result.failedCount} 失败`,
        })
      } else {
        showNotification({ type: 'success', message: `同步完成: ${result.successCount} 条记录` })
      }
    } catch (e: unknown) {
      showNotification({ type: 'error', message: parseError(e) })
    } finally {
      syncing.value = false
    }
  }

  async function setupProgressListener() {
    unlistenProgress = await listen<SyncProgress>('sync-progress', (event) => {
      progress.value = event.payload
    })
  }

  function cleanup() {
    if (unlistenProgress) {
      unlistenProgress()
      unlistenProgress = null
    }
  }

  onMounted(() => {
    setupProgressListener()
  })

  onUnmounted(() => {
    cleanup()
  })

  return {
    loading,
    syncing,
    preview,
    progress,
    syncResult,
    profiles,
    loadProfiles,
    loadPreview,
    startSync,
  }
}

export function useSyncHistory() {
  const loading = ref(false)
  const logs = ref<SyncLog[]>([])
  const total = ref(0)
  const pageIndex = ref(1)
  const pageSize = ref(20)

  const details = ref<SyncLogDetail[]>([])
  const detailTotal = ref(0)
  const detailPageIndex = ref(1)
  const detailPageSize = ref(50)

  async function loadLogs() {
    loading.value = true
    try {
      const result = await syncApi.findSyncLogs(pageIndex.value, pageSize.value)
      logs.value = result.data
      total.value = result.total
    } catch (e: unknown) {
      showNotification({ type: 'error', message: parseError(e) })
    } finally {
      loading.value = false
    }
  }

  async function loadDetails(syncLogId: number, tableName?: string, status?: string) {
    loading.value = true
    try {
      const result = await syncApi.findSyncLogDetails(
        syncLogId,
        tableName,
        status,
        detailPageIndex.value,
        detailPageSize.value,
      )
      details.value = result.data
      detailTotal.value = result.total
    } catch (e: unknown) {
      showNotification({ type: 'error', message: parseError(e) })
    } finally {
      loading.value = false
    }
  }

  async function deleteLog(syncLogId: number) {
    try {
      await syncApi.deleteSyncLog(syncLogId)
      showNotification({ type: 'success', message: '删除成功' })
      await loadLogs()
    } catch (e: unknown) {
      showNotification({ type: 'error', message: parseError(e) })
    }
  }

  async function clearAll() {
    try {
      await syncApi.clearSyncLogs()
      showNotification({ type: 'success', message: '已清空所有同步记录' })
      logs.value = []
      total.value = 0
    } catch (e: unknown) {
      showNotification({ type: 'error', message: parseError(e) })
    }
  }

  async function exportLog(syncLogId: number, path: string) {
    try {
      await syncApi.exportSyncLog(syncLogId, path)
      showNotification({ type: 'success', message: '导出成功' })
    } catch (e: unknown) {
      showNotification({ type: 'error', message: parseError(e) })
    }
  }

  return {
    loading,
    logs,
    total,
    pageIndex,
    pageSize,
    details,
    detailTotal,
    detailPageIndex,
    detailPageSize,
    loadLogs,
    loadDetails,
    deleteLog,
    clearAll,
    exportLog,
  }
}
