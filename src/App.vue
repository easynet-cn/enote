<template>
  <div class="flex h-screen bg-white">
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
      @set-active-note="setActiveNote"
      @update-search-query="handleUpdateSearchQuery"
      @size-change="handleNoteSizeChange"
      @current-change="handleNoteCurrentChange"
      @toggle-collapse="noteListCollapsed = !noteListCollapsed"
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
import Sidebar from './components/Sidebar.vue'
import NoteList from './components/NoteList.vue'
import Editor from './components/Editor.vue'

// 折叠状态
const sidebarCollapsed = ref(false)
const noteListCollapsed = ref(false)

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
</script>
