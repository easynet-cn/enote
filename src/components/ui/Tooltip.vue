<template>
  <div class="relative inline-block" @mouseenter="show" @mouseleave="hide">
    <slot></slot>
    <Transition name="tooltip">
      <div
        v-if="visible"
        :class="[
          'absolute z-50 px-2 py-1 text-xs text-white bg-gray-800 rounded whitespace-nowrap',
          placementClasses,
        ]"
      >
        {{ content }}
        <div :class="['absolute w-2 h-2 bg-gray-800 rotate-45', arrowClasses]"></div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface Props {
  content: string
  placement?: 'top' | 'bottom' | 'left' | 'right'
}

const props = withDefaults(defineProps<Props>(), {
  placement: 'bottom',
})

const visible = ref(false)

const show = () => {
  visible.value = true
}

const hide = () => {
  visible.value = false
}

const placementClasses = computed(() => {
  switch (props.placement) {
    case 'top':
      return 'bottom-full left-1/2 -translate-x-1/2 mb-2'
    case 'bottom':
      return 'top-full left-1/2 -translate-x-1/2 mt-2'
    case 'left':
      return 'right-full top-1/2 -translate-y-1/2 mr-2'
    case 'right':
      return 'left-full top-1/2 -translate-y-1/2 ml-2'
    default:
      return 'top-full left-1/2 -translate-x-1/2 mt-2'
  }
})

const arrowClasses = computed(() => {
  switch (props.placement) {
    case 'top':
      return 'top-full left-1/2 -translate-x-1/2 -mt-1'
    case 'bottom':
      return 'bottom-full left-1/2 -translate-x-1/2 -mb-1'
    case 'left':
      return 'left-full top-1/2 -translate-y-1/2 -ml-1'
    case 'right':
      return 'right-full top-1/2 -translate-y-1/2 -mr-1'
    default:
      return 'bottom-full left-1/2 -translate-x-1/2 -mb-1'
  }
})
</script>

<style scoped>
.tooltip-enter-active,
.tooltip-leave-active {
  transition: opacity 0.15s ease;
}

.tooltip-enter-from,
.tooltip-leave-to {
  opacity: 0;
}
</style>
