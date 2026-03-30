<template>
  <footer
    class="h-10 flex items-center justify-between px-4 text-xs text-content-secondary border-t border-edge bg-surface-alt shrink-0"
    role="contentinfo"
  >
    <div class="flex items-center gap-4">
      <span class="hidden sm:inline"
        >{{ t('editor.statusBar.line') }} {{ cursorPosition.line }},
        {{ t('editor.statusBar.column') }} {{ cursorPosition.column }}</span
      >
      <span v-if="hasSelection" class="text-indigo-600"
        >{{ t('editor.statusBar.selected') }} {{ selectedCount }}
        {{ t('editor.statusBar.characters') }}</span
      >
    </div>
    <div class="flex items-center gap-3">
      <!-- 附件按钮 -->
      <button
        v-if="showPanelButtons"
        class="status-btn"
        :class="{ 'text-indigo-600': attachmentActive }"
        @click="$emit('toggle-attachment')"
        :title="t('attachment.title')"
      >
        <Paperclip class="w-3.5 h-3.5" />
        <span v-if="attachmentCount > 0">{{ attachmentCount }}</span>
      </button>
      <!-- 关联笔记按钮 -->
      <button
        v-if="showPanelButtons"
        class="status-btn"
        :class="{ 'text-indigo-600': linkActive }"
        @click="$emit('toggle-link')"
        :title="t('noteLink.title')"
      >
        <Link2 class="w-3.5 h-3.5" />
      </button>
      <span class="border-l border-edge pl-3"
        >{{ characterCount }} {{ t('editor.statusBar.characters') }}</span
      >
      <span class="hidden sm:inline">{{ wordCount }} {{ t('editor.statusBar.words') }}</span>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { ref, computed, watch, onBeforeUnmount } from 'vue'
import { useI18n } from 'vue-i18n'
import { Paperclip, Link2 } from 'lucide-vue-next'
import type { Editor } from '@tiptap/vue-3'

interface Props {
  editor: Editor
  showPanelButtons?: boolean
  attachmentCount?: number
  attachmentActive?: boolean
  linkActive?: boolean
}

const { t } = useI18n()
const props = withDefaults(defineProps<Props>(), {
  showPanelButtons: false,
  attachmentCount: 0,
  attachmentActive: false,
  linkActive: false,
})

defineEmits<{
  (e: 'toggle-attachment'): void
  (e: 'toggle-link'): void
}>()

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

<style scoped>
.status-btn {
  display: inline-flex;
  align-items: center;
  gap: 3px;
  padding: 2px 6px;
  border-radius: 4px;
  transition: all 0.15s;
  cursor: pointer;
}

.status-btn:hover {
  background: var(--color-surface-dim, #f1f5f9);
  color: var(--color-primary);
}
</style>
