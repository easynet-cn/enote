<template>
  <div class="tiptap-toolbar-wrapper">
    <!-- 左箭头 -->
    <button
      class="scroll-btn scroll-btn-left"
      @click="scrollLeft"
      :disabled="!canScrollLeft"
      :class="{ hidden: !canScrollLeft }"
    >
      <ChevronLeft class="w-5 h-5" />
    </button>

    <!-- 工具栏内容 -->
    <div class="tiptap-toolbar" ref="toolbarRef" @scroll="updateScrollState">
      <!-- 标题和字体 -->
      <div class="toolbar-section">
        <Tooltip content="标题级别" placement="bottom">
          <select
            v-model="headingLevel"
            @change="setHeading"
            class="h-8 px-2 text-sm border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
          >
            <option value="0">正文</option>
            <option value="1">标题 1</option>
            <option value="2">标题 2</option>
            <option value="3">标题 3</option>
            <option value="4">标题 4</option>
            <option value="5">标题 5</option>
            <option value="6">标题 6</option>
          </select>
        </Tooltip>

        <Tooltip content="字体" placement="bottom">
          <select
            v-model="fontFamily"
            @change="setFontFamily"
            class="h-8 px-2 text-sm border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent ml-1"
          >
          <option value="">默认字体</option>
          <optgroup label="无衬线字体">
            <option value="Arial, sans-serif">Arial</option>
            <option value="Helvetica, sans-serif">Helvetica</option>
            <option value="Verdana, sans-serif">Verdana</option>
            <option value="Tahoma, sans-serif">Tahoma</option>
            <option value="Trebuchet MS, sans-serif">Trebuchet MS</option>
            <option value="Microsoft YaHei, sans-serif">微软雅黑</option>
            <option value="PingFang SC, sans-serif">苹方</option>
          </optgroup>
          <optgroup label="衬线字体">
            <option value="Times New Roman, serif">Times New Roman</option>
            <option value="Georgia, serif">Georgia</option>
            <option value="Palatino, serif">Palatino</option>
            <option value="SimSun, serif">宋体</option>
            <option value="KaiTi, serif">楷体</option>
            <option value="FangSong, serif">仿宋</option>
          </optgroup>
          <optgroup label="等宽字体">
            <option value="Courier New, monospace">Courier New</option>
            <option value="Consolas, monospace">Consolas</option>
            <option value="Monaco, monospace">Monaco</option>
            <option value="Source Code Pro, monospace">Source Code Pro</option>
          </optgroup>
          <optgroup label="艺术字体">
            <option value="Comic Sans MS, cursive">Comic Sans MS</option>
            <option value="Impact, fantasy">Impact</option>
            <option value="Brush Script MT, cursive">Brush Script</option>
          </optgroup>
        </select>
        </Tooltip>
      </div>

      <!-- 字体样式 -->
      <div class="toolbar-section">
        <div class="btn-group">
          <Tooltip content="粗体" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive('bold') }]"
              @click="editor.chain().focus().toggleBold().run()"
              :disabled="!editor.can().chain().focus().toggleBold().run()"
            >
              <Bold class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="斜体" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive('italic') }]"
              @click="editor.chain().focus().toggleItalic().run()"
              :disabled="!editor.can().chain().focus().toggleItalic().run()"
            >
              <Italic class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="下划线" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive('underline') }]"
              @click="editor.chain().focus().toggleUnderline().run()"
            >
              <UnderlineIcon class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="删除线" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive('strike') }]"
              @click="editor.chain().focus().toggleStrike().run()"
              :disabled="!editor.can().chain().focus().toggleStrike().run()"
            >
              <Strikethrough class="w-4 h-4" />
            </button>
          </Tooltip>
        </div>
      </div>

      <!-- 文本对齐 -->
      <div class="toolbar-section">
        <div class="btn-group">
          <Tooltip content="左对齐" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive({ textAlign: 'left' }) }]"
              @click="editor.chain().focus().setTextAlign('left').run()"
            >
              <AlignLeft class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="居中" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive({ textAlign: 'center' }) }]"
              @click="editor.chain().focus().setTextAlign('center').run()"
            >
              <AlignCenter class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="右对齐" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive({ textAlign: 'right' }) }]"
              @click="editor.chain().focus().setTextAlign('right').run()"
            >
              <AlignRight class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="两端对齐" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive({ textAlign: 'justify' }) }]"
              @click="editor.chain().focus().setTextAlign('justify').run()"
            >
              <AlignJustify class="w-4 h-4" />
            </button>
          </Tooltip>
        </div>
      </div>

      <!-- 列表 -->
      <div class="toolbar-section">
        <div class="btn-group">
          <Tooltip content="无序列表" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive('bulletList') }]"
              @click="editor.chain().focus().toggleBulletList().run()"
            >
              <List class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="有序列表" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive('orderedList') }]"
              @click="editor.chain().focus().toggleOrderedList().run()"
            >
              <ListOrdered class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="任务列表" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive('taskList') }]"
              @click="editor.chain().focus().toggleTaskList().run()"
            >
              <ListChecks class="w-4 h-4" />
            </button>
          </Tooltip>
        </div>
      </div>

      <!-- 引用和代码 -->
      <div class="toolbar-section">
        <div class="btn-group">
          <Tooltip content="引用" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive('blockquote') }]"
              @click="editor.chain().focus().toggleBlockquote().run()"
            >
              <Quote class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="代码块" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive('codeBlock') }]"
              @click="editor.chain().focus().toggleCodeBlock().run()"
            >
              <Code2 class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="行内代码" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive('code') }]"
              @click="editor.chain().focus().toggleCode().run()"
            >
              <Code class="w-4 h-4" />
            </button>
          </Tooltip>
        </div>
      </div>

      <!-- 链接和图片 -->
      <div class="toolbar-section">
        <div class="btn-group">
          <Tooltip content="链接" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive('link') }]"
              @click="openLinkDialog"
            >
              <LinkIcon class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="取消链接" placement="bottom">
            <button
              class="toolbar-btn"
              @click="editor.chain().focus().unsetLink().run()"
              :disabled="!editor.isActive('link')"
            >
              <Unlink class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="图片" placement="bottom">
            <button class="toolbar-btn" @click="openImageDialog">
              <ImageIcon class="w-4 h-4" />
            </button>
          </Tooltip>
        </div>
      </div>

      <!-- 表格 -->
      <div class="toolbar-section">
        <div class="btn-group">
          <Tooltip content="插入表格" placement="bottom">
            <button class="toolbar-btn" @click="insertTable">
              <TableIcon class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="添加列" placement="bottom">
            <button
              class="toolbar-btn"
              @click="editor.chain().focus().addColumnAfter().run()"
              :disabled="!editor.can().addColumnAfter()"
            >
              <Columns3 class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="添加行" placement="bottom">
            <button
              class="toolbar-btn"
              @click="editor.chain().focus().addRowAfter().run()"
              :disabled="!editor.can().addRowAfter()"
            >
              <Rows3 class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="删除表格" placement="bottom">
            <button
              class="toolbar-btn"
              @click="editor.chain().focus().deleteTable().run()"
              :disabled="!editor.can().deleteTable()"
            >
              <TableOff class="w-4 h-4" />
            </button>
          </Tooltip>
        </div>
      </div>

      <!-- 其他功能 -->
      <div class="toolbar-section">
        <div class="btn-group">
          <Tooltip content="高亮" placement="bottom">
            <button
              :class="['toolbar-btn', { active: editor.isActive('highlight') }]"
              @click="editor.chain().focus().toggleHighlight().run()"
            >
              <Highlighter class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="分隔线" placement="bottom">
            <button class="toolbar-btn" @click="editor.chain().focus().setHorizontalRule().run()">
              <Minus class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="清除格式" placement="bottom">
            <button
              class="toolbar-btn"
              @click="editor.chain().focus().clearNodes().unsetAllMarks().run()"
            >
              <RemoveFormatting class="w-4 h-4" />
            </button>
          </Tooltip>
        </div>
      </div>

      <!-- 撤销和重做 -->
      <div class="toolbar-section">
        <div class="btn-group">
          <Tooltip content="撤销" placement="bottom">
            <button
              class="toolbar-btn"
              @click="editor.chain().focus().undo().run()"
              :disabled="!editor.can().chain().focus().undo().run()"
            >
              <Undo2 class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="重做" placement="bottom">
            <button
              class="toolbar-btn"
              @click="editor.chain().focus().redo().run()"
              :disabled="!editor.can().chain().focus().redo().run()"
            >
              <Redo2 class="w-4 h-4" />
            </button>
          </Tooltip>
        </div>
      </div>

      <!-- 颜色选择器 -->
      <div class="toolbar-section">
        <Tooltip content="文本颜色" placement="bottom">
          <ColorPicker v-model="textColor" @change="setTextColor" :predefine="predefineColors" />
        </Tooltip>

        <Tooltip content="背景颜色" placement="bottom">
          <ColorPicker
            v-model="highlightColor"
            @change="setHighlightColor"
            :predefine="predefineColors"
          />
        </Tooltip>
      </div>

      <!-- Markdown 源码/预览切换 -->
      <div class="toolbar-section">
        <Tooltip :content="sourceMode ? '预览模式' : 'Markdown 源码'" placement="bottom">
          <button
            :class="['toolbar-btn', { active: sourceMode }]"
            @click="emit('toggle-source-mode')"
          >
            <FileCode v-if="!sourceMode" class="w-4 h-4" />
            <Eye v-else class="w-4 h-4" />
          </button>
        </Tooltip>
      </div>
    </div>

    <!-- 右箭头 -->
    <button
      class="scroll-btn scroll-btn-right"
      @click="scrollRight"
      :disabled="!canScrollRight"
      :class="{ hidden: !canScrollRight }"
    >
      <ChevronRight class="w-5 h-5" />
    </button>
  </div>

  <!-- 链接弹窗 -->
  <Dialog v-model="linkDialogVisible" title="插入链接" :width="400">
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">链接地址</label>
        <input
          v-model="linkUrl"
          type="text"
          placeholder="https://example.com"
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
        />
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end gap-2">
        <button
          @click="linkDialogVisible = false"
          class="px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
        >
          取消
        </button>
        <button
          @click="setLink"
          class="px-4 py-2 text-white bg-green-500 rounded-md hover:bg-green-600 transition-colors"
        >
          确定
        </button>
      </div>
    </template>
  </Dialog>

  <!-- 图片弹窗 -->
  <Dialog v-model="imageDialogVisible" title="插入图片" :width="400">
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">图片地址</label>
        <input
          v-model="imageUrl"
          type="text"
          placeholder="https://example.com/image.png"
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
        />
      </div>
      <div class="text-center text-gray-500 text-sm">- 或 -</div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">上传图片</label>
        <input
          type="file"
          accept="image/*"
          @change="handleImageUpload"
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
        />
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end gap-2">
        <button
          @click="imageDialogVisible = false"
          class="px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
        >
          取消
        </button>
        <button
          @click="insertImage"
          class="px-4 py-2 text-white bg-green-500 rounded-md hover:bg-green-600 transition-colors"
        >
          确定
        </button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'
