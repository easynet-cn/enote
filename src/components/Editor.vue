<template>
  <div class="flex-1 flex flex-col overflow-hidden bg-white shadow-sm">
    <main class="flex-1 flex flex-col overflow-hidden p-4" role="main" aria-label="笔记编辑区">
      <!-- TipTap 编辑器工具栏（始终显示） -->
      <div v-if="activeNote">
        <TiptapToolbar
          :editor="editor ?? null"
          :source-mode="sourceMode"
          :content-type="activeNote?.contentType ?? ContentType.Html"
          :is-new-note="isNewNote"
          :edit-mode="editMode"
          :markdown-layout="markdownLayout"
          @toggle-source-mode="toggleSourceMode"
          @update:content-type="handleContentTypeChange"
          @update:markdown-layout="handleMarkdownLayoutChange"
          @edit="handleEdit"
          @save="handleSave"
          @cancel="handleCancel"
          @delete="handleDelete"
          @settings="handleSettings"
          @history="handleHistory"
        />
      </div>

      <!-- 标题输入区域 -->
      <div v-if="activeNote" class="flex items-center my-4">
        <div class="flex-1">
          <input
            ref="titleInput"
            :value="activeNote.title"
            @input="$emit('updateNoteTitle', ($event.target as HTMLInputElement).value)"
            placeholder="笔记标题"
            :readonly="!editMode"
            aria-label="笔记标题"
            class="w-full text-xl font-bold border-0 outline-none bg-transparent focus:ring-0"
            :class="{ 'cursor-default': !editMode }"
          />
        </div>
      </div>

      <!-- TipTap 编辑器 / Markdown 源码编辑器 -->
      <div v-if="activeNote" class="flex-1 overflow-hidden">
        <!-- Markdown 双面板布局 -->
        <div
          v-if="isMarkdownMode && markdownLayout !== MarkdownLayout.None && editMode"
          class="markdown-split-layout"
          :class="
            markdownLayout === MarkdownLayout.Vertical ? 'layout-vertical' : 'layout-horizontal'
          "
        >
          <!-- 源码编辑面板 -->
          <div class="split-panel source-panel" :style="splitPanelStyle">
            <div class="panel-header">源码</div>
            <textarea
              ref="sourcePanel"
              v-model="markdownSource"
              class="markdown-source-panel"
              placeholder="在此输入 Markdown 源码..."
              @input="handleSourceChange"
              @scroll="handleSourceScroll"
            />
          </div>

          <!-- 拖拽分隔条 -->
          <div
            class="split-resizer"
            :class="
              markdownLayout === MarkdownLayout.Vertical ? 'resizer-horizontal' : 'resizer-vertical'
            "
            @mousedown="startResize"
          />

          <!-- 预览面板 -->
          <div class="split-panel preview-panel">
            <div class="panel-header">预览</div>
            <div ref="previewPanel" class="markdown-preview-panel" @scroll="handlePreviewScroll">
              <editor-content :editor="editor" />
            </div>
          </div>
        </div>

        <!-- Markdown 单面板模式（源码或预览） -->
        <template v-else-if="isMarkdownMode">
          <textarea
            v-if="sourceMode && editMode"
            v-model="markdownSource"
            class="markdown-source"
            placeholder="在此输入 Markdown 源码..."
            @input="handleSourceChange"
          />
          <editor-content v-else :editor="editor" :class="editorCls" />
        </template>

        <!-- 富文本模式 -->
        <editor-content v-else :editor="editor" :class="editorCls" />
      </div>
    </main>
  </div>

  <!-- 设置弹窗 -->
  <Dialog v-model="settingDialog" title="设置" :width="500">
    <div class="space-y-4" role="form" aria-label="笔记设置">
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-2">笔记本</label>
        <Select
          v-model="settingForm.notebookId"
          :options="notebookOptions"
          placeholder="请选择笔记本"
          clearable
          filterable
        />
      </div>
      <div>
        <span class="block text-sm font-medium text-slate-700 mb-2" id="tags-label">标签</span>
        <div class="flex flex-wrap gap-2" role="group" aria-labelledby="tags-label">
          <label
            v-for="tag in tags.filter((t) => t.id !== '0')"
            :key="tag.id"
            class="tag-select-item"
            :class="{ 'tag-select-item-active': settingForm.tagIds.includes(tag.id) }"
          >
            <input
              type="checkbox"
              :value="tag.id"
              v-model="settingForm.tagIds"
              class="sr-only"
              :aria-label="tag.name"
            />
            <span class="tag-select-check">
              <Check class="w-3 h-3" />
            </span>
            <span :class="tag.cls" aria-hidden="true">●</span>
            <span class="text-sm">{{ tag.name }}</span>
          </label>
        </div>
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end gap-3">
        <Button type="secondary" @click="settingDialog = false">取消</Button>
        <Button type="primary" @click="handleSettingFormSubmit">保存</Button>
      </div>
    </template>
  </Dialog>

  <History
    v-model:visible="historyVisible"
    v-model:data="historyData"
    v-model:current-page="currentPage"
    v-model:page-size="pageSize"
    v-model:total="total"
    @open="$emit('open')"
    @size-change="handleSizeChange"
    @current-change="handleCurrentChange"
  />

  <!-- 删除笔记确认弹窗 -->
  <ConfirmDialog
    v-model="deleteNoteConfirm"
    title="删除笔记"
    message="确定要删除这篇笔记吗？此操作不可恢复。"
    type="danger"
    confirm-text="删除"
    @confirm="confirmDeleteNote"
  />
