<template>
  <Tooltip :content="tooltip" placement="bottom">
    <button
      :class="['toolbar-btn', { active: active }]"
      :disabled="disabled"
      :aria-label="tooltip"
      @click="$emit('click')"
    >
      <slot />
    </button>
  </Tooltip>
</template>

<script setup lang="ts">
import { Tooltip } from '../ui'

interface Props {
  tooltip: string
  active?: boolean
  disabled?: boolean
}

defineProps<Props>()
defineEmits<{ click: [] }>()
</script>

<style scoped>
.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: var(--radius-md);
  color: var(--color-text-secondary);
  transition: all var(--transition-fast) var(--ease-default);
  border: none;
  background: transparent;
  cursor: pointer;
}

.toolbar-btn:hover:not(:disabled) {
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}

.toolbar-btn.active {
  background: var(--color-primary-lighter);
  color: var(--color-primary);
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