import type { Editor } from '@tiptap/vue-3'
import { Tooltip, ColorPicker, Dialog } from './ui'
import {
  Bold,
  Italic,
  Underline as UnderlineIcon,
  Strikethrough,
  AlignLeft,
  AlignCenter,
  AlignRight,
  AlignJustify,
  List,
  ListOrdered,
  ListChecks,
  Quote,
  Code2,
  Code,
  Highlighter,
  Minus,
  RemoveFormatting,
  Undo2,
  Redo2,
  Link as LinkIcon,
  Unlink,
  Image as ImageIcon,
  Table as TableIcon,
  Columns3,
  Rows3,
  TableProperties as TableOff,
  ChevronLeft,
  ChevronRight,
  FileCode,
  Eye,
} from 'lucide-vue-next'

interface Props {
  editor: Editor
  sourceMode?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  sourceMode: false,
})

const emit = defineEmits<{
  'toggle-source-mode': []
}>()

const headingLevel = ref('0')
const fontFamily = ref('')
const textColor = ref('#000000')
const highlightColor = ref('#FFFF00')

// 链接弹窗
const linkDialogVisible = ref(false)
const linkUrl = ref('')

// 图片弹窗
const imageDialogVisible = ref(false)
const imageUrl = ref('')

// 滚动相关
const toolbarRef = ref<HTMLElement | null>(null)
const canScrollLeft = ref(false)
const canScrollRight = ref(false)
const scrollAmount = 200

