<template>
  <div class="flex-1 flex flex-col overflow-hidden">
    <main class="flex-1 flex flex-col overflow-hidden p-4" role="main" aria-label="笔记编辑区">
      <div v-if="activeNote" class="flex items-center mb-4">
        <div class="flex-1">
          <input
            :value="activeNote.title"
            @input="$emit('updateNoteTitle', ($event.target as HTMLInputElement).value)"
            placeholder="笔记标题"
            :readonly="!editMode"
            aria-label="笔记标题"
            class="w-full text-xl font-bold border-0 outline-none bg-transparent focus:ring-0"
            :class="{ 'cursor-default': !editMode }"
          />
        </div>
        <div class="flex-shrink-0">
          <Dropdown ref="menuDropdownRef" @command="handleCommand">
            <template #trigger>
              <Menu class="w-5 h-5 text-gray-500 hover:text-gray-700 cursor-pointer" />
            </template>
            <DropdownItem v-if="!editMode" command="edit" @command="handleCommand">
              <Pencil class="w-4 h-4" />
              <span>编辑</span>
            </DropdownItem>
            <DropdownItem v-if="editMode" command="save" @command="handleCommand">
              <Check class="w-4 h-4" />
              <span>保存</span>
            </DropdownItem>
            <DropdownItem v-if="editMode" command="cancel" @command="handleCommand">
              <X class="w-4 h-4" />
              <span>取消</span>
            </DropdownItem>
            <DropdownItem command="delete" @command="handleCommand">
              <Trash2 class="w-4 h-4" />
              <span>删除</span>
            </DropdownItem>
            <DropdownItem v-if="editMode" command="setting" @command="handleCommand">
              <Settings class="w-4 h-4" />
              <span>设置</span>
            </DropdownItem>
            <DropdownItem command="history" @command="handleCommand">
              <Eye class="w-4 h-4" />
              <span>历史记录</span>
            </DropdownItem>
          </Dropdown>
        </div>
      </div>

      <!-- TipTap 编辑器工具栏 -->
      <div v-if="editMode && editor">
        <TiptapToolbar :editor="editor" />
      </div>

      <!-- TipTap 编辑器 -->
      <div v-if="activeNote" class="flex-1 overflow-hidden">
        <editor-content :editor="editor" :class="editorCls" />
      </div>
    </main>
  </div>

  <!-- 设置弹窗 -->
  <Dialog v-model="settingDialog" title="设置" :width="500">
    <div class="space-y-4" role="form" aria-label="笔记设置">
      <div>
        <label for="note-notebook" class="block text-sm font-medium text-gray-700 mb-1"
          >笔记本</label
        >
        <select
          id="note-notebook"
          v-model="settingForm.notebookId"
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
        >
          <option value="">请选择笔记本</option>
          <option v-for="notebook in notebooks" :key="notebook.id" :value="notebook.id">
            {{ notebook.name }}
          </option>
        </select>
      </div>
      <div>
        <span class="block text-sm font-medium text-gray-700 mb-1" id="tags-label">标签</span>
        <div class="flex flex-wrap gap-2" role="group" aria-labelledby="tags-label">
          <label
            v-for="tag in tags"
            :key="tag.id"
            class="flex items-center gap-1 px-2 py-1 border rounded cursor-pointer hover:bg-gray-50"
            :class="{ 'bg-green-50 border-green-500': settingForm.tagIds.includes(tag.id) }"
          >
            <input
              type="checkbox"
              :value="tag.id"
              v-model="settingForm.tagIds"
              class="sr-only"
              :aria-label="tag.name"
            />
            <span :class="tag.cls" aria-hidden="true">●</span>
            <span class="text-sm">{{ tag.name }}</span>
          </label>
        </div>
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end gap-2">
        <button
          @click="settingDialog = false"
          class="px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
        >
          取消
        </button>
        <button
          @click="handleSettingFormSubmit"
          class="px-4 py-2 text-white bg-green-500 rounded-md hover:bg-green-600 transition-colors"
        >
          保存
        </button>
      </div>
    </template>
  </Dialog>

  <History
    v-model:visible="historyVisible"
    v-model:data="historyData"
    v-model:current-page="currentPage"
    v-model:page-size="pageSize"
    v-model:total="total"
    @open="$emit('open')"
    @size-change="handleSizeChange"
    @current-change="handleCurrentChange"
  />

  <!-- 删除笔记确认弹窗 -->
  <ConfirmDialog
    v-model="deleteNoteConfirm"
    title="删除笔记"
    message="确定要删除这篇笔记吗？此操作不可恢复。"
    type="danger"
    confirm-text="删除"
    @confirm="confirmDeleteNote"
  />
</template>

