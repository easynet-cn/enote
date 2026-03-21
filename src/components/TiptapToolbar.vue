<template>
  <div class="tiptap-toolbar-wrapper" role="toolbar" :aria-label="t('aria.toolbar')">
    <!-- 左侧固定区域：Content Type -->
    <div class="toolbar-fixed toolbar-fixed-left">
      <!-- 新建笔记时的模式选择 -->
      <div v-if="isNewNote" class="toolbar-section">
        <Tooltip :content="t('editor.toolbarTooltip.contentType')" placement="bottom">
          <select
            :value="contentType"
            @change="
              emit('update:content-type', Number(($event.target as HTMLSelectElement).value))
            "
            class="h-8 px-2 text-sm border border-edge rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent font-medium"
            :class="
              isMarkdownMode
                ? 'bg-slate-800 text-white border-edge-dark'
                : 'bg-indigo-50 text-indigo-700 border-indigo-300'
            "
          >
            <option :value="ContentType.Html">{{ t('editor.contentType.richText') }}</option>
            <option :value="ContentType.Markdown">{{ t('editor.contentType.markdown') }}</option>
          </select>
        </Tooltip>
      </div>

      <!-- 模式标识（已保存的笔记） -->
      <div v-else class="toolbar-section">
        <span
          class="h-8 px-3 text-sm rounded-lg flex items-center font-medium"
          :class="isMarkdownMode ? 'bg-slate-800 text-white' : 'bg-indigo-50 text-indigo-700'"
        >
          {{ isMarkdownMode ? 'Markdown' : t('editor.contentType.richText') }}
        </span>
      </div>
    </div>

    <!-- 左箭头 -->
    <button
      class="scroll-btn scroll-btn-left"
      @click="scrollLeftFn"
      :disabled="!canScrollLeft"
      :class="{ hidden: !canScrollLeft }"
      aria-label="Scroll left"
    >
      <ChevronLeft class="w-5 h-5" />
    </button>

    <!-- 中间可滚动区域：编辑工具 -->
    <div class="tiptap-toolbar" ref="toolbarRef" @scroll="updateScrollState">
      <!-- 富文本模式工具栏 -->
      <template v-if="!isMarkdownMode && editor">
        <HeadingFontToolbar
          :heading-level="headingLevel"
          :font-family="fontFamily"
          :font-size="fontSize"
          :edit-mode="editMode"
          @update:heading-level="setHeading"
          @update:font-family="setFontFamily"
          @update:font-size="setFontSize"
        />

        <!-- 字体样式 (Bold/Italic/Underline/Strike/Super/Sub) -->
        <div class="toolbar-section">
          <div class="btn-group">
            <ToolbarButton
              :tooltip="t('editor.toolbarTooltip.bold')"
              :active="editor.isActive('bold')"
              :disabled="!editMode || !editor.can().chain().focus().toggleBold().run()"
              @click="editor.chain().focus().toggleBold().run()"
            >
              <Bold class="w-4 h-4" />
            </ToolbarButton>
            <ToolbarButton
              :tooltip="t('editor.toolbarTooltip.italic')"
              :active="editor.isActive('italic')"
              :disabled="!editMode || !editor.can().chain().focus().toggleItalic().run()"
              @click="editor.chain().focus().toggleItalic().run()"
            >
              <Italic class="w-4 h-4" />
            </ToolbarButton>
            <ToolbarButton
              :tooltip="t('editor.toolbarTooltip.underline')"
              :active="editor.isActive('underline')"
              :disabled="!editMode"
              @click="editor.chain().focus().toggleUnderline().run()"
            >
              <UnderlineIcon class="w-4 h-4" />
            </ToolbarButton>
            <ToolbarButton
              :tooltip="t('editor.toolbarTooltip.strikethrough')"
              :active="editor.isActive('strike')"
              :disabled="!editMode || !editor.can().chain().focus().toggleStrike().run()"
              @click="editor.chain().focus().toggleStrike().run()"
            >
              <Strikethrough class="w-4 h-4" />
            </ToolbarButton>
            <ToolbarButton
              :tooltip="t('editor.toolbarTooltip.superscript')"
              :active="editor.isActive('superscript')"
              :disabled="!editMode"
              @click="editor.chain().focus().toggleSuperscript().run()"
            >
              <Superscript class="w-4 h-4" />
            </ToolbarButton>
            <ToolbarButton
              :tooltip="t('editor.toolbarTooltip.subscript')"
              :active="editor.isActive('subscript')"
              :disabled="!editMode"
              @click="editor.chain().focus().toggleSubscript().run()"
            >
              <Subscript class="w-4 h-4" />
            </ToolbarButton>
          </div>
        </div>

        <AlignmentToolbar :editor="editor" :edit-mode="editMode" />
        <ListToolbar :editor="editor" :edit-mode="editMode" />
        <IndentToolbar :editor="editor" :edit-mode="editMode" />
        <QuoteCodeToolbar :editor="editor" :edit-mode="editMode" />
        <LinkImageToolbar
          :editor="editor"
          :edit-mode="editMode"
          @open-link-dialog="linkDialogVisible = true"
          @open-image-dialog="imageDialogVisible = true"
        />
        <TableToolbar :editor="editor" :edit-mode="editMode" />
        <UtilitiesToolbar
          :editor="editor"
          :edit-mode="editMode"
          :search-visible="searchDialogVisible"
          :toc-visible="tocVisible"
          @toggle-search="searchDialogVisible = !searchDialogVisible"
          @toggle-toc="emit('toggle-toc')"
        />
        <UndoRedoToolbar :editor="editor" :edit-mode="editMode" />
        <ColorToolbar
          :text-color="textColor"
          :highlight-color="highlightColor"
          :edit-mode="editMode"
          @update:text-color="setTextColor"
          @update:highlight-color="setHighlightColor"
        />
      </template>

      <!-- Markdown 模式工具栏 -->
      <MarkdownToolbar
        v-if="isMarkdownMode"
        :source-mode="sourceMode"
        :markdown-layout="markdownLayout"
        :edit-mode="editMode"
        @toggle-source-mode="emit('toggle-source-mode')"
        @update:markdown-layout="emit('update:markdown-layout', $event)"
      />
    </div>

    <!-- 右箭头 -->
    <button
      class="scroll-btn scroll-btn-right"
      @click="scrollRightFn"
      :disabled="!canScrollRight"
      :class="{ hidden: !canScrollRight }"
      aria-label="Scroll right"
    >
      <ChevronRight class="w-5 h-5" />
    </button>

    <!-- 右侧固定区域：操作按钮 -->
    <div class="toolbar-fixed toolbar-fixed-right">
      <ActionButtons
        :edit-mode="editMode"
        @edit="emit('edit')"
        @save="emit('save')"
        @cancel="emit('cancel')"
        @delete="emit('delete')"
        @settings="emit('settings')"
        @history="emit('history')"
        @export="emit('export')"
        @save-as-template="emit('saveAsTemplate')"
      />
    </div>
  </div>

  <!-- 弹窗 -->
  <LinkDialog v-model="linkDialogVisible" :editor="editor" />
  <ImageDialog v-model="imageDialogVisible" :editor="editor" />
  <SearchReplaceDialog v-model="searchDialogVisible" :editor="editor" />
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, onBeforeUnmount, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'
import { ContentType, MarkdownLayout } from '../types'
import { Tooltip } from './ui'
import {
  Bold,
  Italic,
  Underline as UnderlineIcon,
  Strikethrough,
  Subscript,
  Superscript,
  ChevronLeft,
  ChevronRight,
} from 'lucide-vue-next'
import { ToolbarButton } from './toolbar'
import {
  AlignmentToolbar,
  ListToolbar,
  HeadingFontToolbar,
  IndentToolbar,
  QuoteCodeToolbar,
  TableToolbar,
  UndoRedoToolbar,
  LinkImageToolbar,
  UtilitiesToolbar,
  ColorToolbar,
  MarkdownToolbar,
} from './toolbar'
import ActionButtons from './toolbar/ActionButtons.vue'
import LinkDialog from './toolbar/LinkDialog.vue'
import ImageDialog from './toolbar/ImageDialog.vue'
import SearchReplaceDialog from './toolbar/SearchReplaceDialog.vue'

