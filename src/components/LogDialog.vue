<template>
  <Dialog v-model="visible" :title="t('log.title')" :width="850">
    <div class="space-y-3">
      <!-- Tab 切换 -->
      <div class="flex border-b border-edge">
        <button
          v-for="tab in tabs"
          :key="tab.key"
          class="px-4 py-2 text-sm font-medium transition-colors border-b-2 -mb-px"
          :class="
            activeTab === tab.key
              ? 'text-indigo-600 border-indigo-600'
              : 'text-content-secondary border-transparent hover:text-content'
          "
          @click="activeTab = tab.key"
        >
          {{ tab.label }}
        </button>
      </div>

      <!-- Tab 1: 操作日志（数据库） -->
      <div v-if="activeTab === 'db'" class="space-y-3">
        <!-- 筛选栏 -->
        <div class="flex flex-wrap items-center gap-2">
          <select
            v-model="searchParam.level"
            class="px-2 py-1.5 text-xs border border-edge rounded-lg bg-surface focus:outline-none focus:ring-1 focus:ring-indigo-500"
          >
            <option value="">{{ t('log.allLevels') }}</option>
            <option value="INFO">INFO</option>
            <option value="WARN">WARN</option>
            <option value="ERROR">ERROR</option>
          </select>
          <select
            v-model="searchParam.module"
            class="px-2 py-1.5 text-xs border border-edge rounded-lg bg-surface focus:outline-none focus:ring-1 focus:ring-indigo-500"
          >
            <option value="">{{ t('log.allModules') }}</option>
            <option v-for="m in modules" :key="m" :value="m">{{ m }}</option>
          </select>
          <input
            v-model="searchParam.keyword"
            type="text"
            :placeholder="t('log.searchPlaceholder')"
            class="flex-1 min-w-[160px] px-2.5 py-1.5 text-xs border border-edge rounded-lg bg-surface focus:outline-none focus:ring-1 focus:ring-indigo-500"
            @keyup.enter="loadDbLogs"
          />
          <button
            @click="loadDbLogs"
            class="px-3 py-1.5 text-xs bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
          >
            {{ t('common.search') }}
          </button>
        </div>

        <!-- 日志列表 -->
        <div class="max-h-[400px] overflow-y-auto border border-edge rounded-lg">
          <table class="w-full text-xs">
            <thead class="bg-surface-alt sticky top-0">
              <tr>
                <th class="px-2 py-2 text-left font-medium text-content-secondary w-[140px]">
                  {{ t('log.time') }}
                </th>
                <th class="px-2 py-2 text-left font-medium text-content-secondary w-[50px]">
                  {{ t('log.level') }}
                </th>
                <th class="px-2 py-2 text-left font-medium text-content-secondary w-[70px]">
                  {{ t('log.module') }}
                </th>
                <th class="px-2 py-2 text-left font-medium text-content-secondary w-[70px]">
                  {{ t('log.action') }}
                </th>
                <th class="px-2 py-2 text-left font-medium text-content-secondary">
                  {{ t('log.message') }}
                </th>
                <th class="px-2 py-2 text-right font-medium text-content-secondary w-[50px]"></th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="log in dbLogs"
                :key="log.id"
                class="border-t border-edge hover:bg-surface-alt transition-colors"
              >
                <td class="px-2 py-1.5 text-content-tertiary whitespace-nowrap">
                  {{ log.createTime }}
                </td>
                <td class="px-2 py-1.5">
                  <span
                    class="text-[10px] px-1.5 py-0.5 rounded font-medium"
                    :class="levelClass(log.level)"
                  >
                    {{ log.level }}
                  </span>
                </td>
                <td class="px-2 py-1.5 text-content-secondary">{{ log.module }}</td>
                <td class="px-2 py-1.5 text-content-secondary">{{ log.action }}</td>
                <td class="px-2 py-1.5 text-content truncate max-w-[300px]" :title="log.message">
                  {{ log.message }}
                </td>
                <td class="px-2 py-1.5 text-right">
                  <button
                    @click="deleteDbLog(log.id)"
                    class="text-red-500 hover:text-red-700 transition-colors"
                    :title="t('common.delete')"
                  >
                    <Trash2 class="w-3.5 h-3.5" />
                  </button>
                </td>
              </tr>
              <tr v-if="dbLogs.length === 0 && !dbLoading">
                <td colspan="6" class="text-center py-6 text-content-tertiary">
                  {{ t('log.noLogs') }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- 分页 + 操作 -->
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <button
              @click="clearDbLogs"
              class="px-2.5 py-1 text-xs text-red-600 hover:bg-red-50 rounded-lg transition-colors"
            >
              {{ t('log.clearAll') }}
            </button>
            <button
              @click="cleanupDbLogs"
              class="px-2.5 py-1 text-xs text-content-secondary hover:bg-surface-dim rounded-lg transition-colors"
            >
              {{ t('log.cleanupOld') }}
            </button>
          </div>
          <Pagination
            v-if="dbTotal > dbPageSize"
            :current-page="dbPageIndex"
            :total="dbTotal"
            :page-size="dbPageSize"
            @current-change="
              (p: number) => {
                dbPageIndex = p
                loadDbLogs()
              }
            "
          />
        </div>
      </div>

      <!-- Tab 2: 系统日志（文件） -->
      <div v-if="activeTab === 'file'" class="space-y-3">
        <div class="flex gap-3 h-[420px]">
          <!-- 文件列表 -->
          <div class="w-[200px] border border-edge rounded-lg overflow-y-auto shrink-0">
            <div
              v-for="file in logFiles"
              :key="file.name"
              class="flex items-center justify-between px-2.5 py-2 border-b border-edge last:border-b-0 cursor-pointer hover:bg-surface-alt transition-colors text-xs"
              :class="selectedFile?.name === file.name ? 'bg-indigo-50 text-indigo-700' : ''"
              @click="selectFile(file)"
            >
              <div class="flex-1 min-w-0">
                <div class="truncate font-medium" :class="file.isError ? 'text-red-600' : ''">
                  {{ file.name }}
                </div>
                <div class="text-[10px] text-content-tertiary mt-0.5">
                  {{ formatFileSize(file.size) }}
                </div>
              </div>
            </div>
            <div
              v-if="logFiles.length === 0"
              class="text-center py-6 text-content-tertiary text-xs"
            >
              {{ t('log.noFiles') }}
            </div>
          </div>

          <!-- 文件内容 -->
          <div class="flex-1 border border-edge rounded-lg overflow-hidden flex flex-col">
            <!-- 搜索栏 -->
            <div class="flex items-center gap-2 px-2.5 py-1.5 border-b border-edge bg-surface-alt">
              <input
                v-model="fileKeyword"
                type="text"
                :placeholder="t('log.searchInFile')"
                class="flex-1 px-2 py-1 text-xs border border-edge rounded bg-surface focus:outline-none focus:ring-1 focus:ring-indigo-500"
                @keyup.enter="highlightSearch"
              />
              <button
                v-if="selectedFile"
                @click="deleteSelectedFile"
                class="px-2 py-1 text-xs text-red-600 hover:bg-red-50 rounded transition-colors"
              >
                {{ t('common.delete') }}
              </button>
            </div>
            <!-- 内容区域 -->
            <pre
              v-if="fileContent"
              class="flex-1 overflow-auto p-3 text-[11px] leading-relaxed text-content font-mono whitespace-pre-wrap break-all bg-slate-50 dark:bg-slate-900"
              v-html="highlightedContent"
            ></pre>
            <div
              v-else
              class="flex-1 flex items-center justify-center text-content-tertiary text-xs"
            >
              {{ t('log.selectFileHint') }}
            </div>
          </div>
        </div>

        <!-- 操作栏 -->
        <div class="flex items-center gap-2">
          <button
            @click="cleanupFilesByDays"
            class="px-2.5 py-1 text-xs text-content-secondary hover:bg-surface-dim rounded-lg transition-colors"
          >
            {{ t('log.cleanupOldFiles') }}
          </button>
          <button
            @click="loadLogFiles"
            class="px-2.5 py-1 text-xs text-indigo-600 hover:bg-indigo-50 rounded-lg transition-colors"
          >
            {{ t('log.refresh') }}
          </button>
        </div>
      </div>
    </div>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { Trash2 } from 'lucide-vue-next'
import { Dialog, Pagination } from './ui'
import { appLogApi } from '../api/appLog'
import { showNotification } from './ui/notification'
import type { AppLog, AppLogSearchParam, LogFileInfo } from '../types'

const { t } = useI18n()

const visible = defineModel<boolean>({ default: false })

const activeTab = ref<'db' | 'file'>('db')

const tabs = computed(() => [
  { key: 'db' as const, label: t('log.dbLogs') },
  { key: 'file' as const, label: t('log.fileLogs') },
])

const modules = [
  'notebook',
  'note',
  'tag',
  'backup',
  'sync',
  'settings',
  'encrypt',
  'system',
  'frontend',
]

// ============================================================================
// 数据库日志
// ============================================================================

const dbLogs = ref<AppLog[]>([])
const dbLoading = ref(false)
const dbPageIndex = ref(1)
const dbPageSize = ref(50)
const dbTotal = ref(0)

const searchParam = ref<AppLogSearchParam>({
  pageIndex: 1,
  pageSize: 50,
  level: '',
  module: '',
  keyword: '',
})

async function loadDbLogs() {
  dbLoading.value = true
  try {
    searchParam.value.pageIndex = dbPageIndex.value
    searchParam.value.pageSize = dbPageSize.value
    const result = await appLogApi.searchPage(searchParam.value)
    dbLogs.value = result.data
    dbTotal.value = result.total
  } catch (e: unknown) {
    showNotification({ message: e instanceof Error ? e.message : String(e), type: 'error' })
  } finally {
    dbLoading.value = false
  }
}

async function deleteDbLog(id: number) {
  try {
    await appLogApi.deleteById(id)
    dbLogs.value = dbLogs.value.filter((l) => l.id !== id)
    dbTotal.value--
  } catch (e: unknown) {
    showNotification({ message: e instanceof Error ? e.message : String(e), type: 'error' })
  }
}

async function clearDbLogs() {
  try {
    const count = await appLogApi.clearAll()
    dbLogs.value = []
    dbTotal.value = 0
    showNotification({ message: t('log.clearedCount', { count }), type: 'success' })
  } catch (e: unknown) {
    showNotification({ message: e instanceof Error ? e.message : String(e), type: 'error' })
  }
}

async function cleanupDbLogs() {
  try {
    const days = 30
    const before = new Date(Date.now() - days * 24 * 60 * 60 * 1000)
    const beforeStr = before.toISOString().replace('T', ' ').substring(0, 19)
    const count = await appLogApi.cleanupBefore(beforeStr)
    showNotification({ message: t('log.clearedCount', { count }), type: 'success' })
    loadDbLogs()
  } catch (e: unknown) {
    showNotification({ message: e instanceof Error ? e.message : String(e), type: 'error' })
  }
}

function levelClass(level: string) {
  switch (level) {
    case 'ERROR':
      return 'bg-red-50 text-red-600'
    case 'WARN':
      return 'bg-yellow-50 text-yellow-700'
    case 'INFO':
      return 'bg-blue-50 text-blue-600'
    case 'DEBUG':
      return 'bg-slate-100 text-slate-600'
    default:
      return 'bg-slate-100 text-slate-600'
  }
}

// ============================================================================
// 文件日志
// ============================================================================

const logFiles = ref<LogFileInfo[]>([])
const selectedFile = ref<LogFileInfo | null>(null)
const fileContent = ref('')
const fileKeyword = ref('')

async function loadLogFiles() {
  try {
    logFiles.value = await appLogApi.listLogFiles()
  } catch (e: unknown) {
    showNotification({ message: e instanceof Error ? e.message : String(e), type: 'error' })
  }
}

async function selectFile(file: LogFileInfo) {
  selectedFile.value = file
  try {
    fileContent.value = await appLogApi.readLogFile(file.name)
  } catch (e: unknown) {
    fileContent.value = ''
    showNotification({ message: e instanceof Error ? e.message : String(e), type: 'error' })
  }
}

async function deleteSelectedFile() {
  if (!selectedFile.value) return
  try {
    await appLogApi.deleteLogFiles([selectedFile.value.name])
    fileContent.value = ''
    selectedFile.value = null
    loadLogFiles()
    showNotification({ message: t('log.fileDeleted'), type: 'success' })
  } catch (e: unknown) {
    showNotification({ message: e instanceof Error ? e.message : String(e), type: 'error' })
  }
}

async function cleanupFilesByDays() {
  try {
    const count = await appLogApi.cleanupOldLogFiles(30)
    showNotification({ message: t('log.clearedCount', { count }), type: 'success' })
    loadLogFiles()
  } catch (e: unknown) {
    showNotification({ message: e instanceof Error ? e.message : String(e), type: 'error' })
  }
}

const highlightedContent = computed(() => {
  if (!fileContent.value) return ''
  let content = escapeHtml(fileContent.value)
  if (fileKeyword.value) {
    const escaped = escapeRegex(fileKeyword.value)
    const regex = new RegExp(`(${escaped})`, 'gi')
    content = content.replace(regex, '<mark class="bg-yellow-200">$1</mark>')
  }
  // 高亮 ERROR 行
  content = content.replace(/^(.*\bERROR\b.*)$/gm, '<span class="text-red-600">$1</span>')
  return content
})

function highlightSearch() {
  // 触发 computed 重新计算
}

function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

function escapeHtml(str: string): string {
  return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
}

function escapeRegex(str: string): string {
  return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}

// ============================================================================
// 生命周期
// ============================================================================

watch(visible, (val) => {
  if (val) {
    if (activeTab.value === 'db') {
      dbPageIndex.value = 1
      loadDbLogs()
    } else {
      loadLogFiles()
    }
  }
})

watch(activeTab, (tab) => {
  if (tab === 'db' && dbLogs.value.length === 0) {
    loadDbLogs()
  } else if (tab === 'file' && logFiles.value.length === 0) {
    loadLogFiles()
  }
})
</script>
