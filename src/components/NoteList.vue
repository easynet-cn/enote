<template>
  <aside
    class="bg-surface border-r border-edge flex flex-col relative shadow-sm"
    :class="{ 'transition-all duration-300': !isResizing }"
    :style="{ width: mobile ? '100%' : collapsed ? '48px' : `${width}px` }"
  >
    <!-- 拖拽调整宽度的边界（移动端隐藏） -->
    <div
      v-if="!collapsed && !mobile"
      class="absolute right-0 top-0 h-full w-1 cursor-ew-resize hover:bg-indigo-400 z-20 group"
      @mousedown="startResize"
    >
      <div
        class="absolute right-0 top-0 h-full w-1 bg-transparent group-hover:bg-indigo-400 transition-colors"
      ></div>
    </div>

    <!-- 折叠/展开按钮（右侧边界中间，移动端隐藏） -->
    <button
      v-if="!mobile"
      @click="$emit('toggle-collapse')"
      class="absolute -right-3.5 top-1/2 -translate-y-1/2 z-30 w-7 h-7 bg-surface border border-edge rounded-full shadow-sm flex items-center justify-center text-content-tertiary hover:text-indigo-600 transition-all hover:scale-110 active:scale-95"
      :aria-label="collapsed ? t('noteList.expandList') : t('noteList.collapseList')"
      :title="collapsed ? t('noteList.expandList') : t('noteList.collapseList')"
    >
      <ChevronRight v-if="collapsed" class="w-4 h-4" aria-hidden="true" />
      <ChevronLeft v-else class="w-4 h-4" aria-hidden="true" />
    </button>

    <template v-if="!collapsed">
      <div class="p-4 border-b border-edge">
        <div class="flex justify-between items-center mb-4">
          <div class="flex items-center gap-2">
            <button
              v-if="mobile"
              @click="$emit('open-sidebar')"
              class="p-1.5 -ml-1.5 text-content-secondary hover:text-content hover:bg-surface-dim rounded-lg transition-colors"
              :aria-label="t('sidebar.notebooks')"
            >
              <Menu class="w-5 h-5" />
            </button>
            <h2 class="text-lg font-semibold text-content">
              {{ activeNotebookName }}
            </h2>
          </div>
        </div>

        <div class="relative">
          <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-content-tertiary" />
          <input
            v-model="query"
            type="text"
            :placeholder="t('noteList.searchPlaceholder')"
            :aria-label="t('noteList.searchPlaceholder')"
            class="w-full pl-9 pr-8 py-2.5 bg-surface-alt border border-edge-light rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500/10 focus:border-indigo-500/50 transition-all"
            @keyup.enter="$emit('updateSearchQuery')"
          />
          <button
            v-if="query"
            @click="handlerQueryChange"
            :aria-label="t('noteList.clearSearch')"
            class="absolute right-3 top-1/2 -translate-y-1/2 text-content-tertiary hover:text-content-secondary"
          >
            <X class="w-4 h-4" />
          </button>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto">
        <!-- 加载骨架屏 -->
        <NoteListSkeleton v-if="isLoading" :count="5" />

        <!-- 空态 -->
        <div
          v-else-if="notes.length === 0"
          class="flex flex-col items-center justify-center h-full text-content-tertiary py-12"
        >
          <FileText class="w-12 h-12 mb-3 opacity-50" />
          <p class="text-sm">{{ emptyMessage }}</p>
        </div>

        <!-- 笔记列表（带过渡动画） -->
        <TransitionGroup
          v-else
          name="note-list"
          tag="ul"
          role="listbox"
          :aria-label="t('aria.noteList')"
          class="note-list-container"
        >
          <li
            v-for="(note, index) in notes"
            ref="noteItemRef"
            :key="note.id"
            v-memo="[note.id, note.title, note.updateTime, note.isPinned, activeNote === note.id]"
            role="option"
            :aria-selected="activeNote === note.id"
            tabindex="0"
            class="note-item group"
            :class="{ active: activeNote === note.id }"
            @click="$emit('setActiveNote', note.id)"
            @keydown.enter="$emit('setActiveNote', note.id)"
            @keydown.space.prevent="$emit('setActiveNote', note.id)"
            @keydown.up.prevent="focusPreviousNote(index)"
            @keydown.down.prevent="focusNextNote(index)"
          >
            <div class="flex items-center gap-1 mb-1">
              <Pin
                v-if="note.isPinned"
                class="w-3.5 h-3.5 text-indigo-500 shrink-0"
                :aria-label="t('pin.pinned')"
              />
              <div class="font-semibold text-content truncate">
                {{ note.title || t('noteList.noTitle') }}
              </div>
            </div>
            <div class="text-sm text-content-secondary mb-2 line-clamp-2">
              {{ getPreviewText(note) }}
            </div>
            <div class="flex justify-between items-center text-xs text-content-tertiary">
              <span class="truncate mr-2">{{ note.notebookName }}</span>
              <div class="flex items-center gap-2 shrink-0">
                <Tooltip :content="note.isPinned ? t('pin.unpin') : t('pin.pin')" placement="top">
                  <button
                    class="pin-btn opacity-0 group-hover:opacity-100 transition-opacity p-0.5 rounded hover:bg-surface-dim"
                    :class="{ '!opacity-100': note.isPinned }"
                    @click.stop="$emit('togglePin', note.id)"
                  >
                    <Pin
                      class="w-3 h-3"
                      :class="note.isPinned ? 'text-indigo-500' : 'text-content-tertiary'"
                    />
                  </button>
                </Tooltip>
                <span :aria-label="`${t('noteList.updateTime')}: ${note.updateTime}`">{{
                  note.updateTime
                }}</span>
              </div>
            </div>
          </li>
        </TransitionGroup>
      </div>

      <div
        v-if="notes.length > 0"
        class="sticky bottom-0 bg-surface border-t border-edge h-12 px-4 flex items-center justify-center"
      >
        <Pagination
          :current-page="currentPage!"
          :page-size="pageSize!"
          :total="total!"
          :simple="true"
          @current-change="handleCurrentChange"
        />
      </div>
    </template>
  </aside>
