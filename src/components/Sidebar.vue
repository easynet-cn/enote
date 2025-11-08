<template>
    <el-aside>
        <el-row class="p-2 border-b border-gray-200" justify="center">
            <el-col :span="12">
                <el-button type="success" size="large" @click="$emit('createNewNote')" :icon="Plus" round>
                    创建新笔记
                </el-button>
            </el-col>
        </el-row>
        <el-row class="p-4 border-b border-gray-200">
            <el-col :span="22">
                <h2 class="text-sm font-semibold text-gray-500 mb-3">笔记本</h2>
            </el-col>
            <el-col :span="2">
                <el-icon @click="notebookDialog = true">
                    <Plus />
                </el-icon>
            </el-col>
            <el-col :span="24">
                <ul>
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
            </el-col>
        </el-row>
        <el-row class="p-4">
            <el-col :span="22">
                <h2 class="text-sm font-semibold text-gray-500 mb-3">标签</h2>
            </el-col>
            <el-col :span="2">
                <el-icon @click="tagDialog = true">
                    <Plus />
                </el-icon>
            </el-col>
            <el-col :span="24">
                <ul class="space-y-1">
                    <li v-for="tag in tags" :key="tag.id" class="sidebar-item">
                        <div class="flex items-center">
                            <span :class="['mr-3', tag.cls]">●</span>
                            <span>{{ tag.name }}</span>
                        </div>
                    </li>
                </ul>
            </el-col>
        </el-row>
    </el-aside>
    <el-dialog v-model="notebookDialog" title="添加笔记本" width="500" align-center>
        <el-form>

        </el-form>
        <template #footer>
            <div class="dialog-footer">
                <el-button @click="notebookDialog = false">取消</el-button>
                <el-button type="primary" @click="notebookDialog = false">
                    保存
                </el-button>
            </div>
        </template>
    </el-dialog>
    <el-dialog v-model="tagDialog" title="添加标签" width="500" align-center>
        <el-form>

        </el-form>
        <template #footer>
            <div class="dialog-footer">
                <el-button @click="tagDialog = false">取消</el-button>
                <el-button type="primary" @click="tagDialog = false">
                    保存
                </el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup lang="ts">
import { Plus } from '@element-plus/icons-vue'
import type { ShowNotebook, ShowTag } from '../types'
import { ref } from 'vue';

const notebookDialog = ref(false);
const tagDialog = ref(false);

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