const { t } = useI18n()

interface Props {
  editor: Editor | null
  sourceMode?: boolean
  contentType?: ContentType
  isNewNote?: boolean
  editMode?: boolean
  markdownLayout?: MarkdownLayout
  tocVisible?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  sourceMode: false,
  contentType: ContentType.Html,
  isNewNote: false,
  editMode: false,
  markdownLayout: MarkdownLayout.None,
  tocVisible: false,
})

const emit = defineEmits<{
  'toggle-source-mode': []
  'update:content-type': [contentType: ContentType]
  'update:markdown-layout': [layout: MarkdownLayout]
  'toggle-toc': []
  edit: []
  save: []
  cancel: []
  delete: []
  settings: []
  history: []
  export: []
  saveAsTemplate: []
}>()

// 是否为 Markdown 模式
const isMarkdownMode = computed(() => props.contentType === ContentType.Markdown)

// 编辑器状态同步
const headingLevel = ref('0')
const fontFamily = ref('')
const fontSize = ref('')
const textColor = ref('#000000')
const highlightColor = ref('#FFFF00')

// 弹窗状态
const linkDialogVisible = ref(false)
const imageDialogVisible = ref(false)
const searchDialogVisible = ref(false)

// 滚动相关
const toolbarRef = ref<HTMLElement | null>(null)
const canScrollLeft = ref(false)
const canScrollRight = ref(false)
const scrollAmount = 200

