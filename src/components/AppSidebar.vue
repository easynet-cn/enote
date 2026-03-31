<template>
  <aside
    :class="[
      'flex flex-col bg-surface-alt transition-all duration-300 relative h-full',
      overlay ? 'w-72 shadow-2xl' : collapsed ? 'w-12' : 'w-64',
    ]"
    role="navigation"
    :aria-label="t('aria.sidebar')"
  >
    <!-- 覆盖层模式关闭按钮 -->
    <button
      v-if="overlay"
      @click="$emit('close-overlay')"
      class="absolute top-3 right-3 z-10 w-7 h-7 bg-surface border border-edge rounded-full shadow-sm flex items-center justify-center text-content-tertiary hover:text-indigo-600 transition-all hover:scale-110 active:scale-95"
      :aria-label="t('common.close')"
    >
      <X class="w-4 h-4" aria-hidden="true" />
    </button>

    <!-- 折叠/展开按钮（仅桌面模式） -->
    <button
      v-if="!overlay"
      @click="$emit('toggle-collapse')"
      class="absolute -right-3.5 top-1/2 -translate-y-1/2 z-10 w-7 h-7 bg-surface border border-edge rounded-full shadow-sm flex items-center justify-center text-content-tertiary hover:text-indigo-600 transition-all hover:scale-110 active:scale-95"
      :aria-label="collapsed ? t('sidebar.expand') : t('sidebar.collapse')"
      :title="collapsed ? t('sidebar.expand') : t('sidebar.collapse')"
    >
      <ChevronRight v-if="collapsed" class="w-4 h-4" aria-hidden="true" />
      <ChevronLeft v-else class="w-4 h-4" aria-hidden="true" />
    </button>

    <!-- 新建笔记按钮 -->
    <div :class="collapsed ? 'py-3 flex justify-center' : 'p-4 flex justify-center'">
      <button
        v-if="collapsed"
        @click="$emit('createNewNote')"
        :aria-label="t('editor.newNote')"
        :title="t('editor.newNote')"
        class="w-8 h-8 flex items-center justify-center rounded-lg bg-indigo-600 text-white hover:bg-indigo-700 transition-colors shadow-sm"
      >
        <Plus class="w-4 h-4" aria-hidden="true" />
      </button>
      <Button
        v-else
        type="primary"
        block
        @click="$emit('createNewNote')"
        :aria-label="t('editor.newNote')"
      >
        <template #icon>
          <Plus class="w-4 h-4" aria-hidden="true" />
        </template>
        {{ t('sidebar.createNotebook') }}
      </Button>
    </div>

    <template v-if="!collapsed">
      <!-- 笔记本区块：flex-1 独立滚动 -->
      <div class="flex-1 min-h-0 flex flex-col border-b border-edge">
        <div class="flex justify-between items-center px-4 pt-4 pb-2 shrink-0">
          <h2 class="text-sm font-semibold text-content-secondary uppercase tracking-wider">
            {{ t('sidebar.notebooks') }}
          </h2>
          <Dropdown ref="notebookDropdownRef" @command="handleNotebookCommand">
            <template #trigger>
              <Menu class="w-4 h-4 text-content-secondary hover:text-content cursor-pointer" />
            </template>
            <DropdownItem command="create" @command="handleNotebookCommand">
              <Plus class="w-4 h-4" />
              <span>{{ t('common.add') }}</span>
            </DropdownItem>
            <DropdownItem
              v-if="showNotebookEditAndDelete"
              command="edit"
              @command="handleNotebookCommand"
            >
              <Pencil class="w-4 h-4" />
              <span>{{ t('common.edit') }}</span>
            </DropdownItem>
            <DropdownItem
              v-if="showNotebookEditAndDelete"
              command="delete"
              @command="handleNotebookCommand"
            >
              <Trash2 class="w-4 h-4" />
              <span>{{ t('common.delete') }}</span>
            </DropdownItem>
          </Dropdown>
        </div>

        <ul
          ref="notebookListRef"
          role="listbox"
          :aria-label="t('sidebar.notebooks')"
          class="flex-1 overflow-y-auto px-4 pb-2"
        >
          <!-- "全部" 虚拟项 -->
          <li
            v-if="notebooks.length > 0"
            :data-id="notebooks[0].id"
            role="option"
            :aria-selected="activeNotebook === notebooks[0].id"
            class="sidebar-item"
            :class="{ active: activeNotebook === notebooks[0].id }"
            @click="$emit('setActiveNotebook', notebooks[0].id)"
            @keydown.enter="$emit('setActiveNotebook', notebooks[0].id)"
            tabindex="0"
          >
            <div class="flex items-center">
              <component
                v-if="notebooks[0].icon && iconComponents[notebooks[0].icon]"
                :is="iconComponents[notebooks[0].icon]"
                class="w-4 h-4 mr-3 text-content-secondary"
                aria-hidden="true"
              />
              <span class="flex-1">{{ notebooks[0].name }}</span>
              <span class="text-xs text-content-tertiary">{{ notebooks[0].count }}</span>
            </div>
          </li>

          <!-- 笔记本树 -->
          <NotebookTreeItem
            v-for="node in store.notebookTree"
            :key="node.id"
            :node="node"
            :active-notebook="activeNotebook"
            :depth="0"
            @select="$emit('setActiveNotebook', $event)"
            @toggle="handleNotebookToggle"
          />
        </ul>
      </div>

      <!-- 标签区块：flex-1 独立滚动 -->
      <div class="flex-1 min-h-0 flex flex-col">
        <div class="flex justify-between items-center px-4 pt-4 pb-2 shrink-0">
          <h2 class="text-sm font-semibold text-content-secondary uppercase tracking-wider">
            {{ t('sidebar.tags') }}
          </h2>
          <Dropdown ref="tagDropdownRef" @command="handleTagCommand">
            <template #trigger>
              <Menu class="w-4 h-4 text-content-secondary hover:text-content cursor-pointer" />
            </template>
            <DropdownItem command="create" @command="handleTagCommand">
              <Plus class="w-4 h-4" />
              <span>{{ t('common.add') }}</span>
            </DropdownItem>
            <DropdownItem v-if="showTagEditAndDelete" command="edit" @command="handleTagCommand">
              <Pencil class="w-4 h-4" />
              <span>{{ t('common.edit') }}</span>
            </DropdownItem>
            <DropdownItem v-if="showTagEditAndDelete" command="delete" @command="handleTagCommand">
              <Trash2 class="w-4 h-4" />
              <span>{{ t('common.delete') }}</span>
            </DropdownItem>
          </Dropdown>
        </div>

        <ul
          ref="tagListRef"
          class="space-y-1 flex-1 overflow-y-auto px-4 pb-2"
          role="listbox"
          :aria-label="t('sidebar.tags')"
        >
          <li
            v-for="tag in tags"
            :key="tag.id"
            :data-id="tag.id"
            v-memo="[tag.id, tag.name, tag.icon, tag.cls, tag.mcpAccess, activeTag === tag.id]"
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
                class="w-4 h-4 mr-3 text-content-secondary"
                aria-hidden="true"
              />
              <span v-else-if="tag.cls" :class="['mr-3', tag.cls]" aria-hidden="true">●</span>
              <span class="flex-1">{{ tag.name }}</span>
              <Shield
                v-if="tag.mcpAccess && tag.mcpAccess > 0"
                class="w-3 h-3 text-content-tertiary"
                :title="t('settings.mcpAccess')"
              />
            </div>
          </li>
        </ul>
      </div>

      <!-- 底部工具区域 -->
      <div class="h-12 px-4 border-t border-edge flex items-center gap-2">
        <Tooltip :content="t('sidebar.importNotes')" placement="top">
          <button
            class="flex items-center justify-center p-1.5 text-content-secondary hover:bg-surface-dim rounded-lg transition-colors cursor-pointer"
            @click="$emit('openImport')"
          >
            <Import class="w-4 h-4" />
          </button>
        </Tooltip>
        <Tooltip :content="t('sidebar.dataBackup')" placement="top">
          <button
            class="flex items-center justify-center p-1.5 text-content-secondary hover:bg-surface-dim rounded-lg transition-colors cursor-pointer"
            @click="$emit('openBackup')"
          >
            <Database class="w-4 h-4" />
          </button>
        </Tooltip>
        <Tooltip :content="t('template.title')" placement="top">
          <button
            class="flex items-center justify-center p-1.5 text-content-secondary hover:bg-surface-dim rounded-lg transition-colors cursor-pointer"
            @click="$emit('openTemplates')"
          >
            <LayoutTemplate class="w-4 h-4" />
          </button>
        </Tooltip>
        <Tooltip :content="t('trash.title')" placement="top">
          <button
            class="flex items-center justify-center p-1.5 text-content-secondary hover:bg-surface-dim rounded-lg transition-colors cursor-pointer"
            @click="$emit('openTrash')"
          >
            <Trash2 class="w-4 h-4" />
          </button>
        </Tooltip>
        <Tooltip :content="t('common.settings')" placement="top">
          <button
            class="flex items-center justify-center p-1.5 text-content-secondary hover:bg-surface-dim rounded-lg transition-colors cursor-pointer"
            @click="$emit('openSettings')"
          >
            <Settings class="w-4 h-4" />
          </button>
        </Tooltip>
      </div>
    </template>
  </aside>

  <!-- 笔记本编辑弹窗 -->
  <Dialog v-model="notebookDialog" :title="notebookDialogTitle" :width="500">
    <form @submit.prevent="submitNotebookForm" :aria-label="t('sidebar.notebookForm.title')">
      <div class="space-y-4">
        <div>
          <label for="notebook-name" class="block text-sm font-medium text-content-secondary mb-1"
            >{{ t('sidebar.notebookForm.nameLabel') }}
            <span class="text-red-500" aria-hidden="true">*</span></label
          >
          <input
            id="notebook-name"
            v-model="notebookForm.name"
            type="text"
            required
            aria-required="true"
            :placeholder="t('sidebar.notebookForm.namePlaceholder')"
            class="w-full px-3 py-2 border border-edge rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
          />
        </div>
        <div>
          <label
            for="notebook-parent"
            class="block text-sm font-medium text-content-secondary mb-1"
            >{{ t('sidebar.notebookForm.parentLabel') }}</label
          >
          <AppSelect
            v-model="notebookForm.parentId"
            :options="parentNotebookSelectOptions"
            size="md"
            class="w-full"
          />
        </div>
        <div>
          <label
            for="notebook-desc"
            class="block text-sm font-medium text-content-secondary mb-1"
            >{{ t('sidebar.notebookForm.descriptionLabel') }}</label
          >
          <input
            id="notebook-desc"
            v-model="notebookForm.description"
            type="text"
            :placeholder="t('sidebar.notebookForm.descriptionPlaceholder')"
            class="w-full px-3 py-2 border border-edge rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-content-secondary mb-2">{{
            t('common.icon')
          }}</label>
          <IconPicker
            v-model="notebookForm.icon"
            :placeholder="t('sidebar.selectIcon')"
            clearable
          />
        </div>
        <div>
          <label
            for="notebook-sort"
            class="block text-sm font-medium text-content-secondary mb-1"
            >{{ t('sidebar.notebookForm.sortOrderLabel') }}</label
          >
          <input
            id="notebook-sort"
            v-model.number="notebookForm.sortOrder"
            type="number"
            min="0"
            :placeholder="t('sidebar.notebookForm.sortOrderPlaceholder')"
            class="w-full px-3 py-2 border border-edge rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
          />
        </div>
        <div>
          <label
            for="notebook-mcp-access"
            class="block text-sm font-medium text-content-secondary mb-1"
            >{{ t('settings.mcpAccess') }}</label
          >
          <AppSelect
            v-model="notebookForm.mcpAccess"
            :options="mcpAccessOptions"
            size="md"
            class="w-full"
          />
        </div>
      </div>
    </form>
    <template #footer>
      <div class="flex justify-end gap-3">
        <Button type="secondary" @click="closeNotebookDialog">{{ t('common.cancel') }}</Button>
        <Button type="primary" @click="submitNotebookForm">{{ t('common.save') }}</Button>
      </div>
    </template>
  </Dialog>

  <!-- 标签编辑弹窗 -->
  <Dialog v-model="tagDialog" :title="tagDialogTitle" :width="500">
    <form @submit.prevent="submitTagForm" :aria-label="t('sidebar.tagForm.title')">
      <div class="space-y-4">
        <div>
          <label for="tag-name" class="block text-sm font-medium text-content-secondary mb-1"
            >{{ t('sidebar.tagForm.nameLabel') }}
            <span class="text-red-500" aria-hidden="true">*</span></label
          >
          <input
            id="tag-name"
            v-model="tagForm.name"
            type="text"
            required
            aria-required="true"
            :placeholder="t('sidebar.tagForm.namePlaceholder')"
            class="w-full px-3 py-2 border border-edge rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-content-secondary mb-2">{{
            t('common.icon')
          }}</label>
          <IconPicker
            v-model="tagForm.icon"
            :placeholder="t('sidebar.tagForm.iconPlaceholder')"
            clearable
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-content-secondary mb-2">{{
            t('sidebar.tagForm.colorLabel')
          }}</label>
          <StylePicker
            v-model="tagForm.cls"
            :placeholder="t('sidebar.tagForm.colorPlaceholder')"
            clearable
          />
        </div>
        <div>
          <label for="tag-sort" class="block text-sm font-medium text-content-secondary mb-1">{{
            t('sidebar.tagForm.sortOrderLabel')
          }}</label>
          <input
            id="tag-sort"
            v-model.number="tagForm.sortOrder"
            type="number"
            min="0"
            :placeholder="t('sidebar.tagForm.sortOrderPlaceholder')"
            class="w-full px-3 py-2 border border-edge rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
          />
        </div>
        <div>
          <label
            for="tag-mcp-access"
            class="block text-sm font-medium text-content-secondary mb-1"
            >{{ t('settings.mcpAccess') }}</label
          >
          <AppSelect
            v-model="tagForm.mcpAccess"
            :options="mcpAccessOptions"
            size="md"
            class="w-full"
          />
        </div>
      </div>
    </form>
    <template #footer>
      <div class="flex justify-end gap-3">
        <Button type="secondary" @click="closeTagDialog">{{ t('common.cancel') }}</Button>
        <Button type="primary" @click="submitTagForm">{{ t('common.save') }}</Button>
      </div>
    </template>
  </Dialog>

  <!-- 删除笔记本确认弹窗 -->
  <ConfirmDialog
    v-model="deleteNotebookConfirm"
    :title="t('sidebar.deleteNotebookConfirm.title')"
    :message="t('sidebar.deleteNotebookConfirm.message')"
    type="danger"
    :confirm-text="t('sidebar.deleteNotebookConfirm.confirmText')"
    @confirm="confirmDeleteNotebook"
  />

  <!-- 删除标签确认弹窗 -->
  <ConfirmDialog
    v-model="deleteTagConfirm"
    :title="t('sidebar.deleteTagConfirm.title')"
    :message="t('sidebar.deleteTagConfirm.message')"
    type="danger"
    :confirm-text="t('sidebar.deleteTagConfirm.confirmText')"
    @confirm="confirmDeleteTag"
  />
