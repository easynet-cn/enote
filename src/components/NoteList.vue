<template>
  <aside class="w-80 bg-white border-r border-gray-200 flex flex-col">
    <div class="p-4 border-b border-gray-200">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-lg font-semibold text-gray-800">
          {{ activeNotebookName }}
        </h2>
      </div>

      <div class="relative">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
        <input
          v-model="query"
          type="text"
          placeholder="搜索笔记..."
          aria-label="搜索笔记"
          class="w-full pl-9 pr-8 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
          @keyup.enter="$emit('updateSearchQuery')"
        />
        <button
          v-if="query"
          @click="handlerQueryChange"
          aria-label="清除搜索"
          class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
        >
          <X class="w-4 h-4" />
        </button>
      </div>
    </div>

    <div class="flex-1 overflow-y-auto">
      <!-- 空态 -->
      <div
        v-if="notes.length === 0"
        class="flex flex-col items-center justify-center h-full text-gray-400 py-12"
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
        <div class="font-semibold text-gray-800 mb-1 truncate">
          {{ note.title || '无标题' }}
        </div>
        <div class="text-sm text-gray-500 mb-2 line-clamp-2">
          {{ getPreviewText(note.content) }}
        </div>
        <div class="flex justify-between items-center text-xs text-gray-400">
          <span>{{ note.updateTime }}</span>
        </div>
      </div>
    </div>

    <div
      v-if="notes.length > 0"
      class="sticky bottom-0 bg-white border-t border-gray-200 py-2 px-4"
    >
      <Pagination
        :current-page="currentPage!"
        :page-size="pageSize!"
        :total="total!"
        :simple="true"
        @current-change="handleCurrentChange"
      />
    </div>
  </aside>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Search, X, FileText } from 'lucide-vue-next'
import { Pagination } from './ui'
import { stripHtml, truncateText } from '../utils'
import type { ShowNotebook, ShowNote } from '../types'

interface Props {
  notebooks: ShowNotebook[]
  notes: ShowNote[]
  activeNotebook: string
  activeNote: string | null
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
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
}>()

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
