<template>
    <div class="tiptap-readonly-editor">
        <editor-content :editor="editor" class="editor-content" />
    </div>
</template>

<script setup lang="ts">
import { watch, onBeforeUnmount } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import TextAlign from '@tiptap/extension-text-align'
import { TextStyle } from '@tiptap/extension-text-style'
import Color from '@tiptap/extension-color'
import Highlight from '@tiptap/extension-highlight'
import Underline from '@tiptap/extension-underline'

interface Props {
    modelValue: string
    editable?: boolean
    showToolbar?: boolean
}

const props = withDefaults(defineProps<Props>(), {
    editable: false,
    showToolbar: false
})

// TipTap 编辑器实例 - 只读模式
const editor = useEditor({
    extensions: [
        StarterKit,
        TextAlign.configure({
            types: ['heading', 'paragraph'],
        }),
        TextStyle,
        Color,
        Highlight.configure({ multicolor: true }),
        Underline,
    ],
    content: props.modelValue || '',
    editable: props.editable,
    editorProps: {
        attributes: {
            class: 'prose prose-sm sm:prose lg:prose-lg xl:prose-xl max-w-none',
        },
    },
})

// 监听内容变化
watch(() => props.modelValue, (newValue) => {
    if (editor.value && newValue !== editor.value.getHTML()) {
        editor.value.commands.setContent(newValue)
    }
})

// 监听编辑模式变化
watch(() => props.editable, (newEditable) => {
    if (editor.value) {
        editor.value.setEditable(newEditable)
    }
})

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

/* 只读模式下的样式 */
:deep(.ProseMirror[contenteditable="false"]) {
    cursor: default;
    user-select: text;
}

:deep(.ProseMirror[contenteditable="false"]:focus) {
    outline: none;
}
</style>