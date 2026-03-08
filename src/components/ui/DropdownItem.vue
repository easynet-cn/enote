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
  color: var(--color-text-primary);
  background: transparent;
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  text-align: left;
  transition: all var(--transition-fast) var(--ease-default);
}

.dropdown-item:hover {
  background: var(--color-bg-tertiary);
}

.dropdown-item:active {
  background: var(--color-border);
  transform: scale(0.98);
}

.dropdown-item-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-secondary);
}

.dropdown-item-icon :deep(svg) {
  width: 16px;
  height: 16px;
}

/* 危险操作样式 */
.dropdown-item-danger {
  color: var(--color-danger);
}

.dropdown-item-danger:hover {
  background: #fef2f2;
}

.dropdown-item-danger .dropdown-item-icon {
  color: var(--color-danger);
}
</style>
