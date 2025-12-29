<template>
  <div :class="wrapperClasses">
    <!-- 前置图标/内容 -->
    <span v-if="$slots.prefix || prefixIcon" class="input-prefix">
      <slot name="prefix">
        <component v-if="prefixIcon" :is="prefixIcon" class="input-icon" />
      </slot>
    </span>

    <!-- 输入框 -->
    <input
      ref="inputRef"
      :class="inputClasses"
      :type="showPassword ? 'text' : type"
      :value="modelValue"
      :placeholder="placeholder"
      :disabled="disabled"
      :readonly="readonly"
      :maxlength="maxlength"
      :autofocus="autofocus"
      @input="handleInput"
      @focus="handleFocus"
      @blur="handleBlur"
      @keydown.enter="$emit('enter', $event)"
    />

    <!-- 清除按钮 -->
    <span
      v-if="clearable && modelValue && !disabled && !readonly"
      class="input-clear"
      @click="handleClear"
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="10" />
        <path d="M15 9l-6 6M9 9l6 6" />
      </svg>
    </span>

    <!-- 密码显示/隐藏切换 -->
    <span
      v-if="type === 'password' && showPasswordToggle"
      class="input-password-toggle"
      @click="showPassword = !showPassword"
    >
      <svg
        v-if="showPassword"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
        <circle cx="12" cy="12" r="3" />
      </svg>
      <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path
          d="M17.94 17.94A10.07 10.07 0 0112 20c-7 0-11-8-11-8a18.45 18.45 0 015.06-5.94M9.9 4.24A9.12 9.12 0 0112 4c7 0 11 8 11 8a18.5 18.5 0 01-2.16 3.19m-6.72-1.07a3 3 0 11-4.24-4.24"
        />
        <line x1="1" y1="1" x2="23" y2="23" />
      </svg>
    </span>

    <!-- 后置图标/内容 -->
    <span v-if="$slots.suffix || suffixIcon" class="input-suffix">
      <slot name="suffix">
        <component v-if="suffixIcon" :is="suffixIcon" class="input-icon" />
      </slot>
    </span>

    <!-- 字数统计 -->
    <span v-if="showCount && maxlength" class="input-count">
      {{ modelValue?.length || 0 }}/{{ maxlength }}
    </span>

    <!-- 底部动画线条 -->
    <span class="input-focus-line" />
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import type { Component } from 'vue'

type InputSize = 'small' | 'medium' | 'large'
type InputStatus = 'default' | 'error' | 'warning' | 'success'

interface Props {
  modelValue?: string
  type?: string
  placeholder?: string
  disabled?: boolean
  readonly?: boolean
  clearable?: boolean
  showPasswordToggle?: boolean
  size?: InputSize
  status?: InputStatus
  maxlength?: number
  showCount?: boolean
  autofocus?: boolean
  prefixIcon?: Component
  suffixIcon?: Component
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  type: 'text',
  placeholder: '',
  disabled: false,
  readonly: false,
  clearable: false,
  showPasswordToggle: true,
  size: 'medium',
  status: 'default',
  showCount: false,
  autofocus: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
  input: [value: string]
  focus: [event: FocusEvent]
  blur: [event: FocusEvent]
  clear: []
  enter: [event: KeyboardEvent]
}>()

const inputRef = ref<HTMLInputElement>()
const isFocused = ref(false)
const showPassword = ref(false)

const wrapperClasses = computed(() => {
  const classes = ['input-wrapper', `input-${props.size}`, `input-status-${props.status}`]

  if (isFocused.value) classes.push('input-focused')
  if (props.disabled) classes.push('input-disabled')
  if (props.readonly) classes.push('input-readonly')

  return classes
})

const inputClasses = computed(() => {
  const classes = ['input-inner']
  return classes
})

const handleInput = (event: Event) => {
  const value = (event.target as HTMLInputElement).value
  emit('update:modelValue', value)
  emit('input', value)
}

const handleFocus = (event: FocusEvent) => {
  isFocused.value = true
  emit('focus', event)
}

const handleBlur = (event: FocusEvent) => {
  isFocused.value = false
  emit('blur', event)
}

const handleClear = () => {
  emit('update:modelValue', '')
  emit('clear')
  inputRef.value?.focus()
}

