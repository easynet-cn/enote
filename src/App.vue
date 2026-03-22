<template>
  <!-- 加载中 -->
  <Transition name="fade">
    <div v-if="appMode === 'loading'" class="loading-screen">
      <NotebookPen class="loading-icon" :stroke-width="1.5" />
      <div class="loading-spinner" />
      <span class="loading-text">{{ t('common.loading') }}</span>
    </div>
  </Transition>

  <!-- 设置向导模式 -->
  <SetupWizard
    v-if="appMode === 'setup'"
    :show-back="hasExistingProfiles"
    @complete="onSetupComplete"
    @back="appMode = 'select'"
  />

  <!-- Profile 选择模式 -->
  <ProfileSelector
    v-else-if="appMode === 'select'"
    :show-close="canCloseSelector"
    @create="appMode = 'setup'"
    @edit="onEditProfile"
    @connected="onSetupComplete"
    @close="returnToMain"
  />

  <!-- 主应用 -->
  <div v-else class="flex h-screen bg-surface-alt relative overflow-hidden">
    <!-- 侧边栏遮罩（手机/平板覆盖层模式） -->
    <Transition name="sidebar-overlay">
      <div v-if="sidebarOverlayVisible" class="sidebar-overlay" @click="closeSidebar" />
    </Transition>

    <!-- 侧边栏组件 -->
    <div :class="sidebarContainerClass">
      <AppSidebar
        :mobile="isMobileLayout"
        :overlay="!isDesktopLayout"
        :notebooks="notebooks"
        :tags="tags"
        :active-notebook="activeNotebook"
        :active-tag="activeTag"
        :collapsed="isDesktopLayout ? sidebarCollapsed : false"
        @set-active-notebook="handleSelectNotebook"
        @set-active-tag="handleSelectTag"
        @create-new-note="handleCreateNote"
        @save-notebook="saveNotebook"
        @delete-notebook="deleteNotebook"
        @save-tag="saveTag"
        @delete-tag="deleteTag"
        @toggle-collapse="sidebarCollapsed = !sidebarCollapsed"
        @open-import="importDialogVisible = true"
        @open-backup="backupDialogVisible = true"
        @open-settings="settingsDialogVisible = true"
        @open-trash="trashDialogVisible = true"
        @reorder-notebooks="handleReorderNotebooks"
        @reorder-tags="handleReorderTags"
        @open-templates="templateDialogVisible = true"
        @close-overlay="closeSidebar"
      />
    </div>

    <!-- 笔记列表组件 -->
    <div :class="noteListContainerClass">
      <NoteList
        :notebooks="notebooks"
        :notes="notes"
        :active-notebook="activeNotebook"
        :active-note="activeNote"
        :collapsed="isDesktopLayout ? noteListCollapsed : false"
        :mobile="isMobileLayout"
        :layout="layout"
        v-model:current-page="notePageIndex"
        v-model:page-size="notePageSize"
        v-model:total="noteTotal"
        v-model:query="query"
        v-model:width="noteListWidth"
        @set-active-note="handleSelectNote"
        @update-search-query="handleUpdateSearchQuery"
        @size-change="handleNoteSizeChange"
        @current-change="handleNoteCurrentChange"
        @toggle-collapse="handleNoteListToggle"
        @toggle-pin="handleTogglePin"
        @open-sidebar="openSidebar"
      />
    </div>

    <!-- 编辑器组件 -->
    <div :class="editorContainerClass">
      <NoteEditor
        v-model:history-data="histories"
        v-model:current-page="historyPageIndex"
        v-model:page-size="historyPageSize"
        v-model:total="historyTotal"
        :notebooks="notebooks"
        :tags="tags"
        :active-note="activeNoteData"
        :edit-mode="editMode"
        :history-loading="historyLoading"
        :mobile="isMobileLayout"
        :layout="layout"
        @save-note="saveNote"
        @cancel-edit="cancelEdit"
        @delete-note="deleteNote"
        @toggle-edit-mode="editMode = !editMode"
        @update-note-title="updateNoteTitle"
        @update-note-content="updateNoteContent"
        @update-note-content-type="updateNoteContentType"
        @update-note-setting="
          (notebookId, tagIds, mcpAccess) => updateNoteSetting(notebookId, tagIds, tags, mcpAccess)
        "
        @open="openHistoryDialog"
        @size-change="handleNoteHistorySizeChange"
        @current-change="handleNoteHistoryCurrentChange"
        @save-as-template="handleSaveAsTemplate"
        @back="handleEditorBack"
      />
    </div>

    <!-- 导入对话框 -->
    <ImportDialog
      v-model="importDialogVisible"
      :notebooks="notebooks"
      :tags="tags"
      @imported="refreshAllData"
    />

    <!-- 数据备份对话框 -->
    <BackupDialog v-model="backupDialogVisible" @imported="refreshAllData" />

    <!-- 设置对话框 -->
    <SettingsDialog v-model="settingsDialogVisible" @switch-profile="switchToProfileSelector" />

    <!-- 回收站对话框 -->
    <TrashDialog v-model="trashDialogVisible" @restored="refreshAllData" />

    <!-- 模板对话框 -->
    <TemplateDialog v-model="templateDialogVisible" @use-template="handleUseTemplate" />

    <!-- 快捷命令面板 -->
    <CommandPalette v-model="commandPaletteVisible" :commands="paletteCommands" />

    <!-- 锁屏 -->
    <LockScreen :visible="isLocked" @unlocked="unlock" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, markRaw } from 'vue'