const updateScrollState = () => {
  if (!toolbarRef.value) return
  const el = toolbarRef.value
  canScrollLeft.value = el.scrollLeft > 0
  canScrollRight.value = el.scrollLeft + el.clientWidth < el.scrollWidth - 1
}

const scrollLeftFn = () => {
  toolbarRef.value?.scrollBy({ left: -scrollAmount, behavior: 'smooth' })
}

const scrollRightFn = () => {
  toolbarRef.value?.scrollBy({ left: scrollAmount, behavior: 'smooth' })
}

onMounted(() => {
  updateScrollState()
  window.addEventListener('resize', updateScrollState)
})

onUnmounted(() => {
  window.removeEventListener('resize', updateScrollState)
})

// 标题/字体/字号设置
const setHeading = (value: string) => {
  if (!props.editor) return
  headingLevel.value = value
  const level = parseInt(value)
  if (level === 0) {
    props.editor.chain().focus().setParagraph().run()
  } else {
    props.editor
      .chain()
      .focus()
      .toggleHeading({ level: level as 1 | 2 | 3 | 4 | 5 | 6 })
      .run()
  }
}

const setFontFamily = (value: string) => {
  if (!props.editor) return
  fontFamily.value = value
  if (value) {
    props.editor.chain().focus().setFontFamily(value).run()
  } else {
    props.editor.chain().focus().unsetFontFamily().run()
  }
}

const setFontSize = (value: string) => {
  if (!props.editor) return
  fontSize.value = value
  if (value) {
    props.editor.chain().focus().setFontSize(value).run()
  } else {
    props.editor.chain().focus().unsetFontSize().run()
  }
}

const setTextColor = (color: string) => {
  if (!props.editor) return
  textColor.value = color
  props.editor.chain().focus().setColor(color).run()
}

const setHighlightColor = (color: string) => {
  if (!props.editor) return
  highlightColor.value = color
  props.editor.chain().focus().toggleHighlight({ color }).run()
}

// 同步编辑器状态到工具栏
const syncEditorState = () => {
  const editor = props.editor
  if (!editor) return

  if (!editor.isActive('heading')) {
    headingLevel.value = '0'
  } else {
    for (let i = 1; i <= 6; i++) {
      if (editor.isActive('heading', { level: i })) {
        headingLevel.value = i.toString()
        break
      }
    }
  }

  const textStyle = editor.getAttributes('textStyle')
  fontFamily.value = textStyle?.fontFamily || ''
  fontSize.value = textStyle?.fontSize || ''
  if (textStyle?.color) textColor.value = textStyle.color

  const highlight = editor.getAttributes('highlight')
  if (highlight?.color) highlightColor.value = highlight.color
}

