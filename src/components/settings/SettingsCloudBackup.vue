<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { cloudBackupApi } from '../../api/note'
import { showNotification } from '../ui/notification'
import { AppSelect } from '../ui'
import type { AppSelectOption } from '../ui'
import type { CloudStorageConfig, CloudBackupEntry } from '../../types'

const { t } = useI18n()

const cloudBackupEnabled = defineModel<boolean>('enabled', { required: true })
const cloudBackupRetention = defineModel<string>('retention', { required: true })

const emit = defineEmits<{
  (e: 'save'): void
  (e: 'configChanged'): void
}>()

// 表单状态
const provider = ref('oss')
const endpoint = ref('')
const bucket = ref('')
const region = ref('')
const accessKeyId = ref('')
const secretAccessKey = ref('')
const prefix = ref('enote-backups/')
const username = ref('')
const password = ref('')

// UI 状态
const testing = ref(false)
const saving = ref(false)
const backingUp = ref(false)
const lastCloudBackup = ref('')
const cloudBackups = ref<CloudBackupEntry[]>([])
const showBackupList = ref(false)

const providerOptions = computed<AppSelectOption[]>(() => [
  { value: 'oss', label: t('settings.cloudBackupProviderOss') },
  { value: 's3', label: t('settings.cloudBackupProviderS3') },
  { value: 'cos', label: t('settings.cloudBackupProviderCos') },
  { value: 'minio', label: t('settings.cloudBackupProviderMinio') },
  { value: 'webdav', label: t('settings.cloudBackupProviderWebdav') },
])

const retentionOptions = computed<AppSelectOption[]>(() => [
  { value: '5', label: '5' },
  { value: '10', label: '10' },
  { value: '20', label: '20' },
  { value: '50', label: '50' },
])

const isWebdav = computed(() => provider.value === 'webdav')
const isObjectStorage = computed(() => !isWebdav.value)

const toggleCloudBackup = () => {
  cloudBackupEnabled.value = !cloudBackupEnabled.value
  emit('save')
  emit('configChanged')
}

const buildConfig = (): CloudStorageConfig => ({
  provider: provider.value,
  endpoint: endpoint.value,
  bucket: bucket.value,
  region: region.value,
  accessKeyId: accessKeyId.value,
  secretAccessKey: secretAccessKey.value,
  prefix: prefix.value,
  username: username.value,
  password: password.value,
})

const testConnection = async () => {
  testing.value = true
  try {
    await cloudBackupApi.testConnection(buildConfig())
    showNotification({ type: 'success', message: t('settings.cloudBackupTestSuccess') })
  } catch {
    showNotification({ type: 'error', message: t('settings.cloudBackupTestFailed') })
  } finally {
    testing.value = false
  }
}

const saveConfig = async () => {
  saving.value = true
  try {
    await cloudBackupApi.saveConfig(buildConfig())
    showNotification({ type: 'success', message: t('settings.cloudBackupConfigSaved') })
    emit('save')
    emit('configChanged')
  } catch {
    showNotification({ type: 'error', message: t('settings.cloudBackupConfigSaveFailed') })
  } finally {
    saving.value = false
  }
}

const doCloudBackupNow = async () => {
  backingUp.value = true
  try {
    const filename = await cloudBackupApi.cloudBackupNow()
    lastCloudBackup.value = filename
    const retention = parseInt(cloudBackupRetention.value) || 10
    await cloudBackupApi.cleanupBackups(retention)
    showNotification({
      type: 'success',
      message: t('settings.cloudBackupSuccess', { name: filename }),
    })
  } catch {
    showNotification({ type: 'error', message: t('settings.cloudBackupFailed') })
  } finally {
    backingUp.value = false
  }
}

const loadCloudBackups = async () => {
  try {
    cloudBackups.value = await cloudBackupApi.listBackups()
    if (cloudBackups.value.length > 0) {
      lastCloudBackup.value = cloudBackups.value[0].name
    }
  } catch {
    // ignore
  }
}

const toggleBackupList = async () => {
  showBackupList.value = !showBackupList.value
  if (showBackupList.value) {
    await loadCloudBackups()
  }
}

