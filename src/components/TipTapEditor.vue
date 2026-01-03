<template>
  <div class="tiptap-readonly-editor">
    <editor-content :editor="editor" class="editor-content" />
  </div>
</template>

<script setup lang="ts">
import { watch, onBeforeUnmount } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import { getViewerExtensions } from '../config/editorExtensions'

interface Props {
  modelValue: string
  editable?: boolean
  showToolbar?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  editable: false,
  showToolbar: false,
})

// TipTap 编辑器实例 - 只读模式（使用精简扩展）
const editor = useEditor({
  extensions: getViewerExtensions(),
  content: props.modelValue || '',
  editable: props.editable,
  editorProps: {
    attributes: {
      class: 'prose prose-sm sm:prose lg:prose-lg xl:prose-xl max-w-none',
    },
  },
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

/* 为ProseMirror内容区域设置基本样式 */
:deep(.ProseMirror) {
  outline: none;
  line-height: 1.6;
  height: 100%;
  overflow-y: auto;
}

:deep(.ProseMirror h1) {
  font-size: 2rem;
  font-weight: bold;
  margin: 1rem 0;
  color: #1f2937;
}

:deep(.ProseMirror h2) {
  font-size: 1.5rem;
  font-weight: bold;
  margin: 0.875rem 0;
  color: #374151;
}

:deep(.ProseMirror h3) {
  font-size: 1.25rem;
  font-weight: bold;
  margin: 0.75rem 0;
  color: #4b5563;
}

:deep(.ProseMirror h4) {
  font-size: 1.125rem;
  font-weight: 600;
  margin: 0.625rem 0;
  color: #4b5563;
}

:deep(.ProseMirror h5) {
  font-size: 1rem;
  font-weight: 600;
  margin: 0.5rem 0;
  color: #6b7280;
}

:deep(.ProseMirror h6) {
  font-size: 0.875rem;
  font-weight: 600;
  margin: 0.5rem 0;
  color: #6b7280;
}

:deep(.ProseMirror p) {
  margin-bottom: 0.75rem;
}

:deep(.ProseMirror ul),
:deep(.ProseMirror ol) {
  padding-left: 1.5rem;
  margin-bottom: 0.75rem;
}

:deep(.ProseMirror li) {
  margin-bottom: 0.25rem;
}

:deep(.ProseMirror blockquote) {
  border-left: 4px solid #e5e7eb;
  padding-left: 1rem;
  margin-left: 0;
  margin-right: 0;
  font-style: italic;
  margin-bottom: 0.75rem;
  color: #6b7280;
}

:deep(.ProseMirror code) {
  background-color: #f3f4f6;
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-family: 'Courier New', Courier, monospace;
  color: #dc2626;
}

:deep(.ProseMirror pre) {
  background-color: #1f2937;
  color: #f9fafb;
  padding: 0.75rem;
  border-radius: 0.5rem;
  overflow-x: auto;
  margin-bottom: 0.75rem;
}

:deep(.ProseMirror a) {
  color: #3b82f6;
  text-decoration: underline;
}

:deep(.ProseMirror mark) {
  background-color: #fef08a;
  padding: 0.125rem 0.25rem;
}

:deep(.ProseMirror table) {
  border-collapse: collapse;
  margin-bottom: 0.75rem;
  width: 100%;
}

:deep(.ProseMirror th),
:deep(.ProseMirror td) {
  border: 1px solid #d1d5db;
  padding: 0.5rem;
  text-align: left;
}

:deep(.ProseMirror th) {
  background-color: #f9fafb;
  font-weight: 600;
}

:deep(.ProseMirror img) {
  max-width: 100%;
  height: auto;
  border-radius: 0.375rem;
}

/* 任务列表样式 */
:deep(.ProseMirror ul[data-type='taskList']) {
  list-style: none;
  padding-left: 0;
}

:deep(.ProseMirror ul[data-type='taskList'] li) {
  display: flex;
  align-items: flex-start;
  gap: 0.5rem;
  margin-bottom: 0.25rem;
}

:deep(.ProseMirror ul[data-type='taskList'] li > label) {
  flex-shrink: 0;
  margin-top: 0.25rem;
}

:deep(.ProseMirror ul[data-type='taskList'] li > label input[type='checkbox']) {
  width: 1rem;
  height: 1rem;
  cursor: pointer;
  accent-color: #10b981;
}

:deep(.ProseMirror ul[data-type='taskList'] li > div) {
  flex: 1;
}

:deep(.ProseMirror ul[data-type='taskList'] li[data-checked='true'] > div) {
  text-decoration: line-through;
  color: #9ca3af;
}

/* 链接悬停样式 */
:deep(.ProseMirror a:hover) {
  color: #2563eb;
}

/* 只读模式下的样式 */
:deep(.ProseMirror[contenteditable='false']) {
  cursor: default;
  user-select: text;
}

:deep(.ProseMirror[contenteditable='false']:focus) {
  outline: none;
}

/* 上标和下标样式 */
:deep(.ProseMirror sup) {
  font-size: 0.75em;
  vertical-align: super;
}

:deep(.ProseMirror sub) {
  font-size: 0.75em;
  vertical-align: sub;
}

/* 代码高亮样式 (One Dark 主题) */
:deep(.ProseMirror pre code.hljs) {
  display: block;
  overflow-x: auto;
  padding: 1em;
}

:deep(.ProseMirror code.hljs) {
  padding: 3px 5px;
}

:deep(.ProseMirror .hljs) {
  color: #abb2bf;
  background: #282c34;
}

:deep(.ProseMirror .hljs-comment),
:deep(.ProseMirror .hljs-quote) {
  color: #5c6370;
  font-style: italic;
}

:deep(.ProseMirror .hljs-doctag),
:deep(.ProseMirror .hljs-keyword),
:deep(.ProseMirror .hljs-formula) {
  color: #c678dd;
}

:deep(.ProseMirror .hljs-section),
:deep(.ProseMirror .hljs-name),
:deep(.ProseMirror .hljs-selector-tag),
:deep(.ProseMirror .hljs-deletion),
:deep(.ProseMirror .hljs-subst) {
  color: #e06c75;
}

:deep(.ProseMirror .hljs-literal) {
  color: #56b6c2;
}

:deep(.ProseMirror .hljs-string),
:deep(.ProseMirror .hljs-regexp),
:deep(.ProseMirror .hljs-addition),
:deep(.ProseMirror .hljs-attribute),
:deep(.ProseMirror .hljs-meta .hljs-string) {
  color: #98c379;
}

:deep(.ProseMirror .hljs-attr),
:deep(.ProseMirror .hljs-variable),
:deep(.ProseMirror .hljs-template-variable),
:deep(.ProseMirror .hljs-type),
:deep(.ProseMirror .hljs-selector-class),
:deep(.ProseMirror .hljs-selector-attr),
:deep(.ProseMirror .hljs-selector-pseudo),
:deep(.ProseMirror .hljs-number) {
  color: #d19a66;
}

:deep(.ProseMirror .hljs-symbol),
:deep(.ProseMirror .hljs-bullet),
:deep(.ProseMirror .hljs-link),
:deep(.ProseMirror .hljs-meta),
:deep(.ProseMirror .hljs-selector-id),
:deep(.ProseMirror .hljs-title) {
  color: #61aeee;
}

:deep(.ProseMirror .hljs-built_in),
:deep(.ProseMirror .hljs-title.class_),
:deep(.ProseMirror .hljs-class .hljs-title) {
  color: #e6c07b;
}

:deep(.ProseMirror .hljs-emphasis) {
  font-style: italic;
}

:deep(.ProseMirror .hljs-strong) {
  font-weight: bold;
}

:deep(.ProseMirror .hljs-link) {
  text-decoration: underline;
}
</style>
