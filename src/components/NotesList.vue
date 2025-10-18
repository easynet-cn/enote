<template>
    <div class="w-80 bg-white border-r border-gray-200 flex flex-col">
        <div class="p-4 border-b border-gray-200">
            <div class="flex justify-between items-center mb-4">
                <h2 class="text-lg font-semibold text-gray-800">
                    {{ activeNotebookName }}
                </h2>
                <button
                    class="bg-evernote-green hover:bg-green-600 text-white px-3 py-2 rounded-lg flex items-center transition-colors duration-200"
                    @click="$emit('createNewNote')">
                    <span class="mr-1">+</span> 新建笔记
                </button>
            </div>

            <el-input :model-value="searchQuery" placeholder="搜索笔记..." :prefix-icon="Search" clearable
                @update:model-value="$emit('updateSearchQuery', $event)" />
        </div>

        <div class="flex-1 overflow-y-auto">
            <div v-for="note in notes" :key="note.id" class="note-item" :class="{ active: activeNote === note.id }"
                @click="$emit('setActiveNote', note.id)">
                <div class="font-semibold text-gray-800 mb-1 truncate">{{ note.title }}</div>
                <div class="text-sm text-gray-500 mb-2 line-clamp-2">{{ note.content }}</div>
                <div class="flex justify-between items-center text-xs text-gray-400">
                    <span>{{ formatDate(note.updatedAt) }}</span>
                    <div class="flex space-x-1">
                        <span v-for="tagId in note.tags" :key="tagId" :class="getTagColor(tagId)">
                            ●
                        </span>
                    </div>
                </div>
            </div>

            <div v-if="notes.length === 0" class="flex flex-col items-center justify-center h-full text-gray-400 p-8">
                <div class="text-5xl mb-4">📝</div>
                <p class="mb-4">暂无笔记</p>
                <button
                    class="bg-evernote-green hover:bg-green-600 text-white px-4 py-2 rounded-lg transition-colors duration-200"
                    @click="$emit('createNewNote')">
                    创建第一篇笔记
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Search } from '@element-plus/icons-vue'
import type { Notebook, Note } from '../types'

interface Props {
    notebooks: Notebook[]
    notes: Note[]
    activeNotebook: number
    activeNote: number | null
    searchQuery: string
}

const props = defineProps<Props>()

defineEmits<{
    setActiveNote: [id: number]
    createNewNote: []
    updateSearchQuery: [query: string]
}>()

const activeNotebookName = computed(() => {
    const notebook = props.notebooks.find(n => n.id === props.activeNotebook)
    return notebook ? notebook.name : ''
})

const formatDate = (dateStr: string) => {
    const date = new Date(dateStr)
    const now = new Date()
    const diffTime = Math.abs(now.getTime() - date.getTime())
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))

    if (diffDays === 1) return '昨天'
    if (diffDays === 2) return '前天'
    if (diffDays <= 7) return `${diffDays}天前`

    return dateStr.split(' ')[0]
}

// 模拟标签数据
const tags = [
    { id: 1, color: 'text-red-500' },
    { id: 2, color: 'text-yellow-500' },
    { id: 3, color: 'text-green-500' },
    { id: 4, color: 'text-blue-500' }
]

const getTagColor = (tagId: number) => {
    const tag = tags.find(t => t.id === tagId)
    return tag ? tag.color : 'text-gray-400'
}
</script>