<template>
  <div class="flex-1 h-full flex flex-col overflow-hidden bg-surface shadow-sm">
    <main
      class="flex-1 flex flex-col min-h-0 overflow-hidden px-4 pt-4 pb-2"
      role="main"
      :aria-label="t('aria.noteEditor')"
    >
      <!-- TipTap 编辑器工具栏（始终显示） -->
      <div v-if="activeNote">
        <TiptapToolbar
          :editor="editor ?? null"
          :source-mode="sourceMode"
          :content-type="activeNote?.contentType ?? ContentType.Html"
          :is-new-note="isNewNote"
          :edit-mode="editMode"
          :markdown-layout="markdownLayout"
          :toc-visible="tocVisible"
          @toggle-source-mode="toggleSourceMode"
          @update:content-type="handleContentTypeChange"
          @update:markdown-layout="handleMarkdownLayoutChange"
          @toggle-toc="toggleToc"
          @edit="handleEdit"
          @save="handleSave"
          @cancel="handleCancel"
          @delete="handleDelete"
          @settings="handleSettings"
          @export="handleExport"
          @history="handleHistory"
        />
      </div>

      <!-- 标题输入区域 -->
      <div v-if="activeNote" class="flex items-center mt-4 mb-2">
        <div class="flex-1">
          <input
            ref="titleInput"
            :value="activeNote.title"
            @input="$emit('updateNoteTitle', ($event.target as HTMLInputElement).value)"
            :placeholder="t('editor.noteTitlePlaceholder')"
            :readonly="!editMode"
            :aria-label="t('editor.noteTitle')"
            class="w-full text-xl font-bold border-0 outline-none bg-transparent focus:ring-0"
            :class="{ 'cursor-default': !editMode }"
          />
        </div>
      </div>

      <!-- TipTap 编辑器 / Markdown 源码编辑器 -->
      <div v-if="activeNote" class="relative flex-1">
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
            <div class="panel-header">{{ t('editor.markdownEditor.source') }}</div>
            <textarea
              ref="sourcePanel"
              v-model="markdownSource"
              class="markdown-source-panel"
              :placeholder="t('editor.editorDialog.markdownSource')"
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
            <div class="panel-header">{{ t('editor.markdownEditor.preview') }}</div>
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
            :placeholder="t('editor.editorDialog.markdownSource')"
            @input="handleSourceChange"
          />
          <editor-content
            v-else
            :editor="editor"
            :class="editMode ? 'tiptap-editor-edit' : 'tiptap-editor'"
          />
        </template>

        <!-- 富文本模式 -->
        <editor-content
          v-else
          :editor="editor"
          :class="editMode ? 'tiptap-editor-edit' : 'tiptap-editor'"
        />

        <!-- 目录面板 -->
        <TableOfContents
          :items="tocItems"
          :visible="tocVisible"
          :editor="editor ?? null"
          @close="tocVisible = false"
        />
      </div>
    </main>

    <!-- 设置弹窗 -->
    <EditorSettingsDialog
      v-model="settingDialog"
      :notebooks="notebooks"
      :tags="tags"
      :notebook-id="activeNote?.notebookId ?? ''"
      :selected-tag-ids="activeNote?.tags?.map((t) => t.id) ?? []"
      @save="handleSettingFormSubmit"
    />

    <!-- 导出对话框 -->
    <ExportDialog v-model="exportDialog" :note="activeNote" />

    <NoteHistoryDialog
      v-model:visible="historyVisible"
      v-model:data="historyData"
      v-model:current-page="currentPage"
      v-model:page-size="pageSize"
      v-model:total="total"
      :loading="historyLoading"
      @open="$emit('open')"
      @size-change="handleSizeChange"
      @current-change="handleCurrentChange"
    />

    <!-- 删除笔记确认弹窗 -->
    <ConfirmDialog
      v-model="deleteNoteConfirm"
      :title="t('editor.deleteNoteConfirm.title')"
      :message="t('editor.deleteNoteConfirm.message')"
      type="danger"
      :confirm-text="t('editor.deleteNoteConfirm.confirmText')"
      @confirm="confirmDeleteNote"
    />

    <!-- 关联笔记面板 -->
    <NoteLinkPanel
      v-if="activeNote && !isNewNote"
      :note-id="Number(activeNote.id)"
      @navigate-to-note="(id: number) => $emit('navigateToNote', id)"
    />

    <!-- 底部状态栏 -->
    <EditorStatusBar v-if="activeNote && editor" :editor="editor" />
  </div>