// 更新滚动状态
const updateScrollState = () => {
  if (!toolbarRef.value) return
  const { scrollLeft, scrollWidth, clientWidth } = toolbarRef.value
  canScrollLeft.value = scrollLeft > 0
  canScrollRight.value = scrollLeft + clientWidth < scrollWidth - 1
}

// 向左滚动
const scrollLeft = () => {
  if (!toolbarRef.value) return
  toolbarRef.value.scrollBy({ left: -scrollAmount, behavior: 'smooth' })
}

// 向右滚动
const scrollRight = () => {
  if (!toolbarRef.value) return
  toolbarRef.value.scrollBy({ left: scrollAmount, behavior: 'smooth' })
}

// 监听窗口大小变化
onMounted(() => {
  updateScrollState()
  window.addEventListener('resize', updateScrollState)
})

onUnmounted(() => {
  window.removeEventListener('resize', updateScrollState)
})

// 预定义颜色
const predefineColors = [
  '#000000',
  '#ffffff',
  '#ff4500',
  '#ff8c00',
  '#ffd700',
  '#90ee90',
  '#00ced1',
  '#1e90ff',
  '#c71585',
  '#8b4513',
]

// 设置标题
const setHeading = () => {
  const level = parseInt(headingLevel.value)
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

// 设置字体
const setFontFamily = () => {
  if (fontFamily.value) {
    props.editor.chain().focus().setFontFamily(fontFamily.value).run()
  } else {
    props.editor.chain().focus().unsetFontFamily().run()
  }
}

// 设置文本颜色
const setTextColor = (color: string) => {
  props.editor.chain().focus().setColor(color).run()
}

// 设置背景颜色
const setHighlightColor = (color: string) => {
  props.editor.chain().focus().toggleHighlight({ color }).run()
}

// 打开链接弹窗
const openLinkDialog = () => {
  const previousUrl = props.editor.getAttributes('link').href
  linkUrl.value = previousUrl || ''
  linkDialogVisible.value = true
}

// 设置链接
const setLink = () => {
  if (linkUrl.value) {
    props.editor.chain().focus().extendMarkRange('link').setLink({ href: linkUrl.value }).run()
  }
  linkDialogVisible.value = false
  linkUrl.value = ''
}

// 打开图片弹窗
const openImageDialog = () => {
  imageUrl.value = ''
  imageDialogVisible.value = true
}

// 处理图片上传
const handleImageUpload = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (file) {
    const reader = new FileReader()
    reader.onload = (e) => {
      imageUrl.value = e.target?.result as string
    }
    reader.readAsDataURL(file)
  }
}

