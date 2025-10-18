<template>
    <div class="flex-1 flex flex-col bg-white">
        <div v-if="activeNote" class="border-b border-gray-200">
            <div class="p-4 flex justify-between items-center">
                <input :value="activeNote.title"
                    @input="$emit('updateNoteTitle', ($event.target as HTMLInputElement).value)"
                    class="text-xl font-bold w-full bg-transparent focus:outline-none" placeholder="笔记标题"
                    :readonly="!editMode" />
                <div class="flex space-x-2">
                    <el-button v-if="!editMode" @click="$emit('toggleEditMode')">编辑</el-button>
                    <el-button v-if="editMode" type="primary" @click="$emit('saveNote')">保存</el-button>
                    <el-button v-if="editMode" @click="$emit('cancelEdit')">取消</el-button>
                    <el-button type="danger" @click="$emit('deleteNote')">删除</el-button>
                </div>
            </div>

            <div v-if="editMode" class="px-4 py-2 border-t border-gray-200 flex space-x-2">
                <el-button-group>
                    <el-button class="editor-toolbar-button">粗体</el-button>
                    <el-button class="editor-toolbar-button">斜体</el-button>
                    <el-button class="editor-toolbar-button">下划线</el-button>
                </el-button-group>
                <el-button-group>
                    <el-button class="editor-toolbar-button">列表</el-button>
                    <el-button class="editor-toolbar-button">待办</el-button>
                </el-button-group>
                <el-button-group>
                    <el-button class="editor-toolbar-button">链接</el-button>
                    <el-button class="editor-toolbar-button">图片</el-button>
                    <el-button class="editor-toolbar-button">附件</el-button>
                </el-button-group>
            </div>
        </div>

        <div v-if="activeNote" class="flex-1 p-6 overflow-y-auto note-content-editable" :contenteditable="editMode"
            @input="$emit('updateNoteContent', ($event.target as HTMLElement).innerText)">
            {{ activeNote.content }}
        </div>

        <div v-if="!activeNote" class="flex flex-col items-center justify-center h-full text-gray-400 p-8">
            <div class="text-5xl mb-4">📖</div>
            <p>选择或创建一篇笔记开始编辑</p>
        </div>
    </div>
</template>

<script setup lang="ts">
import type { Note } from '../types'

interface Props {
    activeNote: Note | null
    editMode: boolean
}

defineProps<Props>()

defineEmits<{
    saveNote: []
    cancelEdit: []
    deleteNote: []
    toggleEditMode: []
    updateNoteTitle: [title: string]
    updateNoteContent: [content: string]
}>()
</script>