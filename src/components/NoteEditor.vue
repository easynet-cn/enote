<template>
  <div class="flex-1 h-full flex flex-col overflow-hidden bg-surface shadow-sm relative">
    <main
      class="flex-1 flex flex-col min-h-0 overflow-hidden editor-main"
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
          @print="handlePrint"
          @history="handleHistory"
          @save-as-template="emit('saveAsTemplate')"
        />
      </div>

      <!-- 标题输入区域 -->
      <EditorTitleBar
        ref="titleBarRef"
        :active-note="activeNote"
        :edit-mode="editMode"
        :layout="layout"
        @update-title="(val: string) => emit('updateNoteTitle', val)"
        @back="emit('back')"
      />

      <!-- TipTap 编辑器 / Markdown 源码编辑器 -->
      <div v-if="activeNote" class="relative flex-1">
        <!-- Markdown 双面板布局 -->
        <MarkdownSplitEditor
          v-if="isMarkdownMode && markdownLayout !== MarkdownLayout.None && editMode"
          :layout="markdownLayout"
          :source="markdownSource"
          @update:source="handleMarkdownSourceUpdate"
        >
          <template #preview>
            <editor-content :editor="editor" />
          </template>
        </MarkdownSplitEditor>

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

    <!-- 右侧悬浮抽屉 -->
    <EditorDrawer
      :active-note="activeNote"
      :is-new-note="isNewNote"
      :show-attachment="showAttachmentPanel"
      :show-link="showLinkPanel"
      @switch-tab="handleDrawerTabSwitch"
      @close="closeDrawer"
      @navigate-to-note="(id: number) => emit('navigateToNote', id)"
    />

    <!-- 设置弹窗 -->
    <EditorSettingsDialog
      v-model="settingDialog"
      :notebooks="notebooks"
      :tags="tags"
      :notebook-id="activeNote?.notebookId ?? ''"
      :selected-tag-ids="activeNote?.tags?.map((t) => t.id) ?? []"
      :mcp-access="activeNote?.mcpAccess"
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

    <!-- 底部状态栏 -->
    <EditorStatusBar
      v-if="activeNote && editor"
      :editor="editor"
      :show-panel-buttons="!!activeNote && !isNewNote"
      :attachment-active="showAttachmentPanel"
      :link-active="showLinkPanel"
      @toggle-attachment="toggleAttachment"
      @toggle-link="toggleLink"
    />
  </div>
</template>

