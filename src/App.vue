<template>
  <div class="flex h-screen bg-slate-50 relative">
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
      @update-note-setting="(notebookId, tagIds) => updateNoteSetting(notebookId, tagIds, tags)"
      @open="openHistoryDialog"
      @size-change="handleNoteHistorySizeChange"
      @current-change="handleNoteHistoryCurrentChange"
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { ask } from '@tauri-apps/plugin-dialog'
import { useNotes } from './composables/useNotes'
import { useKeyboardShortcuts } from './composables/useKeyboardShortcuts'
import { useAppStore } from './stores/app'
import AppSidebar from './components/AppSidebar.vue'
import NoteList from './components/NoteList.vue'
import NoteEditor from './components/NoteEditor.vue'
import ImportDialog from './components/ImportDialog.vue'
import BackupDialog from './components/BackupDialog.vue'

const { t } = useI18n()
const appStore = useAppStore()

// 折叠状态
const sidebarCollapsed = ref(false)
const noteListCollapsed = ref(false)

// 导入对话框
const importDialogVisible = ref(false)
// 备份对话框
const backupDialogVisible = ref(false)
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
} = useNotes()

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
])

// 关闭窗口拦截：未保存时提示确认
onMounted(async () => {
  const currentWindow = getCurrentWindow()
  await currentWindow.onCloseRequested(async (event) => {
    if (appStore.isDirty) {
      event.preventDefault()
      const confirmed = await ask(t('editor.unsavedChanges.message'), {
        title: t('editor.unsavedChanges.title'),
        kind: 'warning',
      })
      if (confirmed) {
        getCurrentWindow().destroy()
      }
    }
  })
})
</script>
