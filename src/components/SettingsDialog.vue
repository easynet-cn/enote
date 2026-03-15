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

      <!-- 安全设置 -->
      <div>
        <h3 class="text-sm font-semibold text-content-secondary mb-3">
          {{ t('settings.security') }}
        </h3>
        <div class="space-y-4">
          <!-- 锁屏方式 -->
          <div class="flex items-center justify-between">
            <label class="text-sm text-content-secondary">{{ t('settings.lockMode') }}</label>
            <div class="flex gap-1 bg-surface-dim rounded-lg p-1">
              <button
                v-for="option in lockModeOptions"
                :key="option.value"
                @click="setLockMode(option.value)"
                class="px-3 py-1.5 text-sm rounded-md transition-colors"
                :class="
                  currentLockMode === option.value
                    ? 'bg-surface text-indigo-600 shadow-sm'
                    : 'text-content-secondary hover:text-content'
                "
              >
                {{ option.label }}
              </button>
            </div>
          </div>

          <!-- 设置/修改密码 -->
          <div v-if="currentLockMode === 'password'">
            <!-- 密码表单 -->
            <div v-if="showPasswordForm" class="space-y-3 p-3 bg-surface-dim rounded-lg">
              <div v-if="hasPassword">
                <label class="block text-xs text-content-tertiary mb-1">{{
                  t('settings.currentPassword')
                }}</label>
                <input
                  v-model="passwordForm.current"
                  type="password"
                  class="w-full px-3 py-2 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
              <div>
                <label class="block text-xs text-content-tertiary mb-1">{{
                  t('settings.newPassword')
                }}</label>
                <input
                  v-model="passwordForm.newPwd"
                  type="password"
                  class="w-full px-3 py-2 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
              <div>
                <label class="block text-xs text-content-tertiary mb-1">{{
                  t('settings.confirmPassword')
                }}</label>
                <input
                  v-model="passwordForm.confirm"
                  type="password"
                  class="w-full px-3 py-2 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
              <p v-if="passwordError" class="text-xs text-red-500">{{ passwordError }}</p>
              <div class="flex justify-end gap-2">
                <button
                  @click="showPasswordForm = false"
                  class="px-3 py-1.5 text-sm text-content-secondary hover:text-content rounded-md"
                >
                  {{ t('common.cancel') }}
                </button>
                <button
                  @click="handleSavePassword"
                  :disabled="savingPassword"
                  class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50"
                >
                  {{ t('common.save') }}
                </button>
              </div>
            </div>

            <!-- 密码操作按钮 -->
            <div v-else class="flex items-center justify-between">
              <span class="text-sm text-content-secondary">
                {{ hasPassword ? t('settings.passwordSet') : '' }}
              </span>
              <div class="flex gap-2">
                <button
                  @click="showPasswordForm = true"
                  class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
                >
                  {{ hasPassword ? t('settings.changePassword') : t('settings.setPassword') }}
                </button>
                <button
                  v-if="hasPassword"
                  @click="handleRemovePassword"
                  class="px-3 py-1.5 text-sm text-red-600 hover:bg-red-50 rounded-lg transition-colors"
                >
                  {{ t('settings.removePassword') }}
                </button>
              </div>
            </div>
          </div>

          <!-- 自动锁定 -->
          <div v-if="currentLockMode !== 'none'" class="flex items-center justify-between">
            <label class="text-sm text-content-secondary">{{ t('settings.lockTimeout') }}</label>
            <select
              v-model="lockTimeoutValue"
              @change="saveSettings"
              class="px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
            >
              <option value="0">{{ t('settings.lockTimeoutNone') }}</option>
              <option value="5">{{ t('settings.lockTimeoutMinutes', { n: 5 }) }}</option>
              <option value="15">{{ t('settings.lockTimeoutMinutes', { n: 15 }) }}</option>
              <option value="30">{{ t('settings.lockTimeoutMinutes', { n: 30 }) }}</option>
            </select>
          </div>

          <!-- 最小化时锁定 -->
          <div v-if="currentLockMode !== 'none'" class="flex items-center justify-between">
            <label class="text-sm text-content-secondary">{{ t('settings.lockOnMinimize') }}</label>
            <button
              @click="toggleLockOnMinimize"
              class="relative w-10 h-5 rounded-full transition-colors"
              :class="lockOnMinimizeEnabled ? 'bg-indigo-600' : 'bg-slate-300'"
            >
              <span
                class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full transition-transform shadow-sm"
                :class="lockOnMinimizeEnabled ? 'translate-x-5' : ''"
              />
            </button>
          </div>
        </div>
      </div>

      <!-- MCP 设置 -->
      <div>
        <h3 class="text-sm font-semibold text-content-secondary mb-3">
          {{ t('settings.mcp') }}
        </h3>
        <div class="space-y-3">
          <!-- MCP 总开关 -->
          <div class="flex items-center justify-between">
            <div>
              <label class="text-sm text-content-secondary">{{ t('settings.mcpEnabled') }}</label>
              <p class="text-xs text-content-tertiary mt-0.5">{{ t('settings.mcpEnabledDesc') }}</p>
            </div>
            <button
              @click="toggleMcpEnabled"
              class="relative w-10 h-5 rounded-full transition-colors"
              :class="mcpEnabled ? 'bg-indigo-600' : 'bg-slate-300'"
            >
              <span
                class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full transition-transform shadow-sm"
                :class="mcpEnabled ? 'translate-x-5' : ''"
              />
            </button>
          </div>

          <!-- 各工具开关 -->
          <div v-if="mcpEnabled" class="space-y-2 pl-2 border-l-2 border-edge ml-1">
            <div
              v-for="tool in mcpTools"
              :key="tool.key"
              class="flex items-center justify-between py-1"
            >
              <label class="text-sm text-content-secondary">{{ tool.label }}</label>
              <button
                @click="toggleMcpTool(tool.key)"
                class="relative w-9 h-[18px] rounded-full transition-colors"
                :class="mcpToolEnabled[tool.key] ? 'bg-indigo-600' : 'bg-slate-300'"
              >
                <span
                  class="absolute top-0.5 left-0.5 w-3.5 h-3.5 bg-white rounded-full transition-transform shadow-sm"
                  :class="mcpToolEnabled[tool.key] ? 'translate-x-[18px]' : ''"
                />
              </button>
            </div>
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
import { settingsApi, backupApi, authApi } from '../api/note'
import { setLocale, type LocaleType } from '../i18n'
import { useAppStore } from '../stores/app'
import { useLockScreen } from '../composables/useLockScreen'
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

