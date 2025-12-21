<template>
  <button
    class="w-full flex items-center gap-2 px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 transition-colors text-left"
    @click="handleClick"
  >
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
interface Props {
  command?: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  command: [value: string]
}>()

const handleClick = () => {
  if (props.command) {
    emit('command', props.command)
  }
}
</script>