<script setup lang="ts">
import { watch, onBeforeUnmount, ref, reactive, computed } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import TextAlign from '@tiptap/extension-text-align'
import { TextStyle } from '@tiptap/extension-text-style'
import Color from '@tiptap/extension-color'
import Highlight from '@tiptap/extension-highlight'
import Underline from '@tiptap/extension-underline'
import Link from '@tiptap/extension-link'
import Image from '@tiptap/extension-image'
import { Table } from '@tiptap/extension-table'
import { TableRow } from '@tiptap/extension-table-row'
import { TableCell } from '@tiptap/extension-table-cell'
import { TableHeader } from '@tiptap/extension-table-header'
import FontFamily from '@tiptap/extension-font-family'
import TaskList from '@tiptap/extension-task-list'
import TaskItem from '@tiptap/extension-task-item'
import TiptapToolbar from './TiptapToolbar.vue'
import { Pencil, Check, X, Trash2, Menu, Eye, Settings } from 'lucide-vue-next'
import { Dialog, Dropdown, DropdownItem, ConfirmDialog } from './ui'
import type { NoteHistory, ShowNote, ShowNotebook, ShowTag } from '../types'
import History from './History.vue'

interface Props {
  notebooks: ShowNotebook[]
  tags: ShowTag[]
  activeNote: ShowNote | null
  editMode: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  saveNote: []
  cancelEdit: []
  deleteNote: []
  toggleEditMode: []
  updateNoteTitle: [title: string]
  updateNoteContent: [content: string]
  sizeChange: [pageSize: number]
  currentChange: [currentPage: number]
  open: []
}>()

interface Setting {
  notebookId: string
  tagIds: string[]
}

const settingForm = reactive<Setting>({
  notebookId: '',
  tagIds: [],
})

const settingDialog = ref(false)
const deleteNoteConfirm = ref(false)
const historyData = defineModel<NoteHistory[]>('historyData')
const currentPage = defineModel<number>('currentPage')
const pageSize = defineModel<number>('pageSize')
const total = defineModel<number>('total')

const historyVisible = ref<boolean>(false)
const menuDropdownRef = ref()

// TipTap 编辑器实例
const editor = useEditor({
  extensions: [
    StarterKit,
    TextAlign.configure({
      types: ['heading', 'paragraph'],
    }),
    TextStyle,
    Color,
    Highlight.configure({ multicolor: true }),
    Underline,
    Link.configure({
      openOnClick: false,
      HTMLAttributes: {
        class: 'text-blue-500 underline cursor-pointer',
      },
    }),
    Image.configure({
      inline: true,
      allowBase64: true,
    }),
    Table.configure({
      resizable: true,
    }),
    TableRow,
    TableCell,
    TableHeader,
    FontFamily,
    TaskList,
    TaskItem.configure({
      nested: true,
    }),
  ],
  content: props.activeNote?.content || '',
  editable: false,
  onUpdate: ({ editor }) => {
    const html = editor.getHTML()
    emit('updateNoteContent', html)
  },
})

// 监听活动笔记变化
watch(
  () => props.activeNote,
  (newNote) => {
    if (editor.value && newNote) {
      editor.value.commands.setContent(newNote.content)

      settingForm.notebookId = props.activeNote?.notebookId ?? ''
      settingForm.tagIds = props.activeNote?.tags?.map((t) => t.id) ?? []
    }
  },
)

// 监听编辑模式变化
watch(
  () => props.editMode,
  (newMode) => {
    if (editor.value && newMode) {
      // 切换到编辑模式时，编辑器自动获得焦点
      setTimeout(() => {
        editor.value?.commands.focus()
        editor.value?.setEditable(true)
      }, 100)
    } else {
      editor.value?.setEditable(false)
    }
  },
)

const editorCls = computed(() => {
  return props.editMode ? 'tiptap-editor-edit' : 'tiptap-editor'
})

// 组件卸载时销毁编辑器
onBeforeUnmount(() => {
  if (editor.value) {
    editor.value.destroy()
  }
})

const handleCommand = (command: string) => {
  menuDropdownRef.value?.close()

  if (command === 'edit') {
    emit('toggleEditMode')
  } else if (command === 'save') {
    emit('saveNote')
  } else if (command === 'cancel') {
    emit('cancelEdit')
  } else if (command === 'delete') {
    deleteNoteConfirm.value = true
  } else if (command === 'setting') {
    settingDialog.value = true
  } else if (command === 'history') {
    historyVisible.value = true
  }
}

const handleSizeChange = (val: number) => {
  emit('sizeChange', val)
}

const handleCurrentChange = (val: number) => {
  emit('currentChange', val)
}

const handleSettingFormSubmit = () => {
  if (props.activeNote) {
    props.activeNote.notebookId = settingForm.notebookId
    props.activeNote.tags = props.tags.filter((t) => settingForm.tagIds.includes(t.id))
  }

  settingDialog.value = false
}

