<template>
  <Dialog v-model="visible" :title="dialogTitle" :width="520" @close="handleClose">
    <!-- 步骤 1: 选择来源 -->
    <div v-if="step === 1" class="space-y-4">
      <p class="text-sm text-content-secondary mb-4">{{ t('import.selectFile') }}:</p>
      <div
        v-for="source in IMPORT_SOURCES"
        :key="source.id"
        class="flex items-center gap-3 p-4 border border-edge rounded-lg cursor-pointer hover:border-indigo-400 hover:bg-indigo-50 transition-colors"
        :class="{ 'border-indigo-500 bg-indigo-50': selectedSource === source.id }"
        @click="selectSource(source.id)"
      >
        <div
          class="w-10 h-10 rounded-lg bg-surface-dim flex items-center justify-center text-content-secondary"
        >
          <component :is="getIcon(source.icon)" class="w-5 h-5" />
        </div>
        <div class="flex-1">
          <div class="font-medium text-content">{{ source.name() }}</div>
          <div class="text-sm text-content-secondary">{{ source.description() }}</div>
        </div>
        <div v-if="selectedSource === source.id" class="text-indigo-500">
          <Check class="w-5 h-5" />
        </div>
      </div>
    </div>

    <!-- 步骤 2: 选择文件 -->
    <div v-else-if="step === 2" class="space-y-4">
      <p class="text-sm text-content-secondary">{{ t('import.format') }}:</p>

      <div
        class="border-2 border-dashed border-edge rounded-lg p-8 text-center cursor-pointer hover:border-indigo-400 hover:bg-surface-alt transition-colors"
        @click="handleSelectFile"
      >
        <Upload class="w-12 h-12 mx-auto mb-3 text-content-tertiary" />
        <p v-if="!selectedFilePath" class="text-content-secondary">{{ t('import.selectFile') }}</p>
        <p v-else class="text-indigo-600 font-medium break-all">{{ selectedFileName }}</p>
        <p class="text-xs text-content-tertiary mt-2">
          {{ t('import.supportedFormats') }}:
          {{ currentSourceConfig?.fileTypes.map((t) => '.' + t).join(', ') }}
        </p>
      </div>
    </div>

    <!-- 步骤 3: 选择目标笔记本 -->
    <div v-else-if="step === 3" class="space-y-4">
      <p class="text-sm text-content-secondary">{{ t('import.selectNotebook') }}:</p>

      <AppSelect
        v-model="targetNotebookId"
        :options="notebookSelectOptions"
        size="md"
        class="w-full"
      />

      <div class="flex items-center gap-2 text-sm text-content-secondary">
        <input id="createTags" v-model="createTags" type="checkbox" class="rounded" />
        <label for="createTags">{{ t('import.autoCreateTags') }}</label>
      </div>
    </div>

    <!-- 步骤 4: 导入进度 -->
    <div v-else-if="step === 4" class="space-y-4">
      <div v-if="importing" class="text-center py-4">
        <Loader2 class="w-10 h-10 mx-auto mb-3 text-indigo-500 animate-spin" />
        <p class="text-content-secondary mb-2">
          {{ progress.phase === 'parsing' ? t('import.parsing') : t('import.importing') }}:
          {{ progress.currentTitle }}
        </p>
        <div class="w-full bg-surface-dim rounded-full h-2">
          <div
            class="bg-indigo-500 h-2 rounded-full transition-all duration-300"
            :style="{ width: progressPercent + '%' }"
          />
        </div>
        <p class="text-sm text-content-secondary mt-2">
          {{ progress.current }} / {{ progress.total }}
        </p>
      </div>

      <div v-else class="text-center py-4">
        <CheckCircle
          v-if="importResult?.failed === 0"
          class="w-12 h-12 mx-auto mb-3 text-green-500"
        />
        <AlertCircle v-else class="w-12 h-12 mx-auto mb-3 text-amber-500" />

        <p class="text-lg font-medium text-content mb-2">
          {{ importResult?.failed === 0 ? t('import.success') : t('import.partialSuccess') }}
        </p>

        <div class="flex justify-center gap-6 text-sm">
          <div class="text-green-600">
            <span class="font-bold text-lg">{{ importResult?.success }}</span>
            <span class="ml-1">{{ t('import.success') }}</span>
          </div>
          <div v-if="importResult?.failed" class="text-red-600">
            <span class="font-bold text-lg">{{ importResult?.failed }}</span>
            <span class="ml-1">{{ t('import.error') }}</span>
          </div>
        </div>

        <div
          v-if="importResult?.errors.length"
          class="mt-4 max-h-32 overflow-y-auto text-left text-sm text-red-600 bg-red-50 rounded-lg p-3"
        >
          <div v-for="(error, index) in importResult.errors" :key="index">
            {{ error }}
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-between">
        <div>
          <Button v-if="step > 1 && step < 4" type="text" @click="prevStep">
            {{ t('import.prevStep') }}
          </Button>
        </div>

        <div class="flex gap-3">
          <Button v-if="step < 4 || !importing" type="secondary" @click="handleClose">
            {{ step === 4 ? t('common.close') : t('common.cancel') }}
          </Button>

          <Button v-if="step === 1" type="primary" :disabled="!selectedSource" @click="nextStep">
            {{ t('import.nextStep') }}
          </Button>

          <Button v-if="step === 2" type="primary" :disabled="!selectedFilePath" @click="nextStep">
            {{ t('import.nextStep') }}
          </Button>

          <Button v-if="step === 3" type="primary" @click="startImport">
            {{ t('import.startImport') }}
          </Button>
        </div>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Dialog, Button, AppSelect } from './ui'