</template>

<script setup lang="ts">
import { watch, onBeforeUnmount, ref, reactive, computed } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import TextAlign from '@tiptap/extension-text-align'
import { TextStyle } from '@tiptap/extension-text-style'
import Color from '@tiptap/extension-color'
import Highlight from '@tiptap/extension-highlight'
import Underline from '@tiptap/extension-underline'
import Link from '@tiptap/extension-link'
import Image from '@tiptap/extension-image'
import { Table } from '@tiptap/extension-table'
import { TableRow } from '@tiptap/extension-table-row'
import { TableCell } from '@tiptap/extension-table-cell'
import { TableHeader } from '@tiptap/extension-table-header'
import FontFamily from '@tiptap/extension-font-family'
import TaskList from '@tiptap/extension-task-list'
import TaskItem from '@tiptap/extension-task-item'
import { Markdown } from 'tiptap-markdown'
import TiptapToolbar from './TiptapToolbar.vue'
import { Check } from 'lucide-vue-next'
import { Button, Select, Dialog, ConfirmDialog } from './ui'
import type { SelectOption } from './ui'
import { ContentType, MarkdownLayout } from '../types'
import type { NoteHistory, ShowNote, ShowNotebook, ShowTag } from '../types'
import { isTemporaryId } from '../utils/validation'
import History from './History.vue'

interface Props {
  notebooks: ShowNotebook[]
  tags: ShowTag[]
  activeNote: ShowNote | null
  editMode: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  saveNote: []
  cancelEdit: []
  deleteNote: []
  toggleEditMode: []
  updateNoteTitle: [title: string]
  updateNoteContent: [content: string]
  updateNoteContentType: [contentType: ContentType]
  sizeChange: [pageSize: number]
  currentChange: [currentPage: number]
  open: []
}>()

interface Setting {
  notebookId: string
  tagIds: string[]
}

const settingForm = reactive<Setting>({
  notebookId: '',
  tagIds: [],
})

// 判断是否为新建笔记
const isNewNote = computed(() => isTemporaryId(props.activeNote?.id))

// 笔记本选项（过滤掉"全部"）
const notebookOptions = computed<SelectOption[]>(() => {
  return props.notebooks
    .filter((n) => n.id !== '0')
    .map((n) => ({
      label: n.name || '',
      value: n.id,
    }))
})

const settingDialog = ref(false)
const deleteNoteConfirm = ref(false)
const historyData = defineModel<NoteHistory[]>('historyData')
const currentPage = defineModel<number>('currentPage')
const pageSize = defineModel<number>('pageSize')
const total = defineModel<number>('total')

const historyVisible = ref<boolean>(false)

// Markdown 源码模式
const sourceMode = ref(false)
const markdownSource = ref('')

// Markdown 布局模式
const markdownLayout = ref<MarkdownLayout>(MarkdownLayout.None)
const splitRatio = ref(50) // 分割比例，默认 50%
const isResizing = ref(false)

