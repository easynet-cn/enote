<template>
  <button
    :class="buttonClasses"
    :disabled="disabled || loading"
    :type="nativeType"
    @click="handleClick"
  >
    <!-- Loading 状态 -->
    <span v-if="loading" class="btn-loading">
      <svg class="animate-spin" viewBox="0 0 24 24" fill="none">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
        <path
          class="opacity-75"
          fill="currentColor"
          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
        />
      </svg>
    </span>
    <!-- 图标插槽 -->
    <span v-if="$slots.icon && !loading" class="btn-icon">
      <slot name="icon" />
    </span>
    <!-- 内容 -->
    <span v-if="$slots.default" class="btn-content">
      <slot />
    </span>
    <!-- 波纹效果容器 -->
    <span ref="rippleContainer" class="btn-ripple-container" />
  </button>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'

type ButtonType = 'primary' | 'secondary' | 'tertiary' | 'text' | 'danger' | 'success' | 'warning'
type ButtonSize = 'small' | 'medium' | 'large'

interface Props {
  type?: ButtonType
  size?: ButtonSize
  disabled?: boolean
  loading?: boolean
  round?: boolean
  circle?: boolean
  block?: boolean
  nativeType?: 'button' | 'submit' | 'reset'
}

const props = withDefaults(defineProps<Props>(), {
  type: 'secondary',
  size: 'medium',
  disabled: false,
  loading: false,
  round: false,
  circle: false,
  block: false,
  nativeType: 'button',
})

const emit = defineEmits<{
  click: [event: MouseEvent]
}>()

const rippleContainer = ref<HTMLElement>()

const buttonClasses = computed(() => {
  const classes = ['btn', `btn-${props.type}`, `btn-${props.size}`]

  if (props.round) classes.push('btn-round')
  if (props.circle) classes.push('btn-circle')
  if (props.block) classes.push('btn-block')
  if (props.disabled || props.loading) classes.push('btn-disabled')
  if (props.loading) classes.push('btn-loading-state')

  return classes
})

const handleClick = (event: MouseEvent) => {
  if (props.disabled || props.loading) return

  // 创建波纹效果
  createRipple(event)

  emit('click', event)
}

const createRipple = (event: MouseEvent) => {
  const container = rippleContainer.value
  if (!container) return

  const button = container.parentElement
  if (!button) return

  const rect = button.getBoundingClientRect()
  const size = Math.max(rect.width, rect.height)
  const x = event.clientX - rect.left - size / 2
  const y = event.clientY - rect.top - size / 2

  const ripple = document.createElement('span')
  ripple.className = 'btn-ripple'
  ripple.style.width = ripple.style.height = `${size}px`
  ripple.style.left = `${x}px`
  ripple.style.top = `${y}px`

  container.appendChild(ripple)

  ripple.addEventListener('animationend', () => {
    ripple.remove()
  })
}
</script>

