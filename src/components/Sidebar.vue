<template>
  <aside
    :class="[
      'border-r border-gray-200 flex flex-col bg-gray-50 transition-all duration-300 relative',
      collapsed ? 'w-12' : 'w-60',
    ]"
    role="navigation"
    aria-label="侧边栏导航"
  >
    <!-- 折叠/展开按钮（右侧边界中间） -->
    <button
      @click="$emit('toggle-collapse')"
      class="absolute -right-3 top-1/2 -translate-y-1/2 z-10 w-6 h-6 bg-white border border-gray-300 rounded-full shadow-sm flex items-center justify-center text-gray-500 hover:text-gray-700 hover:bg-gray-50 transition-colors"
      :aria-label="collapsed ? '展开侧边栏' : '收起侧边栏'"
      :title="collapsed ? '展开侧边栏' : '收起侧边栏'"
    >
      <ChevronRight v-if="collapsed" class="w-4 h-4" aria-hidden="true" />
      <ChevronLeft v-else class="w-4 h-4" aria-hidden="true" />
    </button>

    <!-- 新建笔记按钮 -->
    <div class="p-2 border-b border-gray-200 flex justify-center">
      <button
        v-if="collapsed"
        @click="$emit('createNewNote')"
        class="p-2 bg-green-500 text-white rounded-md hover:bg-green-600 transition-colors"
        aria-label="创建新笔记"
        title="创建新笔记"
      >
        <Plus class="w-5 h-5" aria-hidden="true" />
      </button>
      <button
        v-else
        @click="$emit('createNewNote')"
        class="flex items-center gap-2 px-4 py-2 bg-green-500 text-white rounded-full hover:bg-green-600 transition-colors"
        aria-label="创建新笔记"
      >
        <Plus class="w-4 h-4" aria-hidden="true" />
        创建新笔记
      </button>
    </div>

    <template v-if="!collapsed">
      <div class="p-4 border-b border-gray-200">
        <div class="flex justify-between items-center mb-3">
          <h2 class="text-sm font-semibold text-gray-500">笔记本</h2>
          <Dropdown ref="notebookDropdownRef" @command="handleNotebookCommand">
            <template #trigger>
              <Menu class="w-4 h-4 text-gray-500 hover:text-gray-700 cursor-pointer" />
            </template>
            <DropdownItem command="create" @command="handleNotebookCommand">
              <Plus class="w-4 h-4" />
              <span>添加</span>
            </DropdownItem>
            <DropdownItem
              v-if="showNotebookEditAndDelete"
              command="edit"
              @command="handleNotebookCommand"
            >
              <Pencil class="w-4 h-4" />
              <span>编辑</span>
            </DropdownItem>
            <DropdownItem
              v-if="showNotebookEditAndDelete"
              command="delete"
              @command="handleNotebookCommand"
            >
              <Trash2 class="w-4 h-4" />
              <span>删除</span>
            </DropdownItem>
          </Dropdown>
        </div>

        <ul role="listbox" aria-label="笔记本列表">
          <li
            v-for="notebook in notebooks"
            :key="notebook.id"
            role="option"
            :aria-selected="activeNotebook === notebook.id"
            class="sidebar-item"
            :class="{ active: activeNotebook === notebook.id }"
            @click="$emit('setActiveNotebook', notebook.id)"
            @keydown.enter="$emit('setActiveNotebook', notebook.id)"
            tabindex="0"
          >
            <div class="flex items-center">
              <component
                v-if="notebook.icon && iconComponents[notebook.icon]"
                :is="iconComponents[notebook.icon]"
                class="w-4 h-4 mr-3 text-gray-500"
                aria-hidden="true"
              />
              <span v-else-if="notebook.cls" :class="['mr-3', notebook.cls]" aria-hidden="true"
                >●</span
              >
              <span class="flex-1">{{ notebook.name }}</span>
              <span class="text-xs text-gray-400" aria-label="笔记数量">{{ notebook.count }}</span>
            </div>
          </li>
        </ul>
      </div>

      <div class="p-4 flex-1 overflow-y-auto">
        <div class="flex justify-between items-center mb-3">
          <h2 class="text-sm font-semibold text-gray-500">标签</h2>
          <Dropdown ref="tagDropdownRef" @command="handleTagCommand">
            <template #trigger>
              <Menu class="w-4 h-4 text-gray-500 hover:text-gray-700 cursor-pointer" />
            </template>
            <DropdownItem command="create" @command="handleTagCommand">
              <Plus class="w-4 h-4" />
              <span>添加</span>
            </DropdownItem>
            <DropdownItem v-if="showTagEditAndDelete" command="edit" @command="handleTagCommand">
              <Pencil class="w-4 h-4" />
              <span>编辑</span>
            </DropdownItem>
            <DropdownItem v-if="showTagEditAndDelete" command="delete" @command="handleTagCommand">
              <Trash2 class="w-4 h-4" />
              <span>删除</span>
            </DropdownItem>
          </Dropdown>
        </div>

        <ul class="space-y-1" role="listbox" aria-label="标签列表">
          <li
            v-for="tag in tags"
            :key="tag.id"
            role="option"
            :aria-selected="activeTag === tag.id"
            :class="['sidebar-item', { active: activeTag === tag.id }]"
            @click="$emit('setActiveTag', tag.id)"
            @keydown.enter="$emit('setActiveTag', tag.id)"
            tabindex="0"
          >
            <div class="flex items-center">
              <component
                v-if="tag.icon && iconComponents[tag.icon]"
                :is="iconComponents[tag.icon]"
                class="w-4 h-4 mr-3 text-gray-500"
                aria-hidden="true"
              />
              <span v-else-if="tag.cls" :class="['mr-3', tag.cls]" aria-hidden="true">●</span>
              <span>{{ tag.name }}</span>
            </div>
          </li>
        </ul>
      </div>

    </template>
  </aside>

  <!-- 笔记本编辑弹窗 -->
  <Dialog v-model="notebookDialog" :title="notebookDialogTitle" :width="500">
    <form @submit.prevent="submitNotebookForm" aria-label="笔记本表单">
      <div class="space-y-4">
        <div>
          <label for="notebook-name" class="block text-sm font-medium text-gray-700 mb-1"
            >名称 <span class="text-red-500" aria-hidden="true">*</span></label
          >
          <input
            id="notebook-name"
            v-model="notebookForm.name"
            type="text"
            required
            aria-required="true"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
          />
        </div>
        <div>
          <label for="notebook-desc" class="block text-sm font-medium text-gray-700 mb-1"
            >描述</label
          >
          <input
            id="notebook-desc"
            v-model="notebookForm.description"
            type="text"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
          />
        </div>
        <div>
          <label for="notebook-icon" class="block text-sm font-medium text-gray-700 mb-1"
            >图标</label
          >
          <input
            id="notebook-icon"
            v-model="notebookForm.icon"
            type="text"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
          />
        </div>
        <div>
          <label for="notebook-cls" class="block text-sm font-medium text-gray-700 mb-1"
            >样式</label
          >
          <input
            id="notebook-cls"
            v-model="notebookForm.cls"
            type="text"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
          />
        </div>
        <div>
          <label for="notebook-sort" class="block text-sm font-medium text-gray-700 mb-1"
            >排序</label
          >
          <input
            id="notebook-sort"
            v-model.number="notebookForm.sortOrder"
            type="number"
            min="0"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
          />
        </div>
      </div>
    </form>
    <template #footer>
      <div class="flex justify-end gap-2">
        <button
          @click="closeNotebookDialog"
          class="px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
        >
          取消
        </button>
        <button
          @click="submitNotebookForm"
          class="px-4 py-2 text-white bg-green-500 rounded-md hover:bg-green-600 transition-colors"
        >
          保存
        </button>
      </div>
    </template>
  </Dialog>

  <!-- 标签编辑弹窗 -->
  <Dialog v-model="tagDialog" :title="tagDialogTitle" :width="500">
    <form @submit.prevent="submitTagForm" aria-label="标签表单">
      <div class="space-y-4">
        <div>
          <label for="tag-name" class="block text-sm font-medium text-gray-700 mb-1"
            >名称 <span class="text-red-500" aria-hidden="true">*</span></label
          >
          <input
            id="tag-name"
            v-model="tagForm.name"
            type="text"
            required
            aria-required="true"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
          />
        </div>
        <div>
          <label for="tag-icon" class="block text-sm font-medium text-gray-700 mb-1">图标</label>
          <input
            id="tag-icon"
            v-model="tagForm.icon"
            type="text"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
          />
        </div>
        <div>
          <label for="tag-cls" class="block text-sm font-medium text-gray-700 mb-1">样式</label>
          <input
            id="tag-cls"
            v-model="tagForm.cls"
            type="text"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
          />
        </div>
        <div>
          <label for="tag-sort" class="block text-sm font-medium text-gray-700 mb-1">排序</label>
          <input
            id="tag-sort"
            v-model.number="tagForm.sortOrder"
            type="number"
            min="0"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
          />
        </div>
      </div>
    </form>
    <template #footer>
      <div class="flex justify-end gap-2">
        <button
          @click="closeTagDialog"
          class="px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
        >
          取消
        </button>
        <button
          @click="submitTagForm"
          class="px-4 py-2 text-white bg-green-500 rounded-md hover:bg-green-600 transition-colors"
        >
          保存
        </button>
      </div>
    </template>
  </Dialog>

  <!-- 删除笔记本确认弹窗 -->
  <ConfirmDialog
    v-model="deleteNotebookConfirm"
    title="删除笔记本"
    message="确定要删除这个笔记本吗？笔记本下的所有笔记也将被删除，此操作不可恢复。"
    type="danger"
    confirm-text="删除"
    @confirm="confirmDeleteNotebook"
  />

  <!-- 删除标签确认弹窗 -->
  <ConfirmDialog
    v-model="deleteTagConfirm"
    title="删除标签"
    message="确定要删除这个标签吗？此操作不可恢复。"
    type="danger"
    confirm-text="删除"
    @confirm="confirmDeleteTag"
  />
