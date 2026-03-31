<template>
  <Dialog
    v-model="visible"
    :title="t('sync.title')"
    :width="620"
    :close-on-click-overlay="!syncing"
  >
    <!-- 同步配置 -->
    <div v-if="!syncing && !syncResult" class="space-y-5">
      <!-- 源和目标 -->
      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="text-xs font-medium text-content-secondary mb-1 block">{{
            t('sync.source')
          }}</label>
          <div class="px-3 py-2 text-sm bg-surface-dim rounded-lg text-content">
            {{ currentProfileName }}
          </div>
        </div>
        <div>
          <label class="text-xs font-medium text-content-secondary mb-1 block">{{
            t('sync.target')
          }}</label>
          <AppSelect
            v-model="targetProfileId"
            :options="profileSelectOptions"
            :placeholder="t('sync.selectTarget')"
            size="md"
            class="w-full"
            @change="handleTargetChange"
          />
        </div>
      </div>

      <!-- 目标数据库密码 -->
      <div v-if="targetNeedsPassword">
        <label class="text-xs font-medium text-content-secondary mb-1 block">{{
          t('sync.targetPassword')
        }}</label>
        <input
          v-model="targetDbPassword"
          type="password"
          :placeholder="t('sync.targetPasswordHint')"
          class="w-full px-3 py-2 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
        />
      </div>

      <!-- 同步模式 -->
      <div>
        <label class="text-xs font-medium text-content-secondary mb-2 block">{{
          t('sync.mode')
        }}</label>
        <div class="flex gap-3">
          <label
            class="flex-1 flex items-start gap-2 p-3 border rounded-lg cursor-pointer transition-colors"
            :class="
              syncMode === 'append'
                ? 'border-indigo-500 bg-indigo-50/50 dark:bg-indigo-950/20'
                : 'border-edge hover:border-slate-300'
            "
          >
            <input type="radio" v-model="syncMode" value="append" class="mt-0.5" />
            <div>
              <div class="text-sm font-medium text-content">{{ t('sync.modeAppend') }}</div>
              <div class="text-xs text-content-tertiary">{{ t('sync.modeAppendDesc') }}</div>
            </div>
          </label>
          <label
            class="flex-1 flex items-start gap-2 p-3 border rounded-lg cursor-pointer transition-colors"
            :class="
              syncMode === 'overwrite'
                ? 'border-red-500 bg-red-50/50 dark:bg-red-950/20'
                : 'border-edge hover:border-slate-300'
            "
          >
            <input type="radio" v-model="syncMode" value="overwrite" class="mt-0.5" />
            <div>
              <div class="text-sm font-medium text-content">{{ t('sync.modeOverwrite') }}</div>
              <div class="text-xs text-content-tertiary">{{ t('sync.modeOverwriteDesc') }}</div>
            </div>
          </label>
        </div>
      </div>

      <!-- 同步范围 -->
      <div>
        <label class="text-xs font-medium text-content-secondary mb-2 block">{{
          t('sync.scope')
        }}</label>
        <div class="flex flex-wrap gap-x-4 gap-y-2">
          <label class="flex items-center gap-1.5 text-sm text-content">
            <input type="checkbox" v-model="scope.notebooks" />
            {{ t('sync.scopeNotebooks') }}
          </label>
          <label class="flex items-center gap-1.5 text-sm text-content">
            <input type="checkbox" v-model="scope.tags" />
            {{ t('sync.scopeTags') }}
          </label>
          <label class="flex items-center gap-1.5 text-sm text-content">
            <input type="checkbox" v-model="scope.notes" />
            {{ t('sync.scopeNotes') }}
          </label>
          <label class="flex items-center gap-1.5 text-sm text-content">
            <input type="checkbox" v-model="scope.noteHistories" />
            {{ t('sync.scopeNoteHistories') }}
          </label>
          <label class="flex items-center gap-1.5 text-sm text-content">
            <input type="checkbox" v-model="scope.templates" />
            {{ t('sync.scopeTemplates') }}
          </label>
          <label class="flex items-center gap-1.5 text-sm text-content">
            <input type="checkbox" v-model="scope.settings" />
            {{ t('sync.scopeSettings') }}
          </label>
        </div>
      </div>

      <!-- 备份设置 -->
      <div>
        <label class="text-xs font-medium text-content-secondary mb-2 block">{{
          t('sync.backup')
        }}</label>
        <div class="flex items-center gap-4">
          <label class="flex items-center gap-1.5 text-sm text-content">
            <input type="checkbox" v-model="autoBackup" />
            {{ t('sync.autoBackup') }}
          </label>
          <AppSelect
            v-if="autoBackup"
            v-model="backupFormat"
            :options="backupFormatOptions"
            size="sm"
          />
        </div>
      </div>

      <!-- 预览 -->
      <div
        v-if="preview"
        class="p-3 bg-surface-dim rounded-lg border border-edge text-xs space-y-1"
      >
        <div class="font-medium text-content-secondary mb-1">{{ t('sync.preview') }}</div>
        <div class="text-content">
          {{ t('sync.source') }} ({{ preview.source.dbType }}): {{ preview.source.notebookCount }}
          {{ t('sync.scopeNotebooks') }} &middot; {{ preview.source.tagCount }}
          {{ t('sync.scopeTags') }} &middot; {{ preview.source.noteCount }}
          {{ t('sync.scopeNotes') }}
        </div>
        <div class="text-content">
          {{ t('sync.target') }} ({{ preview.target.dbType }}): {{ preview.target.notebookCount }}
          {{ t('sync.scopeNotebooks') }} &middot; {{ preview.target.tagCount }}
          {{ t('sync.scopeTags') }} &middot; {{ preview.target.noteCount }}
          {{ t('sync.scopeNotes') }}
        </div>
      </div>
    </div>

    <!-- 同步进度 -->
    <div v-if="syncing" class="space-y-4">
      <div class="text-sm font-medium text-content">{{ t('sync.syncing') }}</div>

      <div class="space-y-2">
        <div v-for="step in progressSteps" :key="step.key" class="flex items-center gap-2 text-sm">
          <span v-if="step.done" class="text-green-500">&#10003;</span>
          <span v-else-if="step.active" class="text-indigo-500 animate-pulse">&#9679;</span>
          <span v-else class="text-content-tertiary">&#9675;</span>
          <span
            :class="
              step.done ? 'text-content' : step.active ? 'text-indigo-600' : 'text-content-tertiary'
            "
          >
            {{ step.label }}
          </span>
          <span v-if="step.count" class="text-xs text-content-tertiary">{{ step.count }}</span>
        </div>
      </div>

      <!-- 进度条 -->
      <div v-if="progress" class="space-y-1">
        <div class="w-full h-2 bg-surface-dim rounded-full overflow-hidden">
          <div
            class="h-full bg-indigo-500 rounded-full transition-all duration-300"
            :style="{ width: progressPercent + '%' }"
          ></div>
        </div>
        <div class="text-xs text-content-tertiary text-right">{{ progressPercent }}%</div>
      </div>
    </div>

    <!-- 同步结果 -->
    <SyncResultDialog
      v-if="syncResult && !syncing"
      :sync-log="syncResult"
      :embedded="true"
      @close="handleClose"
    />

    <!-- 底部按钮 -->
    <template #footer v-if="!syncing && !syncResult">
      <div class="flex justify-end gap-2">
        <Button @click="handleClose">{{ t('common.cancel') }}</Button>
        <Button
          @click="handleStartSync"
          :disabled="!canSync || loading"
          class="bg-indigo-600 text-white hover:bg-indigo-700 disabled:opacity-50"
        >
          {{ loading ? t('common.loading') : t('sync.startSync') }}
        </Button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Dialog, Button, AppSelect } from './ui'
