<template>
  <Teleport to="body">
    <Transition name="dialog" @after-enter="handleAfterEnter" @after-leave="handleAfterLeave">
      <div v-if="modelValue" class="dialog-overlay" @click.self="handleOverlayClick">
        <!-- Overlay -->
        <div class="dialog-backdrop" aria-hidden="true"></div>

        <!-- Dialog -->
        <div
          ref="dialogRef"
          role="dialog"
          aria-modal="true"
          :aria-labelledby="titleId"
          :class="['dialog-content', { 'dialog-fullscreen': fullscreen }]"
          :style="!fullscreen ? { width: width + 'px' } : {}"
          @keydown.esc="handleClose"
          @keydown.tab="handleTabTrap"
        >
          <!-- Header -->
          <div class="dialog-header">
            <h3 :id="titleId" class="dialog-title">{{ title }}</h3>
            <button class="dialog-close" @click="handleClose" :aria-label="t('aria.closeDialog')">
              <svg
                class="w-5 h-5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                aria-hidden="true"
              >
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
import { watch, ref } from 'vue'
import { useI18n } from 'vue-i18n'

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

const { t } = useI18n()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  open: []
  close: []
}>()

// 生成唯一的 title ID（setup 时一次性生成）
const titleId = `dialog-title-${Math.random().toString(36).slice(2, 9)}`

// Dialog 元素引用
const dialogRef = ref<HTMLElement | null>(null)

// 保存打开对话框前的焦点元素
let previousActiveElement: HTMLElement | null = null

watch(
  () => props.modelValue,
  (val) => {
    if (val) {
      // 保存当前焦点元素
      previousActiveElement = document.activeElement as HTMLElement
      emit('open')
      document.body.style.overflow = 'hidden'
    } else {
      document.body.style.overflow = ''
    }
  },
)

// 对话框打开后聚焦
const handleAfterEnter = () => {
  // 聚焦到对话框内的第一个可聚焦元素
  const focusable = dialogRef.value?.querySelector<HTMLElement>(
    'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
  )
  focusable?.focus()
}

// 对话框关闭后恢复焦点
const handleAfterLeave = () => {
  // 恢复之前的焦点
  previousActiveElement?.focus()
  previousActiveElement = null
}

// 焦点陷阱：Tab/Shift+Tab 在对话框内循环
const handleTabTrap = (e: KeyboardEvent) => {
  if (!dialogRef.value) return

  const focusableSelector =
    'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])'
  const focusableElements = dialogRef.value.querySelectorAll<HTMLElement>(focusableSelector)
  if (focusableElements.length === 0) return

  const firstFocusable = focusableElements[0]
  const lastFocusable = focusableElements[focusableElements.length - 1]

  if (e.shiftKey) {
    // Shift+Tab: 如果焦点在第一个元素，跳到最后一个
    if (document.activeElement === firstFocusable) {
      e.preventDefault()
      lastFocusable.focus()
    }
  } else {
    // Tab: 如果焦点在最后一个元素，跳到第一个
    if (document.activeElement === lastFocusable) {
      e.preventDefault()
      firstFocusable.focus()
    }
  }
}

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
  background: var(--color-bg-primary);
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
  border-bottom: 1px solid var(--color-border);
  background: linear-gradient(180deg, var(--color-bg-secondary) 0%, var(--color-bg-primary) 100%);
}

.dialog-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
}

.dialog-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  color: var(--color-text-tertiary);
  background: transparent;
  border: none;
  cursor: pointer;
  transition: all var(--transition-normal) var(--ease-default);
}

.dialog-close:hover {
  background: var(--color-bg-tertiary);
  color: var(--color-text-secondary);
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
  border-top: 1px solid var(--color-border);
  background: var(--color-bg-secondary);
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