</template>

<script setup lang="ts">
import { watch, onBeforeUnmount, ref, computed, shallowRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { Editor, EditorContent } from '@tiptap/vue-3'
import { getMarkdownExtensions, getRichTextExtensions } from '../config/editorExtensions'
import TiptapToolbar from './TiptapToolbar.vue'
import EditorSettingsDialog from './EditorSettingsDialog.vue'
import ExportDialog from './ExportDialog.vue'
import { ConfirmDialog } from './ui'
import { ContentType, MarkdownLayout } from '../types'
import type { NoteHistory, ShowNote, ShowNotebook, ShowTag } from '../types'
import { getMarkdownFromEditor } from '../types/tiptap-markdown'
import { isTemporaryId } from '../utils/validation'
import { throttle } from '../utils/debounce'
import {
  MARKDOWN_PREVIEW_THROTTLE,
  SCROLL_SYNC_THROTTLE,
  SPLIT_PANEL_MIN_RATIO,
  SPLIT_PANEL_MAX_RATIO,
} from '../config/constants'
import { preprocessMarkdown } from '../utils/markdownWorker'
import NoteHistoryDialog from './NoteHistoryDialog.vue'
import TableOfContents from './TableOfContents.vue'
import EditorStatusBar from './EditorStatusBar.vue'
import NoteLinkPanel from './NoteLinkPanel.vue'
import type { TocItem } from '../extensions'

interface Props {
  notebooks: ShowNotebook[]
  tags: ShowTag[]
  activeNote: ShowNote | null
  editMode: boolean
  historyLoading?: boolean
}

const { t } = useI18n()

const props = defineProps<Props>()

const emit = defineEmits<{
  saveNote: []
  cancelEdit: []
  deleteNote: []
  toggleEditMode: []
  updateNoteTitle: [title: string]
  updateNoteContent: [content: string]
  updateNoteContentType: [contentType: ContentType]
  updateNoteSetting: [notebookId: string, tagIds: string[]]
  sizeChange: [pageSize: number]
  currentChange: [currentPage: number]
  open: []
  navigateToNote: [id: number]
}>()

// 判断是否为新建笔记
const isNewNote = computed(() => isTemporaryId(props.activeNote?.id))

const settingDialog = ref(false)
const exportDialog = ref(false)
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

// 滚动同步相关 - 使用状态机管理滚动方向
const sourcePanel = ref<HTMLTextAreaElement | null>(null)
const previewPanel = ref<HTMLDivElement | null>(null)
// 滚动状态：'idle' | 'source' | 'preview'
// idle: 无滚动同步进行中
// source: 源码面板主导滚动，预览面板被动跟随
// preview: 预览面板主导滚动，源码面板被动跟随
const scrollState = ref<'idle' | 'source' | 'preview'>('idle')
let scrollResetTimer: ReturnType<typeof setTimeout> | null = null

// 标题输入框引用
const titleInput = ref<HTMLInputElement | null>(null)

// Timer refs for cleanup
let editModeFocusTimer: ReturnType<typeof setTimeout> | null = null

// Resize cleanup refs
let resizeCleanup: (() => void) | null = null

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

// TipTap 编辑器实例（根据内容类型动态选择扩展）
const editor = shallowRef<Editor | undefined>(undefined)

// 创建编辑器实例
const createEditor = (contentType: ContentType, content: string) => {
  // 销毁旧编辑器
  if (editor.value) {
    editor.value.destroy()
  }

  // 根据内容类型选择扩展：Markdown 模式使用 Markdown 扩展，富文本模式不使用
  const extensions =
    contentType === ContentType.Markdown ? getMarkdownExtensions() : getRichTextExtensions()

  editor.value = new Editor({
    extensions,
    content,
    editable: false,
    editorProps: {
      attributes: {
        // 禁用浏览器自动大写
        autocapitalize: 'off',
        autocorrect: 'off',
        spellcheck: 'false',
      },
    },
    onUpdate: ({ editor: ed }) => {
      // 根据内容类型决定保存格式
      if (contentType === ContentType.Markdown) {
        emit('updateNoteContent', getMarkdownFromEditor(ed))
      } else {
        emit('updateNoteContent', ed.getHTML())
      }
      updateTocItems()
    },
    onCreate: () => {
      updateTocItems()
    },
  })
}

// 初始化编辑器
if (props.activeNote) {
  createEditor(props.activeNote.contentType ?? ContentType.Html, props.activeNote.content)
}

// 监听活动笔记变化
watch(
  () => props.activeNote,
  (newNote, oldNote) => {
    if (newNote) {
      // 编辑模式下，如果只是内容变化（同一笔记），不重置编辑器
      // 避免输入时因响应式更新导致光标跳转
      if (props.editMode && oldNote && newNote.id === oldNote.id) {
        return
      }

      // 检查内容类型是否变化，需要重新创建编辑器
      const newContentType = newNote.contentType ?? ContentType.Html
      const oldContentType = oldNote?.contentType ?? ContentType.Html
      const contentTypeChanged = newContentType !== oldContentType

      if (contentTypeChanged || !editor.value) {
        // 内容类型变化或编辑器不存在，重新创建
        createEditor(newContentType, newNote.content)
      } else {
        // 仅内容变化，直接设置内容
        editor.value.commands.setContent(newNote.content)
      }

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
      if (editModeFocusTimer) clearTimeout(editModeFocusTimer)
      editModeFocusTimer = setTimeout(() => {
        editModeFocusTimer = null
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

// 目录
const tocVisible = ref(false)
const tocItems = ref<TocItem[]>([])

// 切换目录显示
const toggleToc = () => {
  tocVisible.value = !tocVisible.value
}

// 更新目录项
const updateTocItems = () => {
  if (!editor.value) return
  const storage = (editor.value.storage as unknown as Record<string, unknown>).tableOfContents as
    | { items: typeof tocItems.value }
    | undefined
  if (storage) {
    tocItems.value = storage.items
  }
}

// 组件卸载时销毁编辑器和清理定时器
onBeforeUnmount(() => {
  if (editor.value) {
    editor.value.destroy()
  }
  if (editModeFocusTimer) {
    clearTimeout(editModeFocusTimer)
    editModeFocusTimer = null
  }
  if (scrollResetTimer) {
    clearTimeout(scrollResetTimer)
    scrollResetTimer = null
  }
  if (resizeCleanup) {
    resizeCleanup()
    resizeCleanup = null
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

const handleExport = () => {
  exportDialog.value = true
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

const handleSettingFormSubmit = (notebookId: string, tagIds: string[]) => {
  if (props.activeNote) {
    emit('updateNoteSetting', notebookId, tagIds)
  }
}

const confirmDeleteNote = () => {
  emit('deleteNote')
}

// 处理内容类型变化（仅新建笔记时可用）
const handleContentTypeChange = (contentType: ContentType) => {
  if (props.activeNote && isNewNote.value) {
    const oldContentType = props.activeNote.contentType
    emit('updateNoteContentType', contentType)

    // 内容类型变化时，重新创建编辑器以使用正确的扩展配置
    if (oldContentType !== contentType) {
      createEditor(contentType, props.activeNote.content)
      // 重新设置可编辑状态
      if (props.editMode) {
        editor.value?.setEditable(true)
      }
    }
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
    markdownSource.value = getMarkdownFromEditor(editor.value)
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
    newRatio = Math.max(SPLIT_PANEL_MIN_RATIO, Math.min(SPLIT_PANEL_MAX_RATIO, newRatio))
    splitRatio.value = newRatio
  }

  const onMouseUp = () => {
    isResizing.value = false
    document.removeEventListener('mousemove', onMouseMove)
    document.removeEventListener('mouseup', onMouseUp)
    document.body.style.cursor = ''
    document.body.style.userSelect = ''
    resizeCleanup = null
  }

  document.addEventListener('mousemove', onMouseMove)
  document.addEventListener('mouseup', onMouseUp)
  document.body.style.cursor =
    markdownLayout.value === MarkdownLayout.Vertical ? 'row-resize' : 'col-resize'
  document.body.style.userSelect = 'none'

  // Store cleanup function for onBeforeUnmount
  resizeCleanup = () => {
    document.removeEventListener('mousemove', onMouseMove)
    document.removeEventListener('mouseup', onMouseUp)
    document.body.style.cursor = ''
    document.body.style.userSelect = ''
  }
}

// 切换源码模式
const toggleSourceMode = () => {
  if (!editor.value) return

  if (sourceMode.value) {
    // 从源码模式切换到预览模式：将 Markdown 转换为 HTML
    editor.value.commands.setContent(markdownSource.value)
  } else {
    // 从预览模式切换到源码模式：获取 Markdown
    markdownSource.value = getMarkdownFromEditor(editor.value)
  }
  sourceMode.value = !sourceMode.value
}

// 节流的预览更新函数（100ms 间隔，提升大文件编辑性能）
// 使用 Web Worker 处理大文件的 Markdown 预处理
const throttledUpdatePreview = throttle(async (content: string) => {
  if (editor.value) {
    const processedContent = await preprocessMarkdown(content)
    editor.value.commands.setContent(processedContent)
  }
}, MARKDOWN_PREVIEW_THROTTLE)

// 处理源码内容变化
const handleSourceChange = () => {
  // 根据内容类型决定保存格式
  const contentType = props.activeNote?.contentType ?? ContentType.Html
  if (contentType === ContentType.Markdown) {
    // Markdown 模式直接保存源码
    emit('updateNoteContent', markdownSource.value)

    // 如果在双面板布局下，使用节流更新预览（提升性能）
    if (markdownLayout.value !== MarkdownLayout.None) {
      throttledUpdatePreview(markdownSource.value)
    }
  } else {
    // HTML 模式需要先同步到编辑器再获取 HTML
    if (editor.value) {
      editor.value.commands.setContent(markdownSource.value)
      emit('updateNoteContent', editor.value.getHTML())
    }
  }
}

// 重置滚动状态（延迟执行，避免频繁切换）
const resetScrollState = () => {
  if (scrollResetTimer) {
    clearTimeout(scrollResetTimer)
  }
  scrollResetTimer = setTimeout(() => {
    scrollState.value = 'idle'
  }, 150) // 150ms 后重置为 idle 状态
}

// 滚动同步核心逻辑：from 面板滚动时同步 to 面板
const syncScroll = (from: 'source' | 'preview') => {
  if (!sourcePanel.value || !previewPanel.value) return

  const opposite = from === 'source' ? 'preview' : 'source'
  if (scrollState.value === opposite) return

  scrollState.value = from

  const fromEl = from === 'source' ? sourcePanel.value : previewPanel.value
  const toEl = from === 'source' ? previewPanel.value : sourcePanel.value

  const maxFromScroll = fromEl.scrollHeight - fromEl.clientHeight
  if (maxFromScroll <= 0) return

  const scrollRatio = fromEl.scrollTop / maxFromScroll
  const maxToScroll = toEl.scrollHeight - toEl.clientHeight
  toEl.scrollTop = scrollRatio * maxToScroll

  resetScrollState()
}

// 节流的滚动同步处理（16ms ≈ 60fps，确保流畅体验）
const handleSourceScroll = throttle(() => syncScroll('source'), SCROLL_SYNC_THROTTLE)
const handlePreviewScroll = throttle(() => syncScroll('preview'), SCROLL_SYNC_THROTTLE)
</script>

<!-- 共享 ProseMirror 样式 -->
<style src="../styles/prosemirror.css"></style>

<style scoped>
.tiptap-editor,
.tiptap-editor-edit {
  position: absolute;
  inset: 0;
  padding: 1rem 1.5rem;
  overflow-y: auto;
}

.tiptap-editor:focus {
  outline: none;
}

/* 占位符样式 */
:deep(.ProseMirror p.is-editor-empty:first-child::before) {
  content: attr(data-placeholder);
  float: left;
  color: var(--color-text-disabled);
  pointer-events: none;
  height: 0;
}

/* NoteEditor 特有覆盖样式 */
:deep(.ProseMirror p) {
  min-height: 1.6em;
}

:deep(.ProseMirror p:empty::before) {
  content: '\00a0';
}

:deep(.ProseMirror blockquote) {
  border-left-color: var(--color-primary);
  color: var(--color-text-secondary);
}

:deep(.ProseMirror code) {
  background: var(--color-bg-tertiary);
  padding: 0.2rem 0.4rem;
  font-family:
    ui-monospace, SFMono-Regular, 'SF Mono', Consolas, 'Liberation Mono', Menlo, monospace;
  font-size: 0.9em;
}

:deep(.ProseMirror pre) {
  background: var(--color-code-block-bg);
  color: var(--color-code-block-text);
  padding: 1rem;
  border-radius: 0.75rem;
  font-family:
    ui-monospace, SFMono-Regular, 'SF Mono', Consolas, 'Liberation Mono', Menlo, monospace;
  margin: 1rem 0;
}

:deep(.ProseMirror pre code) {
  background: transparent;
  color: inherit;
  padding: 0;
  border-radius: 0;
  font-size: 0.875rem;
}

:deep(.ProseMirror table) {
  table-layout: fixed;
  margin: 1rem 0;
  overflow: hidden;
}

:deep(.ProseMirror th),
:deep(.ProseMirror td) {
  min-width: 1em;
  border-color: var(--color-border);
  padding: 6px 10px;
  vertical-align: top;
  box-sizing: border-box;
  position: relative;
}

:deep(.ProseMirror th) {
  background-color: var(--color-bg-secondary);
  font-weight: bold;
}

:deep(.ProseMirror img) {
  border-radius: 0.75rem;
  margin: 1rem 0;
  display: block;
}

/* 任务列表覆盖样式 */
:deep(.ProseMirror ul[data-type='taskList'] li > label) {
  user-select: none;
  margin-right: 0.75rem;
}

:deep(.ProseMirror ul[data-type='taskList'] li > label input[type='checkbox']) {
  width: 1.1rem;
  height: 1.1rem;
  accent-color: var(--color-primary);
}

/* 表格选中样式 */
:deep(.ProseMirror .selectedCell) {
  background-color: var(--color-info-light);
}

:deep(.ProseMirror .column-resize-handle) {
  background-color: var(--color-info);
  width: 4px;
  cursor: col-resize;
}

:deep(.ProseMirror .tableWrapper) {
  overflow-x: auto;
  margin-bottom: 0.75rem;
}

/* Markdown 源码编辑器样式 */
.markdown-source {
  width: 100%;
  height: 100%;
  padding: 1.5rem;
  border: none;
  outline: none;
  resize: none;
  font-family: var(--font-mono);
  font-size: 14px;
  line-height: 1.6;
  background-color: var(--color-code-bg);
  color: var(--color-code-text);
  overflow-y: auto;
}

.markdown-source::placeholder {
  color: var(--color-code-placeholder);
}

.markdown-source:focus {
  outline: none;
}

/* Markdown 分割布局样式 */
.markdown-split-layout {
  display: flex;
  height: 100%;
  width: 100%;
  overflow: hidden;
  border: 1px solid var(--color-border);
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
  color: var(--color-text-secondary);
  background-color: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border);
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
  font-family: var(--font-mono);
  font-size: 14px;
  line-height: 1.6;
  background-color: var(--color-code-bg);
  color: var(--color-code-text);
  overflow-y: auto;
  height: 0; /* 关键：让 flex 子元素可以缩小，使 overflow-y: auto 生效 */
  min-height: 0;
}

.markdown-source-panel::placeholder {
  color: var(--color-code-placeholder);
}

.markdown-preview-panel {
  flex: 1;
  padding: 1rem;
  overflow-y: auto;
  background-color: var(--color-bg-primary);
  height: 0; /* 关键：让 flex 子元素可以缩小，使 overflow-y: auto 生效 */
  min-height: 0;
}

/* 分割条样式 */
.split-resizer {
  flex-shrink: 0;
  background-color: var(--color-border);
  transition: background-color var(--transition-fast) var(--ease-default);
}

.split-resizer:hover {
  background-color: var(--color-primary);
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
  min-height: 100%;
}

/* 搜索结果高亮样式 */
:deep(.ProseMirror .search-result) {
  background-color: var(--color-warning-light);
  border-radius: 2px;
}

:deep(.ProseMirror .search-result-current) {
  background-color: var(--color-warning);
  color: white;
}

/* 图片懒加载占位符样式 */
:deep(.ProseMirror img.lazy-image:not([src])),
:deep(.ProseMirror img.lazy-image[src='']) {
  background: linear-gradient(
    90deg,
    var(--color-bg-tertiary) 25%,
    var(--color-border) 50%,
    var(--color-bg-tertiary) 75%
  );
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  min-height: 100px;
  border-radius: 0.75rem;
}

@keyframes shimmer {
  0% {
    background-position: 200% 0;
  }
  100% {
    background-position: -200% 0;
  }
}

/* 图片加载完成后的过渡效果 */
:deep(.ProseMirror img.lazy-image[src]:not([src=''])) {
  animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}
</style>