import type { AppSelectOption } from './ui'
import SyncResultDialog from './SyncResultDialog.vue'
import { useSync } from '../composables/useSync'
import { profileApi } from '../api/note'
import type { SyncMode, SyncScope } from '../types'

const props = defineProps<{
  modelValue: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
}>()

const { t } = useI18n()

const {
  loading,
  syncing,
  preview,
  progress,
  syncResult,
  profiles,
  loadProfiles,
  loadPreview,
  startSync,
} = useSync()

const visible = computed({
  get: () => props.modelValue,
  set: (v) => emit('update:modelValue', v),
})

const targetProfileId = ref('')
const targetDbPassword = ref('')
const syncMode = ref<SyncMode>('append')
const scope = ref<SyncScope>({
  notebooks: true,
  tags: true,
  notes: true,
  noteHistories: false,
  templates: false,
  settings: false,
})
const autoBackup = ref(true)
const backupFormat = ref('sql')

const profileSelectOptions = computed<AppSelectOption[]>(() =>
  profiles.value.map((p) => ({
    label: `${p.name} (${p.dbType})`,
    value: p.id,
  })),
)

const backupFormatOptions: AppSelectOption[] = [
  { label: 'SQL', value: 'sql' },
  { label: 'Excel', value: 'excel' },
  { label: 'CSV', value: 'csv' },
]

