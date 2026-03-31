<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { backupApi } from '../../api/note'
import { showNotification } from '../ui/notification'
import { AppSelect } from '../ui'
import type { AppSelectOption } from '../ui'

const { t } = useI18n()

const autoBackupEnabled = defineModel<boolean>('enabled', { required: true })
const autoBackupInterval = defineModel<string>('interval', { required: true })
const autoBackupRetention = defineModel<string>('retention', { required: true })

const emit = defineEmits<{
  (e: 'save'): void
  (e: 'backupSettingsChanged'): void
}>()

const lastBackupName = ref('')
const backingUp = ref(false)

const toggleAutoBackup = () => {
  autoBackupEnabled.value = !autoBackupEnabled.value
  emit('save')
  emit('backupSettingsChanged')
}

const backupIntervalOptions = computed<AppSelectOption[]>(() => [
  { value: '1', label: `1 ${t('settings.autoBackupIntervalHours', { n: '' }).trim()}` },
  { value: '4', label: `4 ${t('settings.autoBackupIntervalHours', { n: '' }).trim()}` },
  { value: '8', label: `8 ${t('settings.autoBackupIntervalHours', { n: '' }).trim()}` },
  { value: '24', label: `24 ${t('settings.autoBackupIntervalHours', { n: '' }).trim()}` },
])

const backupRetentionOptions = computed<AppSelectOption[]>(() => [
  { value: '5', label: t('settings.autoBackupRetentionCount', { n: 5 }) },
  { value: '10', label: t('settings.autoBackupRetentionCount', { n: 10 }) },
  { value: '20', label: t('settings.autoBackupRetentionCount', { n: 20 }) },
  { value: '50', label: t('settings.autoBackupRetentionCount', { n: 50 }) },
])

const handleIntervalChange = () => {
  emit('save')
  emit('backupSettingsChanged')
}

const doBackupNow = async () => {
  backingUp.value = true
  try {
    const filename = await backupApi.autoBackup()
    lastBackupName.value = filename
    await backupApi.cleanupOldBackups(parseInt(autoBackupRetention.value))
    showNotification({ type: 'success', message: t('settings.backupSuccess', { name: filename }) })
  } catch {
    showNotification({ type: 'error', message: t('settings.backupFailed') })
  } finally {
    backingUp.value = false
  }
}

const loadLastBackup = async () => {
  try {
    const backups = await backupApi.listAutoBackups()
    if (backups.length > 0) {
      lastBackupName.value = backups[0][0]
    }
  } catch {
    // 忽略
  }
}

defineExpose({ loadLastBackup })
</script>

<template>
  <div>
    <h3 class="text-sm font-semibold text-content-secondary mb-3">
      {{ t('settings.autoBackup') }}
    </h3>
    <div class="space-y-4">
      <!-- 启用自动备份 -->
      <div class="flex items-center justify-between">
        <label class="text-sm text-content-secondary">{{ t('settings.autoBackupEnabled') }}</label>
        <button
          @click="toggleAutoBackup"
          class="relative w-10 h-5 rounded-full transition-colors"
          :class="autoBackupEnabled ? 'bg-indigo-600' : 'bg-slate-300'"
        >
          <span
            class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full transition-transform shadow-sm"
            :class="autoBackupEnabled ? 'translate-x-5' : ''"
          />
        </button>
      </div>

      <!-- 备份间隔 -->
      <div v-if="autoBackupEnabled" class="flex items-center justify-between">
        <label class="text-sm text-content-secondary">{{ t('settings.autoBackupInterval') }}</label>
        <AppSelect
          v-model="autoBackupInterval"
          :options="backupIntervalOptions"
          size="sm"
          @change="handleIntervalChange"
        />
      </div>

      <!-- 保留份数 -->
      <div v-if="autoBackupEnabled" class="flex items-center justify-between">
        <label class="text-sm text-content-secondary">{{
          t('settings.autoBackupRetention')
        }}</label>
        <AppSelect
          v-model="autoBackupRetention"
          :options="backupRetentionOptions"
          size="sm"
          @change="emit('save')"
        />
      </div>

      <!-- 立即备份 + 上次备份 -->
      <div class="flex items-center justify-between">
        <span class="text-xs text-content-tertiary">
          {{ t('settings.lastBackup') }}:
          {{ lastBackupName || t('settings.never') }}
        </span>
        <button
          @click="doBackupNow"
          :disabled="backingUp"
          class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50"
        >
          {{ backingUp ? '...' : t('settings.backupNow') }}
        </button>
      </div>
    </div>
  </div>
</template>
