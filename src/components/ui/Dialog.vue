<template>
  <Teleport to="body">
    <Transition name="dialog">
      <div v-if="modelValue" class="dialog-overlay" @click.self="handleOverlayClick">
        <!-- Overlay -->
        <div class="dialog-backdrop"></div>

        <!-- Dialog -->
        <div
          :class="['dialog-content', { 'dialog-fullscreen': fullscreen }]"
          :style="!fullscreen ? { width: width + 'px' } : {}"
        >
          <!-- Header -->
          <div class="dialog-header">
            <h3 class="dialog-title">{{ title }}</h3>
            <button class="dialog-close" @click="handleClose" aria-label="关闭">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M6 18L18 6M6 6l12 12"
                ></path>
              </svg>
            </button>
          </div>

          <!-- Body -->
          <div :class="['dialog-body', { 'dialog-body-fullscreen': fullscreen }]">
            <slot></slot>
          </div>

          <!-- Footer -->
          <div v-if="$slots.footer" class="dialog-footer">
            <slot name="footer"></slot>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { watch } from 'vue'

interface Props {
  modelValue: boolean
  title?: string
  width?: number
  fullscreen?: boolean
  closeOnClickOverlay?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  title: '',
  width: 500,
  fullscreen: false,
  closeOnClickOverlay: true,
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  open: []
  close: []
}>()

watch(
  () => props.modelValue,
  (val) => {
    if (val) {
      emit('open')
      document.body.style.overflow = 'hidden'
    } else {
      document.body.style.overflow = ''
    }
  },
)

const handleClose = () => {
  emit('update:modelValue', false)
  emit('close')
}

const handleOverlayClick = () => {
  if (props.closeOnClickOverlay) {
    handleClose()
  }
}
</script>

<style scoped>
/* Dialog 容器 */
.dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 50;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1rem;
}

/* 背景遮罩 */
.dialog-backdrop {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(4px);
  transition: opacity 0.3s ease;
}

/* Dialog 主体 */
.dialog-content {
  position: relative;
  background: white;
  border-radius: 12px;
  box-shadow:
    0 25px 50px -12px rgba(0, 0, 0, 0.25),
    0 0 0 1px rgba(0, 0, 0, 0.05);
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.dialog-fullscreen {
  width: 100%;
  height: 100%;
  border-radius: 0;
  max-height: 100vh;
}

/* 头部 */
.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.25rem 1.5rem;
  border-bottom: 1px solid #e2e8f0;
  background: linear-gradient(180deg, #f8fafc 0%, #ffffff 100%);
}

.dialog-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: #0f172a;
  margin: 0;
}

.dialog-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  color: #94a3b8;
  background: transparent;
  border: none;
  cursor: pointer;
  transition: all 0.2s ease;
}

.dialog-close:hover {
  background: #f1f5f9;
  color: #475569;
}

.dialog-close:active {
  transform: scale(0.95);
}

/* 内容区 */
.dialog-body {
  padding: 1.5rem;
  overflow-y: auto;
  max-height: 60vh;
  flex: 1;
}

.dialog-body-fullscreen {
  max-height: none;
}

/* 底部 */
.dialog-footer {
  padding: 1rem 1.5rem;
  border-top: 1px solid #e2e8f0;
  background: #f8fafc;
}

/* 入场/离场动画 */
.dialog-enter-active,
.dialog-leave-active {
  transition: opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.dialog-enter-from,
.dialog-leave-to {
  opacity: 0;
}

.dialog-enter-active .dialog-content,
.dialog-leave-active .dialog-content {
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.dialog-enter-from .dialog-content {
  opacity: 0;
  transform: scale(0.9) translateY(20px);
}

.dialog-leave-to .dialog-content {
  opacity: 0;
  transform: scale(0.95) translateY(-10px);
}
</style>
