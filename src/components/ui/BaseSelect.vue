<template>
  <div ref="selectRef" :class="wrapperClasses" @keydown="handleKeydown">
    <!-- 触发器 -->
    <div class="select-trigger" @click="toggleDropdown">
      <!-- 前置图标 -->
      <span v-if="$slots.prefix" class="select-prefix">
        <slot name="prefix" />
      </span>

      <!-- 选中值显示 / 搜索输入 -->
      <div class="select-value-wrapper">
        <input
          v-if="filterable && isOpen"
          ref="inputRef"
          v-model="searchQuery"
          class="select-search"
          :placeholder="selectedLabel || placeholder"
          @input="handleSearch"
          @click.stop
        />
        <span v-else-if="selectedLabel" class="select-value">{{ selectedLabel }}</span>
        <span v-else class="select-placeholder">{{ placeholder }}</span>
      </div>

      <!-- 清除按钮 -->
      <span
        v-if="clearable && modelValue && !disabled"
        class="select-clear"
        @click.stop="handleClear"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10" />
          <path d="M15 9l-6 6M9 9l6 6" />
        </svg>
      </span>

      <!-- 箭头图标 -->
      <span :class="['select-arrow', { 'select-arrow-active': isOpen }]">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="6 9 12 15 18 9" />
        </svg>
      </span>
    </div>

    <!-- 下拉面板 -->
    <Teleport to="body">
      <Transition name="select-dropdown">
        <div v-if="isOpen" ref="dropdownRef" class="select-dropdown" :style="dropdownStyle">
          <!-- 空状态 -->
          <div v-if="filteredOptions.length === 0" class="select-empty">
            <span>{{ emptyText }}</span>
          </div>

          <!-- 选项列表 -->
          <div v-else class="select-options">
            <template v-for="(option, index) in filteredOptions" :key="option.value">
              <!-- 分组标题 -->
              <div v-if="option.isGroup" class="select-group-label">
                {{ option.label }}
              </div>
              <!-- 选项 -->
              <div
                v-else
                :class="[
                  'select-option',
                  {
                    'select-option-selected': option.value === modelValue,
                    'select-option-highlighted': index === highlightedIndex,
                    'select-option-disabled': option.disabled,
                  },
                ]"
                @click="selectOption(option)"
                @mouseenter="highlightedIndex = index"
              >
                <slot name="option" :option="option">
                  <span class="select-option-label">{{ option.label }}</span>
                </slot>
                <!-- 选中图标 -->
                <span v-if="option.value === modelValue" class="select-option-check">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                    <polyline points="20 6 9 17 4 12" />
                  </svg>
                </span>
              </div>
            </template>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from 'vue'

export interface SelectOption {
  label: string
  value: string | number
  disabled?: boolean
  isGroup?: boolean
}

type SelectSize = 'small' | 'medium' | 'large'
type SelectStatus = 'default' | 'error' | 'warning' | 'success'

interface Props {
  modelValue?: string | number | null
  options?: SelectOption[]
  placeholder?: string
  disabled?: boolean
  clearable?: boolean
  filterable?: boolean
  size?: SelectSize
  status?: SelectStatus
  emptyText?: string
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: null,
  options: () => [],
  placeholder: '请选择',
  disabled: false,
  clearable: false,
  filterable: false,
  size: 'medium',
  status: 'default',
  emptyText: '无匹配选项',
})

const emit = defineEmits<{
  'update:modelValue': [value: string | number | null]
  change: [value: string | number | null]
  focus: []
  blur: []
  clear: []
}>()

const selectRef = ref<HTMLElement>()
const inputRef = ref<HTMLInputElement>()
const dropdownRef = ref<HTMLElement>()
const isOpen = ref(false)
const searchQuery = ref('')
const highlightedIndex = ref(-1)
const dropdownStyle = ref<Record<string, string>>({})

// 计算选中项的标签
const selectedLabel = computed(() => {
  const option = props.options.find((o) => o.value === props.modelValue)
  return option?.label || ''
})

// 过滤后的选项
const filteredOptions = computed(() => {
  if (!props.filterable || !searchQuery.value) {
    return props.options
  }
  const query = searchQuery.value.toLowerCase()
  return props.options.filter(
    (option) => option.isGroup || option.label.toLowerCase().includes(query),
  )
})

// 包装器类
const wrapperClasses = computed(() => {
  const classes = ['select-wrapper', `select-${props.size}`, `select-status-${props.status}`]
  if (isOpen.value) classes.push('select-open')
  if (props.disabled) classes.push('select-disabled')
  return classes
})

