<template>
  <div class="toolbar-section">
    <div class="btn-group">
      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.highlight')"
        :active="editor?.isActive('highlight')"
        :disabled="!editMode"
        @click="editor?.chain().focus().toggleHighlight().run()"
      >
        <Highlighter class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.divider')"
        :disabled="!editMode"
        @click="editor?.chain().focus().setHorizontalRule().run()"
      >
        <Minus class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.clearFormat')"
        :disabled="!editMode"
        @click="editor?.chain().focus().clearNodes().unsetAllMarks().run()"
      >
        <RemoveFormatting class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.findReplace')"
        :active="searchVisible"
        :disabled="!editMode"
        @click="$emit('toggle-search')"
      >
        <Search class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.toc')"
        :active="tocVisible"
        @click="$emit('toggle-toc')"
      >
        <ListTree class="w-4 h-4" />
      </ToolbarButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'
import { ToolbarButton } from '.'
import { Highlighter, Minus, RemoveFormatting, Search, ListTree } from 'lucide-vue-next'

defineProps<{
  editor: Editor | null
  editMode: boolean
  searchVisible: boolean
  tocVisible: boolean
}>()

defineEmits<{
  'toggle-search': []
  'toggle-toc': []
}>()

const { t } = useI18n()
</script>
