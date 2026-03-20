<template>
  <div class="toolbar-section">
    <div class="btn-group">
      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.quote')"
        :active="editor?.isActive('blockquote')"
        :disabled="!editMode"
        @click="editor?.chain().focus().toggleBlockquote().run()"
      >
        <Quote class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.codeBlock')"
        :active="editor?.isActive('codeBlock')"
        :disabled="!editMode"
        @click="editor?.chain().focus().toggleCodeBlock().run()"
      >
        <Code2 class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.inlineCode')"
        :active="editor?.isActive('code')"
        :disabled="!editMode"
        @click="editor?.chain().focus().toggleCode().run()"
      >
        <Code class="w-4 h-4" />
      </ToolbarButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'
import { ToolbarButton } from '.'
import { Quote, Code2, Code } from 'lucide-vue-next'

defineProps<{
  editor: Editor | null
  editMode: boolean
}>()

const { t } = useI18n()
</script>
