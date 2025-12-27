<template>
  <div class="flex h-screen bg-slate-50">
    <!-- 侧边栏组件 -->
    <Sidebar
      :notebooks="notebooks"
      :tags="tags"
      :active-notebook="state.activeNotebook"
      :active-tag="state.activeTag"
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
      :active-notebook="state.activeNotebook"
      :active-note="state.activeNote"
      :collapsed="noteListCollapsed"
      v-model:current-page="state.notePageIndex"
      v-model:page-size="state.notePageSize"
      v-model:total="state.noteTotal"
      v-model:query="query"
      v-model:width="noteListWidth"
      @set-active-note="setActiveNote"
      @update-search-query="handleUpdateSearchQuery"
      @size-change="handleNoteSizeChange"
      @current-change="handleNoteCurrentChange"
      @toggle-collapse="handleNoteListToggle"
    />

    <!-- 编辑器组件 -->
    <Editor
      v-model:history-data="histories"
      v-model:current-page="state.historyPageIndex"
      v-model:page-size="state.historyPageSize"
      v-model:total="state.historyTotal"
      :notebooks="notebooks"
      :tags="tags"
      :active-note="activeNoteData"
      :edit-mode="state.editMode"
      :history-loading="state.historyLoading"
      @save-note="saveNote"
      @cancel-edit="cancelEdit"
      @delete-note="deleteNote"
      @toggle-edit-mode="state.editMode = !state.editMode"
      @update-note-title="updateNoteTitle"
      @update-note-content="updateNoteContent"
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
import Sidebar from './components/Sidebar.vue'
import NoteList from './components/NoteList.vue'
import Editor from './components/Editor.vue'

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
  notebooks,
  notes,
  tags,
  query,
  histories,
  state,
  activeNoteData,
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
      if (state.editMode && state.activeNote) {
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
      if (state.activeNote && !state.editMode) {
        state.editMode = true
      }
    },
    description: '编辑笔记',
  },
  {
    key: 'Escape',
    handler: () => {
      if (state.editMode) {
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
