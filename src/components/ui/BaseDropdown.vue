<template>
  <div class="dropdown-wrapper" ref="dropdownRef">
    <div @click="toggle" class="dropdown-trigger">
      <slot name="trigger"></slot>
    </div>

    <Transition name="dropdown">
      <div v-if="visible" class="dropdown-menu">
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
.dropdown-wrapper {
  position: relative;
  display: inline-block;
}

.dropdown-trigger {
  cursor: pointer;
}

.dropdown-menu {
  position: absolute;
  right: 0;
  margin-top: 8px;
  min-width: 180px;
  background: white;
  border-radius: 10px;
  box-shadow:
    0 10px 40px rgba(0, 0, 0, 0.12),
    0 0 0 1px rgba(0, 0, 0, 0.05);
  padding: 6px;
  z-index: 50;
  overflow: hidden;
}

/* 入场/离场动画 */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.95);
}
</style>
