<template>
  <aside
    class="bg-surface border-r border-edge flex flex-col relative shadow-sm h-full"
    :class="{ 'transition-all duration-300': !isResizing }"
    :style="noteListStyle"
  >
    <!-- 拖拽调整宽度的边界（仅桌面模式） -->
    <div
      v-if="!collapsed && layout === 'desktop'"
      class="absolute right-0 top-0 h-full w-1 cursor-ew-resize hover:bg-indigo-400 z-20 group"
      @mousedown="startResize"
    >
      <div
        class="absolute right-0 top-0 h-full w-1 bg-transparent group-hover:bg-indigo-400 transition-colors"
      ></div>
    </div>

    <!-- 折叠/展开按钮（仅桌面模式） -->
    <button
      v-if="layout === 'desktop'"
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
              v-if="layout !== 'desktop'"
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

        <div class="flex items-center gap-2">
          <div class="relative flex-1">
            <Search
              class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-content-tertiary"
            />
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

          <!-- 多选按钮 -->
          <button
            @click="toggleSelectMode"
            class="p-2.5 bg-surface-alt border border-edge-light rounded-xl hover:bg-surface-dim transition-colors"
            :class="{
              'text-indigo-600 border-indigo-500/50': appStore.isSelectMode,
              'text-content-tertiary': !appStore.isSelectMode,
            }"
            :aria-label="t('noteList.selectMode')"
            :title="t('noteList.selectMode')"
          >
            <CheckSquare class="w-4 h-4" />
          </button>

          <!-- 排序按钮 -->
          <Dropdown ref="sortDropdownRef">
            <template #trigger>
              <button
                class="p-2.5 bg-surface-alt border border-edge-light rounded-xl hover:bg-surface-dim transition-colors"
                :class="{
                  'text-indigo-600 border-indigo-500/50': isCustomSort,
                  'text-content-tertiary': !isCustomSort,
                }"
                :aria-label="t('noteList.sortBy')"
                :title="t('noteList.sortBy')"
              >
                <ArrowUpDown class="w-4 h-4" />
              </button>
            </template>

            <div
              class="px-2 py-1.5 text-xs font-medium text-content-tertiary uppercase tracking-wide"
            >
              {{ t('noteList.sortBy') }}
            </div>
            <DropdownItem @click="handleSortChange('update_time')">
              <template #icon>
                <Check
                  v-if="appStore.noteSortField === 'update_time'"
                  class="w-4 h-4 text-indigo-600"
                />
                <span v-else class="w-4 h-4" />
              </template>
              {{ t('noteList.sortUpdateTime') }}
            </DropdownItem>
            <DropdownItem @click="handleSortChange('create_time')">
              <template #icon>
                <Check
                  v-if="appStore.noteSortField === 'create_time'"
                  class="w-4 h-4 text-indigo-600"
                />
                <span v-else class="w-4 h-4" />
              </template>
              {{ t('noteList.sortCreateTime') }}
            </DropdownItem>
            <DropdownItem @click="handleSortChange('title')">
              <template #icon>
                <Check v-if="appStore.noteSortField === 'title'" class="w-4 h-4 text-indigo-600" />
                <span v-else class="w-4 h-4" />
              </template>
              {{ t('noteList.sortTitle') }}
            </DropdownItem>

            <div class="my-1 border-t border-edge"></div>

            <DropdownItem @click="handleOrderChange('desc')">
              <template #icon>
                <Check v-if="appStore.noteSortOrder === 'desc'" class="w-4 h-4 text-indigo-600" />
                <span v-else class="w-4 h-4" />
              </template>
              {{ t('noteList.sortDesc') }}
            </DropdownItem>
            <DropdownItem @click="handleOrderChange('asc')">
              <template #icon>
                <Check v-if="appStore.noteSortOrder === 'asc'" class="w-4 h-4 text-indigo-600" />
                <span v-else class="w-4 h-4" />
              </template>
              {{ t('noteList.sortAsc') }}
            </DropdownItem>
          </Dropdown>
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
            v-memo="[
              note.id,
              note.title,
              note.updateTime,
              note.isPinned,
              note.isStarred,
              activeNote === note.id,
              appStore.isSelectMode,
              appStore.selectedNotes.has(note.id),
            ]"
            role="option"
            :aria-selected="activeNote === note.id"
            tabindex="0"
            class="note-item group"
            :class="{
              active: activeNote === note.id && !appStore.isSelectMode,
              selected: appStore.isSelectMode && appStore.selectedNotes.has(note.id),
            }"
            @click="handleNoteClick(note.id)"
            @keydown.enter="handleNoteClick(note.id)"
            @keydown.space.prevent="handleNoteClick(note.id)"
            @keydown.up.prevent="focusPreviousNote(index)"
            @keydown.down.prevent="focusNextNote(index)"
          >
            <div class="flex items-center gap-2 mb-1">
              <!-- 多选复选框 -->
              <button
                v-if="appStore.isSelectMode"
                class="shrink-0 flex items-center justify-center"
                @click.stop="appStore.toggleNoteSelection(note.id)"
              >
                <CheckSquare
                  v-if="appStore.selectedNotes.has(note.id)"
                  class="w-4 h-4 text-indigo-600"
                />
                <Square v-else class="w-4 h-4 text-content-tertiary" />
              </button>
              <Pin
                v-if="note.isPinned"
                class="w-3.5 h-3.5 text-indigo-500 shrink-0"
                :aria-label="t('pin.pinned')"
              />
              <Star
                v-if="note.isStarred"
                class="w-3.5 h-3.5 text-amber-500 shrink-0 fill-amber-500"
                :aria-label="t('star.starred')"
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
                <Tooltip
                  :content="note.isStarred ? t('star.unstar') : t('star.star')"
                  placement="top"
                >
                  <button
                    class="star-btn opacity-0 group-hover:opacity-100 transition-opacity p-0.5 rounded hover:bg-surface-dim"
                    :class="{ '!opacity-100': note.isStarred }"
                    @click.stop="$emit('toggleStar', note.id)"
                  >
                    <Star
                      class="w-3 h-3"
                      :class="
                        note.isStarred ? 'text-amber-500 fill-amber-500' : 'text-content-tertiary'
                      "
                    />
                  </button>
                </Tooltip>
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

      <!-- 批量操作栏 -->
      <div
        v-if="appStore.isSelectMode && appStore.selectedNotes.size > 0"
        class="sticky bottom-0 bg-surface border-t border-edge px-3 py-2 flex items-center justify-between gap-2 z-10"
      >
        <div class="flex items-center gap-2">
          <span class="text-sm text-content-secondary whitespace-nowrap">
            {{ t('noteList.selectedCount', { count: appStore.selectedNotes.size }) }}
          </span>
          <button
            @click="appStore.selectAllNotes()"
            class="text-xs text-indigo-600 hover:text-indigo-700 whitespace-nowrap"
          >
            {{ t('noteList.selectAll') }}
          </button>
        </div>
        <div class="flex items-center gap-1.5">
          <!-- 移动到笔记本 -->
          <Dropdown ref="batchMoveDropdownRef">
            <template #trigger>
              <button
                class="inline-flex items-center gap-1 px-2.5 py-1.5 text-xs font-medium bg-surface-alt border border-edge-light rounded-lg hover:bg-surface-dim transition-colors text-content-secondary"
              >
                <Move class="w-3.5 h-3.5" />
                {{ t('noteList.batchMove') }}
              </button>
            </template>
            <DropdownItem
              v-for="nb in moveTargetNotebooks"
              :key="nb.id"
              @click="handleBatchMove(nb.id)"
            >
              {{ nb.name }}
            </DropdownItem>
          </Dropdown>

          <!-- 批量删除 -->
          <button
            @click="handleBatchDelete"
            class="inline-flex items-center gap-1 px-2.5 py-1.5 text-xs font-medium bg-red-50 border border-red-200 text-red-600 rounded-lg hover:bg-red-100 transition-colors"
          >
            <Trash2 class="w-3.5 h-3.5" />
            {{ t('noteList.batchDelete') }}
          </button>

          <!-- 取消 -->
          <button
            @click="appStore.clearSelection()"
            class="px-2.5 py-1.5 text-xs font-medium text-content-tertiary hover:text-content-secondary transition-colors"
          >
            {{ t('noteList.cancelSelect') }}
          </button>
        </div>
      </div>

      <!-- 分页 -->
      <div
        v-else-if="notes.length > 0"
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
import {
  Search,
  X,
  FileText,
  ChevronLeft,
  ChevronRight,
  Pin,
  Star,
  Menu,
  ArrowUpDown,
  Check,
  CheckSquare,
  Square,
  Move,
  Trash2,
} from 'lucide-vue-next'
import { Pagination, Tooltip, Dropdown, DropdownItem } from './ui'
import NoteListSkeleton from './NoteListSkeleton.vue'
import { stripHtml, truncateText, markdownToHtml } from '../utils'
import { parseId } from '../utils/validation'
import { noteApi } from '../api/note'
import { showNotification } from './ui/notification'
import { LRUCache } from '../utils/lruCache'
import { throttle } from '../utils/debounce'
import { ContentType, type ShowNotebook, type ShowNote } from '../types'
import type { LayoutMode } from '../composables/usePlatform'
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
  layout?: LayoutMode
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  collapsed: false,
  width: 320,
  minWidth: 200,
  maxWidth: 600,
  mobile: false,
  layout: 'desktop',
})

