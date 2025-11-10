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
                <el-button type="success" :icon="Plus" size="small" text @click="notebookDialog = true">

                </el-button>
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
                <el-button type="success" :icon="Plus" size="small" text @click="tagDialog = true">

                </el-button>
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
        <el-form ref="notebookFormRef" :model="notebookForm" :rules="notebookRules" label-width="auto">
            <el-form-item label="名称" prop="name">
                <el-input v-model="notebookForm.name" />
            </el-form-item>
            <el-form-item label="描述" prop="description">
                <el-input v-model="notebookForm.description" />
            </el-form-item>
            <el-form-item label="图标" prop="icon">
                <el-input v-model="notebookForm.icon" />
            </el-form-item>
            <el-form-item label="样式" prop="cls">
                <el-input v-model="notebookForm.cls" />
            </el-form-item>
            <el-form-item label="排序" prop="sortOrder">
                <el-input v-model="notebookForm.sortOrder" />
            </el-form-item>
        </el-form>
        <template #footer>
            <div class="dialog-footer">
                <el-button @click="closeNotebookDialog">取消</el-button>
                <el-button type="primary" @click="submitNotebookForm(notebookFormRef)">
                    保存
                </el-button>
            </div>
        </template>
    </el-dialog>
    <el-dialog v-model="tagDialog" title="添加标签" width="500" align-center>
        <el-form ref="tagFormRef" :model="tagForm" :rules="tagRules" label-width="auto">
            <el-form-item label="名称" prop="name">
                <el-input v-model="tagForm.name" />
            </el-form-item>
            <el-form-item label="图标" prop="icon">
                <el-input v-model="tagForm.icon" />
            </el-form-item>
            <el-form-item label="样式" prop="cls">
                <el-input v-model="tagForm.cls" />
            </el-form-item>
            <el-form-item label="排序" prop="sortOrder">
                <el-input v-model="tagForm.sortOrder" />
            </el-form-item>
        </el-form>
        <template #footer>
            <div class="dialog-footer">
                <el-button @click="closeTagDialog">取消</el-button>
                <el-button type="primary" @click="submitTagForm(tagFormRef)">
                    保存
                </el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup lang="ts">
import { Plus } from '@element-plus/icons-vue'
import type { FormInstance, FormRules } from 'element-plus'
import type { ShowNotebook, ShowTag } from '../types'
import { reactive, ref } from 'vue';

interface NotebookForm {
    name: string
    description: string
    icon: string
    cls: string
    sortOrder: number
}

interface TagForm {
    name: string
    icon: string
    cls: string
    sortOrder: number
}

const notebookFormRef = ref<FormInstance>()
const notebookForm = reactive<NotebookForm>({
    name: '',
    description: '',
    icon: '',
    cls: '',
    sortOrder: 0
})
const notebookRules = reactive<FormRules<NotebookForm>>({
    name: [
        {
            required: true,
            message: '请输入名称',
            trigger: 'blur'
        }
    ]
})

const tagFormRef = ref<FormInstance>()
const tagForm = reactive<TagForm>({
    name: '',
    icon: '',
    cls: '',
    sortOrder: 0
})
const tagRules = reactive<FormRules<TagForm>>({
    name: [
        {
            required: true,
            message: '请输入名称',
            trigger: 'blur'
        }
    ]
})

const notebookDialog = ref(false);
const tagDialog = ref(false);

defineProps<{
    notebooks: ShowNotebook[]
    tags: ShowTag[]
    activeNotebook: string
}>()

const emit = defineEmits<{
    setActiveNotebook: [id: string]
    createNewNote: []
    saveNotebook: [notebook: ShowNotebook]
    saveTag: [tag: ShowTag]
}>()

const submitNotebookForm = async (form: FormInstance | undefined) => {
    if (!form) {
        return
    }

    await form.validate((valid) => {
        if (valid) {
            emit('saveNotebook', {
                id: '',
                name: notebookForm.name,
                description: notebookForm.description,
                icon: notebookForm.icon,
                cls: notebookForm.cls,
                sortOrder: notebookForm.sortOrder
            })

            notebookDialog.value = false
        }
    })
}

const submitTagForm = async (form: FormInstance | undefined) => {
    if (!form) {
        return
    }

    await form.validate((valid) => {
        if (valid) {
            emit('saveTag', {
                id: '',
                name: tagForm.name,
                icon: tagForm.icon,
                cls: tagForm.cls,
                sortOrder: tagForm.sortOrder
            })

            tagDialog.value = false
        }
    })
}

const closeNotebookDialog = () => {
    notebookFormRef.value?.resetFields()
    notebookDialog.value = false
}

const closeTagDialog = () => {
    tagFormRef.value?.resetFields()
    tagDialog.value = false
}

</script>