const downloadBackup = async (filename: string) => {
  try {
    await cloudBackupApi.downloadBackup(filename)
    showNotification({
      type: 'success',
      message: t('settings.cloudBackupDownloadSuccess', { name: filename }),
    })
  } catch {
    showNotification({ type: 'error', message: t('settings.cloudBackupDownloadFailed') })
  }
}

const formatSize = (bytes: number): string => {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

// 加载已保存的配置
const loadSavedConfig = (settings: Record<string, string>) => {
  if (settings.cloudBackupProvider) provider.value = settings.cloudBackupProvider
  if (settings.cloudBackupEndpoint) endpoint.value = settings.cloudBackupEndpoint
  if (settings.cloudBackupBucket) bucket.value = settings.cloudBackupBucket
  if (settings.cloudBackupRegion) region.value = settings.cloudBackupRegion
  if (settings.cloudBackupAccessKeyId) accessKeyId.value = settings.cloudBackupAccessKeyId
  if (settings.cloudBackupPrefix) prefix.value = settings.cloudBackupPrefix
  if (settings.cloudBackupUsername) username.value = settings.cloudBackupUsername
  // secretAccessKey 和 password 存在 keychain，前端不回显
}

const loadLastCloudBackup = async () => {
  try {
    const backups = await cloudBackupApi.listBackups()
    if (backups.length > 0) {
      lastCloudBackup.value = backups[0].name
    }
  } catch {
    // ignore - cloud backup may not be configured
  }
}

defineExpose({ loadSavedConfig, loadLastCloudBackup })
</script>

<template>
  <div>
    <h3 class="text-sm font-semibold text-content-secondary mb-3">
      {{ t('settings.cloudBackup') }}
    </h3>
    <div class="space-y-4">
      <!-- 启用云备份 -->
      <div class="flex items-center justify-between">
        <label class="text-sm text-content-secondary">{{ t('settings.cloudBackupEnabled') }}</label>
        <button
          @click="toggleCloudBackup"
          class="relative w-10 h-5 rounded-full transition-colors"
          :class="cloudBackupEnabled ? 'bg-indigo-600' : 'bg-slate-300'"
        >
          <span
            class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full transition-transform shadow-sm"
            :class="cloudBackupEnabled ? 'translate-x-5' : ''"
          />
        </button>
      </div>

      <template v-if="cloudBackupEnabled">
        <!-- 存储提供商 -->
        <div class="flex items-center justify-between">
          <label class="text-sm text-content-secondary">{{
            t('settings.cloudBackupProvider')
          }}</label>
          <AppSelect v-model="provider" :options="providerOptions" size="sm" />
        </div>

        <!-- 连接配置 -->
        <div class="space-y-3 p-3 bg-surface-dim rounded-lg">
          <!-- Endpoint -->
          <div>
            <label class="block text-xs text-content-tertiary mb-1">{{
              t('settings.cloudBackupEndpoint')
            }}</label>
            <input
              v-model="endpoint"
              type="text"
              :placeholder="
                isWebdav
                  ? 'https://dav.jianguoyun.com/dav/'
                  : 'https://oss-cn-hangzhou.aliyuncs.com'
              "
              class="w-full px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
            />
          </div>

          <!-- Object Storage 专属字段 -->
          <template v-if="isObjectStorage">
            <div class="grid grid-cols-2 gap-3">
              <div>
                <label class="block text-xs text-content-tertiary mb-1">{{
                  t('settings.cloudBackupBucket')
                }}</label>
                <input
                  v-model="bucket"
                  type="text"
                  class="w-full px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                />
              </div>
              <div>
                <label class="block text-xs text-content-tertiary mb-1">{{
                  t('settings.cloudBackupRegion')
                }}</label>
                <input
                  v-model="region"
                  type="text"
                  placeholder="cn-hangzhou"
                  class="w-full px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                />
              </div>
            </div>

            <div>
              <label class="block text-xs text-content-tertiary mb-1">{{
                t('settings.cloudBackupAccessKeyId')
              }}</label>
              <input
                v-model="accessKeyId"
                type="text"
                class="w-full px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
              />
            </div>
            <div>
              <label class="block text-xs text-content-tertiary mb-1">{{
                t('settings.cloudBackupSecretAccessKey')
              }}</label>
              <input
                v-model="secretAccessKey"
                type="password"
                class="w-full px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
              />
            </div>
          </template>

          <!-- WebDAV 专属字段 -->
          <template v-if="isWebdav">
            <div>
              <label class="block text-xs text-content-tertiary mb-1">{{
                t('settings.cloudBackupUsername')
              }}</label>
              <input
                v-model="username"
                type="text"
                class="w-full px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
              />
            </div>
            <div>
              <label class="block text-xs text-content-tertiary mb-1">{{
                t('settings.cloudBackupPassword')
              }}</label>
              <input
                v-model="password"
                type="password"
                class="w-full px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
              />
            </div>
          </template>

          <!-- 目录前缀 -->
          <div>
            <label class="block text-xs text-content-tertiary mb-1">{{
              t('settings.cloudBackupPrefix')
            }}</label>
            <input
              v-model="prefix"
              type="text"
              placeholder="enote-backups/"
              class="w-full px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
            />
          </div>
        </div>

        <!-- 云端保留份数 -->
        <div class="flex items-center justify-between">
          <label class="text-sm text-content-secondary">{{
            t('settings.cloudBackupRetention')
          }}</label>
          <AppSelect
            v-model="cloudBackupRetention"
            :options="retentionOptions"
            size="sm"
            @change="emit('save')"
          />
        </div>

        <!-- 操作按钮 -->
        <div class="flex items-center gap-2">
          <button
            @click="testConnection"
            :disabled="testing"
            class="px-3 py-1.5 text-sm text-indigo-600 border border-indigo-300 rounded-lg hover:bg-indigo-50 transition-colors disabled:opacity-50"
          >
            {{
              testing ? t('settings.cloudBackupTesting') : t('settings.cloudBackupTestConnection')
            }}
          </button>
          <button
            @click="saveConfig"
            :disabled="saving"
            class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50"
          >
            {{ saving ? '...' : t('common.save') }}
          </button>
          <button
            @click="doCloudBackupNow"
            :disabled="backingUp"
            class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50"
          >
            {{ backingUp ? '...' : t('settings.cloudBackupNow') }}
          </button>
        </div>

        <!-- 最近云端备份 + 列表 -->
        <div>
          <div class="flex items-center justify-between">
            <span class="text-xs text-content-tertiary">
              {{ t('settings.cloudBackupLastBackup') }}:
              {{ lastCloudBackup || t('settings.never') }}
            </span>
            <button @click="toggleBackupList" class="text-xs text-indigo-600 hover:text-indigo-700">
              {{ showBackupList ? t('common.close') : t('settings.cloudBackupListTitle') }}
            </button>
          </div>

          <!-- 云端备份列表 -->
          <div
            v-if="showBackupList"
            class="mt-2 max-h-48 overflow-y-auto border border-edge rounded-lg"
          >
            <div
              v-if="cloudBackups.length === 0"
              class="p-3 text-center text-xs text-content-tertiary"
            >
              {{ t('settings.cloudBackupEmpty') }}
            </div>
            <div
              v-for="backup in cloudBackups"
              :key="backup.name"
              class="flex items-center justify-between px-3 py-2 border-b border-edge last:border-b-0 hover:bg-surface-dim"
            >
              <div class="flex-1 min-w-0">
                <div class="text-xs text-content truncate">{{ backup.name }}</div>
                <div class="text-xs text-content-tertiary">
                  {{ formatSize(backup.size) }}
                  <span v-if="backup.lastModified"> · {{ backup.lastModified }}</span>
                </div>
              </div>
              <button
                @click="downloadBackup(backup.name)"
                class="ml-2 px-2 py-1 text-xs text-indigo-600 hover:bg-indigo-50 rounded transition-colors shrink-0"
              >
                {{ t('settings.cloudBackupDownload') }}
              </button>
            </div>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>