// 滚动同步相关
const sourcePanel = ref<HTMLTextAreaElement | null>(null)
const previewPanel = ref<HTMLDivElement | null>(null)
const isScrollSyncing = ref(false) // 标志位，避免滚动循环触发

// 标题输入框引用
const titleInput = ref<HTMLInputElement | null>(null)

// 是否为 Markdown 模式
const isMarkdownMode = computed(() => props.activeNote?.contentType === ContentType.Markdown)

// 分割面板样式
const splitPanelStyle = computed(() => {
  if (markdownLayout.value === MarkdownLayout.Vertical) {
    return { height: `${splitRatio.value}%` }
  } else {
    return { width: `${splitRatio.value}%` }
  }
})

// TipTap 编辑器实例
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
    Link.configure({
      openOnClick: false,
      HTMLAttributes: {
        class: 'text-blue-500 underline cursor-pointer',
      },
    }),
    Image.configure({
      inline: true,
      allowBase64: true,
    }),
    Table.configure({
      resizable: true,
    }),
    TableRow,
    TableCell,
    TableHeader,
    FontFamily,
    TaskList,
    TaskItem.configure({
      nested: true,
    }),
    Markdown.configure({
      html: true, // 允许解析 HTML
      tightLists: true, // 紧凑列表
      tightListClass: 'tight', // 紧凑列表 CSS 类
      bulletListMarker: '-', // 无序列表标记
      linkify: true, // 自动转换链接
      breaks: true, // 换行符转为 <br>
      transformPastedText: true, // 粘贴时转换 Markdown
      transformCopiedText: true, // 复制时转换为 Markdown
    }),
  ],
  content: props.activeNote?.content || '',
  editable: false,
  onUpdate: ({ editor }) => {
    // 根据内容类型决定保存格式
    const contentType = props.activeNote?.contentType ?? ContentType.Html
    if (contentType === ContentType.Markdown) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const storage = editor.storage as any
      emit('updateNoteContent', storage.markdown.getMarkdown())
    } else {
      emit('updateNoteContent', editor.getHTML())
    }
  },
})

// 监听活动笔记变化
watch(
  () => props.activeNote,
  (newNote) => {
    if (editor.value && newNote) {
      editor.value.commands.setContent(newNote.content)

      settingForm.notebookId = props.activeNote?.notebookId ?? ''
      settingForm.tagIds = props.activeNote?.tags?.map((t) => t.id) ?? []

      // 重置源码模式和布局
      sourceMode.value = false
      markdownLayout.value = MarkdownLayout.None
      splitRatio.value = 50
    }
  },
)

// 监听编辑模式变化
watch(
  () => props.editMode,
  (newMode) => {
    if (editor.value && newMode) {
      editor.value?.setEditable(true)
      // 切换到编辑模式时，根据是否为新建笔记决定焦点位置
      setTimeout(() => {
        if (isNewNote.value) {
          // 新建笔记：焦点在标题
          titleInput.value?.focus()
        } else {
          // 编辑笔记：焦点在内容
          editor.value?.commands.focus()
        }
      }, 100)
    } else {
      editor.value?.setEditable(false)
    }
  },
)

const editorCls = computed(() => {
  return props.editMode ? 'tiptap-editor-edit' : 'tiptap-editor'
})

// 组件卸载时销毁编辑器
onBeforeUnmount(() => {
  if (editor.value) {
    editor.value.destroy()
  }
})

// 工具栏事件处理
const handleEdit = () => {
  emit('toggleEditMode')
}

const handleSave = () => {
  emit('saveNote')
}

const handleCancel = () => {
  emit('cancelEdit')
}

const handleDelete = () => {
  deleteNoteConfirm.value = true
}

const handleSettings = () => {
  settingDialog.value = true
}

const handleHistory = () => {
  historyVisible.value = true
}

const handleSizeChange = (val: number) => {
  emit('sizeChange', val)
}

const handleCurrentChange = (val: number) => {
  emit('currentChange', val)
}

const handleSettingFormSubmit = () => {
  if (props.activeNote) {
    props.activeNote.notebookId = settingForm.notebookId
    props.activeNote.tags = props.tags.filter((t) => settingForm.tagIds.includes(t.id))
  }

  settingDialog.value = false
}

