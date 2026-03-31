<template>
  <Dialog v-if="!embedded" v-model="visible" :title="t('sync.completed')" :width="680">
    <SyncResultContent :sync-log="syncLog" />
    <template #footer>
      <div class="flex justify-end gap-2">
        <Button @click="handleExport">{{ t('sync.exportLog') }}</Button>
        <Button @click="visible = false">{{ t('common.close') }}</Button>
      </div>
    </template>
  </Dialog>

  <!-- 嵌入模式（在 SyncDialog 内使用） -->
  <div v-else>
    <SyncResultContent :sync-log="syncLog" />
    <div class="flex justify-end gap-2 mt-4 pt-4 border-t border-edge">
      <Button @click="handleExport">{{ t('sync.exportLog') }}</Button>
      <Button @click="$emit('close')">{{ t('common.close') }}</Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, defineComponent, h, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { Dialog, Button } from './ui'
import type { SyncLog, SyncLogDetail } from '../types'
import { save } from '@tauri-apps/plugin-dialog'
import { syncApi } from '../api/note'
import { showNotification } from './ui/notification'
import { showError } from '../utils/errorHandler'

const props = defineProps<{
  modelValue?: boolean
  syncLog: SyncLog
  embedded?: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  close: []
}>()

const { t } = useI18n()

const visible = computed({
  get: () => props.modelValue ?? false,
  set: (v) => emit('update:modelValue', v),
})

async function handleExport() {
  try {
    const path = await save({
      defaultPath: `sync_log_${props.syncLog.id}.json`,
      filters: [{ name: 'JSON', extensions: ['json'] }],
    })
    if (path) {
      await syncApi.exportSyncLog(props.syncLog.id, path)
      showNotification({ type: 'success', message: t('sync.exportLog') + ' OK' })
    }
  } catch (e: unknown) {
    showError(e)
  }
}

