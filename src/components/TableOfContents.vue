<template>
  <div
    v-if="visible && items.length > 0"
    class="toc-panel"
    role="navigation"
    :aria-label="t('aria.tocNavigation')"
  >
    <div class="toc-header">
      <h3 class="toc-title">{{ t('toc.title') }}</h3>
      <button
        class="toc-close"
        @click="$emit('close')"
        :aria-label="t('common.close')"
        :title="t('common.close')"
      >
        <X class="w-4 h-4" />
      </button>
    </div>
    <nav class="toc-content">
      <ul class="toc-list">
        <li v-for="item in items" :key="item.id" :class="['toc-item', `toc-level-${item.level}`]">
          <a
            href="#"
            class="toc-link"
            :class="{ 'toc-link-active': activeId === item.id }"
            @click.prevent="scrollToHeading(item)"
          >
            {{ item.text || t('toc.untitledHeading') }}
          </a>
        </li>
      </ul>
    </nav>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { X } from 'lucide-vue-next'
import type { Editor } from '@tiptap/vue-3'

const { t } = useI18n()

export interface TocItem {
  id: string
  level: number
  text: string
  pos: number
}

interface Props {
  items: TocItem[]
  visible: boolean
  editor: Editor | null
}

const props = defineProps<Props>()

defineEmits<{
  close: []
}>()

const activeId = ref<string>('')

// 点击目录项滚动到对应标题
const scrollToHeading = (item: TocItem) => {
  if (!props.editor) return

  // 设置选区到标题位置
  props.editor.chain().focus().setTextSelection(item.pos).run()

  // 滚动到视图
  const { view } = props.editor
  const coords = view.coordsAtPos(item.pos)
  const editorContainer = view.dom.closest('.tiptap-editor, .tiptap-editor-edit')

  if (editorContainer) {
    const containerRect = editorContainer.getBoundingClientRect()
    const scrollTop = editorContainer.scrollTop + coords.top - containerRect.top - 100
    editorContainer.scrollTo({
      top: scrollTop,
      behavior: 'smooth',
    })
  }

  activeId.value = item.id
}

// 监听 items 变化，重置 activeId
watch(
  () => props.items,
  () => {
    if (props.items.length > 0 && !props.items.find((i) => i.id === activeId.value)) {
      activeId.value = ''
    }
  },
)
</script>

<style scoped>
.toc-panel {
  position: absolute;
  top: 0;
  right: 0;
  width: 240px;
  max-height: 100%;
  background: var(--color-bg-primary);
  border-left: 1px solid var(--color-border);
  box-shadow: -4px 0 12px rgba(0, 0, 0, 0.05);
  z-index: 20;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.toc-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid var(--color-border);
  background: var(--color-bg-secondary);
}

.toc-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
}

.toc-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: var(--color-text-secondary);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.toc-close:hover {
  background: var(--color-border);
  color: var(--color-text-primary);
}

.toc-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px 0;
}

.toc-list {
  list-style: none;
  margin: 0;
  padding: 0;
}

.toc-item {
  margin: 0;
}

.toc-link {
  display: block;
  padding: 6px 16px;
  font-size: 13px;
  color: var(--color-text-secondary);
  text-decoration: none;
  transition: all 0.15s ease;
  border-left: 2px solid transparent;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.toc-link:hover {
  color: var(--color-text-primary);
  background: var(--color-bg-tertiary);
}

.toc-link-active {
  color: var(--color-primary);
  background: var(--color-primary-lighter);
  border-left-color: var(--color-primary);
}

/* 缩进层级 */
.toc-level-1 .toc-link {
  padding-left: 16px;
  font-weight: 600;
}

.toc-level-2 .toc-link {
  padding-left: 28px;
  font-weight: 500;
}

.toc-level-3 .toc-link {
  padding-left: 40px;
}

.toc-level-4 .toc-link {
  padding-left: 52px;
  font-size: 12px;
}

.toc-level-5 .toc-link {
  padding-left: 64px;
  font-size: 12px;
}

.toc-level-6 .toc-link {
  padding-left: 76px;
  font-size: 12px;
}
</style>