// 切换下拉
const toggleDropdown = () => {
  if (props.disabled) return
  if (isOpen.value) {
    closeDropdown()
  } else {
    openDropdown()
  }
}

// 打开下拉
const openDropdown = () => {
  isOpen.value = true
  emit('focus')
  updateDropdownPosition()

  nextTick(() => {
    if (props.filterable && inputRef.value) {
      inputRef.value.focus()
    }
    // 高亮当前选中项
    const index = filteredOptions.value.findIndex((o) => o.value === props.modelValue)
    highlightedIndex.value = index >= 0 ? index : 0
  })
}

// 关闭下拉
const closeDropdown = () => {
  isOpen.value = false
  searchQuery.value = ''
  highlightedIndex.value = -1
  emit('blur')
}

// 更新下拉位置
const updateDropdownPosition = () => {
  if (!selectRef.value) return

  const rect = selectRef.value.getBoundingClientRect()
  const spaceBelow = window.innerHeight - rect.bottom
  const spaceAbove = rect.top
  const dropdownHeight = 280 // 最大高度

  // 决定向上还是向下展开
  const showAbove = spaceBelow < dropdownHeight && spaceAbove > spaceBelow

  dropdownStyle.value = {
    position: 'fixed',
    left: `${rect.left}px`,
    width: `${rect.width}px`,
    zIndex: '9999',
    ...(showAbove
      ? { bottom: `${window.innerHeight - rect.top + 4}px` }
      : { top: `${rect.bottom + 4}px` }),
  }
}

// 选择选项
const selectOption = (option: SelectOption) => {
  if (option.disabled || option.isGroup) return

  emit('update:modelValue', option.value)
  emit('change', option.value)
  closeDropdown()
}

// 清除选择
const handleClear = () => {
  emit('update:modelValue', null)
  emit('change', null)
  emit('clear')
}

// 搜索处理
const handleSearch = () => {
  highlightedIndex.value = 0
}

// 键盘导航
const handleKeydown = (event: KeyboardEvent) => {
  if (props.disabled) return

  switch (event.key) {
    case 'Enter':
      event.preventDefault()
      if (isOpen.value && highlightedIndex.value >= 0) {
        const option = filteredOptions.value[highlightedIndex.value]
        if (option && !option.disabled && !option.isGroup) {
          selectOption(option)
        }
      } else {
        openDropdown()
      }
      break

    case 'Escape':
      event.preventDefault()
      closeDropdown()
      break

    case 'ArrowDown':
      event.preventDefault()
      if (!isOpen.value) {
        openDropdown()
      } else {
        const nextIndex = findNextValidIndex(highlightedIndex.value, 1)
        if (nextIndex !== -1) highlightedIndex.value = nextIndex
      }
      break

    case 'ArrowUp':
      event.preventDefault()
      if (isOpen.value) {
        const prevIndex = findNextValidIndex(highlightedIndex.value, -1)
        if (prevIndex !== -1) highlightedIndex.value = prevIndex
      }
      break

    case 'Tab':
      closeDropdown()
      break
  }
}

// 查找下一个有效索引
const findNextValidIndex = (currentIndex: number, direction: number): number => {
  const options = filteredOptions.value
  let index = currentIndex + direction

  while (index >= 0 && index < options.length) {
    const option = options[index]
    if (!option.disabled && !option.isGroup) {
      return index
    }
    index += direction
  }

  return -1
}

// 点击外部关闭
const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as Node
  if (
    selectRef.value &&
    !selectRef.value.contains(target) &&
    dropdownRef.value &&
    !dropdownRef.value.contains(target)
  ) {
    closeDropdown()
  }
}

// 窗口滚动/调整大小时更新位置
const handleScroll = () => {
  if (isOpen.value) {
    updateDropdownPosition()
  }
}

// 监听高亮项变化，滚动到可见区域
watch(highlightedIndex, (index) => {
  if (index >= 0 && dropdownRef.value) {
    const optionEl = dropdownRef.value.querySelectorAll('.select-option')[index] as HTMLElement
    if (optionEl) {
      optionEl.scrollIntoView({ block: 'nearest' })
    }
  }
})

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  window.addEventListener('scroll', handleScroll, true)
  window.addEventListener('resize', handleScroll)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  window.removeEventListener('scroll', handleScroll, true)
  window.removeEventListener('resize', handleScroll)
})

// 暴露方法
defineExpose({
  open: openDropdown,
  close: closeDropdown,
  focus: () => selectRef.value?.focus(),
})
</script>

<style scoped>
.select-wrapper {
  position: relative;
  display: inline-flex;
  width: 100%;
}

/* 尺寸 */
.select-small .select-trigger {
  height: 28px;
  padding: 0 8px;
  font-size: 12px;
}

