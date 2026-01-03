<template>
  <Dialog v-model="visible" :title="dialogTitle" :width="520" @close="handleClose">
    <!-- 步骤 1: 选择来源 -->
    <div v-if="step === 1" class="space-y-3">
      <p class="text-sm text-slate-500 mb-4">选择要导入的笔记来源:</p>
      <div
        v-for="source in IMPORT_SOURCES"
        :key="source.id"
        class="flex items-center gap-3 p-4 border border-slate-200 rounded-lg cursor-pointer hover:border-indigo-400 hover:bg-indigo-50 transition-colors"
        :class="{ 'border-indigo-500 bg-indigo-50': selectedSource === source.id }"
        @click="selectSource(source.id)"
      >
        <div
          class="w-10 h-10 rounded-lg bg-slate-100 flex items-center justify-center text-slate-600"
        >
          <component :is="getIcon(source.icon)" class="w-5 h-5" />
        </div>
        <div class="flex-1">
          <div class="font-medium text-slate-800">{{ source.name }}</div>
          <div class="text-sm text-slate-500">{{ source.description }}</div>
        </div>
        <div v-if="selectedSource === source.id" class="text-indigo-500">
          <Check class="w-5 h-5" />
        </div>
      </div>
    </div>

    <!-- 步骤 2: 选择文件 -->
    <div v-else-if="step === 2" class="space-y-4">
      <p class="text-sm text-slate-500">选择要导入的文件:</p>

      <div
        class="border-2 border-dashed border-slate-300 rounded-lg p-8 text-center cursor-pointer hover:border-indigo-400 hover:bg-slate-50 transition-colors"
        @click="handleSelectFile"
      >
        <Upload class="w-12 h-12 mx-auto mb-3 text-slate-400" />
        <p v-if="!selectedFilePath" class="text-slate-600">点击选择文件</p>
        <p v-else class="text-indigo-600 font-medium break-all">{{ selectedFileName }}</p>
        <p class="text-xs text-slate-400 mt-2">
          支持格式: {{ currentSourceConfig?.fileTypes.map((t) => '.' + t).join(', ') }}
        </p>
      </div>
    </div>

    <!-- 步骤 3: 选择目标笔记本 -->
    <div v-else-if="step === 3" class="space-y-4">
      <p class="text-sm text-slate-500">选择要导入到的笔记本:</p>

      <select
        v-model="targetNotebookId"
        class="w-full h-10 px-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
      >
        <option v-for="nb in availableNotebooks" :key="nb.id" :value="nb.id">
          {{ nb.name }}
        </option>
      </select>

      <div class="flex items-center gap-2 text-sm text-slate-600">
        <input id="createTags" v-model="createTags" type="checkbox" class="rounded" />
        <label for="createTags">自动创建不存在的标签</label>
      </div>
    </div>

    <!-- 步骤 4: 导入进度 -->
    <div v-else-if="step === 4" class="space-y-4">
      <div v-if="importing" class="text-center py-4">
        <Loader2 class="w-10 h-10 mx-auto mb-3 text-indigo-500 animate-spin" />
        <p class="text-slate-600 mb-2">
          {{ progress.phase === 'parsing' ? '正在解析' : '正在导入' }}: {{ progress.currentTitle }}
        </p>
        <div class="w-full bg-slate-200 rounded-full h-2">
          <div
            class="bg-indigo-500 h-2 rounded-full transition-all duration-300"
            :style="{ width: progressPercent + '%' }"
          />
        </div>
        <p class="text-sm text-slate-500 mt-2">{{ progress.current }} / {{ progress.total }}</p>
      </div>

      <div v-else class="text-center py-4">
        <CheckCircle
          v-if="importResult?.failed === 0"
          class="w-12 h-12 mx-auto mb-3 text-green-500"
        />
        <AlertCircle v-else class="w-12 h-12 mx-auto mb-3 text-amber-500" />

        <p class="text-lg font-medium text-slate-800 mb-2">
          {{ importResult?.failed === 0 ? '导入完成' : '导入完成（部分失败）' }}
        </p>

        <div class="flex justify-center gap-6 text-sm">
          <div class="text-green-600">
            <span class="font-bold text-lg">{{ importResult?.success }}</span>
            <span class="ml-1">成功</span>
          </div>
          <div v-if="importResult?.failed" class="text-red-600">
            <span class="font-bold text-lg">{{ importResult?.failed }}</span>
            <span class="ml-1">失败</span>
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
        <button
          v-if="step > 1 && step < 4"
          class="px-4 py-2 text-slate-600 hover:bg-slate-100 rounded-lg transition-colors"
          @click="prevStep"
        >
          上一步
        </button>
        <div v-else></div>

        <div class="flex gap-2">
          <button
            v-if="step < 4 || !importing"
            class="px-4 py-2 text-slate-600 bg-slate-100 hover:bg-slate-200 rounded-lg transition-colors"
            @click="handleClose"
          >
            {{ step === 4 ? '关闭' : '取消' }}
          </button>

          <button
            v-if="step === 1"
            class="px-4 py-2 text-white bg-indigo-600 hover:bg-indigo-700 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            :disabled="!selectedSource"
            @click="nextStep"
          >
            下一步
          </button>

          <button
            v-if="step === 2"
            class="px-4 py-2 text-white bg-indigo-600 hover:bg-indigo-700 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            :disabled="!selectedFilePath"
            @click="nextStep"
          >
            下一步
          </button>

          <button
            v-if="step === 3"
            class="px-4 py-2 text-white bg-indigo-600 hover:bg-indigo-700 rounded-lg transition-colors"
            @click="startImport"
          >
            开始导入
          </button>
        </div>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { Dialog } from './ui'
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
      return '导入笔记 - 选择来源'
    case 2:
      return '导入笔记 - 选择文件'
    case 3:
      return '导入笔记 - 选择笔记本'
    case 4:
      return importing.value ? '导入笔记 - 导入中' : '导入笔记 - 完成'
    default:
      return '导入笔记'
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
        errors.push(`导入 "${note.title}" 失败: ${error}`)
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
      errors: [error instanceof Error ? error.message : '导入失败'],
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
