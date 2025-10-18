<template>
  <div class="flex h-screen bg-white">
    <!-- 侧边栏组件 -->
    <Sidebar :notebooks="notebooks" :tags="tags" :active-notebook="state.activeNotebook"
      @set-active-notebook="setActiveNotebook" />

    <!-- 笔记列表组件 -->
    <NotesList :notebooks="notebooks" :notes="filteredNotes" :active-notebook="state.activeNotebook"
      :active-note="state.activeNote" :search-query="state.searchQuery" @set-active-note="setActiveNote"
      @create-new-note="createNewNote" @update-search-query="state.searchQuery = $event" />

    <!-- 编辑器组件 -->
    <Editor :active-note="activeNoteData" :edit-mode="state.editMode" @save-note="saveNote" @cancel-edit="cancelEdit"
      @delete-note="deleteNote" @toggle-edit-mode="state.editMode = !state.editMode"
      @update-note-title="updateNoteTitle" @update-note-content="updateNoteContent" />
  </div>
</template>

<script setup lang="ts">
import { useNotes } from './composables/useNotes'
import Sidebar from './components/Sidebar.vue'
import NotesList from './components/NotesList.vue'
import Editor from './components/Editor.vue'

const {
  notebooks,
  tags,
  state,
  filteredNotes,
  activeNoteData,
  setActiveNotebook,
  setActiveNote,
  createNewNote,
  saveNote,
  cancelEdit,
  deleteNote,
  updateNoteTitle,
  updateNoteContent
} = useNotes()
</script>
