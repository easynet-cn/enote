<template>
  <Dialog v-model="visible" :title="t('settings.title')" :width="520">
    <div class="space-y-6">
      <!-- 外观设置 -->
      <SettingsAppearance
        ref="appearanceRef"
        v-model:theme="currentTheme"
        v-model:layout-mode="currentLayoutMode"
        v-model:editor-font-size="editorFontSize"
        @save="saveSettings"
      />

      <!-- 快捷键设置 -->
      <SettingsShortcuts />

      <!-- 自动备份设置 -->
      <SettingsBackup
        ref="backupRef"
        v-model:enabled="autoBackupEnabled"
        v-model:interval="autoBackupInterval"
        v-model:retention="autoBackupRetention"
        @save="saveSettings"
        @backup-settings-changed="emit('backupSettingsChanged')"
      />

      <!-- 配置管理 -->
      <div>
        <h3 class="text-sm font-semibold text-content-secondary mb-3">
          {{ t('profile.currentProfile') }}
        </h3>
        <div class="flex items-center justify-between p-3 bg-surface-dim rounded-lg">
          <span class="text-sm text-content-secondary">{{ t('profile.manageProfiles') }}</span>
          <button
            @click="switchProfile"
            class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-md hover:bg-indigo-700 transition-colors"
          >
            {{ t('profile.switchProfile') }}
          </button>
        </div>
      </div>

      <!-- 安全设置 -->
      <SettingsSecurity
        ref="securityRef"
        v-model:lock-mode="currentLockMode"
        v-model:lock-timeout="lockTimeoutValue"
        v-model:lock-on-minimize="lockOnMinimizeEnabled"
        @save="saveSettings"
      />

      <!-- MCP 设置 -->
      <SettingsMcp
        v-model:enabled="mcpEnabled"
        v-model:tool-enabled="mcpToolEnabled"
        @save="saveSettings"
      />

      <!-- 帮助手册 -->
      <div class="flex items-center justify-between p-3 bg-surface-dim rounded-lg">
        <div>
          <span class="text-sm text-content">{{ t('help.title') }}</span>
          <p class="text-xs text-content-tertiary mt-0.5">{{ t('help.description') }}</p>
        </div>
        <button
          @click="openHelpManual"
          class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-md hover:bg-indigo-700 transition-colors shrink-0"
        >
          {{ t('help.viewButton') }}
        </button>
      </div>

      <!-- 日志管理 -->
      <div>
        <h3 class="text-sm font-semibold text-content-secondary mb-3">
          {{ t('log.title') }}
        </h3>
        <div class="space-y-3">
          <!-- 前端日志级别 -->
          <div class="flex items-center justify-between">
            <label class="text-sm text-content-secondary">{{ t('log.frontendLogLevel') }}</label>
            <select
              v-model="frontendLogLevel"
              @change="saveSettings"
              class="px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
            >
              <option value="debug">DEBUG</option>
              <option value="info">INFO</option>
              <option value="warn">WARN</option>
              <option value="error">ERROR</option>
              <option value="none">{{ t('settings.lockModeNone') }}</option>
            </select>
          </div>

          <!-- 打开日志管理 -->
          <div class="flex items-center justify-between p-3 bg-surface-dim rounded-lg">
            <div>
              <span class="text-sm text-content">{{ t('log.title') }}</span>
              <p class="text-xs text-content-tertiary mt-0.5">{{ t('log.description') }}</p>
            </div>
            <button
              @click="logDialogVisible = true"
              class="px-3 py-1.5 text-sm text-indigo-600 hover:bg-indigo-100 rounded-md transition-colors shrink-0"
            >
              {{ t('log.openLogManager') }}
            </button>
          </div>
        </div>
      </div>

      <!-- 系统维护 -->
      <div>
        <h3 class="text-sm font-semibold text-content-secondary mb-3">
          {{ t('settings.maintenance') }}
        </h3>
        <div class="space-y-2">
          <!-- 跨 Profile 同步 -->
          <div class="flex items-center justify-between p-3 bg-surface-dim rounded-lg">
            <div>
              <span class="text-sm text-content">{{ t('sync.title') }}</span>
              <p class="text-xs text-content-tertiary mt-0.5">{{ t('settings.syncDesc') }}</p>
            </div>
            <button
              @click="syncDialogVisible = true"
              class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-md hover:bg-indigo-700 transition-colors shrink-0"
            >
              {{ t('sync.startSync') }}
            </button>
          </div>
          <!-- 同步记录 -->
          <div class="flex items-center justify-between p-3 bg-surface-dim rounded-lg">
            <div>
              <span class="text-sm text-content">{{ t('sync.history') }}</span>
              <p class="text-xs text-content-tertiary mt-0.5">
                {{ t('settings.syncHistoryDesc') }}
              </p>
            </div>
            <button
              @click="syncHistoryDialogVisible = true"
              class="px-3 py-1.5 text-sm text-indigo-600 hover:bg-indigo-100 rounded-md transition-colors shrink-0"
            >
              {{ t('sync.viewDetails') }}
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

  <!-- 跨 Profile 同步对话框 -->
  <SyncDialog v-model="syncDialogVisible" />

  <!-- 同步记录管理对话框 -->
  <SyncHistoryDialog v-model="syncHistoryDialogVisible" />

  <!-- 日志管理对话框 -->
  <LogDialog v-model="logDialogVisible" />
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { Dialog, Button } from './ui'
import SyncDialog from './SyncDialog.vue'
import SyncHistoryDialog from './SyncHistoryDialog.vue'
import LogDialog from './LogDialog.vue'
import SettingsAppearance from './settings/SettingsAppearance.vue'
import SettingsShortcuts from './settings/SettingsShortcuts.vue'
import SettingsBackup from './settings/SettingsBackup.vue'
import SettingsSecurity from './settings/SettingsSecurity.vue'
import SettingsMcp from './settings/SettingsMcp.vue'
import { settingsApi } from '../api/note'
import { initLogger, setLogLevel } from '../utils/logger'
import { setLocale, type LocaleType } from '../i18n'
import { useAppStore } from '../stores/app'
import { usePlatform, type LayoutMode } from '../composables/usePlatform'
import { showNotification } from './ui/notification'

