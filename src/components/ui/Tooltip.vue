<template>
  <div
    ref="triggerRef"
    class="inline-block"
    @mouseenter="show"
    @mouseleave="hide"
  >
    <slot></slot>
    <Teleport to="body">
      <Transition name="tooltip">
        <div
          v-if="visible"
          ref="tooltipRef"
          class="fixed z-[9999] px-2 py-1 text-xs text-white bg-gray-800 rounded whitespace-nowrap pointer-events-none"
          :style="tooltipStyle"
        >
          {{ content }}
          <div
            class="absolute w-2 h-2 bg-gray-800 rotate-45"
            :style="arrowStyle"
          ></div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'

interface Props {
  content: string
  placement?: 'top' | 'bottom' | 'left' | 'right'
}

const props = withDefaults(defineProps<Props>(), {
  placement: 'bottom',
})

const visible = ref(false)
const triggerRef = ref<HTMLElement | null>(null)
const tooltipRef = ref<HTMLElement | null>(null)
const position = ref({ top: 0, left: 0 })

const show = async () => {
  visible.value = true
  await nextTick()
  updatePosition()
}

const hide = () => {
  visible.value = false
}

const updatePosition = () => {
  if (!triggerRef.value || !tooltipRef.value) return

  const triggerRect = triggerRef.value.getBoundingClientRect()
  const tooltipRect = tooltipRef.value.getBoundingClientRect()
  const gap = 8

  let top = 0
  let left = 0

  switch (props.placement) {
    case 'top':
      top = triggerRect.top - tooltipRect.height - gap
      left = triggerRect.left + (triggerRect.width - tooltipRect.width) / 2
      break
    case 'bottom':
      top = triggerRect.bottom + gap
      left = triggerRect.left + (triggerRect.width - tooltipRect.width) / 2
      break
    case 'left':
      top = triggerRect.top + (triggerRect.height - tooltipRect.height) / 2
      left = triggerRect.left - tooltipRect.width - gap
      break
    case 'right':
      top = triggerRect.top + (triggerRect.height - tooltipRect.height) / 2
      left = triggerRect.right + gap
      break
  }

  position.value = { top, left }
}

const tooltipStyle = computed(() => ({
  top: `${position.value.top}px`,
  left: `${position.value.left}px`,
}))

const arrowStyle = computed(() => {
  switch (props.placement) {
    case 'top':
      return { bottom: '-4px', left: '50%', transform: 'translateX(-50%)' }
    case 'bottom':
      return { top: '-4px', left: '50%', transform: 'translateX(-50%)' }
    case 'left':
      return { right: '-4px', top: '50%', transform: 'translateY(-50%)' }
    case 'right':
      return { left: '-4px', top: '50%', transform: 'translateY(-50%)' }
    default:
      return { top: '-4px', left: '50%', transform: 'translateX(-50%)' }
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
