<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const mcpEnabled = defineModel<boolean>('enabled', { required: true })
const mcpToolEnabled = defineModel<Record<string, boolean>>('toolEnabled', { required: true })

const emit = defineEmits<{
  (e: 'save'): void
}>()

const mcpTools = computed(() => [
  { key: 'search_notes', label: t('settings.mcpToolSearch') },
  { key: 'get_note', label: t('settings.mcpToolGetNote') },
  { key: 'create_note', label: t('settings.mcpToolCreateNote') },
  { key: 'update_note', label: t('settings.mcpToolUpdateNote') },
  { key: 'delete_note', label: t('settings.mcpToolDeleteNote') },
  { key: 'list_notebooks', label: t('settings.mcpToolListNotebooks') },
  { key: 'create_notebook', label: t('settings.mcpToolCreateNotebook') },
  { key: 'list_tags', label: t('settings.mcpToolListTags') },
  { key: 'create_tag', label: t('settings.mcpToolCreateTag') },
  { key: 'note_stats', label: t('settings.mcpToolNoteStats') },
])

const toggleMcpEnabled = () => {
  mcpEnabled.value = !mcpEnabled.value
  emit('save')
}

const toggleMcpTool = (key: string) => {
  mcpToolEnabled.value[key] = !mcpToolEnabled.value[key]
  emit('save')
}
</script>

<template>
  <div>
    <h3 class="text-sm font-semibold text-content-secondary mb-3">
      {{ t('settings.mcp') }}
    </h3>
    <div class="space-y-3">
      <!-- MCP 总开关 -->
      <div class="flex items-center justify-between">
        <div>
          <label class="text-sm text-content-secondary">{{ t('settings.mcpEnabled') }}</label>
          <p class="text-xs text-content-tertiary mt-0.5">{{ t('settings.mcpEnabledDesc') }}</p>
        </div>
        <button
          @click="toggleMcpEnabled"
          class="relative w-10 h-5 rounded-full transition-colors"
          :class="mcpEnabled ? 'bg-indigo-600' : 'bg-slate-300'"
        >
          <span
            class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full transition-transform shadow-sm"
            :class="mcpEnabled ? 'translate-x-5' : ''"
          />
        </button>
      </div>

      <!-- 各工具开关 -->
      <div v-if="mcpEnabled" class="space-y-2 pl-2 border-l-2 border-edge ml-1">
        <div
          v-for="tool in mcpTools"
          :key="tool.key"
          class="flex items-center justify-between py-1"
        >
          <label class="text-sm text-content-secondary">{{ tool.label }}</label>
          <button
            @click="toggleMcpTool(tool.key)"
            class="relative w-9 h-[18px] rounded-full transition-colors"
            :class="mcpToolEnabled[tool.key] ? 'bg-indigo-600' : 'bg-slate-300'"
          >
            <span
              class="absolute top-0.5 left-0.5 w-3.5 h-3.5 bg-white rounded-full transition-transform shadow-sm"
              :class="mcpToolEnabled[tool.key] ? 'translate-x-[18px]' : ''"
            />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