const { t, locale } = useI18n()
const appStore = useAppStore()
const { setLayoutOverride } = usePlatform()

const emit = defineEmits<{
  (e: 'switchProfile'): void
  (e: 'backupSettingsChanged'): void
}>()

const visible = defineModel<boolean>({ default: false })

// 子组件 refs
const appearanceRef = ref<InstanceType<typeof SettingsAppearance>>()
const backupRef = ref<InstanceType<typeof SettingsBackup>>()
const securityRef = ref<InstanceType<typeof SettingsSecurity>>()

// ============================================================================
// 全局设置状态（由子组件通过 v-model 双向绑定）
// ============================================================================

const currentTheme = ref('light')
const currentLayoutMode = ref<LayoutMode | 'auto'>('auto')
const editorFontSize = ref('14')

// 自动备份
const autoBackupEnabled = ref(false)
const autoBackupInterval = ref('24')
const autoBackupRetention = ref('10')

// 安全
const currentLockMode = ref<'none' | 'password' | 'biometric'>('none')
const lockTimeoutValue = ref('0')
const lockOnMinimizeEnabled = ref(false)

// MCP
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

// 日志
const frontendLogLevel = ref('info')

// 子对话框
const syncDialogVisible = ref(false)
const syncHistoryDialogVisible = ref(false)
const logDialogVisible = ref(false)

// ============================================================================
// 统一保存（所有子组件 @save 事件的处理器）
// ============================================================================

const saveSettings = async () => {
  try {
    const enabledTools = Object.entries(mcpToolEnabled.value)
      .filter(([, v]) => v)
      .map(([k]) => k)
      .join(',')

    await settingsApi.save({
      theme: currentTheme.value,
      locale: locale.value,
      layoutMode: currentLayoutMode.value,
      editorFontSize: editorFontSize.value,
      autoBackupEnabled: autoBackupEnabled.value ? '1' : '0',
      autoBackupInterval: autoBackupInterval.value,
      autoBackupRetention: autoBackupRetention.value,
      lockMode: currentLockMode.value,
      lockTimeout: lockTimeoutValue.value,
      lockOnMinimize: lockOnMinimizeEnabled.value ? '1' : '0',
      mcpEnabled: mcpEnabled.value ? '1' : '0',
      mcpEnabledTools: enabledTools,
      frontendLogLevel: frontendLogLevel.value,
    })
    setLogLevel(frontendLogLevel.value)
  } catch {
    showNotification({ type: 'error', message: t('settings.saveFailed') })
  }
}

// ============================================================================
// 操作处理
// ============================================================================

const switchProfile = () => {
  visible.value = false
  emit('switchProfile')
}

const openHelpManual = async () => {
  visible.value = false
  const { openHelpInNewWindow } = await import('../utils/multiWindow')
  await openHelpInNewWindow()
}

// ============================================================================
// 初始化加载
// ============================================================================

const loadSettings = async () => {
  try {
    const settings = await settingsApi.getAll()
    if (settings.theme) {
      currentTheme.value = settings.theme
      appearanceRef.value?.applyTheme(settings.theme)
    }
    if (settings.locale) {
      setLocale(settings.locale as LocaleType)
      appStore.updateDefaultItems()
    }
    if (settings.layoutMode) {
      const mode = settings.layoutMode as LayoutMode | 'auto'
      currentLayoutMode.value = mode
      setLayoutOverride(mode)
    }
    if (settings.editorFontSize) {
      editorFontSize.value = settings.editorFontSize
      document.documentElement.style.setProperty(
        '--editor-font-size',
        `${settings.editorFontSize}px`,
      )
    }

    autoBackupEnabled.value = settings.autoBackupEnabled === '1'
    if (settings.autoBackupInterval) autoBackupInterval.value = settings.autoBackupInterval
    if (settings.autoBackupRetention) autoBackupRetention.value = settings.autoBackupRetention

    currentLockMode.value = (settings.lockMode as 'none' | 'password' | 'biometric') || 'none'
    lockTimeoutValue.value = settings.lockTimeout || '0'
    lockOnMinimizeEnabled.value = settings.lockOnMinimize === '1'
    securityRef.value?.initHasPassword(!!settings.lockPasswordHash)

    if (settings.frontendLogLevel) {
      frontendLogLevel.value = settings.frontendLogLevel
      setLogLevel(settings.frontendLogLevel)
    }
    initLogger(frontendLogLevel.value)

    mcpEnabled.value = settings.mcpEnabled === '1'
    if (settings.mcpEnabledTools !== undefined) {
      const enabledSet = new Set(settings.mcpEnabledTools.split(',').filter(Boolean))
      for (const key of Object.keys(mcpToolEnabled.value)) {
        mcpToolEnabled.value[key] = enabledSet.has(key)
      }
    }

    backupRef.value?.loadLastBackup()
  } catch {
    // 使用默认设置
  }
}

onMounted(loadSettings)
</script>
