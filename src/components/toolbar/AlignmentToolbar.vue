<template>
  <div class="toolbar-section">
    <div class="btn-group">
      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.alignLeft')"
        :active="editor?.isActive({ textAlign: 'left' })"
        :disabled="!editMode"
        @click="editor?.chain().focus().setTextAlign('left').run()"
      >
        <AlignLeft class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.alignCenter')"
        :active="editor?.isActive({ textAlign: 'center' })"
        :disabled="!editMode"
        @click="editor?.chain().focus().setTextAlign('center').run()"
      >
        <AlignCenter class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.alignRight')"
        :active="editor?.isActive({ textAlign: 'right' })"
        :disabled="!editMode"
        @click="editor?.chain().focus().setTextAlign('right').run()"
      >
        <AlignRight class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.alignJustify')"
        :active="editor?.isActive({ textAlign: 'justify' })"
        :disabled="!editMode"
        @click="editor?.chain().focus().setTextAlign('justify').run()"
      >
        <AlignJustify class="w-4 h-4" />
      </ToolbarButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Editor } from '@tiptap/vue-3'
import { useI18n } from 'vue-i18n'
import { AlignLeft, AlignCenter, AlignRight, AlignJustify } from 'lucide-vue-next'
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
  border-radius: 8px;
  padding: 2px;
}
</style>
