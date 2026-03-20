<template>
  <div class="toolbar-section">
    <div class="btn-group">
      <ToolbarButton
        :tooltip="
          sourceMode
            ? t('editor.toolbarTooltip.sourceMode')
            : t('editor.toolbarTooltip.markdownSource')
        "
        :active="sourceMode"
        :disabled="!editMode"
        @click="$emit('toggle-source-mode')"
      >
        <FileCode v-if="!sourceMode" class="w-4 h-4" />
        <Eye v-else class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.verticalLayout')"
        :active="markdownLayout === MarkdownLayout.Vertical"
        :disabled="!editMode"
        @click="toggleLayout(MarkdownLayout.Vertical)"
      >
        <PanelTop class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.horizontalLayout')"
        :active="markdownLayout === MarkdownLayout.Horizontal"
        :disabled="!editMode"
        @click="toggleLayout(MarkdownLayout.Horizontal)"
      >
        <PanelLeft class="w-4 h-4" />
      </ToolbarButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { MarkdownLayout } from '../../types'
import { ToolbarButton } from '.'
import { FileCode, Eye, PanelTop, PanelLeft } from 'lucide-vue-next'

const props = defineProps<{
  sourceMode: boolean
  markdownLayout: MarkdownLayout
  editMode: boolean
}>()

const emit = defineEmits<{
  'toggle-source-mode': []
  'update:markdown-layout': [layout: MarkdownLayout]
}>()

const { t } = useI18n()

const toggleLayout = (layout: MarkdownLayout) => {
  if (props.markdownLayout === layout) {
    emit('update:markdown-layout', MarkdownLayout.None)
  } else {
    emit('update:markdown-layout', layout)
  }
}
</script>
