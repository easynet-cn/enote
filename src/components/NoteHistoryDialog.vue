<template>
  <Dialog
    :model-value="visible ?? false"
    @update:model-value="visible = $event"
    :title="t('history.title')"
    :width="1100"
    @open="$emit('open')"
  >
    <div class="overflow-hidden flex flex-col max-h-[65vh]">
      <div class="flex-1 overflow-auto">
        <table class="w-full border-collapse" role="table" :aria-label="t('history.title')">
          <thead class="bg-surface-alt sticky top-0 z-10">
            <tr>
              <th
                class="px-4 py-3 text-left text-sm font-medium text-content-secondary border-b border-edge"
              >
                ID
              </th>
              <th
                class="px-4 py-3 text-left text-sm font-medium text-content-secondary border-b border-edge"
              >
                {{ t('history.notebookName') }}
              </th>
              <th
                class="px-4 py-3 text-left text-sm font-medium text-content-secondary border-b border-edge"
              >
                {{ t('history.noteTitle') }}
              </th>
              <th
                class="px-4 py-3 text-left text-sm font-medium text-content-secondary border-b border-edge"
              >
                {{ t('history.contentType') }}
              </th>
              <th
                class="px-4 py-3 text-left text-sm font-medium text-content-secondary border-b border-edge"
              >
                {{ t('history.tags') }}
              </th>
              <th
                class="px-4 py-3 text-left text-sm font-medium text-content-secondary border-b border-edge"
              >
                {{ t('history.operateType') }}
              </th>
              <th
                class="px-4 py-3 text-left text-sm font-medium text-content-secondary border-b border-edge"
              >
                {{ t('history.operateTime') }}
              </th>
              <th
                class="px-4 py-3 text-left text-sm font-medium text-content-secondary border-b border-edge"
              >
                {{ t('history.action') }}
              </th>
            </tr>
          </thead>
          <!-- 加载骨架屏 -->
          <HistoryTableSkeleton v-if="loading" :count="5" />

          <!-- 数据内容 -->
          <tbody v-else>
            <tr v-if="!showData?.length">
              <td colspan="8" class="px-4 py-12 text-center text-content-secondary">
                <div class="flex flex-col items-center">
                  <Clock class="w-10 h-10 mb-2 opacity-40" />
                  <span>{{ t('history.empty') }}</span>
                </div>
              </td>
            </tr>
            <tr
              v-for="row in showData"
              :key="row.id"
              class="hover:bg-surface-alt transition-colors"
            >
              <td class="px-4 py-3 text-sm text-content border-b border-edge">{{ row.id }}</td>
              <td class="px-4 py-3 text-sm text-content border-b border-edge">
                {{ row.notebookName }}
              </td>
              <td class="px-4 py-3 text-sm text-content border-b border-edge">{{ row.title }}</td>
              <td class="px-4 py-3 text-sm text-content border-b border-edge">
                {{ row.contentType }}
              </td>
              <td class="px-4 py-3 text-sm text-content border-b border-edge">{{ row.tags }}</td>
              <td class="px-4 py-3 text-sm text-content border-b border-edge">
                {{ row.operateType }}
              </td>
              <td class="px-4 py-3 text-sm text-content border-b border-edge">
                {{ row.operateTime }}
              </td>
              <td class="px-4 py-3 text-sm border-b border-edge">
                <button
                  @click="handleView(row)"
                  class="px-3 py-1 text-sm text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors"
                  :aria-label="`${t('history.view')} #${row.id}`"
                >
                  {{ t('history.view') }}
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end">
        <Pagination
          :current-page="currentPage!"
          :page-size="pageSize!"
          :total="total!"
          :page-sizes="[20, 50, 100, 200]"
          :show-total="true"
          :show-sizes="true"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
        />
      </div>
    </template>
  </Dialog>

  <!-- 内容查看对话框 -->
  <Dialog v-model="viewVisible" :title="t('history.contentView')" :width="1200">
    <!-- 视图模式切换 -->
    <div class="flex justify-center mb-4">
      <div
        class="inline-flex rounded-lg border border-slate-300 dark:border-slate-600 overflow-hidden"
      >
        <button
          v-for="mode in viewModes"
          :key="mode.value"
          class="px-5 py-2 text-sm font-medium transition-colors"
          :class="
            viewMode === mode.value
              ? 'bg-indigo-600 text-white'
              : 'bg-slate-100 text-slate-600 hover:bg-slate-200 dark:bg-slate-700 dark:text-slate-300 dark:hover:bg-slate-600'
          "
          @click="viewMode = mode.value"
        >
          {{ mode.label }}
        </button>
      </div>
    </div>

    <!-- 并排视图 -->
    <div class="flex gap-4">
      <div class="flex-1 min-w-0">
        <div class="text-sm font-semibold mb-3 text-content-secondary">
          {{ t('history.oldContent') }}
        </div>
        <div
          ref="leftPanelRef"
          class="history-scroll-panel h-[60vh] overflow-auto bg-surface-alt p-4 rounded-lg border border-edge"
          @scroll="onSyncScroll('left')"
        >
          <TipTapEditor :model-value="viewOldContent" :editable="false" :show-toolbar="false" />
        </div>
      </div>
      <div class="flex-1 min-w-0">
        <div class="text-sm font-semibold mb-3 text-content-secondary">
          {{ t('history.newContent') }}
        </div>
        <div
          v-if="viewMode === 'side-by-side'"
          ref="rightPanelRef"
          class="history-scroll-panel h-[60vh] overflow-auto p-4 rounded-lg border border-edge bg-surface"
          @scroll="onSyncScroll('right')"
        >
          <TipTapEditor :model-value="viewNewContent" :editable="false" :show-toolbar="false" />
        </div>
        <div
          v-else
          ref="rightPanelRef"
          class="h-[60vh] overflow-auto p-4 rounded-lg border border-edge bg-surface diff-view"
          v-html="diffResult"
          @scroll="onSyncScroll('right')"
        ></div>
      </div>
    </div>
  </Dialog>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { Clock } from 'lucide-vue-next'
