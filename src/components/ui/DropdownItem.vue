<template>
  <button class="dropdown-item" :class="{ 'dropdown-item-danger': danger }" @click="handleClick">
    <span v-if="$slots.icon" class="dropdown-item-icon">
      <slot name="icon"></slot>
    </span>
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
interface Props {
  command?: string
  danger?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  danger: false,
})

const emit = defineEmits<{
  command: [value: string]
}>()

const handleClick = () => {
  if (props.command) {
    emit('command', props.command)
  }
}
</script>

<style scoped>
.dropdown-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  font-size: 14px;
  color: #374151;
  background: transparent;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  text-align: left;
  transition: all 0.15s ease;
}

.dropdown-item:hover {
  background: #f3f4f6;
}

.dropdown-item:active {
  background: #e5e7eb;
  transform: scale(0.98);
}

.dropdown-item-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: #6b7280;
}

.dropdown-item-icon :deep(svg) {
  width: 16px;
  height: 16px;
}

/* 危险操作样式 */
.dropdown-item-danger {
  color: #ef4444;
}

.dropdown-item-danger:hover {
  background: #fef2f2;
}

.dropdown-item-danger .dropdown-item-icon {
  color: #ef4444;
}
</style>