// 插入图片
const insertImage = () => {
  if (imageUrl.value) {
    props.editor.chain().focus().setImage({ src: imageUrl.value }).run()
  }
  imageDialogVisible.value = false
  imageUrl.value = ''
}

// 插入表格
const insertTable = () => {
  props.editor.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run()
}

// 监听编辑器活动状态更新选择框
watch(
  () => props.editor?.isActive('heading'),
  (isActive) => {
    if (!isActive) {
      headingLevel.value = '0'
    } else {
      // 检查具体是哪个级别的标题
      for (let i = 1; i <= 6; i++) {
        if (props.editor.isActive('heading', { level: i })) {
          headingLevel.value = i.toString()
          break
        }
      }
    }
  },
)

// 监听字体变化
watch(
  () => props.editor?.getAttributes('textStyle')?.fontFamily,
  (font) => {
    fontFamily.value = font || ''
  },
)

// 监听文本颜色变化
watch(
  () => props.editor?.getAttributes('textStyle')?.color,
  (color) => {
    if (color) {
      textColor.value = color
    }
  },
)

// 监听高亮颜色变化
watch(
  () => props.editor?.getAttributes('highlight')?.color,
  (color) => {
    if (color) {
      highlightColor.value = color
    }
  },
)
</script>

<style scoped>
/* 工具栏外层容器 */
.tiptap-toolbar-wrapper {
  display: flex;
  align-items: center;
  border-bottom: 1px solid #e5e7eb;
  background-color: #f9fafb;
  position: relative;
}

/* 滚动按钮 */
.scroll-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 100%;
  min-height: 48px;
  background-color: #f9fafb;
  border: none;
  cursor: pointer;
  color: #6b7280;
  transition: all 0.15s ease;
  flex-shrink: 0;
  z-index: 10;
}

.scroll-btn:hover:not(:disabled) {
  background-color: #e5e7eb;
  color: #374151;
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
  border-right: 1px solid #e5e7eb;
}

.scroll-btn-right {
  border-left: 1px solid #e5e7eb;
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
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE/Edge */
}

.tiptap-toolbar::-webkit-scrollbar {
  display: none; /* Chrome/Safari/Opera */
}

.toolbar-section {
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.btn-group {
  display: flex;
  border: 1px solid #e5e7eb;
  border-radius: 0.375rem;
}

.btn-group .toolbar-btn {
  border-radius: 0;
  border: none;
  border-right: 1px solid #e5e7eb;
}

.btn-group .toolbar-btn:last-child {
  border-right: none;
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: 1px solid #e5e7eb;
  background-color: white;
  border-radius: 0.375rem;
  cursor: pointer;
  transition: all 0.15s ease;
  flex-shrink: 0;
}

.toolbar-btn:hover:not(:disabled) {
  background-color: #f3f4f6;
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toolbar-btn.active {
  background-color: #10b981;
  color: white;
  border-color: #10b981;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .tiptap-toolbar {
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
  }

  .scroll-btn {
    width: 24px;
  }
}
</style>