// 安全设置状态
const { lockMode, lockOnMinimize } = useLockScreen()
const currentLockMode = ref<'none' | 'password' | 'biometric'>('none')
const lockTimeoutValue = ref('0')
const lockOnMinimizeEnabled = ref(false)
const hasPassword = ref(false)
const showPasswordForm = ref(false)
const passwordError = ref('')
const savingPassword = ref(false)
const passwordForm = ref({ current: '', newPwd: '', confirm: '' })

// MCP 设置状态
const mcpEnabled = ref(false)
const mcpToolEnabled = ref<Record<string, boolean>>({
  search_notes: true,
  get_note: true,
  create_note: true,
  update_note: true,
  delete_note: true,
  list_notebooks: true,
  create_notebook: true,
  list_tags: true,
  create_tag: true,
  note_stats: true,
})

const mcpTools = computed(() => [
  { key: 'search_notes', label: t('settings.mcpToolSearch') },
  { key: 'get_note', label: t('settings.mcpToolGetNote') },
  { key: 'create_note', label: t('settings.mcpToolCreateNote') },
  { key: 'update_note', label: t('settings.mcpToolUpdateNote') },
  { key: 'delete_note', label: t('settings.mcpToolDeleteNote') },
  { key: 'list_notebooks', label: t('settings.mcpToolListNotebooks') },
  { key: 'create_notebook', label: t('settings.mcpToolCreateNotebook') },
  { key: 'list_tags', label: t('settings.mcpToolListTags') },
  { key: 'create_tag', label: t('settings.mcpToolCreateTag') },
  { key: 'note_stats', label: t('settings.mcpToolNoteStats') },
])

const toggleMcpEnabled = () => {
  mcpEnabled.value = !mcpEnabled.value
  saveSettings()
}

const toggleMcpTool = (key: string) => {
  mcpToolEnabled.value[key] = !mcpToolEnabled.value[key]
  saveSettings()
}

