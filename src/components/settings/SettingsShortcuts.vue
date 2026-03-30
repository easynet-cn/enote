<script setup lang="ts">
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useShortcutSettings } from '../../composables/useShortcutSettings'
import { SHORTCUT_DEFS, bindingToText, type KeyBinding } from '../../config/shortcuts'
import ShortcutRecorder from '../ui/ShortcutRecorder.vue'
import { showNotification } from '../ui/notification'

const { t } = useI18n()

const {
  getBinding,
  setBinding: setShortcutBinding,
  resetBinding: resetShortcutBinding,
  resetAll: resetAllShortcuts,
  isCustomized: isShortcutCustomized,
  checkConflict: checkShortcutConflict,
  save: saveShortcuts,
} = useShortcutSettings()

const shortcutDefs = SHORTCUT_DEFS
const shortcutConflicts = ref<Record<string, string>>({})

const shortcutBindings = computed(() => {
  const result: Record<string, KeyBinding> = {}
  for (const def of SHORTCUT_DEFS) {
    result[def.id] = getBinding(def.id)
  }
  return result
})

const shortcutCustomized = computed(() => {
  const result: Record<string, boolean> = {}
  for (const def of SHORTCUT_DEFS) {
    result[def.id] = isShortcutCustomized(def.id)
  }
  return result
})

const handleShortcutRecord = async (id: string, binding: KeyBinding) => {
  const conflicts = { ...shortcutConflicts.value }
  delete conflicts[id]

  const conflict = checkShortcutConflict(id, binding)
  if (conflict) {
    conflicts[id] = t(conflict.descriptionKey)
    shortcutConflicts.value = conflicts
    return
  }

  shortcutConflicts.value = conflicts
  setShortcutBinding(id, binding)
  await saveShortcuts()
}

const handleResetShortcut = async (id: string) => {
  const conflicts = { ...shortcutConflicts.value }
  delete conflicts[id]
  shortcutConflicts.value = conflicts
  resetShortcutBinding(id)
  await saveShortcuts()
}

const handleResetAllShortcuts = async () => {
  shortcutConflicts.value = {}
  resetAllShortcuts()
  await saveShortcuts()
  showNotification({ type: 'success', message: t('settings.shortcutsResetAllDone') })
}
</script>

<template>
  <div>
    <div class="flex items-center justify-between mb-3">
      <h3 class="text-sm font-semibold text-content-secondary">
        {{ t('settings.shortcutsTitle') }}
      </h3>
      <button
        @click="handleResetAllShortcuts"
        class="text-xs text-content-tertiary hover:text-indigo-600 transition-colors"
      >
        {{ t('settings.shortcutsResetAll') }}
      </button>
    </div>
    <div class="space-y-2">
      <div
        v-for="def in shortcutDefs"
        :key="def.id"
        class="flex items-center justify-between py-1.5"
      >
        <label class="text-sm text-content-secondary flex-1 min-w-0 truncate mr-3">
          {{ t(def.descriptionKey) }}
        </label>
        <div class="flex items-center gap-2 shrink-0">
          <template v-if="def.system">
            <span
              class="inline-flex items-center h-8 px-3 text-xs font-mono border border-edge rounded-md bg-surface-dim text-content-secondary"
            >
              {{ bindingToText(def.defaultBinding) }}
            </span>
            <span class="text-xs text-content-tertiary w-10 text-center">{{
              t('settings.shortcutsSystem')
            }}</span>
          </template>
          <template v-else>
            <div class="flex flex-col items-end gap-0.5">
              <ShortcutRecorder
                :model-value="shortcutBindings[def.id]"
                :customized="shortcutCustomized[def.id]"
                :conflict-name="shortcutConflicts[def.id]"
                @record="(b: KeyBinding) => handleShortcutRecord(def.id, b)"
              />
              <span v-if="shortcutConflicts[def.id]" class="text-xs text-red-500">
                {{ t('settings.shortcutsConflict', { name: shortcutConflicts[def.id] }) }}
              </span>
            </div>
            <button
              v-if="shortcutCustomized[def.id]"
              @click="handleResetShortcut(def.id)"
              class="text-xs text-content-tertiary hover:text-indigo-600 transition-colors w-10 text-center"
            >
              {{ t('settings.shortcutsReset') }}
            </button>
            <span v-else class="w-10" />
          </template>
        </div>
      </div>
    </div>
  </div>
</template>