</template>

<script setup lang="ts">
import {
  Plus,
  Pencil,
  Trash2,
  Menu,
  BookOpen,
  Book,
  Tags,
  Tag,
  Folder,
  FileText,
  ChevronLeft,
  ChevronRight,
} from 'lucide-vue-next'
import type { Component } from 'vue'

// Lucide 图标映射
const iconComponents: Record<string, Component> = {
  BookOpen,
  Book,
  Tags,
  Tag,
  Folder,
  FileText,
}
import { Dialog, Dropdown, DropdownItem, ConfirmDialog } from './ui'
import type { ShowNotebook, ShowTag } from '../types'
import { computed, reactive, ref } from 'vue'

interface NotebookForm {
  id: string
  name: string
  description: string
  icon: string
  cls: string
  sortOrder: number
}

interface TagForm {
  id: string
  name: string
  icon: string
  cls: string
  sortOrder: number
}

const notebookForm = reactive<NotebookForm>({
  id: '',
  name: '',
  description: '',
  icon: '',
  cls: '',
  sortOrder: 0,
})

const tagForm = reactive<TagForm>({
  id: '',
  name: '',
  icon: '',
  cls: '',
  sortOrder: 0,
})

const notebookDialog = ref(false)
const notebookDialogTitle = ref('添加笔记本')
const tagDialog = ref(false)
const tagDialogTitle = ref('添加标签')