<style scoped>
/* 基础按钮样式 */
.btn {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  font-weight: 500;
  border: none;
  cursor: pointer;
  outline: none;
  overflow: hidden;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  white-space: nowrap;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

/* 尺寸 */
.btn-small {
  height: 28px;
  padding: 0 12px;
  font-size: 12px;
  border-radius: 4px;
}

.btn-medium {
  height: 40px;
  padding: 0 20px;
  font-size: 14px;
  border-radius: 12px;
}

.btn-large {
  height: 44px;
  padding: 0 24px;
  font-size: 16px;
  border-radius: 8px;
}

/* 类型样式 - Primary */
.btn-primary {
  background: linear-gradient(135deg, #4f46e5 0%, #4338ca 100%);
  color: white;
  box-shadow: 0 4px 12px rgba(79, 70, 229, 0.3);
}

.btn-primary:hover:not(.btn-disabled) {
  background: linear-gradient(135deg, #4338ca 0%, #3730a3 100%);
  box-shadow: 0 6px 16px rgba(79, 70, 229, 0.4);
}

.btn-primary:active:not(.btn-disabled) {
  transform: scale(0.95);
  box-shadow: 0 2px 8px rgba(79, 70, 229, 0.3);
}

/* 类型样式 - Secondary */
.btn-secondary {
  background: white;
  color: #334155;
  border: 1px solid #cbd5e1;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.btn-secondary:hover:not(.btn-disabled) {
  background: #f8fafc;
  border-color: #94a3b8;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.btn-secondary:active:not(.btn-disabled) {
  background: #f1f5f9;
}

/* 类型样式 - Tertiary */
.btn-tertiary {
  background: #f1f5f9;
  color: #334155;
  border: none;
}

.btn-tertiary:hover:not(.btn-disabled) {
  background: #e2e8f0;
}

.btn-tertiary:active:not(.btn-disabled) {
  background: #cbd5e1;
}

/* 类型样式 - Text */
.btn-text {
  background: transparent;
  color: #4f46e5;
  border: none;
  padding-left: 8px;
  padding-right: 8px;
}

.btn-text:hover:not(.btn-disabled) {
  background: rgba(79, 70, 229, 0.1);
}

.btn-text:active:not(.btn-disabled) {
  background: rgba(79, 70, 229, 0.2);
}

/* 类型样式 - Danger */
.btn-danger {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white;
  box-shadow: 0 2px 4px rgba(239, 68, 68, 0.3);
}

.btn-danger:hover:not(.btn-disabled) {
  background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.4);
  transform: translateY(-1px);
}

.btn-danger:active:not(.btn-disabled) {
  transform: translateY(0);
}

/* 类型样式 - Success */
.btn-success {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
  box-shadow: 0 2px 4px rgba(16, 185, 129, 0.3);
}

.btn-success:hover:not(.btn-disabled) {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  box-shadow: 0 4px 12px rgba(16, 185, 129, 0.4);
  transform: translateY(-1px);
}

/* 类型样式 - Warning */
.btn-warning {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  color: white;
  box-shadow: 0 2px 4px rgba(245, 158, 11, 0.3);
}

.btn-warning:hover:not(.btn-disabled) {
  background: linear-gradient(135deg, #d97706 0%, #b45309 100%);
  box-shadow: 0 4px 12px rgba(245, 158, 11, 0.4);
  transform: translateY(-1px);
}

/* 圆角样式 */
.btn-round {
  border-radius: 9999px;
}

/* 圆形按钮 */
.btn-circle {
  border-radius: 50%;
  padding: 0;
}

.btn-circle.btn-small {
  width: 28px;
  height: 28px;
}

.btn-circle.btn-medium {
  width: 40px;
  height: 40px;
}

.btn-circle.btn-large {
  width: 44px;
  height: 44px;
}

/* 块级按钮 */
.btn-block {
  width: 100%;
}

/* 禁用状态 */
.btn-disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Loading 状态 */
.btn-loading-state {
  cursor: wait;
}

.btn-loading svg {
  width: 1em;
  height: 1em;
}

/* 图标 */
.btn-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.btn-icon :deep(svg) {
  width: 1em;
  height: 1em;
}

/* 波纹效果容器 */
.btn-ripple-container {
  position: absolute;
  inset: 0;
  overflow: hidden;
  border-radius: inherit;
  pointer-events: none;
}

/* 波纹效果 */
:deep(.btn-ripple) {
  position: absolute;
  border-radius: 50%;
  background: currentColor;
  opacity: 0.3;
  transform: scale(0);
  animation: ripple-effect 0.6s ease-out;
  pointer-events: none;
}

@keyframes ripple-effect {
  to {
    transform: scale(2.5);
    opacity: 0;
  }
}

/* Focus 样式 */
.btn:focus-visible {
  outline: 2px solid #4f46e5;
  outline-offset: 2px;
}

.btn-danger:focus-visible {
  outline-color: #ef4444;
}

.btn-warning:focus-visible {
  outline-color: #f59e0b;
}
</style>