// 根据布局模式计算列表宽度样式
const noteListStyle = computed(() => {
  if (props.layout === 'mobile') return { width: '100%' }
  if (props.layout === 'tablet') return { width: '320px' }
  // 桌面模式
  return { width: props.collapsed ? '48px' : `${props.width}px` }
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
  toggleStar: [id: string]
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

// 排序相关
const sortDropdownRef = ref<InstanceType<typeof Dropdown> | null>(null)

const isCustomSort = computed(() => {
  return appStore.noteSortField !== 'update_time' || appStore.noteSortOrder !== 'desc'
})

const handleSortChange = (field: string) => {
  appStore.noteSortField = field
  sortDropdownRef.value?.close()
  emit('updateSearchQuery')
}

const handleOrderChange = (order: string) => {
  appStore.noteSortOrder = order
  sortDropdownRef.value?.close()
  emit('updateSearchQuery')
}

// 多选模式
const toggleSelectMode = () => {
  appStore.toggleSelectMode()
}

const handleNoteClick = (noteId: string) => {
  if (appStore.isSelectMode) {
    appStore.toggleNoteSelection(noteId)
  } else {
    emit('setActiveNote', noteId)
  }
}

// 移动目标笔记本（排除 "全部"）
const moveTargetNotebooks = computed(() => {
  return props.notebooks.filter((nb) => nb.id !== '0')
})

const batchMoveDropdownRef = ref<InstanceType<typeof Dropdown> | null>(null)

const handleBatchMove = async (notebookId: string) => {
  const ids = Array.from(appStore.selectedNotes).map((id) => parseId(id))
  try {
    await noteApi.batchMoveNotes(ids, parseId(notebookId))
    appStore.clearSelection()
    emit('updateSearchQuery')
  } catch (error) {
    showNotification({ type: 'error', message: String(error) })
  }
  batchMoveDropdownRef.value?.close()
}

const handleBatchDelete = async () => {
  const count = appStore.selectedNotes.size
  if (!confirm(t('noteList.batchDeleteConfirm', { count }))) return
  const ids = Array.from(appStore.selectedNotes).map((id) => parseId(id))
  try {
    await noteApi.batchDeleteNotes(ids)
    appStore.clearSelection()
    emit('updateSearchQuery')
  } catch (error) {
    showNotification({ type: 'error', message: String(error) })
  }
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

/* 笔记项选中样式（多选模式） */
.note-item.selected {
  background-color: var(--color-primary-lighter, #eef2ff);
  border-left: 3px solid var(--color-primary, #4f46e5);
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
