<template>
    <el-aside>
        <div class="p-6 border-b border-gray-200">
            <el-button type="success" size="large" @click="$emit('createNewNote')" :icon="Plus" round>
                创建新笔记
            </el-button>
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
                        <span :class="['mr-3', tag.cls]">●</span>
                        <span>{{ tag.name }}</span>
                    </div>
                </li>
            </ul>
        </div>
    </el-aside>
</template>

<script setup lang="ts">
import { Plus } from '@element-plus/icons-vue'
import type { ShowNotebook, ShowTag } from '../types'


defineProps<{
    notebooks: ShowNotebook[]
    tags: ShowTag[]
    activeNotebook: string
}>()

defineEmits<{
    setActiveNotebook: [id: string]
    createNewNote: []
}>()

</script>