import { useI18n } from 'vue-i18n'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { ask } from '@tauri-apps/plugin-dialog'
import { noteApi, settingsApi, backupApi, templateApi, profileApi } from './api/note'
import { noteToShowNote } from './utils/converters'
import { useNotes } from './composables/useNotes'
import { useKeyboardShortcuts } from './composables/useKeyboardShortcuts'
import { useShortcutSettings } from './composables/useShortcutSettings'
import { useLockScreen } from './composables/useLockScreen'
import { useAppStore } from './stores/app'
import AppSidebar from './components/AppSidebar.vue'
import NoteList from './components/NoteList.vue'
import NoteEditor from './components/NoteEditor.vue'
import ImportDialog from './components/ImportDialog.vue'
import BackupDialog from './components/BackupDialog.vue'
import SettingsDialog from './components/SettingsDialog.vue'
import TrashDialog from './components/TrashDialog.vue'
import CommandPalette from './components/CommandPalette.vue'
import TemplateDialog from './components/TemplateDialog.vue'
import SetupWizard from './components/SetupWizard.vue'
import ProfileSelector from './components/ProfileSelector.vue'
import { showNotification } from './components/ui/notification'
import { parseError } from './utils/errorHandler'
import { usePlatform } from './composables/usePlatform'
import LockScreen from './components/LockScreen.vue'
import type { PaletteCommand } from './components/CommandPalette.vue'
import {
  Plus,
  Save,
  Pencil,
  Trash2,
  Moon,
  Sun,
  PanelLeft,
  Settings,
  Database,
  LayoutTemplate,
  FileDown,
  Lock,
  Monitor,
  NotebookPen,
} from 'lucide-vue-next'

const { t } = useI18n()
const appStore = useAppStore()
const {
  isMobileLayout,
  isTabletLayout,
  isDesktopLayout,
  layout,
  isTauriMobile,
  layoutOverride,
  setLayoutOverride,
} = usePlatform()

// 移动端导航状态
const mobileView = ref<'sidebar' | 'list' | 'editor'>('list')

// 侧边栏覆盖层（手机/平板模式）
const sidebarVisible = ref(false)
const sidebarOverlayVisible = computed(() => !isDesktopLayout.value && sidebarVisible.value)

const openSidebar = () => {
  if (isDesktopLayout.value) return
  sidebarVisible.value = true
}

const closeSidebar = () => {
  sidebarVisible.value = false
}

// 布局容器样式计算
const sidebarContainerClass = computed(() => {
  if (isDesktopLayout.value) {
    return 'sidebar-container-desktop'
  }
  return [
    'sidebar-container-overlay',
    sidebarVisible.value ? 'sidebar-overlay-visible' : 'sidebar-overlay-hidden',
  ]
})

const noteListContainerClass = computed(() => {
  if (isDesktopLayout.value) {
    return 'notelist-container-desktop'
  }
  if (isTabletLayout.value) {
    return 'notelist-container-tablet'
  }
  // 手机模式
  return [
    'notelist-container-mobile',
    mobileView.value === 'list' ? 'mobile-view-active' : 'mobile-view-hidden-left',
  ]
})

