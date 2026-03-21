<template>
  <div
    ref="recorderRef"
    tabindex="0"
    class="shortcut-recorder"
    :class="{
      recording: isRecording,
      conflict: !!conflictName,
      customized: customized,
    }"
    @click="startRecording"
    @keydown="handleKeyDown"
    @blur="cancelRecording"
  >
    <span v-if="isRecording" class="recording-text">{{ t('settings.shortcutsRecording') }}</span>
    <span v-else class="binding-text">{{ displayText }}</span>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { bindingToText, type KeyBinding } from '../../config/shortcuts'

const { t } = useI18n()

const props = defineProps<{
  modelValue: KeyBinding
  disabled?: boolean
  customized?: boolean
  conflictName?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [binding: KeyBinding]
  record: [binding: KeyBinding]
}>()

const recorderRef = ref<HTMLElement>()
const isRecording = ref(false)

const displayText = computed(() => bindingToText(props.modelValue))

const startRecording = () => {
  if (props.disabled) return
  isRecording.value = true
  nextTick(() => recorderRef.value?.focus())
}

const cancelRecording = () => {
  isRecording.value = false
}

const handleKeyDown = (e: KeyboardEvent) => {
  if (!isRecording.value) return

  e.preventDefault()
  e.stopPropagation()

  // Escape 取消录制
  if (e.key === 'Escape') {
    isRecording.value = false
    return
  }

  // 忽略单独的修饰键
  if (['Control', 'Shift', 'Alt', 'Meta'].includes(e.key)) {
    return
  }

  const binding: KeyBinding = {
    key: e.key,
  }
  if (e.ctrlKey || e.metaKey) binding.ctrl = true
  if (e.shiftKey) binding.shift = true
  if (e.altKey) binding.alt = true

  isRecording.value = false
  emit('update:modelValue', binding)
  emit('record', binding)
}
</script>

<style scoped>
.shortcut-recorder {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 100px;
  height: 32px;
  padding: 0 12px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  background: var(--color-bg-primary);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-default);
  font-size: 13px;
  font-family: var(--font-mono);
  color: var(--color-text-primary);
  outline: none;
  user-select: none;
}

.shortcut-recorder:hover {
  border-color: var(--color-primary);
}

.shortcut-recorder:focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px var(--color-primary-light);
}

.shortcut-recorder.recording {
  border-color: var(--color-primary);
  background: var(--color-primary-lighter);
  box-shadow: 0 0 0 2px var(--color-primary-light);
}

.shortcut-recorder.conflict {
  border-color: var(--color-danger);
  box-shadow: 0 0 0 2px var(--color-danger-light);
}

.shortcut-recorder.customized {
  border-color: var(--color-info);
}

.recording-text {
  color: var(--color-primary);
  font-family: inherit;
  font-size: 12px;
}

.binding-text {
  white-space: nowrap;
}
</style>