const confirmDeleteNote = () => {
  emit('deleteNote')
}

// 处理内容类型变化（仅新建笔记时可用）
const handleContentTypeChange = (contentType: ContentType) => {
  if (props.activeNote && isNewNote.value) {
    props.activeNote.contentType = contentType
    emit('updateNoteContentType', contentType)
  }
}

// 处理 Markdown 布局变化
const handleMarkdownLayoutChange = (layout: MarkdownLayout) => {
  // 取消布局后再打开，重置为初始状态（50%）
  if (layout !== MarkdownLayout.None && markdownLayout.value === MarkdownLayout.None) {
    splitRatio.value = 50
  }

  markdownLayout.value = layout

  // 切换到双面板布局时，同步源码
  if (layout !== MarkdownLayout.None && editor.value) {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const storage = editor.value.storage as any
    markdownSource.value = storage.markdown.getMarkdown()
  }
}

// 拖拽调整面板大小
const startResize = (e: MouseEvent) => {
  isResizing.value = true
  const startPos = markdownLayout.value === MarkdownLayout.Vertical ? e.clientY : e.clientX
  const startRatio = splitRatio.value

  const onMouseMove = (moveEvent: MouseEvent) => {
    if (!isResizing.value) return

    const container = (moveEvent.target as HTMLElement).closest('.markdown-split-layout')
    if (!container) return

    const rect = container.getBoundingClientRect()
    const currentPos =
      markdownLayout.value === MarkdownLayout.Vertical ? moveEvent.clientY : moveEvent.clientX
    const containerSize =
      markdownLayout.value === MarkdownLayout.Vertical ? rect.height : rect.width

    const delta = currentPos - startPos
    const deltaRatio = (delta / containerSize) * 100

    let newRatio = startRatio + deltaRatio

    // 限制最小和最大比例
    newRatio = Math.max(20, Math.min(80, newRatio))
    splitRatio.value = newRatio
  }

  const onMouseUp = () => {
    isResizing.value = false
    document.removeEventListener('mousemove', onMouseMove)
    document.removeEventListener('mouseup', onMouseUp)
    document.body.style.cursor = ''
    document.body.style.userSelect = ''
  }

  document.addEventListener('mousemove', onMouseMove)
  document.addEventListener('mouseup', onMouseUp)
  document.body.style.cursor =
    markdownLayout.value === MarkdownLayout.Vertical ? 'row-resize' : 'col-resize'
  document.body.style.userSelect = 'none'
}

// 切换源码模式
const toggleSourceMode = () => {
  if (!editor.value) return

  if (sourceMode.value) {
    // 从源码模式切换到预览模式：将 Markdown 转换为 HTML
    editor.value.commands.setContent(markdownSource.value)
  } else {
    // 从预览模式切换到源码模式：获取 Markdown
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const storage = editor.value.storage as any
    markdownSource.value = storage.markdown.getMarkdown()
  }
  sourceMode.value = !sourceMode.value
}

// 预处理Markdown内容，保留多个连续空行
const preprocessMarkdown = (content: string): string => {
  // 将连续的空行（3个及以上换行）转换为带 <br> 的格式，保留空行效果
  return content.replace(/\n{3,}/g, (match) => {
    // 每个额外的换行转换为 <br>
    const extraLines = match.length - 2
    return '\n\n' + '<br>\n'.repeat(extraLines)
  })
}

// 处理源码内容变化
const handleSourceChange = () => {
  // 根据内容类型决定保存格式
  const contentType = props.activeNote?.contentType ?? ContentType.Html
  if (contentType === ContentType.Markdown) {
    // Markdown 模式直接保存源码
    emit('updateNoteContent', markdownSource.value)

    // 如果在双面板布局下，同步更新预览
    if (markdownLayout.value !== MarkdownLayout.None && editor.value) {
      // 预处理内容以保留多个空行
      const processedContent = preprocessMarkdown(markdownSource.value)
      editor.value.commands.setContent(processedContent)
    }
  } else {
    // HTML 模式需要先同步到编辑器再获取 HTML
    if (editor.value) {
      editor.value.commands.setContent(markdownSource.value)
      emit('updateNoteContent', editor.value.getHTML())
    }
  }
}