const confirmDeleteNote = () => {
  emit('deleteNote')
}
</script>

<style scoped>
.tiptap-editor {
  padding: 1.5rem;
  min-height: 500px;
  max-height: 92vh;
  overflow-y: auto;
}

.tiptap-editor-edit {
  padding: 1.5rem;
  min-height: 500px;
  max-height: 88vh;
  overflow-y: auto;
}

.tiptap-editor:focus {
  outline: none;
}

/* 为ProseMirror内容区域设置基本样式 */
:deep(.ProseMirror) {
  outline: none;
  line-height: 1.6;
}

:deep(.ProseMirror h1) {
  font-size: 2rem;
  font-weight: bold;
  margin: 1rem 0;
  color: #1f2937;
}

:deep(.ProseMirror h2) {
  font-size: 1.5rem;
  font-weight: bold;
  margin: 0.875rem 0;
  color: #374151;
}

:deep(.ProseMirror h3) {
  font-size: 1.25rem;
  font-weight: bold;
  margin: 0.75rem 0;
  color: #4b5563;
}

:deep(.ProseMirror h4) {
  font-size: 1.125rem;
  font-weight: 600;
  margin: 0.625rem 0;
  color: #4b5563;
}

:deep(.ProseMirror h5) {
  font-size: 1rem;
  font-weight: 600;
  margin: 0.5rem 0;
  color: #6b7280;
}

:deep(.ProseMirror h6) {
  font-size: 0.875rem;
  font-weight: 600;
  margin: 0.5rem 0;
  color: #6b7280;
}

:deep(.ProseMirror p) {
  margin-bottom: 0.75rem;
}

:deep(.ProseMirror ul),
:deep(.ProseMirror ol) {
  padding-left: 1.5rem;
  margin-bottom: 0.75rem;
}

:deep(.ProseMirror li) {
  margin-bottom: 0.25rem;
}

:deep(.ProseMirror blockquote) {
  border-left: 4px solid #e5e7eb;
  padding-left: 1rem;
  margin-left: 0;
  margin-right: 0;
  font-style: italic;
  margin-bottom: 0.75rem;
  color: #6b7280;
}

:deep(.ProseMirror code) {
  background-color: #f3f4f6;
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-family: 'Courier New', Courier, monospace;
  color: #dc2626;
}

:deep(.ProseMirror pre) {
  background-color: #1f2937;
  color: #f9fafb;
  padding: 0.75rem;
  border-radius: 0.5rem;
  overflow-x: auto;
  margin-bottom: 0.75rem;
}

:deep(.ProseMirror a) {
  color: #3b82f6;
  text-decoration: underline;
}

:deep(.ProseMirror mark) {
  background-color: #fef08a;
  padding: 0.125rem 0.25rem;
}

:deep(.ProseMirror table) {
  border-collapse: collapse;
  margin-bottom: 0.75rem;
  width: 100%;
}

:deep(.ProseMirror th),
:deep(.ProseMirror td) {
  border: 1px solid #d1d5db;
  padding: 0.5rem;
  text-align: left;
}

:deep(.ProseMirror th) {
  background-color: #f9fafb;
  font-weight: 600;
}

:deep(.ProseMirror img) {
  max-width: 100%;
  height: auto;
  border-radius: 0.375rem;
}

/* 任务列表样式 */
:deep(.ProseMirror ul[data-type='taskList']) {
  list-style: none;
  padding-left: 0;
}

:deep(.ProseMirror ul[data-type='taskList'] li) {
  display: flex;
  align-items: flex-start;
  gap: 0.5rem;
  margin-bottom: 0.25rem;
}

:deep(.ProseMirror ul[data-type='taskList'] li > label) {
  flex-shrink: 0;
  margin-top: 0.25rem;
}

:deep(.ProseMirror ul[data-type='taskList'] li > label input[type='checkbox']) {
  width: 1rem;
  height: 1rem;
  cursor: pointer;
  accent-color: #10b981;
}

:deep(.ProseMirror ul[data-type='taskList'] li > div) {
  flex: 1;
}

:deep(.ProseMirror ul[data-type='taskList'] li[data-checked='true'] > div) {
  text-decoration: line-through;
  color: #9ca3af;
}

/* 表格选中样式 */
:deep(.ProseMirror .selectedCell) {
  background-color: #dbeafe;
}

:deep(.ProseMirror .column-resize-handle) {
  background-color: #3b82f6;
  width: 4px;
  cursor: col-resize;
}

:deep(.ProseMirror .tableWrapper) {
  overflow-x: auto;
  margin-bottom: 0.75rem;
}

/* 链接悬停样式 */
:deep(.ProseMirror a:hover) {
  color: #2563eb;
}
</style>
