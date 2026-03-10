<template>
  <Dialog v-model="visible" :title="t('settings.title')" :width="520">
    <div class="space-y-6">
      <!-- 外观设置 -->
      <div>
        <h3 class="text-sm font-semibold text-content-secondary mb-3">
          {{ t('settings.appearance') }}
        </h3>
        <div class="space-y-4">
          <!-- 主题 -->
          <div class="flex items-center justify-between">
            <label class="text-sm text-content-secondary">{{ t('settings.theme') }}</label>
            <div class="flex gap-1 bg-surface-dim rounded-lg p-1">
              <button
                v-for="option in themeOptions"
                :key="option.value"
                @click="setTheme(option.value)"
                class="px-3 py-1.5 text-sm rounded-md transition-colors"
                :class="
                  currentTheme === option.value
                    ? 'bg-surface text-indigo-600 shadow-sm'
                    : 'text-content-secondary hover:text-content'
                "
              >
                {{ option.label }}
              </button>
            </div>
          </div>

          <!-- 语言 -->
          <div class="flex items-center justify-between">
            <label class="text-sm text-content-secondary">{{ t('settings.language') }}</label>
            <div class="flex gap-1 bg-surface-dim rounded-lg p-1">
              <button
                @click="switchLocale('zh-CN')"
                class="px-3 py-1.5 text-sm rounded-md transition-colors"
                :class="
                  currentLocale === 'zh-CN'
                    ? 'bg-surface text-indigo-600 shadow-sm'
                    : 'text-content-secondary hover:text-content'
                "
              >
                {{ t('settings.languageZh') }}
              </button>
              <button
                @click="switchLocale('en-US')"
                class="px-3 py-1.5 text-sm rounded-md transition-colors"
                :class="
                  currentLocale === 'en-US'
                    ? 'bg-surface text-indigo-600 shadow-sm'
                    : 'text-content-secondary hover:text-content'
                "
              >
                {{ t('settings.languageEn') }}
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 自动备份设置 -->
      <div>
        <h3 class="text-sm font-semibold text-content-secondary mb-3">
          {{ t('settings.autoBackup') }}
        </h3>
        <div class="space-y-4">
          <!-- 启用自动备份 -->
          <div class="flex items-center justify-between">
            <label class="text-sm text-content-secondary">{{
              t('settings.autoBackupEnabled')
            }}</label>
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
            <label class="text-sm text-content-secondary">{{
              t('settings.autoBackupInterval')
            }}</label>
            <select
              v-model="autoBackupInterval"
              @change="saveSettings"
              class="px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
            >
              <option value="1">
                1 {{ t('settings.autoBackupIntervalHours', { n: '' }).trim() }}
              </option>
              <option value="4">
                4 {{ t('settings.autoBackupIntervalHours', { n: '' }).trim() }}
              </option>
              <option value="8">
                8 {{ t('settings.autoBackupIntervalHours', { n: '' }).trim() }}
              </option>
              <option value="24">
                24 {{ t('settings.autoBackupIntervalHours', { n: '' }).trim() }}
              </option>
            </select>
          </div>

          <!-- 保留份数 -->
          <div v-if="autoBackupEnabled" class="flex items-center justify-between">
            <label class="text-sm text-content-secondary">{{
              t('settings.autoBackupRetention')
            }}</label>
            <select
              v-model="autoBackupRetention"
              @change="saveSettings"
              class="px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
            >
              <option value="5">{{ t('settings.autoBackupRetentionCount', { n: 5 }) }}</option>
              <option value="10">{{ t('settings.autoBackupRetentionCount', { n: 10 }) }}</option>
              <option value="20">{{ t('settings.autoBackupRetentionCount', { n: 20 }) }}</option>
              <option value="50">{{ t('settings.autoBackupRetentionCount', { n: 50 }) }}</option>
            </select>
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
    </div>

    <template #footer>
      <div class="flex justify-end gap-3">
        <Button type="secondary" @click="visible = false">{{ t('common.close') }}</Button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { Dialog, Button } from './ui'
import { settingsApi, backupApi } from '../api/note'
import { setLocale, type LocaleType } from '../i18n'
import { useAppStore } from '../stores/app'
import { showNotification } from './ui/notification'

const { t, locale } = useI18n()
const appStore = useAppStore()

const visible = defineModel<boolean>({ default: false })

const currentTheme = ref<string>('light')
const currentLocale = computed(() => locale.value as LocaleType)

// 自动备份状态
const autoBackupEnabled = ref(false)
const autoBackupInterval = ref('24')
const autoBackupRetention = ref('10')
const lastBackupName = ref('')
const backingUp = ref(false)

const themeOptions = computed(() => [
  { value: 'light', label: t('settings.themeLight') },
  { value: 'dark', label: t('settings.themeDark') },
  { value: 'system', label: t('settings.themeSystem') },
])

const applyTheme = (theme: string) => {
  const root = document.documentElement
  if (theme === 'system') {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    root.setAttribute('data-theme', prefersDark ? 'dark' : 'light')
  } else {
    root.setAttribute('data-theme', theme)
  }
}

const setTheme = async (theme: string) => {
  currentTheme.value = theme
  applyTheme(theme)
  await saveSettings()
}

const switchLocale = async (newLocale: LocaleType) => {
  setLocale(newLocale)
  appStore.updateDefaultItems()
  await saveSettings()
}

const toggleAutoBackup = () => {
  autoBackupEnabled.value = !autoBackupEnabled.value
  saveSettings()
}

const doBackupNow = async () => {
  backingUp.value = true
  try {
    const filename = await backupApi.autoBackup()
    lastBackupName.value = filename
    // 清理旧备份
    await backupApi.cleanupOldBackups(parseInt(autoBackupRetention.value))
    showNotification({ type: 'success', message: t('settings.backupSuccess', { name: filename }) })
  } catch {
    showNotification({ type: 'error', message: t('settings.backupFailed') })
  } finally {
    backingUp.value = false
  }
}

const saveSettings = async () => {
  try {
    await settingsApi.save({
      theme: currentTheme.value,
      locale: locale.value,
      autoBackupEnabled: autoBackupEnabled.value ? '1' : '0',
      autoBackupInterval: autoBackupInterval.value,
      autoBackupRetention: autoBackupRetention.value,
    })
  } catch {
    showNotification({ type: 'error', message: t('settings.saveFailed') })
  }
}

const loadSettings = async () => {
  try {
    const settings = await settingsApi.getAll()
    if (settings.theme) {
      currentTheme.value = settings.theme
      applyTheme(settings.theme)
    }
    if (settings.locale) {
      setLocale(settings.locale as LocaleType)
      appStore.updateDefaultItems()
    }
    autoBackupEnabled.value = settings.autoBackupEnabled === '1'
    if (settings.autoBackupInterval) autoBackupInterval.value = settings.autoBackupInterval
    if (settings.autoBackupRetention) autoBackupRetention.value = settings.autoBackupRetention

    // 加载最近备份文件名
    try {
      const backups = await backupApi.listAutoBackups()
      if (backups.length > 0) {
        lastBackupName.value = backups[0][0]
      }
    } catch {
      // 忽略
    }
  } catch {
    // 使用默认设置
  }
}

// 监听系统主题变化
const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
mediaQuery.addEventListener('change', () => {
  if (currentTheme.value === 'system') {
    applyTheme('system')
  }
})

onMounted(loadSettings)
</script>
