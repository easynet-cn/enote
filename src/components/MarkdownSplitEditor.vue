<template>
  <div
    class="markdown-split-layout"
    :class="layout === MarkdownLayout.Vertical ? 'layout-vertical' : 'layout-horizontal'"
  >
    <!-- 源码编辑面板 -->
    <div class="split-panel source-panel" :style="splitPanelStyle">
      <div class="panel-header">源码</div>
      <textarea
        ref="sourcePanel"
        :value="source"
        class="markdown-source-panel"
        placeholder="在此输入 Markdown 源码..."
        @input="handleInput"
        @scroll="handleSourceScroll"
      />
    </div>

    <!-- 拖拽分隔条 -->
    <div
      class="split-resizer"
      :class="layout === MarkdownLayout.Vertical ? 'resizer-horizontal' : 'resizer-vertical'"
      @mousedown="startResize"
    />

    <!-- 预览面板 -->
    <div class="split-panel preview-panel">
      <div class="panel-header">预览</div>
      <div ref="previewPanel" class="markdown-preview-panel" @scroll="handlePreviewScroll">
        <slot name="preview" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { MarkdownLayout } from '../types'

interface Props {
  layout: MarkdownLayout
  source: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:source': [value: string]
}>()

const splitRatio = ref(50)
const isResizing = ref(false)
const sourcePanel = ref<HTMLTextAreaElement | null>(null)
const previewPanel = ref<HTMLDivElement | null>(null)
const isScrollSyncing = ref(false)

// 分割面板样式
const splitPanelStyle = computed(() => {
  if (props.layout === MarkdownLayout.Vertical) {
    return { height: `${splitRatio.value}%` }
  } else {
    return { width: `${splitRatio.value}%` }
  }
})

const handleInput = (e: Event) => {
  const target = e.target as HTMLTextAreaElement
  emit('update:source', target.value)
}

// 拖拽调整面板大小
const startResize = (e: MouseEvent) => {
  isResizing.value = true
  const startPos = props.layout === MarkdownLayout.Vertical ? e.clientY : e.clientX
  const startRatio = splitRatio.value

  const onMouseMove = (moveEvent: MouseEvent) => {
    if (!isResizing.value) return

    const container = (moveEvent.target as HTMLElement).closest('.markdown-split-layout')
    if (!container) return

    const rect = container.getBoundingClientRect()
    const currentPos =
      props.layout === MarkdownLayout.Vertical ? moveEvent.clientY : moveEvent.clientX
    const containerSize = props.layout === MarkdownLayout.Vertical ? rect.height : rect.width

    const delta = currentPos - startPos
    const deltaRatio = (delta / containerSize) * 100

    let newRatio = startRatio + deltaRatio
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
    props.layout === MarkdownLayout.Vertical ? 'row-resize' : 'col-resize'
  document.body.style.userSelect = 'none'
}

// 滚动同步
const handleSourceScroll = () => {
  if (isScrollSyncing.value || !sourcePanel.value || !previewPanel.value) return

  isScrollSyncing.value = true
  const source = sourcePanel.value
  const preview = previewPanel.value
  const scrollRatio = source.scrollTop / (source.scrollHeight - source.clientHeight || 1)
  preview.scrollTop = scrollRatio * (preview.scrollHeight - preview.clientHeight)

  requestAnimationFrame(() => {
    isScrollSyncing.value = false
  })
}

const handlePreviewScroll = () => {
  if (isScrollSyncing.value || !sourcePanel.value || !previewPanel.value) return

  isScrollSyncing.value = true
  const source = sourcePanel.value
  const preview = previewPanel.value
  const scrollRatio = preview.scrollTop / (preview.scrollHeight - preview.clientHeight || 1)
  source.scrollTop = scrollRatio * (source.scrollHeight - source.clientHeight)

  requestAnimationFrame(() => {
    isScrollSyncing.value = false
  })
}

// 重置分割比例
const resetSplitRatio = () => {
  splitRatio.value = 50
}

defineExpose({
  resetSplitRatio,
})
</script>

<style scoped>
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
  height: 0;
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
  height: 0;
  min-height: 0;
}

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

.layout-vertical .source-panel {
  border-bottom: none;
}

.layout-vertical .preview-panel {
  border-top: none;
}

.layout-horizontal .source-panel {
  border-right: none;
}

.layout-horizontal .preview-panel {
  border-left: none;
}
</style>