watch(
  () => props.editor,
  (editor, oldEditor) => {
    if (oldEditor) oldEditor.off('selectionUpdate', syncEditorState)
    if (editor) {
      editor.on('selectionUpdate', syncEditorState)
      syncEditorState()
    }
  },
  { immediate: true },
)

onBeforeUnmount(() => {
  if (props.editor) props.editor.off('selectionUpdate', syncEditorState)
})
</script>

<style scoped>
/* 工具栏外层容器 */
.tiptap-toolbar-wrapper {
  display: flex;
  align-items: center;
  border-bottom: 1px solid var(--color-border);
  background-color: var(--color-bg-secondary);
  position: relative;
}

/* 固定区域 */
.toolbar-fixed {
  display: flex;
  align-items: center;
  padding: 0.5rem;
  background-color: var(--color-bg-secondary);
  flex-shrink: 0;
  z-index: 5;
}

.toolbar-fixed-left {
  border-right: 1px solid var(--color-border);
  background: linear-gradient(to left, var(--color-bg-tertiary), var(--color-bg-secondary));
  padding-left: 0.75rem;
  padding-right: 0.75rem;
}

.toolbar-fixed-right {
  border-left: 1px solid var(--color-border);
  background: linear-gradient(to right, var(--color-bg-tertiary), var(--color-bg-secondary));
}

/* 滚动按钮 */
.scroll-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 100%;
  min-height: 48px;
  background-color: var(--color-bg-secondary);
  border: none;
  cursor: pointer;
  color: var(--color-text-secondary);
  transition: all var(--transition-fast) ease;
  flex-shrink: 0;
  z-index: 10;
}

.scroll-btn:hover:not(:disabled) {
  background-color: var(--color-border);
  color: var(--color-text-primary);
}

.scroll-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.scroll-btn.hidden {
  opacity: 0;
  pointer-events: none;
}

.scroll-btn-left {
  border-right: 1px solid var(--color-border);
}

.scroll-btn-right {
  border-left: 1px solid var(--color-border);
}

/* 工具栏内容区域 */
.tiptap-toolbar {
  display: flex;
  flex-wrap: nowrap;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  align-items: center;
  overflow-x: auto;
  flex: 1;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.tiptap-toolbar::-webkit-scrollbar {
  display: none;
}

:deep(.toolbar-section) {
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

:deep(.btn-group) {
  display: flex;
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
}

:deep(.btn-group .toolbar-btn) {
  border-radius: 0;
  border: none;
  border-right: 1px solid var(--color-border);
}

:deep(.btn-group .toolbar-btn:last-child) {
  border-right: none;
}

:deep(.toolbar-btn) {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: 1px solid var(--color-border);
  background-color: var(--color-bg-primary);
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all var(--transition-fast) ease;
  flex-shrink: 0;
}

:deep(.toolbar-btn:hover:not(:disabled)) {
  background-color: var(--color-bg-tertiary);
}

:deep(.toolbar-btn:disabled) {
  opacity: 0.5;
  cursor: not-allowed;
}

:deep(.toolbar-btn.active) {
  background-color: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

/* 响应式设计 - 平板 */
@media (min-width: 640px) and (max-width: 1024px) {
  .tiptap-toolbar {
    gap: 0.25rem;
    padding: 0.375rem 0.5rem;
  }

  .toolbar-fixed {
    padding: 0.375rem;
  }

  .toolbar-fixed-left {
    padding-left: 0.5rem;
    padding-right: 0.5rem;
  }

  :deep(.toolbar-btn) {
    width: 28px;
    height: 28px;
  }
}

/* 响应式设计 - 手机 */
@media (max-width: 639px) {
  .tiptap-toolbar-wrapper {
    flex-wrap: nowrap;
  }

  .tiptap-toolbar {
    gap: 0.125rem;
    padding: 0.25rem 0.375rem;
  }

  .toolbar-fixed {
    padding: 0.25rem;
  }

  .toolbar-fixed-left {
    padding-left: 0.375rem;
    padding-right: 0.375rem;
  }

  .scroll-btn {
    width: 22px;
    min-height: 40px;
  }

  :deep(.toolbar-btn) {
    width: 28px;
    height: 28px;
  }
}
</style>
