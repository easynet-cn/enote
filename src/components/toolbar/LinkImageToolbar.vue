<template>
  <div class="toolbar-section">
    <div class="btn-group">
      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.link')"
        :active="editor?.isActive('link')"
        :disabled="!editMode"
        @click="$emit('open-link-dialog')"
      >
        <LinkIcon class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.unlink')"
        :disabled="!editMode || !editor?.isActive('link')"
        @click="editor?.chain().focus().unsetLink().run()"
      >
        <Unlink class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.image')"
        :disabled="!editMode"
        @click="$emit('open-image-dialog')"
      >
        <ImageIcon class="w-4 h-4" />
      </ToolbarButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'
import { ToolbarButton } from '.'
import { Link as LinkIcon, Unlink, Image as ImageIcon } from 'lucide-vue-next'

defineProps<{
  editor: Editor | null
  editMode: boolean
}>()

defineEmits<{
  'open-link-dialog': []
  'open-image-dialog': []
}>()

const { t } = useI18n()
</script>