const currentProfileName = ref('')

const targetNeedsPassword = computed(() => {
  const p = profiles.value.find((p) => p.id === targetProfileId.value)
  return p && p.dbType !== 'sqlite'
})

const canSync = computed(() => {
  return (
    targetProfileId.value &&
    (scope.value.notebooks ||
      scope.value.tags ||
      scope.value.notes ||
      scope.value.noteHistories ||
      scope.value.templates ||
      scope.value.settings)
  )
})

const progressPercent = computed(() => {
  if (!progress.value || progress.value.total === 0) return 0
  // Rough estimate based on current stage
  return Math.min(99, Math.round((progress.value.current / progress.value.total) * 100))
})

const progressSteps = computed(() => {
  const p = progress.value
  const stages = [
    { key: 'backup', label: t('sync.autoBackup'), done: false, active: false, count: '' },
    { key: 'notebook', label: t('sync.scopeNotebooks'), done: false, active: false, count: '' },
    { key: 'tag', label: t('sync.scopeTags'), done: false, active: false, count: '' },
    { key: 'note', label: t('sync.scopeNotes'), done: false, active: false, count: '' },
    {
      key: 'note_history',
      label: t('sync.scopeNoteHistories'),
      done: false,
      active: false,
      count: '',
    },
    {
      key: 'note_template',
      label: t('sync.scopeTemplates'),
      done: false,
      active: false,
      count: '',
    },
    { key: 'settings', label: t('sync.scopeSettings'), done: false, active: false, count: '' },
  ]

  if (!p) return stages

  const currentKey = p.tableName || p.stage
  let found = false
  for (let i = stages.length - 1; i >= 0; i--) {
    if (stages[i].key === currentKey) {
      stages[i].active = true
      stages[i].count = `${p.current}/${p.total}`
      found = true
    } else if (!found) {
      // Not yet reached
    } else {
      stages[i].done = true
    }
  }

  return stages
})

watch(visible, async (v) => {
  if (v) {
    await loadProfiles()
    // Get current profile name
    try {
      const index = await profileApi.getProfileIndex()
      const allProfiles = await profileApi.listProfiles()
      const active = allProfiles.find((p) => p.isActive)
      currentProfileName.value = active
        ? `${active.name} (${active.dbType})`
        : index.active || 'Unknown'
    } catch {
      currentProfileName.value = 'Current Profile'
    }
  } else {
    // Reset state
    targetProfileId.value = ''
    targetDbPassword.value = ''
    syncResult.value = null
    preview.value = null
  }
})

async function handleTargetChange() {
  if (targetProfileId.value) {
    await loadPreview(targetProfileId.value, targetDbPassword.value || undefined)
  } else {
    preview.value = null
  }
}

async function handleStartSync() {
  if (syncMode.value === 'overwrite') {
    if (!confirm(t('sync.confirmOverwrite'))) return
  }

  await startSync(
    {
      targetProfileId: targetProfileId.value,
      mode: syncMode.value,
      scope: scope.value,
      autoBackup: autoBackup.value,
      backupFormat: autoBackup.value ? backupFormat.value : undefined,
    },
    targetDbPassword.value || undefined,
  )
}

function handleClose() {
  visible.value = false
}
</script>