// 滚动同步：源码面板滚动时同步预览面板
const handleSourceScroll = () => {
  if (isScrollSyncing.value || !sourcePanel.value || !previewPanel.value) return

  isScrollSyncing.value = true

  const source = sourcePanel.value
  const preview = previewPanel.value

  // 计算滚动比例
  const scrollRatio = source.scrollTop / (source.scrollHeight - source.clientHeight || 1)

  // 应用到预览面板
  preview.scrollTop = scrollRatio * (preview.scrollHeight - preview.clientHeight)

  // 重置标志位
  requestAnimationFrame(() => {
    isScrollSyncing.value = false
  })
}

// 滚动同步：预览面板滚动时同步源码面板
const handlePreviewScroll = () => {
  if (isScrollSyncing.value || !sourcePanel.value || !previewPanel.value) return

  isScrollSyncing.value = true

  const source = sourcePanel.value
  const preview = previewPanel.value

  // 计算滚动比例
  const scrollRatio = preview.scrollTop / (preview.scrollHeight - preview.clientHeight || 1)

  // 应用到源码面板
  source.scrollTop = scrollRatio * (source.scrollHeight - source.clientHeight)

  // 重置标志位
  requestAnimationFrame(() => {
    isScrollSyncing.value = false
  })
}
</script>

<style scoped>
.tiptap-editor {
  padding: 1.5rem;
  min-height: 500px;
  max-height: 92vh;
  overflow-y: auto;
}

.tiptap-editor-edit {
  padding: 1.5rem;
  min-height: 500px;
  max-height: 88vh;
  overflow-y: auto;
}

.tiptap-editor:focus {
  outline: none;
}

/* 为ProseMirror内容区域设置基本样式 */
:deep(.ProseMirror) {
  outline: none;
  line-height: 1.6;
  min-height: 100%;
}

/* 占位符样式 */
:deep(.ProseMirror p.is-editor-empty:first-child::before) {
  content: attr(data-placeholder);
  float: left;
  color: #cbd5e1;
  pointer-events: none;
  height: 0;
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
  min-height: 1.6em; /* 确保空段落也有高度 */
}

:deep(.ProseMirror p:empty::before) {
  content: '\00a0'; /* 空段落显示不可见空格，保持高度 */
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
  border-left: 4px solid #4f46e5;
  padding-left: 1rem;
  margin-left: 0;
  margin-right: 0;
  font-style: italic;
  margin-bottom: 0.75rem;
  color: #64748b;
}

:deep(.ProseMirror code) {
  background: #f1f5f9;
  padding: 0.2rem 0.4rem;
  border-radius: 0.25rem;
  font-family:
    ui-monospace, SFMono-Regular, 'SF Mono', Consolas, 'Liberation Mono', Menlo, monospace;
  font-size: 0.9em;
}

:deep(.ProseMirror pre) {
  background: #1e293b;
  color: #f8fafc;
  padding: 1rem;
  border-radius: 0.75rem;
  font-family:
    ui-monospace, SFMono-Regular, 'SF Mono', Consolas, 'Liberation Mono', Menlo, monospace;
  overflow-x: auto;
  margin: 1rem 0;
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
  table-layout: fixed;
  width: 100%;
  margin: 1rem 0;
  overflow: hidden;
}

:deep(.ProseMirror th),
:deep(.ProseMirror td) {
  min-width: 1em;
  border: 1px solid #e2e8f0;
  padding: 6px 10px;
  vertical-align: top;
  box-sizing: border-box;
  position: relative;
}

:deep(.ProseMirror th) {
  background-color: #f8fafc;
  font-weight: bold;
  text-align: left;
}

:deep(.ProseMirror img) {
  max-width: 100%;
  height: auto;
  border-radius: 0.75rem;
  margin: 1rem 0;
  display: block;
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
  user-select: none;
  margin-right: 0.75rem;
  margin-top: 0.25rem;
}

:deep(.ProseMirror ul[data-type='taskList'] li > label input[type='checkbox']) {
  width: 1.1rem;
  height: 1.1rem;
  cursor: pointer;
  accent-color: #4f46e5;
}

:deep(.ProseMirror ul[data-type='taskList'] li > div) {
  flex: 1;
}

:deep(.ProseMirror ul[data-type='taskList'] li[data-checked='true'] > div) {
  text-decoration: line-through;
  color: #9ca3af;
}

