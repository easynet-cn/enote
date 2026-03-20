<template>
  <div class="toolbar-section">
    <div class="btn-group">
      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.undo')"
        :disabled="!editMode || !editor?.can().chain().focus().undo().run()"
        @click="editor?.chain().focus().undo().run()"
      >
        <Undo2 class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.redo')"
        :disabled="!editMode || !editor?.can().chain().focus().redo().run()"
        @click="editor?.chain().focus().redo().run()"
      >
        <Redo2 class="w-4 h-4" />
      </ToolbarButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'
import { ToolbarButton } from '.'
import { Undo2, Redo2 } from 'lucide-vue-next'

defineProps<{
  editor: Editor | null
  editMode: boolean
}>()

const { t } = useI18n()
</script>
