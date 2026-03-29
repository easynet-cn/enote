<template>
  <BaseDialog v-model="showDialog" :title="t('updater.newVersion')" :width="460">
    <div class="update-content">
      <!-- 版本信息 -->
      <div class="version-info">
        <div class="version-badge">
          <ArrowUpCircle class="w-5 h-5 text-indigo-500" />
          <span class="text-content-secondary text-sm">
            {{ t('updater.currentVersion', { version: currentVersion }) }}
          </span>
        </div>
        <div class="new-version">
          {{ t('updater.newVersionAvailable', { version: newVersion }) }}
        </div>
      </div>

      <!-- 更新说明 -->
      <div v-if="releaseNotes" class="release-notes">
        <h4 class="text-sm font-medium text-content-secondary mb-2">
          {{ t('updater.releaseNotes') }}
        </h4>
        <div class="notes-body" v-html="releaseNotes" />
      </div>

      <!-- 下载进度 -->
      <div v-if="downloading" class="progress-section">
        <div class="progress-bar-bg">
          <div class="progress-bar-fill" :style="{ width: downloadPercent + '%' }" />
        </div>
        <span class="text-xs text-content-secondary mt-1">
          {{ t('updater.downloadProgress', { percent: downloadPercent }) }}
        </span>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-end gap-3">
        <button v-if="!downloading" class="btn btn-secondary" @click="showDialog = false">
          {{ t('updater.later') }}
        </button>
        <button class="btn btn-primary" :disabled="downloading" @click="handleUpdate">
          <Loader2 v-if="downloading" class="w-4 h-4 animate-spin" />
          <Download v-else class="w-4 h-4" />
          <span>{{ downloading ? t('updater.downloading') : t('updater.updateNow') }}</span>
        </button>
      </div>
    </template>
  </BaseDialog>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { check, type Update } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import { ArrowUpCircle, Download, Loader2 } from 'lucide-vue-next'
import { getVersion } from '@tauri-apps/api/app'
import BaseDialog from './ui/BaseDialog.vue'
import { showNotification } from './ui/notification'
import { logger } from '../utils/logger'

const { t } = useI18n()

const showDialog = ref(false)
const currentVersion = ref('')
const newVersion = ref('')
const releaseNotes = ref('')
const downloading = ref(false)
const downloadPercent = ref(0)

let pendingUpdate: Update | null = null

onMounted(async () => {
  currentVersion.value = await getVersion()
  // 延迟 3 秒再检查更新，避免启动时阻塞
  setTimeout(() => checkForUpdates(false), 3000)
})

async function checkForUpdates(manual = true) {
  logger.info(
    'updater',
    'check',
    `Checking for updates (manual=${manual}), current version: ${currentVersion.value}`,
  )
  try {
    const update = await check()
    if (update) {
      pendingUpdate = update
      newVersion.value = update.version
      releaseNotes.value = update.body ?? ''
      showDialog.value = true
      logger.info('updater', 'found', `New version available: ${update.version}`)
    } else {
      logger.info('updater', 'check', `Already up to date (v${currentVersion.value})`)
      if (manual) {
        showNotification({
          type: 'success',
          message: t('updater.upToDate'),
        })
      }
    }
  } catch (e) {
    logger.error('updater', 'check', 'Update check failed', String(e))
    if (manual) {
      showNotification({
        type: 'error',
        message: t('updater.checkFailed'),
      })
    }
  }
}

async function handleUpdate() {
  if (!pendingUpdate) return

  downloading.value = true
  downloadPercent.value = 0
  logger.info('updater', 'download', `Downloading update v${newVersion.value}`)

  let contentLength = 0
  let downloaded = 0

  try {
    await pendingUpdate.downloadAndInstall((event) => {
      if (event.event === 'Started') {
        contentLength = event.data.contentLength ?? 0
        downloaded = 0
      } else if (event.event === 'Progress') {
        downloaded += event.data.chunkLength
        if (contentLength > 0) {
          downloadPercent.value = Math.min(100, Math.round((downloaded / contentLength) * 100))
        }
      } else if (event.event === 'Finished') {
        downloadPercent.value = 100
      }
    })

    logger.info('updater', 'install', `Update v${newVersion.value} downloaded, restarting`)
    showDialog.value = false
    showNotification({
      type: 'success',
      message: t('updater.restartRequired'),
    })

    // 短暂延迟后重启
    setTimeout(() => relaunch(), 1500)
  } catch (e) {
    downloading.value = false
    logger.error('updater', 'download', 'Update download failed', String(e))
    showNotification({
      type: 'error',
      message: t('updater.downloadFailed'),
    })
  }
}

// 暴露手动检查方法供外部调用
defineExpose({ checkForUpdates })
</script>

<style scoped>
.update-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.version-info {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.version-badge {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.new-version {
  font-size: 1rem;
  font-weight: 500;
  color: var(--color-text-primary);
}

.release-notes {
  padding: 0.75rem;
  background: var(--color-bg-secondary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--color-border);
}

.notes-body {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  line-height: 1.6;
  max-height: 200px;
  overflow-y: auto;
}

.progress-section {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.progress-bar-bg {
  width: 100%;
  height: 6px;
  background: var(--color-bg-tertiary);
  border-radius: var(--radius-full);
  overflow: hidden;
}

.progress-bar-fill {
  height: 100%;
  background: var(--color-primary);
  border-radius: var(--radius-full);
  transition: width 0.3s ease;
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.5rem 1rem;
  border-radius: var(--radius-lg);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  border: none;
  transition: all var(--transition-normal) var(--ease-default);
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--color-primary);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.btn-secondary {
  background: var(--color-bg-tertiary);
  color: var(--color-text-secondary);
}

.btn-secondary:hover {
  background: var(--color-bg-quaternary, var(--color-bg-tertiary));
  color: var(--color-text-primary);
}
</style>