import type { AppSelectOption } from './ui'
import {
  FileText,
  FileArchive,
  FileCode,
  Check,
  Upload,
  Loader2,
  CheckCircle,
  AlertCircle,
} from 'lucide-vue-next'
import {
  IMPORT_SOURCES,
  selectImportFile,
  parseImportFile,
  type ImportSource,
  type ImportResult,
  type ImportProgress,
  type ImportSourceConfig,
} from '../utils/import'
import { noteApi } from '../api/note'
import { ContentType, type ShowNotebook, type ShowTag, type Tag } from '../types'

const { t } = useI18n()

interface Props {
  notebooks: ShowNotebook[]
  tags: ShowTag[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  imported: []
}>()

const visible = defineModel<boolean>({ default: false })

// 步骤状态
const step = ref(1)
const selectedSource = ref<ImportSource | null>(null)
const selectedFilePath = ref<string | null>(null)
const targetNotebookId = ref<number>(0)
const createTags = ref(true)

// 导入状态
const importing = ref(false)
const progress = ref<ImportProgress>({
  current: 0,
  total: 0,
  currentTitle: '',
  phase: 'parsing',
})
const importResult = ref<ImportResult | null>(null)

// 计算属性
const dialogTitle = computed(() => {
  switch (step.value) {
    case 1:
      return `${t('import.title')} - ${t('import.selectFile')}`
    case 2:
      return `${t('import.title')} - ${t('import.format')}`
    case 3:
      return `${t('import.title')} - ${t('import.selectNotebook')}`
    case 4:
      return importing.value
        ? `${t('import.title')} - ${t('import.importing')}`
        : `${t('import.title')} - ${t('import.success')}`
    default:
      return t('import.title')
  }
})

const currentSourceConfig = computed<ImportSourceConfig | undefined>(() => {
  return IMPORT_SOURCES.find((s) => s.id === selectedSource.value)
})

const selectedFileName = computed(() => {
  if (!selectedFilePath.value) return ''
  return selectedFilePath.value.split('/').pop() || selectedFilePath.value
})

const availableNotebooks = computed(() => {
  // 过滤掉 "全部" 虚拟笔记本
  return props.notebooks.filter((nb) => nb.id !== '0')
})

const notebookSelectOptions = computed<AppSelectOption[]>(() =>
  availableNotebooks.value.map((nb) => ({
    label: nb.name || '',
    value: nb.id,
  })),
)

const progressPercent = computed(() => {
  if (progress.value.total === 0) return 0
  return Math.round((progress.value.current / progress.value.total) * 100)
})

// 图标映射
const iconMap: Record<string, typeof FileText> = {
  FileText,
  FileArchive,
  FileCode,
}

function getIcon(iconName: string) {
  return iconMap[iconName] || FileText
}

// 步骤控制
function selectSource(source: ImportSource) {
  selectedSource.value = source
}

function nextStep() {
  step.value++
}

function prevStep() {
  step.value--
}

// 文件选择
async function handleSelectFile() {
  if (!selectedSource.value) return

  const filePath = await selectImportFile(selectedSource.value)
  if (filePath) {
    selectedFilePath.value = filePath
  }
}

// 开始导入
async function startImport() {
  if (!selectedSource.value || !selectedFilePath.value) return

  step.value = 4
  importing.value = true
  importResult.value = null

  try {
    // 解析文件
    const result = await parseImportFile(selectedSource.value, selectedFilePath.value, (p) => {
      progress.value = p
    })

    // 导入笔记到数据库
    progress.value.phase = 'importing'
    let successCount = 0
    let failedCount = 0
    const errors: string[] = []

    // 获取现有标签映射 (ShowTag -> Tag 转换)
    const existingTags = new Map<string, Tag>()
    props.tags.forEach((showTag) => {
      const tag: Tag = {
        id: Number(showTag.id),
        name: showTag.name,
        icon: showTag.icon,
        cls: showTag.cls,
        sortOrder: showTag.sortOrder,
        createTime: showTag.createTime || null,
        updateTime: showTag.updateTime || null,
      }
      existingTags.set(tag.name.toLowerCase(), tag)
    })

    const notebookId = targetNotebookId.value || Number(availableNotebooks.value[0]?.id) || 1

    for (let i = 0; i < result.notes.length; i++) {
      const note = result.notes[i]

      progress.value = {
        current: i + 1,
        total: result.notes.length,
        currentTitle: note.title,
        phase: 'importing',
      }

      try {
        // 处理标签
        const noteTags: Tag[] = []
        if (createTags.value && note.tags.length > 0) {
          for (const tagName of note.tags) {
            let tag = existingTags.get(tagName.toLowerCase())
            if (!tag) {
              // 创建新标签
              tag = await noteApi.createTag({
                id: 0,
                name: tagName,
                icon: '',
                cls: '',
                sortOrder: 0,
              })
              existingTags.set(tagName.toLowerCase(), tag)
            }
            noteTags.push(tag)
          }
        }

        // 创建笔记
        await noteApi.createNote(
          notebookId,
          note.title,
          note.content,
          note.contentType === 'markdown' ? ContentType.Markdown : ContentType.Html,
          noteTags,
        )

        successCount++
      } catch (error) {
        failedCount++
        errors.push(`${t('import.error')}: "${note.title}" - ${error}`)
      }
    }

    importResult.value = {
      success: successCount,
      failed: failedCount,
      errors: [...result.errors, ...errors],
      notes: result.notes,
    }

    if (successCount > 0) {
      emit('imported')
    }
  } catch (error) {
    importResult.value = {
      success: 0,
      failed: 0,
      errors: [error instanceof Error ? error.message : t('import.error')],
      notes: [],
    }
  } finally {
    importing.value = false
  }
}

// 关闭对话框
function handleClose() {
  visible.value = false
}

// 重置状态
watch(visible, (newVal) => {
  if (newVal) {
    step.value = 1
    selectedSource.value = null
    selectedFilePath.value = null
    targetNotebookId.value = Number(availableNotebooks.value[0]?.id) || 0
    createTags.value = true
    importing.value = false
    importResult.value = null
  }
})
</script>