// 内嵌的结果内容组件
const SyncResultContent = defineComponent({
  props: {
    syncLog: { type: Object as () => SyncLog, required: true },
  },
  setup(props) {
    const { t } = useI18n()
    const expandedTable = ref<string | null>(null)
    const details = ref<SyncLogDetail[]>([])
    const detailLoading = ref(false)
    const detailPage = ref(1)
    const detailTotal = ref(0)
    const filterTable = ref('')
    const filterStatus = ref('')

    const log = computed(() => props.syncLog)
    const duration = computed(() => {
      if (!log.value.startedAt || !log.value.finishedAt) return '-'
      const start = new Date(log.value.startedAt).getTime()
      const end = new Date(log.value.finishedAt).getTime()
      const ms = end - start
      if (ms < 1000) return `${ms}ms`
      return `${(ms / 1000).toFixed(1)}s`
    })

    // 表名映射
    const tableNames = ['notebook', 'tag', 'note', 'note_history', 'note_template', 'settings']
    const tableLabels: Record<string, string> = {
      notebook: t('sync.scopeNotebooks'),
      tag: t('sync.scopeTags'),
      note: t('sync.scopeNotes'),
      note_history: t('sync.scopeNoteHistories'),
      note_template: t('sync.scopeTemplates'),
      settings: t('sync.scopeSettings'),
    }

    async function toggleTable(tableName: string) {
      if (expandedTable.value === tableName) {
        expandedTable.value = null
        return
      }
      expandedTable.value = tableName
      filterTable.value = tableName
      filterStatus.value = ''
      detailPage.value = 1
      await loadDetails()
    }

    async function loadDetails() {
      detailLoading.value = true
      try {
        const result = await syncApi.findSyncLogDetails(
          log.value.id,
          filterTable.value || undefined,
          filterStatus.value || undefined,
          detailPage.value,
          50,
        )
        details.value = result.data
        detailTotal.value = result.total
      } catch {
        details.value = []
      } finally {
        detailLoading.value = false
      }
    }

    return () => {
      return h('div', { class: 'space-y-4' }, [
        // 头部信息
        h('div', { class: 'text-sm text-content space-y-1' }, [
          h('div', { class: 'font-medium' }, [
            `${log.value.sourceProfile} (${log.value.sourceDbType})`,
            ' → ',
            `${log.value.targetProfile} (${log.value.targetDbType})`,
          ]),
          h('div', { class: 'text-xs text-content-tertiary' }, [
            `${log.value.syncMode === 'append' ? t('sync.modeAppend') : t('sync.modeOverwrite')}`,
            ` · ${log.value.startedAt}`,
            ` · ${t('sync.duration')}: ${duration.value}`,
          ]),
        ]),

        // 备份信息
        log.value.sourceBackup || log.value.targetBackup
          ? h('div', { class: 'text-xs p-2 bg-surface-dim rounded-lg space-y-0.5' }, [
              h('div', { class: 'font-medium text-content-secondary mb-1' }, t('sync.backupFiles')),
              log.value.sourceBackup
                ? h('div', { class: 'text-content-tertiary' }, [
                    `${t('sync.source')}: ${log.value.sourceBackup}`,
                  ])
                : null,
              log.value.targetBackup
                ? h('div', { class: 'text-content-tertiary' }, [
                    `${t('sync.target')}: ${log.value.targetBackup}`,
                  ])
                : null,
            ])
          : null,

        // 汇总表格
        h('div', { class: 'border border-edge rounded-lg overflow-hidden' }, [
          h('table', { class: 'w-full text-sm' }, [
            h('thead', [
              h('tr', { class: 'bg-surface-dim' }, [
                h(
                  'th',
                  { class: 'px-3 py-2 text-left text-xs font-medium text-content-secondary' },
                  t('sync.table'),
                ),
                h(
                  'th',
                  { class: 'px-3 py-2 text-right text-xs font-medium text-content-secondary' },
                  t('sync.total'),
                ),
                h(
                  'th',
                  { class: 'px-3 py-2 text-right text-xs font-medium text-green-600' },
                  t('sync.successCount'),
                ),
                h(
                  'th',
                  { class: 'px-3 py-2 text-right text-xs font-medium text-red-600' },
                  t('sync.failedCount'),
                ),
              ]),
            ]),
            h('tbody', [
              h('tr', { class: 'border-t border-edge bg-surface-dim/50 font-medium' }, [
                h('td', { class: 'px-3 py-2 text-content' }, t('sync.total')),
                h('td', { class: 'px-3 py-2 text-right text-content' }, log.value.totalCount),
                h('td', { class: 'px-3 py-2 text-right text-green-600' }, log.value.successCount),
                h('td', { class: 'px-3 py-2 text-right text-red-600' }, log.value.failedCount),
              ]),
            ]),
          ]),
        ]),

        // 详情展开区
        h('div', { class: 'space-y-2' }, [
          h(
            'button',
            {
              class: 'text-sm text-indigo-600 hover:text-indigo-700',
              onClick: () => toggleTable(expandedTable.value ? '' : 'note'),
            },
            expandedTable.value ? t('sync.collapseDetails') : t('sync.viewDetails'),
          ),
          expandedTable.value
            ? h('div', { class: 'space-y-2' }, [
                // 筛选器
                h('div', { class: 'flex gap-2' }, [
                  h(
                    'select',
                    {
                      class: 'px-2 py-1 text-xs border border-edge rounded bg-surface text-content',
                      value: filterTable.value,
                      onChange: (e: Event) => {
                        filterTable.value = (e.target as HTMLSelectElement).value
                        detailPage.value = 1
                        loadDetails()
                      },
                    },
                    [
                      h('option', { value: '' }, t('sync.allTables')),
                      ...tableNames.map((tn) => h('option', { value: tn }, tableLabels[tn] || tn)),
                    ],
                  ),
                  h(
                    'select',
                    {
                      class: 'px-2 py-1 text-xs border border-edge rounded bg-surface text-content',
                      value: filterStatus.value,
                      onChange: (e: Event) => {
                        filterStatus.value = (e.target as HTMLSelectElement).value
                        detailPage.value = 1
                        loadDetails()
                      },
                    },
                    [
                      h('option', { value: '' }, t('sync.allStatus')),
                      h('option', { value: 'success' }, t('sync.statusSuccess')),
                      h('option', { value: 'failed' }, t('sync.statusFailed')),
                      h('option', { value: 'skipped' }, t('sync.statusSkipped')),
                    ],
                  ),
                ]),
                // 明细列表
                detailLoading.value
                  ? h('div', { class: 'text-sm text-content-tertiary py-2' }, t('common.loading'))
                  : h(
                      'div',
                      { class: 'max-h-60 overflow-y-auto space-y-1' },
                      details.value.map((d) =>
                        h(
                          'div',
                          {
                            key: d.id,
                            class:
                              'flex items-start gap-2 px-2 py-1.5 text-xs rounded hover:bg-surface-dim',
                          },
                          [
                            h(
                              'span',
                              {
                                class:
                                  d.status === 'success'
                                    ? 'text-green-500 shrink-0'
                                    : d.status === 'failed'
                                      ? 'text-red-500 shrink-0'
                                      : 'text-slate-400 shrink-0',
                              },
                              d.status === 'success' ? '✓' : d.status === 'failed' ? '✗' : '○',
                            ),
                            h(
                              'span',
                              { class: 'text-content-tertiary shrink-0 w-10' },
                              `#${d.sourceId}`,
                            ),
                            h('span', { class: 'text-content truncate flex-1' }, d.recordName),
                            d.targetId
                              ? h(
                                  'span',
                                  { class: 'text-content-tertiary shrink-0' },
                                  `→ #${d.targetId}`,
                                )
                              : null,
                            d.errorMessage
                              ? h(
                                  'span',
                                  {
                                    class: 'text-red-500 text-[10px] shrink-0 max-w-40 truncate',
                                    title: d.errorMessage,
                                  },
                                  d.errorMessage,
                                )
                              : null,
                          ],
                        ),
                      ),
                    ),
                // 分页
                detailTotal.value > 50
                  ? h('div', { class: 'flex items-center justify-center gap-3 pt-2 text-xs' }, [
                      h(
                        'button',
                        {
                          class:
                            'px-2 py-1 rounded border border-edge hover:bg-surface-dim disabled:opacity-40',
                          disabled: detailPage.value <= 1,
                          onClick: () => {
                            detailPage.value--
                            loadDetails()
                          },
                        },
                        '← Prev',
                      ),
                      h(
                        'span',
                        { class: 'text-content-secondary' },
                        `${detailPage.value} / ${Math.ceil(detailTotal.value / 50)}`,
                      ),
                      h(
                        'button',
                        {
                          class:
                            'px-2 py-1 rounded border border-edge hover:bg-surface-dim disabled:opacity-40',
                          disabled: detailPage.value >= Math.ceil(detailTotal.value / 50),
                          onClick: () => {
                            detailPage.value++
                            loadDetails()
                          },
                        },
                        'Next →',
                      ),
                    ])
                  : null,
              ])
            : null,
        ]),
      ])
    }
  },
})
</script>