.select-medium .select-trigger {
  height: 36px;
  padding: 0 12px;
  font-size: 14px;
}

.select-large .select-trigger {
  height: 44px;
  padding: 0 16px;
  font-size: 16px;
}

/* 触发器 */
.select-trigger {
  display: flex;
  align-items: center;
  width: 100%;
  background: white;
  border: 1px solid #cbd5e1;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  gap: 8px;
}

.select-trigger:hover:not(.select-disabled .select-trigger) {
  border-color: #94a3b8;
}

.select-open .select-trigger {
  border-color: #4f46e5;
  box-shadow: 0 0 0 3px rgba(79, 70, 229, 0.1);
}

/* 禁用状态 */
.select-disabled .select-trigger {
  background: #f8fafc;
  cursor: not-allowed;
  opacity: 0.6;
}

/* 前置内容 */
.select-prefix {
  display: inline-flex;
  align-items: center;
  color: #94a3b8;
  flex-shrink: 0;
}

/* 值显示区域 */
.select-value-wrapper {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
}

.select-value {
  color: #0f172a;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.select-placeholder {
  color: #94a3b8;
}

/* 搜索输入 */
.select-search {
  width: 100%;
  border: none;
  outline: none;
  background: transparent;
  font-size: inherit;
  color: #0f172a;
}

.select-search::placeholder {
  color: #94a3b8;
}

/* 清除按钮 */
.select-clear {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  color: #94a3b8;
  cursor: pointer;
  opacity: 0;
  transition: all 0.2s;
  flex-shrink: 0;
}

.select-clear svg {
  width: 14px;
  height: 14px;
}

.select-wrapper:hover .select-clear,
.select-open .select-clear {
  opacity: 1;
}

.select-clear:hover {
  color: #64748b;
}

/* 箭头 */
.select-arrow {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: #94a3b8;
  transition: transform 0.2s ease;
  flex-shrink: 0;
}

.select-arrow svg {
  width: 16px;
  height: 16px;
}

.select-arrow-active {
  transform: rotate(180deg);
}

/* 下拉面板 */
.select-dropdown {
  background: white;
  border-radius: 12px;
  box-shadow:
    0 10px 40px rgba(0, 0, 0, 0.12),
    0 0 0 1px rgba(0, 0, 0, 0.05);
  max-height: 280px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* 选项列表 */
.select-options {
  padding: 6px;
  overflow-y: auto;
  flex: 1;
}

/* 选项 */
.select-option {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s ease;
  gap: 8px;
}

.select-option:hover,
.select-option-highlighted {
  background: #f1f5f9;
}

.select-option-selected {
  color: #4f46e5;
  font-weight: 500;
}

.select-option-selected:hover,
.select-option-selected.select-option-highlighted {
  background: #eef2ff;
}

.select-option-disabled {
  color: #cbd5e1;
  cursor: not-allowed;
}

.select-option-disabled:hover {
  background: transparent;
}

.select-option-label {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 选中图标 */
.select-option-check {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: #4f46e5;
  flex-shrink: 0;
}

.select-option-check svg {
  width: 16px;
  height: 16px;
}

/* 分组标题 */
.select-group-label {
  padding: 8px 12px 4px;
  font-size: 12px;
  font-weight: 600;
  color: #94a3b8;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

/* 空状态 */
.select-empty {
  padding: 24px 16px;
  text-align: center;
  color: #94a3b8;
  font-size: 14px;
}

/* 状态样式 */
.select-status-error .select-trigger {
  border-color: #ef4444;
}

.select-status-error.select-open .select-trigger {
  border-color: #ef4444;
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
}

.select-status-warning .select-trigger {
  border-color: #f59e0b;
}

.select-status-warning.select-open .select-trigger {
  border-color: #f59e0b;
  box-shadow: 0 0 0 3px rgba(245, 158, 11, 0.1);
}

.select-status-success .select-trigger {
  border-color: #10b981;
}

.select-status-success.select-open .select-trigger {
  border-color: #10b981;
  box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.1);
}

/* 下拉动画 */
.select-dropdown-enter-active,
.select-dropdown-leave-active {
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.select-dropdown-enter-from,
.select-dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.95);
}

/* 自定义滚动条 */
.select-options::-webkit-scrollbar {
  width: 6px;
}

.select-options::-webkit-scrollbar-track {
  background: transparent;
}

.select-options::-webkit-scrollbar-thumb {
  background: #e2e8f0;
  border-radius: 10px;
}

.select-options::-webkit-scrollbar-thumb:hover {
  background: #cbd5e1;
}
</style>
