<template>
  <div class="toolbar-section">
    <div class="btn-group">
      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.outdent')"
        :disabled="!editMode"
        @click="editor?.chain().focus().outdent().run()"
      >
        <IndentDecrease class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.indent')"
        :disabled="!editMode"
        @click="editor?.chain().focus().indent().run()"
      >
        <IndentIncrease class="w-4 h-4" />
      </ToolbarButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'
import { ToolbarButton } from '.'
import { IndentDecrease, IndentIncrease } from 'lucide-vue-next'

defineProps<{
  editor: Editor | null
  editMode: boolean
}>()

const { t } = useI18n()
</script>
