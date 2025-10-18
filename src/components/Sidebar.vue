<template>
    <div class="w-64 bg-gray-50 border-r border-gray-200 flex flex-col">
        <div class="p-6 border-b border-gray-200">
            <h1 class="text-2xl font-bold text-evernote-green">易-笔记</h1>
        </div>

        <div class="p-4 border-b border-gray-200">
            <h2 class="text-sm font-semibold text-gray-500 mb-3">笔记本</h2>
            <ul class="space-y-1">
                <li v-for="notebook in notebooks" :key="notebook.id" class="sidebar-item"
                    :class="{ active: activeNotebook === notebook.id }"
                    @click="$emit('setActiveNotebook', notebook.id)">
                    <div class="flex items-center">
                        <span class="mr-3">{{ notebook.icon }}</span>
                        <span class="flex-1">{{ notebook.name }}</span>
                        <span class="text-xs text-gray-400">{{ notebook.count }}</span>
                    </div>
                </li>
            </ul>
        </div>

        <div class="p-4">
            <h2 class="text-sm font-semibold text-gray-500 mb-3">标签</h2>
            <ul class="space-y-1">
                <li v-for="tag in tags" :key="tag.id" class="sidebar-item">
                    <div class="flex items-center">
                        <span :class="['mr-3', tag.color]">●</span>
                        <span>{{ tag.name }}</span>
                    </div>
                </li>
            </ul>
        </div>
    </div>
</template>

<script setup lang="ts">
import type { Notebook, Tag } from '../types'

defineProps<{
    notebooks: Notebook[]
    tags: Tag[]
    activeNotebook: number
}>()

defineEmits<{
    setActiveNotebook: [id: number]
}>()
</script>