<script setup lang="ts">
import { watch, onBeforeUnmount, onMounted, ref, computed, shallowRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { Editor, EditorContent } from '@tiptap/vue-3'
import { getMarkdownExtensions, getRichTextExtensions } from '../config/editorExtensions'
import TiptapToolbar from './TiptapToolbar.vue'
import EditorSettingsDialog from './EditorSettingsDialog.vue'
import ExportDialog from './ExportDialog.vue'
import { ConfirmDialog } from './ui'
import { ContentType, McpAccess, MarkdownLayout } from '../types'
import type { NoteHistory, ShowNote, ShowNotebook, ShowTag } from '../types'
import type { LayoutMode } from '../composables/usePlatform'
import { getMarkdownFromEditor } from '../types/tiptap-markdown'
import { isTemporaryId } from '../utils/validation'
import { throttle } from '../utils/debounce'
import { MARKDOWN_PREVIEW_THROTTLE } from '../config/constants'
import { preprocessMarkdown } from '../utils/markdownWorker'
import NoteHistoryDialog from './NoteHistoryDialog.vue'
import TableOfContents from './TableOfContents.vue'
import EditorStatusBar from './EditorStatusBar.vue'
import EditorDrawer from './EditorDrawer.vue'
import EditorTitleBar from './EditorTitleBar.vue'
import MarkdownSplitEditor from './MarkdownSplitEditor.vue'
import type { TocItem } from '../extensions'
import { exportAsPdf } from '../utils/export'

interface Props {
  notebooks: ShowNotebook[]
  tags: ShowTag[]
  activeNote: ShowNote | null
  editMode: boolean
  historyLoading?: boolean
  mobile?: boolean
  layout?: LayoutMode
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
  updateNoteSetting: [notebookId: string, tagIds: string[], mcpAccess: McpAccess]
  sizeChange: [pageSize: number]
  currentChange: [currentPage: number]
  open: []
  navigateToNote: [id: number]
  saveAsTemplate: []
  back: []
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

// 标题栏引用
const titleBarRef = ref<InstanceType<typeof EditorTitleBar> | null>(null)

// Timer refs for cleanup
let editModeFocusTimer: ReturnType<typeof setTimeout> | null = null

// 是否为 Markdown 模式
const isMarkdownMode = computed(() => props.activeNote?.contentType === ContentType.Markdown)

// TipTap 编辑器实例（根据内容类型动态选择扩展）
const editor = shallowRef<Editor | undefined>(undefined)

// 编辑器创建请求 ID，用于防止快速切换笔记时的竞态条件
let editorCreateRequestId = 0

// 创建编辑器实例
const createEditor = async (contentType: ContentType, content: string) => {
  const requestId = ++editorCreateRequestId

  // 销毁旧编辑器
  if (editor.value) {
    editor.value.destroy()
  }

  // 检查是否已被更新的请求取代
  if (requestId !== editorCreateRequestId) return

  // 根据内容类型选择扩展：Markdown 模式使用 Markdown 扩展，富文本模式不使用
  const extensions =
    contentType === ContentType.Markdown
      ? await getMarkdownExtensions()
      : await getRichTextExtensions()

  // 异步加载后再次检查竞态
  if (requestId !== editorCreateRequestId) return

  editor.value = new Editor({
    extensions,
    content,
    editable: false,
    editorProps: {
      attributes: {
        autocapitalize: 'off',
        autocorrect: 'off',
        spellcheck: 'false',
      },
    },
    onUpdate: ({ editor: ed }) => {
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
      if (props.editMode && oldNote && newNote.id === oldNote.id) {
        return
      }

      const newContentType = newNote.contentType ?? ContentType.Html
      const oldContentType = oldNote?.contentType ?? ContentType.Html
      const contentTypeChanged = newContentType !== oldContentType

      if (contentTypeChanged || !editor.value) {
        createEditor(newContentType, newNote.content)
      } else {
        editor.value.commands.setContent(newNote.content)
      }

      // 重置源码模式和布局
      sourceMode.value = false
      markdownLayout.value = MarkdownLayout.None
      // 关闭抽屉面板
      closeDrawer()
    }
  },
)

// 监听编辑模式变化
watch(
  () => props.editMode,
  (newMode) => {
    if (editor.value && newMode) {
      editor.value?.setEditable(true)
      if (editModeFocusTimer) clearTimeout(editModeFocusTimer)
      editModeFocusTimer = setTimeout(() => {
        editModeFocusTimer = null
        if (isNewNote.value) {
          titleBarRef.value?.focus()
        } else {
          editor.value?.commands.focus('start')
        }
      }, 100)
    } else {
      editor.value?.setEditable(false)
    }
  },
)

// 抽屉面板状态
const showAttachmentPanel = ref(false)
const showLinkPanel = ref(false)

const handleDrawerTabSwitch = (tab: 'attachment' | 'link') => {
  if (tab === 'attachment') {
    showAttachmentPanel.value = true
    showLinkPanel.value = false
  } else {
    showLinkPanel.value = true
    showAttachmentPanel.value = false
  }
}

const toggleAttachment = () => {
  showAttachmentPanel.value = !showAttachmentPanel.value
  showLinkPanel.value = false
}

const toggleLink = () => {
  showLinkPanel.value = !showLinkPanel.value
  showAttachmentPanel.value = false
}

const closeDrawer = () => {
  showAttachmentPanel.value = false
  showLinkPanel.value = false
}

// ESC 关闭抽屉
const handleDrawerEsc = (e: KeyboardEvent) => {
  if (e.key === 'Escape' && (showAttachmentPanel.value || showLinkPanel.value)) {
    closeDrawer()
  }
}
onMounted(() => window.addEventListener('keydown', handleDrawerEsc))
onBeforeUnmount(() => window.removeEventListener('keydown', handleDrawerEsc))

// 目录
const tocVisible = ref(false)
const tocItems = ref<TocItem[]>([])

const toggleToc = () => {
  tocVisible.value = !tocVisible.value
}

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

const handlePrint = () => {
  if (!props.activeNote || !editor.value) return
  const title = props.activeNote.title || t('export.untitledNote')
  const content =
    props.activeNote.contentType === ContentType.Markdown
      ? editor.value.getHTML()
      : props.activeNote.content
  exportAsPdf(title, content)
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

const handleSettingFormSubmit = (notebookId: string, tagIds: string[], mcpAccess: McpAccess) => {
  if (props.activeNote) {
    emit('updateNoteSetting', notebookId, tagIds, mcpAccess)
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

    if (oldContentType !== contentType) {
      createEditor(contentType, props.activeNote.content)
      if (props.editMode) {
        editor.value?.setEditable(true)
      }
    }
  }
}

// 处理 Markdown 布局变化
const handleMarkdownLayoutChange = (layout: MarkdownLayout) => {
  if (layout !== MarkdownLayout.None && markdownLayout.value === MarkdownLayout.None) {
    // 重置分割比例 (MarkdownSplitEditor 内部管理)
  }

  markdownLayout.value = layout

  // 切换到双面板布局时，同步源码
  if (layout !== MarkdownLayout.None && editor.value) {
    markdownSource.value = getMarkdownFromEditor(editor.value)
  }
}

// 切换源码模式
const toggleSourceMode = () => {
  if (!editor.value) return

  if (sourceMode.value) {
    editor.value.commands.setContent(markdownSource.value)
  } else {
    markdownSource.value = getMarkdownFromEditor(editor.value)
  }
  sourceMode.value = !sourceMode.value
}

// 节流的预览更新函数
const throttledUpdatePreview = throttle(async (content: string) => {
  if (editor.value) {
    const processedContent = await preprocessMarkdown(content)
    editor.value.commands.setContent(processedContent)
  }
}, MARKDOWN_PREVIEW_THROTTLE)

// 处理 MarkdownSplitEditor 的源码更新
const handleMarkdownSourceUpdate = (value: string) => {
  markdownSource.value = value
  handleSourceChange()
}

// 处理源码内容变化
const handleSourceChange = () => {
  const contentType = props.activeNote?.contentType ?? ContentType.Html
  if (contentType === ContentType.Markdown) {
    emit('updateNoteContent', markdownSource.value)

    if (markdownLayout.value !== MarkdownLayout.None) {
      throttledUpdatePreview(markdownSource.value)
    }
  } else {
    if (editor.value) {
      editor.value.commands.setContent(markdownSource.value)
      emit('updateNoteContent', editor.value.getHTML())
    }
  }
}
</script>

<!-- 共享 ProseMirror 样式 -->
<style src="../styles/prosemirror.css"></style>

<style scoped>
.editor-main {
  padding: 1rem 1rem 0.5rem;
}

@media (min-width: 640px) {
  .editor-main {
    padding: 1rem 1.5rem 0.5rem;
  }
}

.tiptap-editor,
.tiptap-editor-edit {
  position: absolute;
  inset: 0;
  padding: 1rem 1rem;
  overflow-y: auto;
}

@media (min-width: 640px) {
  .tiptap-editor,
  .tiptap-editor-edit {
    padding: 1rem 1.5rem;
  }
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

/* Markdown 源码编辑器样式（单面板模式） */
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
