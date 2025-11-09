<template>
  <el-container class="flex h-screen bg-white">
    <!-- 侧边栏组件 -->
    <Sidebar :notebooks="notebooks" :tags="tags" :active-notebook="state.activeNotebook"
      @set-active-notebook="setActiveNotebook" @create-new-note="createNewNote" @save-notebook="saveNotebook" />

    <!-- 笔记列表组件 -->
    <NotesList :notebooks="notebooks" :notes="notes" :active-notebook="state.activeNotebook"
      :active-note="state.activeNote" v-model:query="query" @set-active-note="setActiveNote"
      @update-search-query="handleUpdateSearchQuery" />

    <!-- 编辑器组件 -->
    <Editor :active-note="activeNoteData" :edit-mode="state.editMode" @save-note="saveNote" @cancel-edit="cancelEdit"
      @delete-note="deleteNote" @toggle-edit-mode="state.editMode = !state.editMode"
      @update-note-title="updateNoteTitle" @update-note-content="updateNoteContent" />
  </el-container>
</template>

<script setup lang="ts">
import { useNotes } from './composables/useNotes'
import Sidebar from './components/Sidebar.vue'
import NotesList from './components/NotesList.vue'
import Editor from './components/Editor.vue'

const {
  notebooks,
  notes,
  tags,
  query,
  state,
  activeNoteData,
  saveNotebook,
  setActiveNotebook,
  setActiveNote,
  createNewNote,
  saveNote,
  cancelEdit,
  deleteNote,
  updateNoteTitle,
  updateNoteContent,
  handleUpdateSearchQuery
} = useNotes()
</script>
