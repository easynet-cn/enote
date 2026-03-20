<template>
  <div class="toolbar-section">
    <div class="btn-group">
      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.insertTable')"
        :disabled="!editMode"
        @click="
          editor?.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run()
        "
      >
        <TableIcon class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.addColumn')"
        :disabled="!editMode || !editor?.can().addColumnAfter()"
        @click="editor?.chain().focus().addColumnAfter().run()"
      >
        <Columns3 class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.addRow')"
        :disabled="!editMode || !editor?.can().addRowAfter()"
        @click="editor?.chain().focus().addRowAfter().run()"
      >
        <Rows3 class="w-4 h-4" />
      </ToolbarButton>

      <ToolbarButton
        :tooltip="t('editor.toolbarTooltip.deleteTable')"
        :disabled="!editMode || !editor?.can().deleteTable()"
        @click="editor?.chain().focus().deleteTable().run()"
      >
        <TableOff class="w-4 h-4" />
      </ToolbarButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'
import { ToolbarButton } from '.'
import { Table as TableIcon, Columns3, Rows3, TableProperties as TableOff } from 'lucide-vue-next'

defineProps<{
  editor: Editor | null
  editMode: boolean
}>()

const { t } = useI18n()
</script>