</template>

<script setup lang="ts">
import {
  Plus,
  Pencil,
  Trash2,
  Menu,
  ChevronLeft,
  ChevronRight,
  Import,
  Database,
  Settings,
  LayoutTemplate,
  Shield,
  X,
} from 'lucide-vue-next'
import {
  Button,
  IconPicker,
  StylePicker,
  Dialog,
  Dropdown,
  DropdownItem,
  ConfirmDialog,
  Tooltip,
  AppSelect,
} from './ui'
import type { AppSelectOption } from './ui'
import NotebookTreeItem from './NotebookTreeItem.vue'
import { iconComponents } from './ui/icons'
import type { ShowNotebook, ShowTag } from '../types'
import { McpAccess } from '../types'
import { useAppStore } from '../stores/app'
import { computed, reactive, ref, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import Sortable from 'sortablejs'

const { t } = useI18n()
const store = useAppStore()

const notebookListRef = ref<HTMLElement>()
const tagListRef = ref<HTMLElement>()
let notebookSortable: Sortable | null = null
let tagSortable: Sortable | null = null

interface NotebookForm {
  id: string
  parentId: number
  name: string
  description: string
  icon: string
  cls: string
  sortOrder: number
  mcpAccess: McpAccess
}

interface TagForm {
  id: string
  name: string
  icon: string
  cls: string
  sortOrder: number
  mcpAccess: McpAccess
}

const notebookForm = reactive<NotebookForm>({
  id: '',
  parentId: 0,
  name: '',
  description: '',
  icon: '',
  cls: '',
  sortOrder: 0,
  mcpAccess: McpAccess.Inherit,
})

const tagForm = reactive<TagForm>({
  id: '',
  name: '',
  icon: '',
  cls: '',
  sortOrder: 0,
  mcpAccess: McpAccess.Inherit,
})

const notebookDialog = ref(false)
const notebookDialogTitle = ref(t('sidebar.notebookForm.createTitle'))
const tagDialog = ref(false)
const tagDialogTitle = ref(t('sidebar.tagForm.createTitle'))

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
  activeNote: string | null
  collapsed: boolean
  mobile?: boolean
  overlay?: boolean
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
  openImport: []
  openBackup: []
  openSettings: []
  openTrash: []
  reorderNotebooks: [orders: [string, number][]]
  reorderTags: [orders: [string, number][]]
  openTemplates: []
  'close-overlay': []
}>()