</template>

<script setup lang="ts">
import { computed, ref, onUnmounted, useTemplateRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { Search, X, FileText, ChevronLeft, ChevronRight, Pin, Menu } from 'lucide-vue-next'
import { Pagination, Tooltip } from './ui'
import NoteListSkeleton from './NoteListSkeleton.vue'
import { stripHtml, truncateText, markdownToHtml } from '../utils'
import { LRUCache } from '../utils/lruCache'
import { throttle } from '../utils/debounce'
import { ContentType, type ShowNotebook, type ShowNote } from '../types'
import { PREVIEW_CACHE_MAX_SIZE, PREVIEW_TEXT_MAX_LENGTH } from '../config/constants'
import { useAppStore } from '../stores/app'

const { t } = useI18n()

// 预览文本缓存（使用 LRUCache 自动淘汰最久未使用的项）
const previewTextCache = new LRUCache<string, string>(PREVIEW_CACHE_MAX_SIZE, 0)

// 生成缓存 key：使用 id + 内容哈希 + 内容类型作为标识
// 使用内容长度代替 updateTime，避免每次保存后缓存失效
const getCacheKey = (note: ShowNote): string => {
  return `${note.id}-${note.content.length}-${note.contentType ?? 0}`
}

interface Props {
  notebooks: ShowNotebook[]
  notes: ShowNote[]
  activeNotebook: string
  activeNote: string | null
  loading?: boolean
  collapsed?: boolean
  width?: number
  minWidth?: number
  maxWidth?: number
  mobile?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  collapsed: false,
  width: 320,
  minWidth: 200,
  maxWidth: 600,
  mobile: false,
})

const appStore = useAppStore()
const isLoading = computed(() => props.loading || appStore.notesLoading)

const query = defineModel<string>('query')
const currentPage = defineModel<number>('currentPage')
const pageSize = defineModel<number>('pageSize')
const total = defineModel<number>('total')

const emit = defineEmits<{
  sizeChange: [pageSize: number]
  currentChange: [currentPage: number]
  setActiveNote: [id: string]
  updateSearchQuery: []
  'toggle-collapse': []
  'update:width': [width: number]
  togglePin: [id: string]
  'open-sidebar': []
}>()

