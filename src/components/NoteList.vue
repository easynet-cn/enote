<template>
  <el-aside class="w-80 bg-white border-r border-gray-200 flex flex-col">
    <div class="p-4 border-b border-gray-200">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-lg font-semibold text-gray-800">
          {{ activeNotebookName }}
        </h2>
      </div>

      <el-input
        v-model="query"
        placeholder="搜索笔记..."
        :prefix-icon="Search"
        clearable
        @keyup.enter="$emit('updateSearchQuery')"
      />
    </div>
    <el-scrollbar>
      <div
        v-for="note in notes"
        :key="note.id"
        class="note-item"
        :class="{ active: activeNote === note.id }"
        @click="$emit('setActiveNote', note.id)"
      >
        <div class="font-semibold text-gray-800 mb-1 truncate">
          {{ note.title }}
        </div>
        <div class="text-sm text-gray-500 mb-2 line-clamp-2" v-html="note.content"></div>
        <div class="flex justify-between items-center text-xs text-gray-400">
          <span>{{ note.updateTime }}</span>
        </div>
      </div>
    </el-scrollbar>
    <el-affix position="bottom" :offset="40">
      <el-pagination
        class="flex justify-center items-center"
        layout="prev, pager, next"
        v-model:current-page="currentPage"
        v-model:page-size="pageSize"
        :total="total"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
      />
    </el-affix>
  </el-aside>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Search } from '@element-plus/icons-vue'
import type { ShowNotebook, ShowNote } from '../types'

interface Props {
  notebooks: ShowNotebook[]
  notes: ShowNote[]
  activeNotebook: string
  activeNote: string | null
}

const props = defineProps<Props>()

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

const handleSizeChange = (val: number) => {
  emit('sizeChange', val)
}

const handleCurrentChange = (val: number) => {
  emit('currentChange', val)
}
</script>