// 父笔记本选项（排除自身和自身的子孙，避免循环）
const parentNotebookOptions = computed(() => {
  const editingId = notebookForm.id
  const allNotebooks = props.notebooks.filter((n) => n.id !== '0')

  // 收集当前笔记本的所有后代 ID
  const descendantIds = new Set<string>()
  if (editingId) {
    const collectDescendants = (parentId: string) => {
      for (const n of allNotebooks) {
        if (String(n.parentId ?? 0) === parentId) {
          descendantIds.add(n.id)
          collectDescendants(n.id)
        }
      }
    }
    collectDescendants(editingId)
  }

  return allNotebooks.filter((n) => n.id !== editingId && !descendantIds.has(n.id))
})

const parentNotebookSelectOptions = computed<AppSelectOption[]>(() => [
  { label: t('sidebar.notebookForm.noParent'), value: 0 },
  ...parentNotebookOptions.value.map((nb) => ({
    label: nb.name || '',
    value: Number(nb.id),
  })),
])

const mcpAccessOptions = computed<AppSelectOption[]>(() => [
  { label: t('settings.mcpAccessInherit'), value: 0 },
  { label: t('settings.mcpAccessReadWrite'), value: 1 },
  { label: t('settings.mcpAccessReadOnly'), value: 2 },
  { label: t('settings.mcpAccessDeny'), value: 3 },
])

