<template>
  <div v-if="isInCodeBlock" class="toolbar-section">
    <Tooltip :content="t('editor.toolbarTooltip.codeLanguage')" placement="bottom">
      <select
        :value="currentLanguage"
        @change="setLanguage(($event.target as HTMLSelectElement).value)"
        :disabled="!editMode"
        class="code-lang-select"
      >
        <option v-for="lang in languages" :key="lang.value" :value="lang.value">
          {{ lang.label }}
        </option>
      </select>
    </Tooltip>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onBeforeUnmount, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'
import { Tooltip } from '../ui'

const props = defineProps<{
  editor: Editor | null
  editMode: boolean
}>()

const { t } = useI18n()

const isInCodeBlock = ref(false)
const currentLanguage = ref('')

const syncState = () => {
  const editor = props.editor
  if (!editor) {
    isInCodeBlock.value = false
    currentLanguage.value = ''
    return
  }
  isInCodeBlock.value = editor.isActive('codeBlock')
  if (isInCodeBlock.value) {
    currentLanguage.value = (editor.getAttributes('codeBlock').language as string) || ''
  } else {
    currentLanguage.value = ''
  }
}

watch(
  () => props.editor,
  (editor, oldEditor) => {
    if (oldEditor) {
      oldEditor.off('selectionUpdate', syncState)
      oldEditor.off('update', syncState)
    }
    if (editor) {
      editor.on('selectionUpdate', syncState)
      editor.on('update', syncState)
      syncState()
    }
  },
  { immediate: true },
)

onBeforeUnmount(() => {
  if (props.editor) {
    props.editor.off('selectionUpdate', syncState)
    props.editor.off('update', syncState)
  }
})

const setLanguage = (lang: string) => {
  props.editor?.chain().focus().updateAttributes('codeBlock', { language: lang }).run()
}

const languages = computed(() => [
  { value: '', label: t('editor.toolbarTooltip.plainText') },
  { value: 'javascript', label: 'JavaScript' },
  { value: 'typescript', label: 'TypeScript' },
  { value: 'python', label: 'Python' },
  { value: 'java', label: 'Java' },
  { value: 'csharp', label: 'C#' },
  { value: 'cpp', label: 'C++' },
  { value: 'c', label: 'C' },
  { value: 'go', label: 'Go' },
  { value: 'rust', label: 'Rust' },
  { value: 'ruby', label: 'Ruby' },
  { value: 'php', label: 'PHP' },
  { value: 'swift', label: 'Swift' },
  { value: 'kotlin', label: 'Kotlin' },
  { value: 'html', label: 'HTML' },
  { value: 'css', label: 'CSS' },
  { value: 'scss', label: 'SCSS' },
  { value: 'json', label: 'JSON' },
  { value: 'yaml', label: 'YAML' },
  { value: 'xml', label: 'XML' },
  { value: 'markdown', label: 'Markdown' },
  { value: 'sql', label: 'SQL' },
  { value: 'bash', label: 'Bash' },
  { value: 'shell', label: 'Shell' },
  { value: 'dockerfile', label: 'Dockerfile' },
])
</script>

<style scoped>
.code-lang-select {
  height: 32px;
  padding: 0 0.5rem;
  font-size: 0.8125rem;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  background-color: var(--color-bg-primary);
  color: var(--color-text-secondary);
  cursor: pointer;
  outline: none;
  transition: all var(--transition-fast) var(--ease-default);
  max-width: 130px;
}

.code-lang-select:hover:not(:disabled) {
  border-color: var(--color-primary);
  color: var(--color-text-primary);
}

.code-lang-select:focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px var(--color-primary-light);
}

.code-lang-select:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