// 拖拽状态
const isResizing = ref(false)
const startX = ref(0)
const startWidth = ref(0)

// 节流处理拖拽移动（约 60fps）
const handleMouseMove = throttle((e: MouseEvent) => {
  if (!isResizing.value) return
  const delta = e.clientX - startX.value
  let newWidth = startWidth.value + delta
  newWidth = Math.max(props.minWidth, Math.min(props.maxWidth, newWidth))
  emit('update:width', newWidth)
}, 16)

const startResize = (e: MouseEvent) => {
  isResizing.value = true
  startX.value = e.clientX
  startWidth.value = props.width
  document.addEventListener('mousemove', handleMouseMove)
  document.addEventListener('mouseup', stopResize)
  document.body.style.cursor = 'ew-resize'
  document.body.style.userSelect = 'none'
}

const stopResize = () => {
  isResizing.value = false
  document.removeEventListener('mousemove', handleMouseMove)
  document.removeEventListener('mouseup', stopResize)
  document.body.style.cursor = ''
  document.body.style.userSelect = ''
}

onUnmounted(() => {
  document.removeEventListener('mousemove', handleMouseMove)
  document.removeEventListener('mouseup', stopResize)
})

const activeNotebookName = computed(() => {
  const notebook = props.notebooks.find((n) => n.id === props.activeNotebook)
  return notebook ? notebook.name : ''
})

const emptyMessage = computed(() => {
  if (query.value) {
    return t('noteList.noResults')
  }
  return t('noteList.empty')
})

/**
 * 获取预览文本（纯文本，无 HTML）
 * 如果是 Markdown 类型，先转换为 HTML 再提取纯文本
 * 使用 LRUCache 缓存避免重复的 Markdown 转换
 */
const getPreviewText = (note: ShowNote): string => {
  const cacheKey = getCacheKey(note)

  // 检查缓存（LRUCache 自动处理淘汰）
  const cached = previewTextCache.get(cacheKey)
  if (cached !== undefined) {
    return cached
  }

  // 计算预览文本
  let html = note.content
  if (note.contentType === ContentType.Markdown) {
    html = markdownToHtml(note.content)
  }
  const text = stripHtml(html)
  const result = truncateText(text, PREVIEW_TEXT_MAX_LENGTH) || t('noteList.noContent')

  // 存入缓存（LRUCache 自动淘汰旧项）
  previewTextCache.set(cacheKey, result)

  return result
}

const handleCurrentChange = (val: number) => {
  emit('currentChange', val)
}

const handlerQueryChange = () => {
  query.value = ''
  emit('updateSearchQuery')
}

// 笔记列表项 refs
const noteItemRefs = useTemplateRef<HTMLElement[]>('noteItemRef')

// 键盘导航：聚焦上一个笔记
const focusPreviousNote = (currentIndex: number) => {
  if (currentIndex > 0) {
    noteItemRefs.value?.[currentIndex - 1]?.focus()
  }
}

// 键盘导航：聚焦下一个笔记
const focusNextNote = (currentIndex: number) => {
  if (currentIndex < props.notes.length - 1) {
    noteItemRefs.value?.[currentIndex + 1]?.focus()
  }
}
</script>

<style scoped>
/* 笔记列表过渡动画 */
.note-list-move,
.note-list-enter-active,
.note-list-leave-active {
  transition: all 0.2s ease;
}

.note-list-enter-from {
  opacity: 0;
  transform: translateX(-10px);
}

.note-list-leave-to {
  opacity: 0;
  transform: translateX(10px);
}

/* 确保离开的元素不影响布局 */
.note-list-leave-active {
  position: absolute;
  width: 100%;
}

/* 笔记列表容器 */
.note-list-container {
  list-style: none;
  margin: 0;
  padding: 0;
}

/* 笔记项聚焦样式 */
.note-item:focus {
  outline: 2px solid var(--color-primary);
  outline-offset: -2px;
}

.note-item:focus-visible {
  outline: 2px solid var(--color-primary);
  outline-offset: -2px;
}
</style>
