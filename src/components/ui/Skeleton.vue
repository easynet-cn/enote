<template>
  <div
    class="skeleton"
    :class="[variantClass, { 'skeleton-animate': animate }]"
    :style="customStyle"
  />
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  variant?: 'text' | 'circular' | 'rectangular' | 'rounded'
  width?: string | number
  height?: string | number
  animate?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'text',
  animate: true,
})

const variantClass = computed(() => `skeleton-${props.variant}`)

const customStyle = computed(() => {
  const style: Record<string, string> = {}

  if (props.width) {
    style.width = typeof props.width === 'number' ? `${props.width}px` : props.width
  }

  if (props.height) {
    style.height = typeof props.height === 'number' ? `${props.height}px` : props.height
  }

  return style
})
</script>

<style scoped>
.skeleton {
  background-color: #e2e8f0;
  display: block;
}

.skeleton-animate {
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

@keyframes skeleton-pulse {
  0% {
    opacity: 1;
  }
  50% {
    opacity: 0.4;
  }
  100% {
    opacity: 1;
  }
}

.skeleton-text {
  height: 1em;
  border-radius: 4px;
  width: 100%;
}

.skeleton-circular {
  border-radius: 50%;
}

.skeleton-rectangular {
  border-radius: 0;
}

.skeleton-rounded {
  border-radius: 8px;
}
</style>
