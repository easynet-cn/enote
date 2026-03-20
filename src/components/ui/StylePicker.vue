<template>
  <div ref="pickerRef" class="style-picker-wrapper">
    <!-- 触发器 -->
    <div
      class="style-picker-trigger"
      role="combobox"
      :aria-expanded="isOpen"
      aria-haspopup="listbox"
      tabindex="0"
      @click="toggleDropdown"
      @keydown.enter.prevent="toggleDropdown"
      @keydown.space.prevent="toggleDropdown"
      @keydown.escape="closeDropdown"
    >
      <!-- 颜色预览 -->
      <div class="style-picker-preview">
        <span v-if="modelValue" :class="['style-picker-dot', modelValue]">●</span>
        <span class="style-picker-value">{{ displayValue }}</span>
      </div>

      <!-- 清除按钮 -->
      <span v-if="clearable && modelValue" class="style-picker-clear" @click.stop="handleClear">
        <X :size="14" />
      </span>

      <!-- 箭头 -->
      <span :class="['style-picker-arrow', { 'style-picker-arrow-active': isOpen }]">
        <ChevronDown :size="16" />
      </span>
    </div>

    <!-- 下拉面板 -->
    <Teleport to="body">
      <Transition name="style-picker-dropdown">
        <div v-if="isOpen" ref="dropdownRef" class="style-picker-dropdown" :style="dropdownStyle">
          <!-- 预设颜色 -->
          <div class="style-picker-section">
            <div class="style-picker-section-title">{{ t('stylePicker.presetColors') }}</div>
            <div class="style-picker-colors">
              <div
                v-for="color in presetColors"
                :key="color.value"
                :class="[
                  'style-picker-color-item',
                  { 'style-picker-color-selected': modelValue === color.value },
                ]"
                :title="color.label"
                @click="selectColor(color.value)"
              >
                <span :class="['style-picker-color-dot', color.value]">●</span>
              </div>
            </div>
          </div>

          <!-- 色阶选择 -->
          <div v-if="selectedColorFamily" class="style-picker-section">
            <div class="style-picker-section-title">
              {{ selectedColorFamily.label }} {{ t('stylePicker.shades') }}
            </div>
            <div class="style-picker-shades">
              <div
                v-for="shade in selectedColorFamily.shades"
                :key="shade.value"
                :class="[
                  'style-picker-shade-item',
                  { 'style-picker-shade-selected': modelValue === shade.value },
                ]"
                :title="shade.label"
                @click="selectColor(shade.value)"
              >
                <span :class="['style-picker-shade-dot', shade.value]">●</span>
                <span class="style-picker-shade-label">{{ shade.name }}</span>
              </div>
            </div>
          </div>

          <!-- 自定义输入 -->
          <div class="style-picker-section">
            <div class="style-picker-section-title">{{ t('stylePicker.customStyle') }}</div>
            <div class="style-picker-custom">
              <input
                v-model="customInput"
                type="text"
                :placeholder="t('stylePicker.inputPlaceholder')"
                class="style-picker-custom-input"
                @keydown.enter="applyCustom"
              />
              <button
                class="style-picker-custom-btn"
                :disabled="!customInput.trim()"
                @click="applyCustom"
              >
                {{ t('stylePicker.apply') }}
              </button>
            </div>
            <div v-if="customInput" class="style-picker-custom-preview">
              <span>{{ t('stylePicker.preview') }}</span>
              <span :class="customInput.trim()">●</span>
            </div>
          </div>

          <!-- 已选信息 -->
          <div v-if="modelValue" class="style-picker-footer">
            <span class="style-picker-footer-text"
              >{{ t('stylePicker.current') }} {{ modelValue }}</span
            >
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { X, ChevronDown } from 'lucide-vue-next'
import { presetColors, colorFamilies } from '../../config/colors'

const { t } = useI18n()

interface Props {
  modelValue?: string | null
  placeholder?: string
  clearable?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: null,
  placeholder: '',
  clearable: true,
})

const emit = defineEmits<{
  'update:modelValue': [value: string | null]
  change: [value: string | null]
}>()

const pickerRef = ref<HTMLElement>()
const dropdownRef = ref<HTMLElement>()
const isOpen = ref(false)
const customInput = ref('')
const dropdownStyle = ref<Record<string, string>>({})

// 显示值
const displayValue = computed(() => {
  if (!props.modelValue) return props.placeholder || t('stylePicker.selectStyle')
  const preset = presetColors.find((c) => c.value === props.modelValue)
  if (preset) return preset.label
  return props.modelValue
})

// 当前选中的颜色族
const selectedColorFamily = computed(() => {
  if (!props.modelValue) return null
  // 从值中提取颜色族，如 text-red-500 -> red
  const match = props.modelValue.match(/text-(\w+)-\d+/)
  if (match && colorFamilies[match[1]]) {
    return colorFamilies[match[1]]
  }
  return null
})

// 切换下拉
const toggleDropdown = () => {
  if (isOpen.value) {
    closeDropdown()
  } else {
    openDropdown()
  }
}

// 打开下拉
const openDropdown = () => {
  isOpen.value = true
  updateDropdownPosition()
}

// 关闭下拉
const closeDropdown = () => {
  isOpen.value = false
  customInput.value = ''
}

// 更新下拉位置
const updateDropdownPosition = () => {
  if (!pickerRef.value) return

  const rect = pickerRef.value.getBoundingClientRect()
  const spaceBelow = window.innerHeight - rect.bottom
  const dropdownHeight = 450

  const showAbove = spaceBelow < dropdownHeight && rect.top > spaceBelow

  dropdownStyle.value = {
    position: 'fixed',
    left: `${Math.max(8, rect.left)}px`,
    width: '300px',
    zIndex: '9999',
    ...(showAbove
      ? { bottom: `${window.innerHeight - rect.top + 4}px` }
      : { top: `${rect.bottom + 4}px` }),
  }
}

