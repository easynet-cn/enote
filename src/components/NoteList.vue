<template>
  <aside
    class="bg-white border-r border-slate-200 flex flex-col relative shadow-sm"
    :class="{ 'transition-all duration-300': !isResizing }"
    :style="{ width: collapsed ? '48px' : `${width}px` }"
  >
    <!-- 拖拽调整宽度的边界 -->
    <div
      v-if="!collapsed"
      class="absolute right-0 top-0 h-full w-1 cursor-ew-resize hover:bg-indigo-400 z-20 group"
      @mousedown="startResize"
    >
      <div
        class="absolute right-0 top-0 h-full w-1 bg-transparent group-hover:bg-indigo-400 transition-colors"
      ></div>
    </div>

    <!-- 折叠/展开按钮（右侧边界中间） -->
    <button
      @click="$emit('toggle-collapse')"
      class="absolute -right-3.5 top-1/2 -translate-y-1/2 z-30 w-7 h-7 bg-white border border-slate-200 rounded-full shadow-sm flex items-center justify-center text-slate-400 hover:text-indigo-600 transition-all hover:scale-110 active:scale-95"
      :aria-label="collapsed ? '展开列表' : '收起列表'"
      :title="collapsed ? '展开列表' : '收起列表'"
    >
      <ChevronRight v-if="collapsed" class="w-4 h-4" aria-hidden="true" />
      <ChevronLeft v-else class="w-4 h-4" aria-hidden="true" />
    </button>

    <template v-if="!collapsed">
      <div class="p-4 border-b border-slate-200">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-lg font-semibold text-slate-900">
            {{ activeNotebookName }}
          </h2>
        </div>

        <div class="relative">
          <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400" />
          <input
            v-model="query"
            type="text"
            placeholder="搜索笔记..."
            aria-label="搜索笔记"
            class="w-full pl-9 pr-8 py-2.5 bg-slate-50 border border-slate-100 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500/10 focus:border-indigo-500/50 transition-all"
            @keyup.enter="$emit('updateSearchQuery')"
          />
          <button
            v-if="query"
            @click="handlerQueryChange"
            aria-label="清除搜索"
            class="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600"
          >
            <X class="w-4 h-4" />
          </button>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto">
        <!-- 空态 -->
        <div
          v-if="notes.length === 0"
          class="flex flex-col items-center justify-center h-full text-slate-400 py-12"
        >
          <FileText class="w-12 h-12 mb-3 opacity-50" />
          <p class="text-sm">{{ emptyMessage }}</p>
        </div>

        <!-- 笔记列表 -->
        <div
          v-else
          v-for="note in notes"
          :key="note.id"
          class="note-item"
          :class="{ active: activeNote === note.id }"
          @click="$emit('setActiveNote', note.id)"
        >
          <div class="font-semibold text-slate-900 mb-1 truncate">
            {{ note.title || '无标题' }}
          </div>
          <div class="text-sm text-slate-500 mb-2 line-clamp-2">
            {{ getPreviewText(note.content) }}
          </div>
          <div class="flex justify-between items-center text-xs text-slate-400">
            <span class="truncate mr-2">{{ note.notebookName }}</span>
            <span class="shrink-0">{{ note.updateTime }}</span>
          </div>
        </div>
      </div>

      <div
        v-if="notes.length > 0"
        class="sticky bottom-0 bg-white border-t border-slate-200 py-2 px-4"
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
import { computed, ref, onUnmounted } from 'vue'
import { Search, X, FileText, ChevronLeft, ChevronRight } from 'lucide-vue-next'
import { Pagination } from './ui'
import { stripHtml, truncateText } from '../utils'
import type { ShowNotebook, ShowNote } from '../types'

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
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  collapsed: false,
  width: 320,
  minWidth: 200,
  maxWidth: 600,
})

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
}>()

// 拖拽状态
const isResizing = ref(false)
const startX = ref(0)
const startWidth = ref(0)

const startResize = (e: MouseEvent) => {
  isResizing.value = true
  startX.value = e.clientX
  startWidth.value = props.width
  document.addEventListener('mousemove', handleMouseMove)
  document.addEventListener('mouseup', stopResize)
  document.body.style.cursor = 'ew-resize'
  document.body.style.userSelect = 'none'
}

const handleMouseMove = (e: MouseEvent) => {
  if (!isResizing.value) return
  const delta = e.clientX - startX.value
  let newWidth = startWidth.value + delta
  newWidth = Math.max(props.minWidth, Math.min(props.maxWidth, newWidth))
  emit('update:width', newWidth)
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
    return '没有找到匹配的笔记'
  }
  return '还没有笔记，点击创建新笔记'
})

/**
 * 获取预览文本（纯文本，无 HTML）
 */
const getPreviewText = (html: string): string => {
  const text = stripHtml(html)
  return truncateText(text, 80) || '无内容'
}

const handleCurrentChange = (val: number) => {
  emit('currentChange', val)
}

const handlerQueryChange = () => {
  query.value = ''
  emit('updateSearchQuery')
}
</script>