/* 表格选中样式 */
:deep(.ProseMirror .selectedCell) {
  background-color: #dbeafe;
}

:deep(.ProseMirror .column-resize-handle) {
  background-color: #3b82f6;
  width: 4px;
  cursor: col-resize;
}

:deep(.ProseMirror .tableWrapper) {
  overflow-x: auto;
  margin-bottom: 0.75rem;
}

/* 链接悬停样式 */
:deep(.ProseMirror a:hover) {
  color: #2563eb;
}

/* Markdown 源码编辑器样式 */
.markdown-source {
  width: 100%;
  height: 100%;
  min-height: 500px;
  max-height: 88vh;
  padding: 1.5rem;
  border: none;
  outline: none;
  resize: none;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.6;
  background-color: #1e1e1e;
  color: #d4d4d4;
  overflow-y: auto;
}

.markdown-source::placeholder {
  color: #6b7280;
}

.markdown-source:focus {
  outline: none;
}

/* 标签选择样式 */
.tag-select-item {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border: 2px solid #e2e8f0;
  border-radius: 20px;
  cursor: pointer;
  transition: all 0.2s ease;
  background: white;
  user-select: none;
}

.tag-select-item:hover {
  border-color: #cbd5e1;
  background: #f8fafc;
}

.tag-select-item-active {
  border-color: #4f46e5;
  background: linear-gradient(135deg, #eef2ff 0%, #e0e7ff 100%);
  box-shadow: 0 2px 8px rgba(79, 70, 229, 0.25);
}

.tag-select-item-active:hover {
  border-color: #4338ca;
  background: linear-gradient(135deg, #e0e7ff 0%, #c7d2fe 100%);
}

.tag-select-check {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  border: 2px solid #cbd5e1;
  background: white;
  color: transparent;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.tag-select-item-active .tag-select-check {
  border-color: #4f46e5;
  background: #4f46e5;
  color: white;
}

/* Markdown 分割布局样式 */
.markdown-split-layout {
  display: flex;
  height: 100%;
  width: 100%;
  overflow: hidden;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
}

.layout-vertical {
  flex-direction: column;
}

.layout-horizontal {
  flex-direction: row;
}

.split-panel {
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
  min-height: 0;
}

.source-panel {
  flex-shrink: 0;
}

.preview-panel {
  flex: 1;
  min-width: 0;
  min-height: 0;
}

.panel-header {
  padding: 0.5rem 1rem;
  font-size: 0.75rem;
  font-weight: 600;
  color: #64748b;
  background-color: #f8fafc;
  border-bottom: 1px solid #e2e8f0;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.markdown-source-panel {
  flex: 1;
  width: 100%;
  padding: 1rem;
  border: none;
  outline: none;
  resize: none;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.6;
  background-color: #1e1e1e;
  color: #d4d4d4;
  overflow-y: auto;
  height: 0; /* 关键：让 flex 子元素可以缩小，使 overflow-y: auto 生效 */
  min-height: 0;
}

.markdown-source-panel::placeholder {
  color: #6b7280;
}

.markdown-preview-panel {
  flex: 1;
  padding: 1rem;
  overflow-y: auto;
  background-color: #fff;
  height: 0; /* 关键：让 flex 子元素可以缩小，使 overflow-y: auto 生效 */
  min-height: 0;
}

/* 分割条样式 */
.split-resizer {
  flex-shrink: 0;
  background-color: #e2e8f0;
  transition: background-color 0.15s ease;
}

.split-resizer:hover {
  background-color: #4f46e5;
}

.resizer-horizontal {
  height: 6px;
  cursor: row-resize;
}

.resizer-vertical {
  width: 6px;
  cursor: col-resize;
}

/* 上下布局时的边框调整 */
.layout-vertical .source-panel {
  border-bottom: none;
}

.layout-vertical .preview-panel {
  border-top: none;
}

/* 左右布局时的边框调整 */
.layout-horizontal .source-panel {
  border-right: none;
}

.layout-horizontal .preview-panel {
  border-left: none;
}

/* 预览面板中的 ProseMirror 样式继承 */
.markdown-preview-panel :deep(.ProseMirror) {
  outline: none;
  line-height: 1.6;
  min-height: 100%;
}
</style>
