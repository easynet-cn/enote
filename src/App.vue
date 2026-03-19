<template>
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
  <div v-else class="flex h-screen bg-surface-alt relative">
    <!-- 侧边栏组件 -->
    <AppSidebar
      :notebooks="notebooks"
      :tags="tags"
      :active-notebook="activeNotebook"
      :active-tag="activeTag"
      :collapsed="sidebarCollapsed"
      @set-active-notebook="setActiveNotebook"
      @set-active-tag="setActiveTag"
      @create-new-note="createNewNote"
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
    />

    <!-- 笔记列表组件 -->
    <NoteList
      :notebooks="notebooks"
      :notes="notes"
      :active-notebook="activeNotebook"
      :active-note="activeNote"
      :collapsed="noteListCollapsed"
      v-model:current-page="notePageIndex"
      v-model:page-size="notePageSize"
      v-model:total="noteTotal"
      v-model:query="query"
      v-model:width="noteListWidth"
      @set-active-note="setActiveNote"
      @update-search-query="handleUpdateSearchQuery"
      @size-change="handleNoteSizeChange"
      @current-change="handleNoteCurrentChange"
      @toggle-collapse="handleNoteListToggle"
      @toggle-pin="handleTogglePin"
    />

    <!-- 编辑器组件 -->
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
    />

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
} from 'lucide-vue-next'

const { t } = useI18n()
const appStore = useAppStore()

// 应用模式：'loading' | 'setup' | 'select' | 'main'
const appMode = ref<'loading' | 'setup' | 'select' | 'main'>('loading')
const hasExistingProfiles = ref(false)

const onSetupComplete = () => {
  // 应用会重启，这里只是占位
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
    const msg = e instanceof Error ? e.message : String(e)
    showNotification({ type: 'error', message: msg })
  }
}

const handleReorderTags = async (orders: [string, number][]) => {
  try {
    await noteApi.reorderTags(orders.map(([id, order]) => [Number(id), order]))
    await refreshAllData()
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : String(e)
    showNotification({ type: 'error', message: msg })
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
    const msg = e instanceof Error ? e.message : String(e)
    showNotification({ type: 'error', message: msg })
  }
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
    const msg = e instanceof Error ? e.message : String(e)
    showNotification({ type: 'error', message: msg })
  }
}

// 命令面板命令列表
const paletteCommands = computed<PaletteCommand[]>(() => [
  {
    id: 'new-note',
    name: t('commandPalette.commands.newNote'),
    icon: markRaw(Plus),
    category: t('commandPalette.categories.notes'),
    shortcut: 'Ctrl+N',
    handler: () => createNewNote(),
  },
  {
    id: 'save-note',
    name: t('commandPalette.commands.saveNote'),
    icon: markRaw(Save),
    category: t('commandPalette.categories.notes'),
    shortcut: 'Ctrl+S',
    handler: () => {
      if (editMode.value && activeNote.value) saveNote()
    },
  },
  {
    id: 'edit-note',
    name: t('commandPalette.commands.editNote'),
    icon: markRaw(Pencil),
    category: t('commandPalette.categories.notes'),
    shortcut: 'Ctrl+E',
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
    shortcut: 'Ctrl+B',
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
    shortcut: 'Ctrl+L',
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
])

// 键盘快捷键
useKeyboardShortcuts([
  {
    key: 's',
    ctrl: true,
    handler: () => {
      if (editMode.value && activeNote.value) {
        saveNote()
      }
    },
    description: t('shortcuts.save'),
  },
  {
    key: 'n',
    ctrl: true,
    handler: () => {
      createNewNote()
    },
    description: t('shortcuts.newNote'),
  },
  {
    key: 'e',
    ctrl: true,
    handler: () => {
      if (activeNote.value && !editMode.value) {
        editMode.value = true
      }
    },
    description: t('shortcuts.editNote'),
  },
  {
    key: 'Escape',
    handler: () => {
      if (editMode.value) {
        cancelEdit()
      }
    },
    description: t('shortcuts.cancelEdit'),
  },
  {
    key: 'b',
    ctrl: true,
    handler: () => {
      sidebarCollapsed.value = !sidebarCollapsed.value
    },
    description: t('shortcuts.toggleSidebar'),
  },
  {
    key: 'l',
    ctrl: true,
    handler: () => lock(),
    description: t('shortcuts.lockApp'),
  },
  {
    key: 'p',
    ctrl: true,
    handler: () => {
      commandPaletteVisible.value = !commandPaletteVisible.value
    },
    description: t('shortcuts.commandPalette'),
  },
])

// 关闭窗口拦截：未保存时提示确认
// 自动备份定时器
let autoBackupTimer: ReturnType<typeof setTimeout> | ReturnType<typeof setInterval> | null = null

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
  } catch {
    // 自动备份初始化失败，静默忽略
  }
}

const doAutoBackup = async (retention: number) => {
  try {
    await backupApi.autoBackup()
    await backupApi.cleanupOldBackups(retention)
  } catch {
    // 静默忽略
  }
}

/** 进入主应用模式后的初始化 */
const enterMainMode = async () => {
  appMode.value = 'main'

  // 初始化数据
  await initializeNotes()

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
    // 最小化到托盘而非退出
    await currentWindow.hide()
  })

  // 启动自动备份
  startAutoBackup()

  // 检查启动锁屏
  await checkStartupLock()
  setupMinimizeListener()
}

onMounted(async () => {
  // 检查应用启动模式
  try {
    const needsSetup = await profileApi.checkSetupNeeded()
    if (needsSetup) {
      appMode.value = 'setup'
      // 向导/选择模式：不拦截窗口关闭，系统 ✕ 直接退出应用
      return
    }

    // 检查是否有多个 profile 需要选择
    const profiles = await profileApi.listProfiles()
    hasExistingProfiles.value = profiles.length > 0
    const index = await profileApi.getProfileIndex()

    if (profiles.length > 1 && !index.autoConnect) {
      appMode.value = 'select'
      // 不拦截窗口关闭，系统 ✕ 直接退出应用
      return
    }
  } catch {
    // 如果 check 失败，说明后端已正常初始化（兼容旧模式），直接进入主界面
  }

  // 进入主应用模式
  await enterMainMode()
})

onUnmounted(() => {
  if (autoBackupTimer) {
    clearInterval(autoBackupTimer)
    autoBackupTimer = null
  }
})
</script>
