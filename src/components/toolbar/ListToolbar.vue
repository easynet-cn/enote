<template>
  <div class="toolbar-section">
    <div class="btn-group">
      <ToolbarButton
        tooltip="无序列表"
        :active="editor?.isActive('bulletList')"
        :disabled="!editMode"
        @click="editor?.chain().focus().toggleBulletList().run()"
      >
        <List class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        tooltip="有序列表"
        :active="editor?.isActive('orderedList')"
        :disabled="!editMode"
        @click="editor?.chain().focus().toggleOrderedList().run()"
      >
        <ListOrdered class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        tooltip="任务列表"
        :active="editor?.isActive('taskList')"
        :disabled="!editMode"
        @click="editor?.chain().focus().toggleTaskList().run()"
      >
        <CheckSquare class="w-4 h-4" />
      </ToolbarButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Editor } from '@tiptap/vue-3'
import { List, ListOrdered, CheckSquare } from 'lucide-vue-next'
import ToolbarButton from './ToolbarButton.vue'

interface Props {
  editor: Editor | null
  editMode: boolean
}

defineProps<Props>()
</script>

<style scoped>
.toolbar-section {
  display: flex;
  align-items: center;
  gap: 2px;
}

.btn-group {
  display: flex;
  align-items: center;
  background: #f8fafc;
  border-radius: 8px;
  padding: 2px;
}
</style>
