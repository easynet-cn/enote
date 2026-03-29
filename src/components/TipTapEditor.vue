<template>
  <div class="tiptap-readonly-editor">
    <editor-content :editor="editor" class="editor-content" />
  </div>
</template>

<script setup lang="ts">
import { watch, onBeforeUnmount, onMounted, shallowRef } from 'vue'
import { Editor, EditorContent } from '@tiptap/vue-3'
import { getRichTextExtensions } from '../config/editorExtensions'

interface Props {
  modelValue: string
  editable?: boolean
  showToolbar?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  editable: false,
  showToolbar: false,
})

// 使用 shallowRef 手动管理编辑器实例（异步初始化）
const editor = shallowRef<Editor | undefined>(undefined)

onMounted(async () => {
  const extensions = await getRichTextExtensions()
  editor.value = new Editor({
    extensions,
    content: props.modelValue || '',
    editable: props.editable,
    editorProps: {
      attributes: {
        class: 'prose prose-sm sm:prose lg:prose-lg xl:prose-xl max-w-none',
      },
    },
  })
})

// 监听内容变化
watch(
  () => props.modelValue,
  (newValue) => {
    if (editor.value && newValue !== editor.value.getHTML()) {
      editor.value.commands.setContent(newValue)
    }
  },
)

// 监听编辑模式变化
watch(
  () => props.editable,
  (newEditable) => {
    if (editor.value) {
      editor.value.setEditable(newEditable)
    }
  },
)

// 组件卸载时销毁编辑器
onBeforeUnmount(() => {
  if (editor.value) {
    editor.value.destroy()
  }
})
</script>

<!-- 共享 ProseMirror 样式 -->
<style src="../styles/prosemirror.css"></style>

<style scoped>
.tiptap-readonly-editor {
  height: 100%;
  overflow: hidden;
}

.editor-content {
  height: 100%;
  overflow-y: auto;
  padding: 0;
}

/* TipTapEditor 特有样式 */
:deep(.ProseMirror) {
  height: 100%;
  overflow-y: auto;
}

:deep(.ProseMirror code) {
  color: var(--color-danger);
}

/* 只读模式下的样式 */
:deep(.ProseMirror[contenteditable='false']) {
  cursor: default;
  user-select: text;
}

:deep(.ProseMirror[contenteditable='false']:focus) {
  outline: none;
}
</style>