// 暴露方法
defineExpose({
  focus: () => inputRef.value?.focus(),
  blur: () => inputRef.value?.blur(),
  select: () => inputRef.value?.select(),
})
</script>

<style scoped>
.input-wrapper {
  position: relative;
  display: inline-flex;
  align-items: center;
  width: 100%;
  background: white;
  border: 1px solid #cbd5e1;
  border-radius: 8px;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
}

/* 尺寸 */
.input-small {
  height: 28px;
  padding: 0 8px;
  font-size: 12px;
}

.input-medium {
  height: 36px;
  padding: 0 12px;
  font-size: 14px;
}

.input-large {
  height: 44px;
  padding: 0 16px;
  font-size: 16px;
}

/* 输入框本体 */
.input-inner {
  flex: 1;
  width: 100%;
  height: 100%;
  border: none;
  outline: none;
  background: transparent;
  color: #0f172a;
  font-size: inherit;
}

.input-inner::placeholder {
  color: #94a3b8;
  transition: color 0.2s;
}

/* 图标和附加元素 */
.input-prefix,
.input-suffix {
  display: inline-flex;
  align-items: center;
  color: #94a3b8;
  flex-shrink: 0;
}

.input-prefix {
  margin-right: 8px;
}

.input-suffix {
  margin-left: 8px;
}

.input-icon {
  width: 16px;
  height: 16px;
}

/* 清除按钮 */
.input-clear {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  margin-left: 4px;
  color: #94a3b8;
  cursor: pointer;
  opacity: 0;
  transition: all 0.2s;
}

.input-clear svg {
  width: 14px;
  height: 14px;
}

.input-wrapper:hover .input-clear,
.input-focused .input-clear {
  opacity: 1;
}

.input-clear:hover {
  color: #64748b;
}

/* 密码切换 */
.input-password-toggle {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  margin-left: 4px;
  color: #94a3b8;
  cursor: pointer;
  transition: color 0.2s;
}

.input-password-toggle svg {
  width: 16px;
  height: 16px;
}

.input-password-toggle:hover {
  color: #64748b;
}

/* 字数统计 */
.input-count {
  margin-left: 8px;
  color: #94a3b8;
  font-size: 12px;
  flex-shrink: 0;
}

/* Focus 底部线条动画 */
.input-focus-line {
  position: absolute;
  left: 50%;
  bottom: -1px;
  width: 0;
  height: 2px;
  background: linear-gradient(90deg, #4f46e5, #4338ca);
  transform: translateX(-50%);
  transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border-radius: 0 0 8px 8px;
}

/* Hover 状态 */
.input-wrapper:hover:not(.input-disabled):not(.input-readonly) {
  border-color: #94a3b8;
}

/* Focus 状态 */
.input-focused:not(.input-disabled) {
  border-color: #4f46e5;
  box-shadow: 0 0 0 3px rgba(79, 70, 229, 0.1);
}

.input-focused .input-focus-line {
  width: 100%;
}

.input-focused .input-inner::placeholder {
  color: #64748b;
}

/* 禁用状态 */
.input-disabled {
  background: #f8fafc;
  cursor: not-allowed;
  opacity: 0.6;
}

.input-disabled .input-inner {
  cursor: not-allowed;
}

/* 只读状态 */
.input-readonly {
  background: #f8fafc;
}

/* 状态样式 - Error */
.input-status-error {
  border-color: #ef4444;
}

.input-status-error.input-focused {
  border-color: #ef4444;
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
}

.input-status-error .input-focus-line {
  background: linear-gradient(90deg, #ef4444, #dc2626);
}

/* 状态样式 - Warning */
.input-status-warning {
  border-color: #f59e0b;
}

.input-status-warning.input-focused {
  border-color: #f59e0b;
  box-shadow: 0 0 0 3px rgba(245, 158, 11, 0.1);
}

.input-status-warning .input-focus-line {
  background: linear-gradient(90deg, #f59e0b, #d97706);
}

/* 状态样式 - Success */
.input-status-success {
  border-color: #10b981;
}

.input-status-success.input-focused {
  border-color: #10b981;
  box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.1);
}

.input-status-success .input-focus-line {
  background: linear-gradient(90deg, #10b981, #059669);
}
</style>
