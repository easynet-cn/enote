<template>
  <Dialog v-model="visible" :title="dialogTitle" :width="viewMode === 'editor' ? 800 : 600">
    <!-- Level 1: Template List -->
    <div v-if="viewMode === 'list'" class="space-y-4">
      <button
        @click="handleNewTemplate"
        class="w-full px-4 py-2 text-sm border border-dashed border-edge rounded-lg text-content-secondary hover:border-indigo-400 hover:text-indigo-600 transition-colors"
      >
        + {{ t('template.create') }}
      </button>

      <div v-if="templates.length === 0" class="text-center py-8 text-content-tertiary text-sm">
        {{ t('template.noTemplates') }}
      </div>
      <div v-else class="space-y-2 max-h-80 overflow-y-auto">
        <div
          v-for="tpl in templates"
          :key="tpl.id"
          class="flex items-center justify-between p-3 rounded-lg border border-edge hover:bg-surface-alt transition-colors"
        >
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2">
              <span class="text-sm font-medium text-content truncate">{{ tpl.name }}</span>
              <span
                class="text-[10px] px-1.5 py-0.5 rounded font-medium shrink-0"
                :class="
                  tpl.contentType === 1 ? 'bg-slate-800 text-white' : 'bg-indigo-50 text-indigo-600'
                "
              >
                {{ tpl.contentType === 1 ? 'MD' : 'RT' }}
              </span>
            </div>
            <div class="text-xs text-content-tertiary mt-0.5">{{ tpl.updateTime }}</div>
          </div>
          <div class="flex items-center gap-2 ml-3">
            <button
              @click="$emit('use-template', tpl)"
              class="px-3 py-1 text-xs bg-indigo-50 text-indigo-600 rounded-md hover:bg-indigo-100 transition-colors"
            >
              {{ t('template.useTemplate') }}
            </button>
            <button
              @click="handleEditTemplate(tpl)"
              class="px-3 py-1 text-xs text-indigo-600 hover:bg-indigo-50 rounded-md transition-colors"
            >
              {{ t('common.edit') }}
            </button>
            <button
              @click="handleDelete(tpl)"
              class="px-3 py-1 text-xs text-red-600 hover:bg-red-50 rounded-md transition-colors"
            >
              {{ t('common.delete') }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Level 2: Template Editor -->
    <div v-else class="space-y-4">
      <button
        @click="handleBack"
        class="flex items-center gap-1 text-sm text-content-secondary hover:text-content transition-colors"
      >
        <ArrowLeft class="w-4 h-4" />
        {{ t('template.back') }}
      </button>

      <div class="flex gap-2">
        <input
          v-model="templateName"
          :placeholder="t('template.namePlaceholder')"
          class="flex-1 px-3 py-2 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
        />
        <div class="flex gap-1 bg-surface-dim rounded-lg p-1">
          <button
            @click="switchContentType(0)"
            class="px-3 py-1.5 text-xs rounded-md transition-colors"
            :class="
              templateContentType === 0
                ? 'bg-indigo-50 text-indigo-700 shadow-sm'
                : 'text-content-secondary hover:text-content'
            "
          >
            {{ t('editor.contentType.richText') }}
          </button>
          <button
            @click="switchContentType(1)"
            class="px-3 py-1.5 text-xs rounded-md transition-colors"
            :class="
              templateContentType === 1
                ? 'bg-slate-800 text-white shadow-sm'
                : 'text-content-secondary hover:text-content'
            "
          >
            Markdown
          </button>
        </div>
      </div>

      <div class="template-editor-container border border-edge rounded-lg overflow-hidden">
        <!-- 富文本工具栏 -->
        <div v-if="templateEditor && templateContentType === 0" class="template-toolbar">
          <select
            :value="currentHeading"
            @change="setHeading(($event.target as HTMLSelectElement).value)"
            class="h-7 px-1.5 text-xs border border-edge rounded bg-surface text-content"
          >
            <option value="0">{{ t('editor.headingOptions.normal') }}</option>
            <option value="1">H1</option>
            <option value="2">H2</option>
            <option value="3">H3</option>
          </select>

          <div class="toolbar-divider" />

          <button
            :class="['t-btn', { active: templateEditor.isActive('bold') }]"
            @click="templateEditor.chain().focus().toggleBold().run()"
            title="Bold"
          >
            <Bold class="w-3.5 h-3.5" />
          </button>
          <button
            :class="['t-btn', { active: templateEditor.isActive('italic') }]"
            @click="templateEditor.chain().focus().toggleItalic().run()"
            title="Italic"
          >
            <Italic class="w-3.5 h-3.5" />
          </button>
          <button
            :class="['t-btn', { active: templateEditor.isActive('underline') }]"
            @click="templateEditor.chain().focus().toggleUnderline().run()"
            title="Underline"
          >
            <UnderlineIcon class="w-3.5 h-3.5" />
          </button>
          <button
            :class="['t-btn', { active: templateEditor.isActive('strike') }]"
            @click="templateEditor.chain().focus().toggleStrike().run()"
            title="Strikethrough"
          >
            <Strikethrough class="w-3.5 h-3.5" />
          </button>

          <div class="toolbar-divider" />

          <button
            :class="['t-btn', { active: templateEditor.isActive('bulletList') }]"
            @click="templateEditor.chain().focus().toggleBulletList().run()"
            title="Bullet List"
          >
            <List class="w-3.5 h-3.5" />
          </button>
          <button
            :class="['t-btn', { active: templateEditor.isActive('orderedList') }]"
            @click="templateEditor.chain().focus().toggleOrderedList().run()"
            title="Ordered List"
          >
            <ListOrdered class="w-3.5 h-3.5" />
          </button>
          <button
            :class="['t-btn', { active: templateEditor.isActive('taskList') }]"
            @click="templateEditor.chain().focus().toggleTaskList().run()"
            title="Task List"
          >
            <ListChecks class="w-3.5 h-3.5" />
          </button>

          <div class="toolbar-divider" />

          <button
            :class="['t-btn', { active: templateEditor.isActive('blockquote') }]"
            @click="templateEditor.chain().focus().toggleBlockquote().run()"
            title="Blockquote"
          >
            <Quote class="w-3.5 h-3.5" />
          </button>
          <button
            :class="['t-btn', { active: templateEditor.isActive('codeBlock') }]"
            @click="templateEditor.chain().focus().toggleCodeBlock().run()"
            title="Code Block"
          >
            <Code class="w-3.5 h-3.5" />
          </button>
          <button
            class="t-btn"
            @click="templateEditor.chain().focus().setHorizontalRule().run()"
            title="Horizontal Rule"
          >
            <Minus class="w-3.5 h-3.5" />
          </button>

          <div class="toolbar-divider" />

          <button
            class="t-btn"
            @click="templateEditor.chain().focus().undo().run()"
            :disabled="!templateEditor.can().undo()"
            title="Undo"
          >
            <Undo class="w-3.5 h-3.5" />
          </button>
          <button
            class="t-btn"
            @click="templateEditor.chain().focus().redo().run()"
            :disabled="!templateEditor.can().redo()"
            title="Redo"
          >
            <Redo class="w-3.5 h-3.5" />
          </button>
        </div>

        <EditorContent v-if="templateEditor" :editor="templateEditor" />
      </div>
    </div>

    <template #footer>
      <div class="flex justify-end gap-2">
        <template v-if="viewMode === 'editor'">
          <Button type="secondary" @click="handleBack">{{ t('common.cancel') }}</Button>
          <Button type="primary" @click="handleSave" :disabled="!templateName.trim()">{{
            t('common.save')
          }}</Button>
        </template>
        <template v-else>
          <Button type="secondary" @click="visible = false">{{ t('common.close') }}</Button>
        </template>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, shallowRef, watch, onBeforeUnmount, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { Editor, EditorContent } from '@tiptap/vue-3'
import {
  ArrowLeft,
  Bold,
  Italic,
  Underline as UnderlineIcon,
  Strikethrough,
  List,
  ListOrdered,
  ListChecks,
  Quote,
  Code,
  Minus,
  Undo,
  Redo,
} from 'lucide-vue-next'
import { Dialog, Button } from './ui'
import { templateApi } from '../api/note'
import { showNotification } from './ui/notification'
import { parseError } from '../utils/errorHandler'
import { getRichTextExtensions, getMarkdownExtensions } from '../config/editorExtensions'
import { ContentType } from '../types'
import type { NoteTemplate } from '../types'

const { t } = useI18n()

const visible = defineModel<boolean>({ default: false })
defineEmits<{
  'use-template': [template: NoteTemplate]
}>()

type ViewMode = 'list' | 'editor'
const viewMode = ref<ViewMode>('list')
const editingTemplate = ref<NoteTemplate | null>(null)
const templateName = ref('')
const templates = ref<NoteTemplate[]>([])
const templateEditor = shallowRef<Editor | undefined>(undefined)
const templateContentType = ref<number>(0)

const currentHeading = computed(() => {
  if (!templateEditor.value) return '0'
  for (let i = 1; i <= 3; i++) {
    if (templateEditor.value.isActive('heading', { level: i })) return String(i)
  }
  return '0'
})

const setHeading = (value: string) => {
  if (!templateEditor.value) return
  const level = parseInt(value)
  if (level === 0) {
    templateEditor.value.chain().focus().setParagraph().run()
  } else {
    templateEditor.value
      .chain()
      .focus()
      .toggleHeading({ level: level as 1 | 2 | 3 })
      .run()
  }
}

const dialogTitle = computed(() => {
  if (viewMode.value === 'list') return t('template.title')
  return editingTemplate.value ? t('template.edit') : t('template.create')
})

const loadTemplates = async () => {
  try {
    templates.value = await templateApi.findAll()
  } catch {
    // ignore
  }
}

const createTemplateEditor = (content: string, contentType: number = 0) => {
  if (templateEditor.value) {
    templateEditor.value.destroy()
  }
  const placeholder = t('template.editorPlaceholder')
  const extensions =
    contentType === ContentType.Markdown
      ? getMarkdownExtensions(placeholder)
      : getRichTextExtensions(placeholder)

  templateEditor.value = new Editor({
    extensions,
    content,
    editable: true,
    editorProps: {
      attributes: {
        autocapitalize: 'off',
        autocorrect: 'off',
        spellcheck: 'false',
      },
    },
  })
}

const switchContentType = (type: number) => {
  if (type === templateContentType.value) return
  // 保存当前内容
  const content = templateEditor.value?.getHTML() || ''
  templateContentType.value = type
  createTemplateEditor(content, type)
}

const destroyTemplateEditor = () => {
  if (templateEditor.value) {
    templateEditor.value.destroy()
    templateEditor.value = undefined
  }
}

const handleNewTemplate = () => {
  editingTemplate.value = null
  templateName.value = ''
  templateContentType.value = 0
  viewMode.value = 'editor'
  createTemplateEditor('', 0)
}

const handleEditTemplate = (tpl: NoteTemplate) => {
  editingTemplate.value = tpl
  templateName.value = tpl.name
  templateContentType.value = tpl.contentType ?? 0
  viewMode.value = 'editor'
  createTemplateEditor(tpl.content, tpl.contentType ?? 0)
}

const handleBack = () => {
  destroyTemplateEditor()
  viewMode.value = 'list'
  editingTemplate.value = null
}

const handleSave = async () => {
  const name = templateName.value.trim()
  if (!name) return
  const content = templateEditor.value?.getHTML() || ''

  try {
    if (editingTemplate.value) {
      await templateApi.update({
        ...editingTemplate.value,
        name,
        content,
        contentType: templateContentType.value,
      })
    } else {
      await templateApi.create({
        id: 0,
        name,
        content,
        contentType: templateContentType.value,
        sortOrder: 0,
        createTime: null,
        updateTime: null,
      })
    }
    showNotification({ type: 'success', message: t('template.saveSuccess') })
    await loadTemplates()
    handleBack()
  } catch (e: unknown) {
    showNotification({ type: 'error', message: parseError(e) })
  }
}

const handleDelete = async (tpl: NoteTemplate) => {
  try {
    await templateApi.delete(tpl.id)
    await loadTemplates()
  } catch (e: unknown) {
    showNotification({ type: 'error', message: parseError(e) })
  }
}

watch(visible, (val) => {
  if (val) {
    viewMode.value = 'list'
    loadTemplates()
  } else {
    destroyTemplateEditor()
  }
})

onBeforeUnmount(() => {
  destroyTemplateEditor()
})
</script>

<style src="../styles/prosemirror.css"></style>

<style scoped>
.template-editor-container {
  min-height: 300px;
  max-height: 400px;
  overflow-y: auto;
}

.template-toolbar {
  display: flex;
  align-items: center;
  gap: 2px;
  padding: 6px 8px;
  border-bottom: 1px solid var(--color-border-edge, #e2e8f0);
  background: var(--color-bg-surface-alt, #f8fafc);
  flex-wrap: wrap;
}

.toolbar-divider {
  width: 1px;
  height: 20px;
  background: var(--color-border-edge, #e2e8f0);
  margin: 0 4px;
}

.t-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 4px;
  color: var(--color-text-content-secondary, #64748b);
  transition: all 0.15s;
}

.t-btn:hover {
  background: var(--color-bg-surface, #ffffff);
  color: var(--color-text-content, #0f172a);
}

.t-btn.active {
  background: var(--color-primary-light, #e0e7ff);
  color: var(--color-primary, #4f46e5);
}

.t-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.template-editor-container :deep(.tiptap) {
  padding: 16px;
  min-height: 260px;
  outline: none;
}
</style>