import { ContentType, NoteHistory } from '../types'
import { diffHtml, diffText } from '../utils/diff'
import TipTapEditor from './TipTapEditor.vue'
import HistoryTableSkeleton from './HistoryTableSkeleton.vue'
import { Dialog, Pagination } from './ui'

const { t } = useI18n()

interface Props {
  loading?: boolean
}

withDefaults(defineProps<Props>(), {
  loading: false,
})

const visible = defineModel<boolean>('visible')
const data = defineModel<NoteHistory[]>('data')
const currentPage = defineModel<number>('currentPage')
const pageSize = defineModel<number>('pageSize')
const total = defineModel<number>('total')

// 查看对话框相关状态
const viewVisible = ref(false)
const leftPanelRef = ref<HTMLElement | null>(null)
const rightPanelRef = ref<HTMLElement | null>(null)
let isSyncingScroll = false

const onSyncScroll = (source: 'left' | 'right') => {
  if (isSyncingScroll) return
  isSyncingScroll = true

  const from = source === 'left' ? leftPanelRef.value : rightPanelRef.value
  const to = source === 'left' ? rightPanelRef.value : leftPanelRef.value

  if (from && to) {
    const maxFrom = from.scrollHeight - from.clientHeight
    const maxTo = to.scrollHeight - to.clientHeight
    if (maxFrom > 0) {
      to.scrollTop = (from.scrollTop / maxFrom) * maxTo
    }
  }

  requestAnimationFrame(() => {
    isSyncingScroll = false
  })
}
const viewOldContent = ref('')
const viewNewContent = ref('')
const viewContentType = ref<ContentType>(ContentType.Html)

type ViewMode = 'side-by-side' | 'diff'
const viewMode = ref<ViewMode>('side-by-side')
const viewModes = computed(() => [
  { value: 'side-by-side' as ViewMode, label: t('history.sideBySideView') },
  { value: 'diff' as ViewMode, label: t('history.diffView') },
])

const diffResult = computed(() => {
  if (viewContentType.value === ContentType.Markdown) {
    return diffText(viewOldContent.value, viewNewContent.value)
  }
  return diffHtml(viewOldContent.value, viewNewContent.value)
})

interface ShowRow {
  id: string
  notebookId: number
  notebookName: string
  title: string
  contentType: string
  contentTypeEnum: ContentType
  tags: string
  oldContent: string
  newContent: string
  operateType: string
  operateTime: string
}

const showData = computed(() => {
  return data.value?.map((item): ShowRow => {
    return {
      id: item.id,
      notebookId: item.extra.notebookId,
      notebookName: item.extra.notebookName,
      title: item.extra.title,
      contentType:
        item.extra.contentType === ContentType.Markdown ? 'Markdown' : t('history.richText'),
      contentTypeEnum: item.extra.contentType,
      tags: item.extra.tags.map((t) => t.name).join(' '),
      oldContent: item.oldContent,
      newContent: item.newContent,
      operateType:
        item.operateType === 1
          ? t('history.operateCreate')
          : item.operateType === 2
            ? t('history.operateUpdate')
            : item.operateType === 3
              ? t('history.operateDelete')
              : '',
      operateTime: item.operateTime,
    }
  })
})

const emit = defineEmits<{
  sizeChange: [pageSize: number]
  currentChange: [currentPage: number]
  open: []
}>()

const handleSizeChange = (val: number) => {
  emit('sizeChange', val)
}

const handleCurrentChange = (val: number) => {
  emit('currentChange', val)
}

const handleView = (row: ShowRow) => {
  viewOldContent.value = row.oldContent || ''
  viewNewContent.value = row.newContent || ''
  viewContentType.value = row.contentTypeEnum
  viewVisible.value = true
}
</script>

<style scoped>
/* Diff 对比视图样式 */
.diff-view :deep(ins),
.diff-view :deep(.diff-ins) {
  background-color: #d1fae5;
  text-decoration: none;
  padding: 1px 2px;
  border-radius: 2px;
}

.diff-view :deep(del),
.diff-view :deep(.diff-del) {
  background-color: #fee2e2;
  text-decoration: line-through;
  padding: 1px 2px;
  border-radius: 2px;
}

.diff-view :deep(.diff-pre) {
  white-space: pre-wrap;
  word-break: break-word;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  font-size: 0.875rem;
  line-height: 1.6;
  margin: 0;
}

/* 暗色模式适配 */
:root.dark .diff-view :deep(ins),
:root.dark .diff-view :deep(.diff-ins) {
  background-color: rgba(16, 185, 129, 0.2);
}

:root.dark .diff-view :deep(del),
:root.dark .diff-view :deep(.diff-del) {
  background-color: rgba(239, 68, 68, 0.2);
}

/* 历史面板：禁用 TipTap 内部滚动，让外层容器统一滚动 */
.history-scroll-panel :deep(.tiptap-readonly-editor),
.history-scroll-panel :deep(.editor-content),
.history-scroll-panel :deep(.ProseMirror) {
  height: auto;
  overflow: visible;
}
</style>