// 删除确认弹窗状态
const deleteNotebookConfirm = ref(false)
const deleteTagConfirm = ref(false)

const notebookDropdownRef = ref()
const tagDropdownRef = ref()

const props = defineProps<{
  notebooks: ShowNotebook[]
  tags: ShowTag[]
  activeNotebook: string
  activeTag: string
  collapsed: boolean
}>()

const emit = defineEmits<{
  setActiveNotebook: [id: string]
  createNewNote: []
  saveNotebook: [notebook: ShowNotebook]
  deleteNotebook: [id: string]
  setActiveTag: [id: string]
  saveTag: [tag: ShowTag]
  deleteTag: [id: string]
  'toggle-collapse': []
}>()

const showNotebookEditAndDelete = computed(() => {
  return props.notebooks.length > 0 && props.activeNotebook !== '' && props.activeNotebook !== '0'
})
const showTagEditAndDelete = computed(() => {
  return props.tags.length > 0 && props.activeTag !== '' && props.activeTag !== '0'
})

const submitNotebookForm = () => {
  if (!notebookForm.name.trim()) {
    return
  }

  emit('saveNotebook', {
    id: notebookForm.id,
    name: notebookForm.name,
    description: notebookForm.description,
    icon: notebookForm.icon,
    cls: notebookForm.cls,
    sortOrder: notebookForm.sortOrder,
  })

  notebookDialog.value = false
}