const lockModeOptions = computed(() => [
  { value: 'none' as const, label: t('settings.lockModeNone') },
  { value: 'password' as const, label: t('settings.lockModePassword') },
])

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

const setLockMode = async (mode: 'none' | 'password' | 'biometric') => {
  if (mode === 'password' && !hasPassword.value) {
    // Switch to password mode but need to set a password first
    currentLockMode.value = 'password'
    showPasswordForm.value = true
    // Save mode first so UI reflects, password required to actually lock
    lockMode.value = mode
    await saveSettings()
    return
  }
  if (mode === 'none' && hasPassword.value) {
    await authApi.clearLockPassword()
    hasPassword.value = false
  }
  currentLockMode.value = mode
  lockMode.value = mode
  showPasswordForm.value = false
  await saveSettings()
}

const handleSavePassword = async () => {
  passwordError.value = ''

  if (hasPassword.value && !passwordForm.value.current) {
    passwordError.value = t('settings.currentPassword')
    return
  }
  if (passwordForm.value.newPwd.length < 4) {
    passwordError.value = t('settings.passwordTooShort')
    return
  }
  if (passwordForm.value.newPwd !== passwordForm.value.confirm) {
    passwordError.value = t('settings.passwordMismatch')
    return
  }

  savingPassword.value = true
  try {
    // Verify current password if changing
    if (hasPassword.value) {
      const valid = await authApi.verifyLockPassword(passwordForm.value.current)
      if (!valid) {
        passwordError.value = t('settings.passwordIncorrect')
        savingPassword.value = false
        return
      }
    }

    await authApi.setLockPassword(passwordForm.value.newPwd)
    hasPassword.value = true
    showPasswordForm.value = false
    passwordForm.value = { current: '', newPwd: '', confirm: '' }
    showNotification({
      type: 'success',
      message: t(hasPassword.value ? 'settings.passwordChanged' : 'settings.passwordSet'),
    })
  } catch {
    showNotification({ type: 'error', message: t('settings.saveFailed') })
  } finally {
    savingPassword.value = false
  }
}

const handleRemovePassword = async () => {
  try {
    await authApi.clearLockPassword()
    hasPassword.value = false
    currentLockMode.value = 'none'
    lockMode.value = 'none'
    await saveSettings()
    showNotification({ type: 'success', message: t('settings.passwordRemoved') })
  } catch {
    showNotification({ type: 'error', message: t('settings.saveFailed') })
  }
}

const toggleLockOnMinimize = () => {
  lockOnMinimizeEnabled.value = !lockOnMinimizeEnabled.value
  lockOnMinimize.value = lockOnMinimizeEnabled.value
  saveSettings()
}

const saveSettings = async () => {
  try {
    // MCP 工具权限序列化为逗号分隔的已启用工具列表
    const enabledTools = Object.entries(mcpToolEnabled.value)
      .filter(([, v]) => v)
      .map(([k]) => k)
      .join(',')

    await settingsApi.save({
      theme: currentTheme.value,
      locale: locale.value,
      autoBackupEnabled: autoBackupEnabled.value ? '1' : '0',
      autoBackupInterval: autoBackupInterval.value,
      autoBackupRetention: autoBackupRetention.value,
      lockMode: currentLockMode.value,
      lockTimeout: lockTimeoutValue.value,
      lockOnMinimize: lockOnMinimizeEnabled.value ? '1' : '0',
      mcpEnabled: mcpEnabled.value ? '1' : '0',
      mcpEnabledTools: enabledTools,
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

    // 加载安全设置
    currentLockMode.value = (settings.lockMode as 'none' | 'password' | 'biometric') || 'none'
    lockTimeoutValue.value = settings.lockTimeout || '0'
    lockOnMinimizeEnabled.value = settings.lockOnMinimize === '1'
    hasPassword.value = !!settings.lockPasswordHash

    // 加载 MCP 设置
    mcpEnabled.value = settings.mcpEnabled === '1'
    if (settings.mcpEnabledTools !== undefined) {
      const enabledSet = new Set(settings.mcpEnabledTools.split(',').filter(Boolean))
      for (const key of Object.keys(mcpToolEnabled.value)) {
        mcpToolEnabled.value[key] = enabledSet.has(key)
      }
    }

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
