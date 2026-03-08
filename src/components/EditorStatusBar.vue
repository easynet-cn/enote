<template>
  <footer
    class="h-12 flex items-center justify-between px-4 text-xs text-slate-500 border-t border-slate-200 bg-slate-50"
    role="contentinfo"
  >
    <div class="flex items-center gap-4">
      <span
        >{{ t('editor.statusBar.line') }} {{ cursorPosition.line }},
        {{ t('editor.statusBar.column') }} {{ cursorPosition.column }}</span
      >
      <span v-if="hasSelection" class="text-indigo-600"
        >{{ t('editor.statusBar.selected') }} {{ selectedCount }}
        {{ t('editor.statusBar.characters') }}</span
      >
    </div>
    <div class="flex items-center gap-4">
      <span>{{ characterCount }} {{ t('editor.statusBar.characters') }}</span>
      <span>{{ wordCount }} {{ t('editor.statusBar.words') }}</span>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { ref, computed, watch, onBeforeUnmount } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'

interface Props {
  editor: Editor
}

const { t } = useI18n()
const props = defineProps<Props>()

const characterCount = ref(0)
const wordCount = ref(0)
const cursorPosition = ref({ line: 1, column: 1 })
const selectedCount = ref(0)
const hasSelection = computed(() => selectedCount.value > 0)

const updateStats = () => {
  characterCount.value = props.editor.storage.characterCount.characters() ?? 0
  wordCount.value = props.editor.storage.characterCount.words() ?? 0
}

const updateCursorPosition = () => {
  const { state } = props.editor
  const { from, to } = state.selection

  selectedCount.value = to - from

  const textBeforeCursor = state.doc.textBetween(0, from, '\n', '\n')
  const lines = textBeforeCursor.split('\n')
  const line = lines.length
  const column = (lines[lines.length - 1]?.length ?? 0) + 1

  cursorPosition.value = { line, column }
}

// 清理事件监听器
const detachListeners = (ed: Editor) => {
  ed.off('update', updateStats)
  ed.off('create', updateStats)
  ed.off('selectionUpdate', updateCursorPosition)
}

watch(
  () => props.editor,
  (ed, oldEd) => {
    if (oldEd) {
      detachListeners(oldEd)
    }
    ed.on('update', updateStats)
    ed.on('create', updateStats)
    ed.on('selectionUpdate', updateCursorPosition)
    updateStats()
  },
  { immediate: true },
)

onBeforeUnmount(() => {
  detachListeners(props.editor)
})
</script>