const editorContainerClass = computed(() => {
  if (isDesktopLayout.value || isTabletLayout.value) {
    return 'editor-container-flex'
  }
  // 手机模式
  return [
    'editor-container-mobile',
    mobileView.value === 'editor' ? 'mobile-view-active' : 'mobile-view-hidden-right',
  ]
})

// 应用模式：'loading' | 'setup' | 'select' | 'main'
const appMode = ref<'loading' | 'setup' | 'select' | 'main'>('loading')
const hasExistingProfiles = ref(false)

const onSetupComplete = async () => {
  // 热重连后进入主界面（不再依赖进程重启）
  await enterMainMode()
}

// 是否可以关闭 ProfileSelector 返回主界面（从设置进入时可以，首次启动时不行）
const canCloseSelector = ref(false)

const switchToProfileSelector = () => {
  hasExistingProfiles.value = true
  canCloseSelector.value = true
  appMode.value = 'select'
}

const returnToMain = async () => {
  await enterMainMode()
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const onEditProfile = async (_profileId: string) => {
  // TODO: 编辑已有 profile
  appMode.value = 'setup'
}

// 锁屏
const { isLocked, lock, unlock, checkStartupLock, setupMinimizeListener } = useLockScreen()

// 折叠状态
const sidebarCollapsed = ref(false)
const noteListCollapsed = ref(false)

// 导入对话框
const importDialogVisible = ref(false)
// 备份对话框
const backupDialogVisible = ref(false)
// 设置对话框
const settingsDialogVisible = ref(false)
// 回收站对话框
const trashDialogVisible = ref(false)
// 模板对话框
const templateDialogVisible = ref(false)
// 命令面板
const commandPaletteVisible = ref(false)
// NoteList宽度
const noteListWidth = ref(320)
const DEFAULT_NOTE_LIST_WIDTH = 320

const handleNoteListToggle = () => {
  noteListCollapsed.value = !noteListCollapsed.value
  noteListWidth.value = DEFAULT_NOTE_LIST_WIDTH
}

const {
  // 数据状态
  notebooks,
  notes,
  tags,
  query,
  histories,
  // UI 状态 refs
  activeNotebook,
  activeTag,
  activeNote,
  editMode,
  notePageIndex,
  notePageSize,
  noteTotal,
  historyPageIndex,
  historyPageSize,
  historyTotal,
  historyLoading,
  // 笔记数据
  activeNoteData,
  // 操作方法
  saveNotebook,
  deleteNotebook,
  saveTag,
  deleteTag,
  setActiveNotebook,
  setActiveTag,
  setActiveNote,
  createNewNote,
  saveNote,
  cancelEdit,
  deleteNote,
  updateNoteTitle,
  updateNoteContent,
  updateNoteContentType,
  updateNoteSetting,
  handleUpdateSearchQuery,
  handleNoteSizeChange,
  handleNoteCurrentChange,
  openHistoryDialog,
  handleNoteHistorySizeChange,
  handleNoteHistoryCurrentChange,
  refreshAllData,
  initialize: initializeNotes,
} = useNotes()

// 拖拽排序
const handleReorderNotebooks = async (orders: [string, number][]) => {
  try {
    await noteApi.reorderNotebooks(orders.map(([id, order]) => [Number(id), order]))
    await refreshAllData()
  } catch (e: unknown) {
    showNotification({ type: 'error', message: parseError(e) })
  }
}

const handleReorderTags = async (orders: [string, number][]) => {
  try {
    await noteApi.reorderTags(orders.map(([id, order]) => [Number(id), order]))
    await refreshAllData()
  } catch (e: unknown) {
    showNotification({ type: 'error', message: parseError(e) })
  }
}

// 切换笔记置顶
const handleTogglePin = async (noteId: string) => {
  try {
    const updatedNote = await noteApi.toggleNotePin(Number(noteId))
    if (updatedNote) {
      appStore.updateNote(noteToShowNote(updatedNote))
      // 刷新列表以更新排序
      await refreshAllData()
    }
  } catch (e: unknown) {
    showNotification({ type: 'error', message: parseError(e) })
  }
}

// ============================================================================
// 响应式导航处理
// ============================================================================

const handleSelectNotebook = (id: string) => {
  setActiveNotebook(id)
  closeSidebar()
  if (isMobileLayout.value) mobileView.value = 'list'
}

const handleSelectTag = (id: string) => {
  setActiveTag(id)
  closeSidebar()
  if (isMobileLayout.value) mobileView.value = 'list'
}

const handleSelectNote = (id: string) => {
  setActiveNote(id)
  if (isMobileLayout.value) mobileView.value = 'editor'
}

const handleCreateNote = () => {
  createNewNote()
  closeSidebar()
  if (isMobileLayout.value) mobileView.value = 'editor'
}

const handleEditorBack = () => {
  if (isMobileLayout.value) mobileView.value = 'list'
}

// 使用模板创建新笔记
const handleUseTemplate = (template: import('./types').NoteTemplate) => {
  templateDialogVisible.value = false
  createNewNote(template.contentType)
  // 将模板内容填入新笔记
  if (template.content) {
    updateNoteContent(template.content)
  }
  if (template.name) {
    updateNoteTitle(template.name)
  }
}

// 将当前笔记存为模板
const handleSaveAsTemplate = async () => {
  if (!activeNoteData.value) return
  try {
    await templateApi.create({
      id: 0,
      name: activeNoteData.value.title || t('template.name'),
      content: activeNoteData.value.content || '',
      contentType: activeNoteData.value.contentType ?? 0,
      sortOrder: 0,
      createTime: null,
      updateTime: null,
    })
    showNotification({ type: 'success', message: t('template.saveAsTemplateSuccess') })
  } catch (e: unknown) {
    showNotification({ type: 'error', message: parseError(e) })
  }
}

// ============================================================================
// 快捷键配置驱动
// ============================================================================

const { getBindingText, buildHandlers, load: loadShortcuts } = useShortcutSettings()

// 快捷键处理函数映射（id → handler）
const switchLayoutHandler = () => {
  const modes: Array<'auto' | 'desktop' | 'tablet' | 'mobile'> = [
    'auto',
    'desktop',
    'tablet',
    'mobile',
  ]
  const currentIndex = modes.indexOf(layoutOverride.value as (typeof modes)[number])
  const nextIndex = (currentIndex + 1) % modes.length
  setLayoutOverride(modes[nextIndex])
}

const shortcutHandlerMap: Record<string, () => void> = {
  'save-note': () => {
    if (editMode.value && activeNote.value) saveNote()
  },
  'new-note': () => createNewNote(),
  'edit-note': () => {
    if (activeNote.value && !editMode.value) editMode.value = true
  },
  'cancel-edit': () => {
    if (editMode.value) cancelEdit()
  },
  'toggle-sidebar': () => {
    sidebarCollapsed.value = !sidebarCollapsed.value
  },
  'lock-app': () => lock(),
  'command-palette': () => {
    commandPaletteVisible.value = !commandPaletteVisible.value
  },
  'switch-layout': switchLayoutHandler,
}

// 响应式快捷键（当用户修改绑定后自动更新）
const shortcutHandlers = computed(() => buildHandlers(shortcutHandlerMap))
useKeyboardShortcuts(shortcutHandlers)

// 命令面板命令列表（快捷键显示从配置读取）
const paletteCommands = computed<PaletteCommand[]>(() => [
  {
    id: 'new-note',
    name: t('commandPalette.commands.newNote'),
    icon: markRaw(Plus),
    category: t('commandPalette.categories.notes'),
    shortcut: getBindingText('new-note'),
    handler: () => createNewNote(),
  },
  {
    id: 'save-note',
    name: t('commandPalette.commands.saveNote'),
    icon: markRaw(Save),
    category: t('commandPalette.categories.notes'),
    shortcut: getBindingText('save-note'),
    handler: () => {
      if (editMode.value && activeNote.value) saveNote()
    },
  },
  {
    id: 'edit-note',
    name: t('commandPalette.commands.editNote'),
    icon: markRaw(Pencil),
    category: t('commandPalette.categories.notes'),
    shortcut: getBindingText('edit-note'),
    handler: () => {
      if (activeNote.value && !editMode.value) editMode.value = true
    },
  },
  {
    id: 'delete-note',
    name: t('commandPalette.commands.deleteNote'),
    icon: markRaw(Trash2),
    category: t('commandPalette.categories.notes'),
    handler: () => {
      if (activeNote.value) deleteNote()
    },
  },
  {
    id: 'toggle-sidebar',
    name: t('commandPalette.commands.toggleSidebar'),
    icon: markRaw(PanelLeft),
    category: t('commandPalette.categories.view'),
    shortcut: getBindingText('toggle-sidebar'),
    handler: () => {
      sidebarCollapsed.value = !sidebarCollapsed.value
    },
  },
  {
    id: 'toggle-dark-mode',
    name: t('commandPalette.commands.toggleDarkMode'),
    icon: markRaw(document.documentElement.getAttribute('data-theme') === 'dark' ? Sun : Moon),
    category: t('commandPalette.categories.view'),
    handler: () => {
      const current = document.documentElement.getAttribute('data-theme')
      const next = current === 'dark' ? 'light' : 'dark'
      document.documentElement.setAttribute('data-theme', next)
    },
  },
  {
    id: 'open-settings',
    name: t('commandPalette.commands.openSettings'),
    icon: markRaw(Settings),
    category: t('commandPalette.categories.app'),
    handler: () => {
      settingsDialogVisible.value = true
    },
  },
  {
    id: 'open-trash',
    name: t('commandPalette.commands.openTrash'),
    icon: markRaw(Trash2),
    category: t('commandPalette.categories.app'),
    handler: () => {
      trashDialogVisible.value = true
    },
  },
  {
    id: 'open-backup',
    name: t('commandPalette.commands.openBackup'),
    icon: markRaw(Database),
    category: t('commandPalette.categories.app'),
    handler: () => {
      backupDialogVisible.value = true
    },
  },
  {
    id: 'open-templates',
    name: t('commandPalette.commands.openTemplates'),
    icon: markRaw(LayoutTemplate),
    category: t('commandPalette.categories.app'),
    handler: () => {
      templateDialogVisible.value = true
    },
  },
  {
    id: 'new-from-template',
    name: t('commandPalette.commands.newFromTemplate'),
    icon: markRaw(LayoutTemplate),
    category: t('commandPalette.categories.notes'),
    handler: () => {
      templateDialogVisible.value = true
    },
  },
  {
    id: 'lock-app',
    name: t('shortcuts.lockApp'),
    icon: markRaw(Lock),
    category: t('commandPalette.categories.app'),
    shortcut: getBindingText('lock-app'),
    handler: () => lock(),
  },
  {
    id: 'save-as-template',
    name: t('commandPalette.commands.saveAsTemplate'),
    icon: markRaw(FileDown),
    category: t('commandPalette.categories.notes'),
    handler: () => {
      if (activeNoteData.value) handleSaveAsTemplate()
    },
  },
  {
    id: 'switch-layout',
    name: t('shortcuts.switchLayout'),
    icon: markRaw(Monitor),
    category: t('commandPalette.categories.view'),
    shortcut: getBindingText('switch-layout'),
    handler: switchLayoutHandler,
  },
])

// 关闭窗口拦截：未保存时提示确认
// 自动备份定时器
let autoBackupTimer: ReturnType<typeof setTimeout> | ReturnType<typeof setInterval> | null = null
let autoBackupErrorNotified = false

const startAutoBackup = async () => {
  try {
    const settings = await settingsApi.getAll()
    if (settings.autoBackupEnabled !== '1') return

    const intervalHours = parseInt(settings.autoBackupInterval || '24')
    const retention = parseInt(settings.autoBackupRetention || '10')

    // 检查是否需要立即备份（距上次备份超过间隔）
    const backups = await backupApi.listAutoBackups()
    if (backups.length > 0) {
      // 从文件名解析时间：enote_backup_20260310_143000.sql
      const match = backups[0][0].match(/enote_backup_(\d{8})_(\d{6})/)
      if (match) {
        const dateStr = `${match[1].slice(0, 4)}-${match[1].slice(4, 6)}-${match[1].slice(6, 8)}T${match[2].slice(0, 2)}:${match[2].slice(2, 4)}:${match[2].slice(4, 6)}`
        const lastTime = new Date(dateStr).getTime()
        const elapsed = Date.now() - lastTime
        if (elapsed < intervalHours * 3600 * 1000) {
          // 距上次备份未超过间隔，设置剩余时间后的定时器
          const remaining = intervalHours * 3600 * 1000 - elapsed
          autoBackupTimer = setTimeout(async () => {
            await doAutoBackup(retention)
            // 之后按间隔定时
            autoBackupTimer = setInterval(
              () => doAutoBackup(retention),
              intervalHours * 3600 * 1000,
            )
          }, remaining)
          return
        }
      }
    }

    // 需要立即备份
    await doAutoBackup(retention)
    // 设置定时
    autoBackupTimer = setInterval(() => doAutoBackup(retention), intervalHours * 3600 * 1000)
  } catch (e: unknown) {
    // 自动备份初始化失败，首次通知用户
    if (!autoBackupErrorNotified) {
      autoBackupErrorNotified = true
      showNotification({
        type: 'warning',
        message: `${t('settings.autoBackupFailed')}: ${parseError(e)}`,
        duration: 5000,
      })
    }
  }
}

const doAutoBackup = async (retention: number) => {
  try {
    await backupApi.autoBackup()
    await backupApi.cleanupOldBackups(retention)
    autoBackupErrorNotified = false // 成功后重置，下次失败可再通知
  } catch (e: unknown) {
    if (!autoBackupErrorNotified) {
      autoBackupErrorNotified = true
      showNotification({
        type: 'warning',
        message: `${t('settings.autoBackupFailed')}: ${parseError(e)}`,
        duration: 5000,
      })
    }
  }
}

/** 进入主应用模式后的初始化 */
const enterMainMode = async () => {
  appMode.value = 'main'

  // 初始化数据
  await loadShortcuts()
  await initializeNotes()

  // 桌面端：窗口关闭时最小化到托盘（Tauri 平台特性）
  if (!isTauriMobile.value) {
    const currentWindow = getCurrentWindow()
    await currentWindow.onCloseRequested(async (event) => {
      event.preventDefault()
      if (appStore.isDirty) {
        const confirmed = await ask(t('editor.unsavedChanges.message'), {
          title: t('editor.unsavedChanges.title'),
          kind: 'warning',
        })
        if (!confirmed) return
      }
      await currentWindow.hide()
    })
  }

  // 启动自动备份
  startAutoBackup()

  // 检查启动锁屏
  await checkStartupLock()
  setupMinimizeListener()
}

const initApp = async () => {
  try {
    const needsSetup = await profileApi.checkSetupNeeded()
    if (needsSetup) {
      appMode.value = 'setup'
      return
    }

    const profiles = await profileApi.listProfiles()
    hasExistingProfiles.value = profiles.length > 0
    const index = await profileApi.getProfileIndex()

    if (profiles.length > 1 && !index.autoConnect) {
      appMode.value = 'select'
      return
    }
  } catch (err: unknown) {
    const shouldRetry = await ask(`${t('error.startupErrorMessage')}\n\n${parseError(err)}`, {
      title: t('error.startupError'),
      kind: 'error',
      okLabel: t('common.retry'),
      cancelLabel: t('common.close'),
    })
    if (shouldRetry) {
      await initApp()
    } else {
      await getCurrentWindow().close()
    }
    return
  }

  await enterMainMode()
}

onMounted(() => initApp())

onUnmounted(() => {
  if (autoBackupTimer) {
    clearInterval(autoBackupTimer)
    autoBackupTimer = null
  }
})
</script>

<style scoped>
/* 加载屏 */
.loading-screen {
  position: fixed;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 24px;
  background: var(--color-bg-primary, #ffffff);
  z-index: 9999;
}

.loading-icon {
  width: 48px;
  height: 48px;
  color: var(--color-primary, #4f46e5);
}

.loading-spinner {
  width: 28px;
  height: 28px;
  border: 3px solid var(--color-border, #e2e8f0);
  border-top-color: var(--color-primary, #4f46e5);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

.loading-text {
  font-size: 13px;
  color: var(--color-text-secondary, #94a3b8);
  letter-spacing: 0.5px;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* 淡出过渡 */
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-leave-to {
  opacity: 0;
}
</style>