// 选择颜色
const selectColor = (value: string) => {
  emit('update:modelValue', value)
  emit('change', value)
}

// 应用自定义样式
const applyCustom = () => {
  const value = customInput.value.trim()
  if (value) {
    emit('update:modelValue', value)
    emit('change', value)
    closeDropdown()
  }
}

// 清除选择
const handleClear = () => {
  emit('update:modelValue', null)
  emit('change', null)
}

// 点击外部关闭
const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as Node
  if (
    pickerRef.value &&
    !pickerRef.value.contains(target) &&
    dropdownRef.value &&
    !dropdownRef.value.contains(target)
  ) {
    closeDropdown()
  }
}

// 窗口事件
const handleScroll = () => {
  if (isOpen.value) {
    updateDropdownPosition()
  }
}

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
})
</script>

<style scoped>
.style-picker-wrapper {
  position: relative;
  display: inline-flex;
  width: 100%;
}

/* 触发器 */
.style-picker-trigger {
  display: flex;
  align-items: center;
  width: 100%;
  height: 36px;
  padding: 0 12px;
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border-dark, #d1d5db);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  gap: 8px;
}

.style-picker-trigger:hover {
  border-color: var(--color-text-tertiary, #9ca3af);
}

/* 预览 */
.style-picker-preview {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 8px;
}

.style-picker-dot {
  font-size: 16px;
}

.style-picker-value {
  font-size: 14px;
  color: var(--color-text-primary, #374151);
}

/* 清除按钮 */
.style-picker-clear {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-tertiary, #9ca3af);
  cursor: pointer;
  opacity: 0;
  transition: all 0.2s;
}

.style-picker-wrapper:hover .style-picker-clear {
  opacity: 1;
}

.style-picker-clear:hover {
  color: var(--color-text-secondary, #6b7280);
}

/* 箭头 */
.style-picker-arrow {
  display: inline-flex;
  align-items: center;
  color: var(--color-text-tertiary, #9ca3af);
  transition: transform 0.2s ease;
}

.style-picker-arrow-active {
  transform: rotate(180deg);
}

/* 下拉面板 */
.style-picker-dropdown {
  background: var(--color-bg-primary);
  border-radius: 12px;
  box-shadow: var(--shadow-dropdown);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  max-height: 450px;
  overflow-y: auto;
}

/* 区块 */
.style-picker-section {
  padding: 12px;
  border-bottom: 1px solid var(--color-border, #e5e7eb);
}

.style-picker-section:last-child {
  border-bottom: none;
}

.style-picker-section-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-secondary, #6b7280);
  margin-bottom: 8px;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

/* 预设颜色网格 */
.style-picker-colors {
  display: grid;
  grid-template-columns: repeat(10, 1fr);
  gap: 4px;
}

.style-picker-color-item {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
  border: 2px solid transparent;
}

.style-picker-color-item:hover {
  transform: scale(1.2);
}

.style-picker-color-selected {
  border-color: var(--color-text-primary, #1f2937);
  transform: scale(1.1);
}

.style-picker-color-dot {
  font-size: 18px;
}

/* 色阶选择 */
.style-picker-shades {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.style-picker-shade-item {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
  border: 1px solid var(--color-border, #e5e7eb);
  font-size: 12px;
}

.style-picker-shade-item:hover {
  background: var(--color-bg-tertiary, #f3f4f6);
  border-color: var(--color-border-dark, #d1d5db);
}

.style-picker-shade-selected {
  background: var(--color-success-lighter, #f0fdf4);
  border-color: var(--color-success, #22c55e);
}

.style-picker-shade-dot {
  font-size: 14px;
}

.style-picker-shade-label {
  color: var(--color-text-secondary, #6b7280);
}

/* 自定义输入 */
.style-picker-custom {
  display: flex;
  gap: 8px;
}

.style-picker-custom-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--color-border-dark, #d1d5db);
  border-radius: 6px;
  font-size: 13px;
  outline: none;
  color: var(--color-text-primary);
  background: var(--color-bg-primary);
  transition: all 0.2s ease;
}

.style-picker-custom-input:focus {
  border-color: var(--color-success, #22c55e);
  box-shadow: 0 0 0 3px var(--color-success-ring, rgba(34, 197, 94, 0.1));
}

.style-picker-custom-btn {
  padding: 8px 16px;
  background: var(--color-success, #22c55e);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.style-picker-custom-btn:hover:not(:disabled) {
  background: var(--color-success-dark, #16a34a);
}

.style-picker-custom-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 自定义预览 */
.style-picker-custom-preview {
  margin-top: 8px;
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--color-text-secondary, #6b7280);
}

/* 底部信息 */
.style-picker-footer {
  padding: 8px 12px;
  border-top: 1px solid var(--color-border, #e5e7eb);
  background: var(--color-bg-secondary, #f9fafb);
}

.style-picker-footer-text {
  font-size: 12px;
  color: var(--color-text-secondary, #6b7280);
}

/* 下拉动画 */
.style-picker-dropdown-enter-active,
.style-picker-dropdown-leave-active {
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.style-picker-dropdown-enter-from,
.style-picker-dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.95);
}

/* 滚动条 */
.style-picker-dropdown::-webkit-scrollbar {
  width: 6px;
}

.style-picker-dropdown::-webkit-scrollbar-track {
  background: transparent;
}

.style-picker-dropdown::-webkit-scrollbar-thumb {
  background: var(--color-border, #d1d5db);
  border-radius: 3px;
}

.style-picker-dropdown::-webkit-scrollbar-thumb:hover {
  background: var(--color-text-tertiary, #9ca3af);
}
</style>
