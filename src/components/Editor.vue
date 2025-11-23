<template>
  <el-container>
    <el-main>
      <el-row v-if="activeNote">
        <el-col :span="23">
          <el-input
            :model-value="activeNote.title"
            @update:model-value="$emit('updateNoteTitle', $event)"
            placeholder="笔记标题"
            :readonly="!editMode"
            size="large"
            class="editor-title-input"
          />
        </el-col>
        <el-col :span="1">
          <div class="toolbar">
            <el-dropdown @command="handleCommand">
              <el-icon class="mr-2 mt-1">
                <icon-menu />
              </el-icon>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item v-if="!editMode" command="edit">
                    <el-icon>
                      <edit />
                    </el-icon>
                    <span>编辑</span>
                  </el-dropdown-item>
                  <el-dropdown-item v-if="editMode" command="save">
                    <el-icon>
                      <check />
                    </el-icon>
                    <span>保存</span>
                  </el-dropdown-item>
                  <el-dropdown-item v-if="editMode" command="cancel">
                    <el-icon>
                      <close />
                    </el-icon>
                    <span>取消</span>
                  </el-dropdown-item>
                  <el-dropdown-item command="delete">
                    <el-icon>
                      <delete />
                    </el-icon>
                    <span>删除</span>
                  </el-dropdown-item>
                  <el-dropdown-item v-if="editMode" command="setting">
                    <el-icon>
                      <setting />
                    </el-icon>
                    <span>设置</span>
                  </el-dropdown-item>
                  <el-dropdown-item command="history">
                    <el-icon>
                      <icon-view />
                    </el-icon>
                    <span>历史记录</span>
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </div>
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="24">
          <!-- TipTap 编辑器工具栏 -->
          <TiptapToolbar v-if="editMode && editor" :editor="editor" />
        </el-col>
      </el-row>
      <el-row>
        <!-- TipTap 编辑器 -->
        <el-col :span="24" v-if="activeNote" class="flex-1 overflow-hidden">
          <editor-content :editor="editor" class="tiptap-editor" />
        </el-col>
      </el-row>
    </el-main>
  </el-container>
  <el-dialog v-model="settingDialog" title="设置" width="500" align-center>
    <el-form :model="settingForm" label-width="auto">
      <el-form-item label="笔记本">
        <el-select v-model="settingForm.notebookId" placeholder="请选择笔记本" clearable>
          <el-option
            v-for="notebook in notebooks"
            :key="notebook.id"
            :label="notebook.name"
            :value="notebook.id"
          />
        </el-select>
      </el-form-item>
      <el-form-item label="标签">
        <el-select v-model="settingForm.tagIds" placeholder="请选择标签" multiple clearable>
          <el-option v-for="tag in tags" :key="tag.id" :label="tag.name" :value="tag.id" />
        </el-select>
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="settingDialog = false">取消</el-button>
        <el-button type="primary" @click="handleSettingFormSubmit"> 保存 </el-button>
      </div>
    </template>
  </el-dialog>
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
</template>

<script setup lang="ts">
import { watch, onBeforeUnmount, ref, reactive } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import TextAlign from '@tiptap/extension-text-align'
import { TextStyle } from '@tiptap/extension-text-style'
import Color from '@tiptap/extension-color'
import Highlight from '@tiptap/extension-highlight'
import Underline from '@tiptap/extension-underline'
import TiptapToolbar from './TiptapToolbar.vue'
import {
  Edit,
  Check,
  Close,
  Delete,
  Menu as IconMenu,
  View as IconView,
  Setting,
} from '@element-plus/icons-vue'
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
const historyData = defineModel<NoteHistory[]>('historyData')
const currentPage = defineModel<number>('currentPage')
const pageSize = defineModel<number>('pageSize')
const total = defineModel<number>('total')

const historyVisible = ref<boolean>(false)

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

// 组件卸载时销毁编辑器
onBeforeUnmount(() => {
  if (editor.value) {
    editor.value.destroy()
  }
})

const handleCommand = (command: string | number | object) => {
  if (command === 'edit') {
    emit('toggleEditMode')
  } else if (command === 'save') {
    emit('saveNote')
  } else if (command === 'cancel') {
    emit('cancelEdit')
  } else if (command === 'delete') {
    emit('deleteNote')
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
</script>

<style scoped>
.editor-title-input {
  font-size: 1.25rem;
  font-weight: bold;
}

:deep(.editor-title-input .el-input__wrapper) {
  box-shadow: none;
  padding: 0;
}

:deep(.editor-title-input .el-input__inner) {
  font-size: 1.25rem;
  font-weight: bold;
}

.tiptap-editor {
  padding: 1.5rem;
  min-height: 500px;
  max-height: calc(100vh - 200px);
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
</style>
