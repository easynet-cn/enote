<template>
  <div class="relative inline-block" ref="dropdownRef">
    <div @click="toggle" class="cursor-pointer">
      <slot name="trigger"></slot>
    </div>

    <Transition name="dropdown">
      <div
        v-if="visible"
        class="absolute right-0 mt-1 min-w-[160px] bg-white rounded-lg shadow-lg border border-gray-200 py-1 z-50"
      >
        <slot></slot>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const visible = ref(false)
const dropdownRef = ref<HTMLElement>()

const toggle = () => {
  visible.value = !visible.value
}

const close = () => {
  visible.value = false
}

const handleClickOutside = (event: MouseEvent) => {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
    close()
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})

defineExpose({ close })
</script>

<style scoped>
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.15s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