const handleNotebookToggle = (id: string) => {
  store.toggleNotebookExpand(id)
}

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
    parentId: notebookForm.parentId,
    name: notebookForm.name,
    description: notebookForm.description,
    icon: notebookForm.icon,
    cls: notebookForm.cls,
    sortOrder: notebookForm.sortOrder,
    mcpAccess: notebookForm.mcpAccess,
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
    mcpAccess: tagForm.mcpAccess,
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
  notebookForm.parentId = 0
  notebookForm.name = ''
  notebookForm.description = ''
  notebookForm.icon = ''
  notebookForm.cls = ''
  notebookForm.sortOrder = 0
  notebookForm.mcpAccess = McpAccess.Inherit
}

const handleNotebookCommand = (command: string) => {
  notebookDropdownRef.value?.close()

  if (command === 'create') {
    resetNotebookForm()
    notebookDialogTitle.value = t('sidebar.notebookForm.createTitle')
    notebookDialog.value = true
  } else if (command === 'edit') {
    const notebook = props.notebooks.find((n) => n.id === props.activeNotebook)

    if (notebook) {
      notebookForm.id = notebook.id ?? ''
      notebookForm.parentId = notebook.parentId ?? 0
      notebookForm.name = notebook.name ?? ''
      notebookForm.description = notebook.description ?? ''
      notebookForm.icon = notebook.icon ?? ''
      notebookForm.cls = notebook.cls ?? ''
      notebookForm.sortOrder = notebook.sortOrder ?? 0
      notebookForm.mcpAccess = notebook.mcpAccess ?? McpAccess.Inherit
    }

    notebookDialogTitle.value = t('sidebar.notebookForm.editTitle')
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
  tagForm.mcpAccess = McpAccess.Inherit
}

const handleTagCommand = (command: string) => {
  tagDropdownRef.value?.close()

  if (command === 'create') {
    resetTagForm()
    tagDialogTitle.value = t('sidebar.tagForm.createTitle')
    tagDialog.value = true
  } else if (command === 'edit') {
    const tag = props.tags.find((t) => t.id === props.activeTag)

    if (tag) {
      tagForm.id = tag.id ?? ''
      tagForm.name = tag.name ?? ''
      tagForm.icon = tag.icon ?? ''
      tagForm.cls = tag.cls ?? ''
      tagForm.sortOrder = tag.sortOrder ?? 0
      tagForm.mcpAccess = tag.mcpAccess ?? McpAccess.Inherit
    }

    tagDialogTitle.value = t('sidebar.tagForm.editTitle')
    tagDialog.value = true
  } else if (command === 'delete') {
    deleteTagConfirm.value = true
  }
}

const confirmDeleteTag = () => {
  emit('deleteTag', props.activeTag)
}

// 拖拽排序初始化
const initSortable = () => {
  if (notebookListRef.value) {
    notebookSortable = Sortable.create(notebookListRef.value, {
      animation: 150,
      handle: '.sidebar-item',
      filter: '[data-id="0"]',
      onEnd: () => {
        const items = notebookListRef.value?.querySelectorAll('[data-id]')
        if (!items) return
        const orders: [string, number][] = []
        items.forEach((item, index) => {
          const id = item.getAttribute('data-id')
          if (id && id !== '0') {
            orders.push([id, items.length - index])
          }
        })
        if (orders.length > 0) {
          emit('reorderNotebooks', orders)
        }
      },
    })
  }

  if (tagListRef.value) {
    tagSortable = Sortable.create(tagListRef.value, {
      animation: 150,
      handle: '.sidebar-item',
      filter: '[data-id="0"]',
      onEnd: () => {
        const items = tagListRef.value?.querySelectorAll('[data-id]')
        if (!items) return
        const orders: [string, number][] = []
        items.forEach((item, index) => {
          const id = item.getAttribute('data-id')
          if (id && id !== '0') {
            orders.push([id, items.length - index])
          }
        })
        if (orders.length > 0) {
          emit('reorderTags', orders)
        }
      },
    })
  }
}

const destroySortable = () => {
  notebookSortable?.destroy()
  tagSortable?.destroy()
  notebookSortable = null
  tagSortable = null
}

watch(
  () => props.collapsed,
  (collapsed) => {
    if (!collapsed) {
      nextTick(initSortable)
    } else {
      destroySortable()
    }
  },
)

onMounted(() => {
  if (!props.collapsed) {
    nextTick(initSortable)
  }
})

onUnmounted(destroySortable)
</script>

<style scoped>
/* 折叠/展开过渡 */
.collapse-enter-active,
.collapse-leave-active {
  transition:
    max-height 0.2s ease,
    opacity 0.2s ease;
  overflow: hidden;
  max-height: 500px;
}

.collapse-enter-from,
.collapse-leave-to {
  max-height: 0;
  opacity: 0;
}
</style>
