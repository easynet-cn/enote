<template>
  <div class="toolbar-section">
    <div class="btn-group">
      <ToolbarButton
        tooltip="粗体"
        :active="editor?.isActive('bold')"
        :disabled="!editMode || !editor?.can().chain().focus().toggleBold().run()"
        @click="editor?.chain().focus().toggleBold().run()"
      >
        <Bold class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        tooltip="斜体"
        :active="editor?.isActive('italic')"
        :disabled="!editMode || !editor?.can().chain().focus().toggleItalic().run()"
        @click="editor?.chain().focus().toggleItalic().run()"
      >
        <Italic class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        tooltip="下划线"
        :active="editor?.isActive('underline')"
        :disabled="!editMode"
        @click="editor?.chain().focus().toggleUnderline().run()"
      >
        <UnderlineIcon class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        tooltip="删除线"
        :active="editor?.isActive('strike')"
        :disabled="!editMode || !editor?.can().chain().focus().toggleStrike().run()"
        @click="editor?.chain().focus().toggleStrike().run()"
      >
        <Strikethrough class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        tooltip="行内代码"
        :active="editor?.isActive('code')"
        :disabled="!editMode || !editor?.can().chain().focus().toggleCode().run()"
        @click="editor?.chain().focus().toggleCode().run()"
      >
        <Code class="w-4 h-4" />
      </ToolbarButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Editor } from '@tiptap/vue-3'
import { Bold, Italic, Underline as UnderlineIcon, Strikethrough, Code } from 'lucide-vue-next'
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