const submitTagForm = () => {
  if (!tagForm.name.trim()) {
    return
  }

  emit('saveTag', {
    id: tagForm.id,
    name: tagForm.name,
    icon: tagForm.icon,
    cls: tagForm.cls,
    sortOrder: tagForm.sortOrder,
  })

  tagDialog.value = false
}

const closeNotebookDialog = () => {
  resetNotebookForm()
  notebookDialog.value = false
}

const closeTagDialog = () => {
  resetTagForm()
  tagDialog.value = false
}

const resetNotebookForm = () => {
  notebookForm.id = ''
  notebookForm.name = ''
  notebookForm.description = ''
  notebookForm.icon = ''
  notebookForm.cls = ''
  notebookForm.sortOrder = 0
}

const handleNotebookCommand = (command: string) => {
  notebookDropdownRef.value?.close()

  if (command === 'create') {
    resetNotebookForm()
    notebookDialogTitle.value = '添加笔记本'
    notebookDialog.value = true
  } else if (command === 'edit') {
    let notebook = props.notebooks.find((n) => n.id === props.activeNotebook)

    if (notebook) {
      notebookForm.id = notebook.id ?? ''
      notebookForm.name = notebook.name ?? ''
      notebookForm.description = notebook.description ?? ''
      notebookForm.icon = notebook.icon ?? ''
      notebookForm.cls = notebook.cls ?? ''
      notebookForm.sortOrder = notebook.sortOrder ?? 0
    }

    notebookDialogTitle.value = '编辑笔记本'
    notebookDialog.value = true
  } else if (command === 'delete') {
    deleteNotebookConfirm.value = true
  }
}

const confirmDeleteNotebook = () => {
  emit('deleteNotebook', props.activeNotebook)
}

const resetTagForm = () => {
  tagForm.id = ''
  tagForm.name = ''
  tagForm.icon = ''
  tagForm.cls = ''
  tagForm.sortOrder = 0
}

const handleTagCommand = (command: string) => {
  tagDropdownRef.value?.close()

  if (command === 'create') {
    resetTagForm()
    tagDialogTitle.value = '添加标签'
    tagDialog.value = true
  } else if (command === 'edit') {
    let tag = props.tags.find((t) => t.id === props.activeTag)

    if (tag) {
      tagForm.id = tag.id ?? ''
      tagForm.name = tag.name ?? ''
      tagForm.icon = tag.icon ?? ''
      tagForm.cls = tag.cls ?? ''
      tagForm.sortOrder = tag.sortOrder ?? 0
    }

    tagDialogTitle.value = '编辑标签'
    tagDialog.value = true
  } else if (command === 'delete') {
    deleteTagConfirm.value = true
  }
}

const confirmDeleteTag = () => {
  emit('deleteTag', props.activeTag)
}
</script>
