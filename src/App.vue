<template>
  <div class="flex h-screen bg-slate-50">
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
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useNotes } from './composables/useNotes'
import { useKeyboardShortcuts } from './composables/useKeyboardShortcuts'
import AppSidebar from './components/AppSidebar.vue'
import NoteList from './components/NoteList.vue'
import NoteEditor from './components/NoteEditor.vue'

// 折叠状态
const sidebarCollapsed = ref(false)
const noteListCollapsed = ref(false)
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
    description: '保存笔记',
  },
  {
    key: 'n',
    ctrl: true,
    handler: () => {
      createNewNote()
    },
    description: '新建笔记',
  },
  {
    key: 'e',
    ctrl: true,
    handler: () => {
      if (activeNote.value && !editMode.value) {
        editMode.value = true
      }
    },
    description: '编辑笔记',
  },
  {
    key: 'Escape',
    handler: () => {
      if (editMode.value) {
        cancelEdit()
      }
    },
    description: '取消编辑',
  },
  {
    key: 'b',
    ctrl: true,
    handler: () => {
      sidebarCollapsed.value = !sidebarCollapsed.value
    },
    description: '切换侧边栏',
  },
])
</script>
