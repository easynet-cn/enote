<template>
  <div v-if="isInCodeBlock" class="toolbar-section">
    <Tooltip :content="t('editor.toolbarTooltip.codeLanguage')" placement="bottom">
      <AppSelect
        :modelValue="currentLanguage"
        @update:modelValue="setLanguage(String($event))"
        :options="languages"
        :disabled="!editMode"
        size="sm"
        class="code-lang-select"
      />
    </Tooltip>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onBeforeUnmount, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'
import { Tooltip, AppSelect } from '../ui'

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
  max-width: 130px;
}
</style>
