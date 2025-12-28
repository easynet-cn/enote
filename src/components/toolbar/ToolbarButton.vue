<template>
  <Tooltip :content="tooltip" placement="bottom">
    <button
      :class="['toolbar-btn', { active: active }]"
      :disabled="disabled"
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
  border-radius: 6px;
  color: #64748b;
  transition: all 0.15s ease;
  border: none;
  background: transparent;
  cursor: pointer;
}

.toolbar-btn:hover:not(:disabled) {
  background: #f1f5f9;
  color: #334155;
}

.toolbar-btn.active {
  background: #eef2ff;
  color: #4f46e5;
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
