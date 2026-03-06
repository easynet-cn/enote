<template>
  <div class="toolbar-section">
    <div class="btn-group">
      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.bold')"
        :active="editor?.isActive('bold')"
        :disabled="!editMode || !editor?.can().chain().focus().toggleBold().run()"
        @click="editor?.chain().focus().toggleBold().run()"
      >
        <Bold class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.italic')"
        :active="editor?.isActive('italic')"
        :disabled="!editMode || !editor?.can().chain().focus().toggleItalic().run()"
        @click="editor?.chain().focus().toggleItalic().run()"
      >
        <Italic class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.underline')"
        :active="editor?.isActive('underline')"
        :disabled="!editMode"
        @click="editor?.chain().focus().toggleUnderline().run()"
      >
        <UnderlineIcon class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.strikethrough')"
        :active="editor?.isActive('strike')"
        :disabled="!editMode || !editor?.can().chain().focus().toggleStrike().run()"
        @click="editor?.chain().focus().toggleStrike().run()"
      >
        <Strikethrough class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.inlineCode')"
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
import { useI18n } from 'vue-i18n'
import { Bold, Italic, Underline as UnderlineIcon, Strikethrough, Code } from 'lucide-vue-next'
import ToolbarButton from './ToolbarButton.vue'

const { t } = useI18n()

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
  background: var(--color-bg-secondary);
  border-radius: var(--radius-lg);
  padding: 2px;
}
</style>
