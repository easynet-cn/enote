<template>
  <div ref="pickerRef" class="icon-picker-wrapper">
    <!-- 触发器 -->
    <div class="icon-picker-trigger" @click="toggleDropdown">
      <!-- 已选图标预览 -->
      <div class="icon-picker-preview">
        <component
          v-if="modelValue && iconComponents[modelValue]"
          :is="iconComponents[modelValue]"
          class="icon-picker-selected"
        />
        <span v-else class="icon-picker-placeholder">{{ placeholder }}</span>
      </div>

      <!-- 清除按钮 -->
      <span v-if="clearable && modelValue" class="icon-picker-clear" @click.stop="handleClear">
        <X :size="14" />
      </span>

      <!-- 箭头 -->
      <span :class="['icon-picker-arrow', { 'icon-picker-arrow-active': isOpen }]">
        <ChevronDown :size="16" />
      </span>
    </div>

    <!-- 下拉面板 -->
    <Teleport to="body">
      <Transition name="icon-picker-dropdown">
        <div v-if="isOpen" ref="dropdownRef" class="icon-picker-dropdown" :style="dropdownStyle">
          <!-- 搜索框 -->
          <div class="icon-picker-search">
            <Search :size="16" class="icon-picker-search-icon" />
            <input
              ref="searchInputRef"
              v-model="searchQuery"
              type="text"
              placeholder="搜索图标..."
              class="icon-picker-search-input"
            />
            <button v-if="searchQuery" class="icon-picker-search-clear" @click="searchQuery = ''">
              <X :size="14" />
            </button>
          </div>

          <!-- 分类标签 -->
          <div class="icon-picker-categories">
            <button
              v-for="category in categories"
              :key="category.key"
              :class="[
                'icon-picker-category',
                { 'icon-picker-category-active': activeCategory === category.key },
              ]"
              @click="activeCategory = category.key"
            >
              {{ category.label }}
            </button>
          </div>

          <!-- 图标网格 -->
          <div class="icon-picker-grid">
            <div
              v-for="iconName in displayedIcons"
              :key="iconName"
              :class="[
                'icon-picker-item',
                { 'icon-picker-item-selected': modelValue === iconName },
              ]"
              :title="iconName"
              @click="selectIcon(iconName)"
            >
              <component :is="iconComponents[iconName]" :size="20" />
            </div>

            <!-- 空状态 -->
            <div v-if="displayedIcons.length === 0" class="icon-picker-empty">未找到匹配的图标</div>
          </div>

          <!-- 已选信息 -->
          <div v-if="modelValue" class="icon-picker-footer">
            <span class="icon-picker-footer-text">已选: {{ modelValue }}</span>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick, onMounted, onUnmounted } from 'vue'
import { X, ChevronDown, Search } from 'lucide-vue-next'
import { iconComponents, iconCategories, getIconsByCategory } from './icons'

interface Props {
  modelValue?: string | null
  placeholder?: string
  clearable?: boolean
  disabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: null,
  placeholder: '选择图标',
  clearable: true,
  disabled: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: string | null]
  change: [value: string | null]
}>()

// 分类
const categories = iconCategories

const pickerRef = ref<HTMLElement>()
const dropdownRef = ref<HTMLElement>()
const searchInputRef = ref<HTMLInputElement>()
const isOpen = ref(false)
const searchQuery = ref('')
const activeCategory = ref('all')
const dropdownStyle = ref<Record<string, string>>({})

// 显示的图标
const displayedIcons = computed(() => {
  let icons = getIconsByCategory(activeCategory.value)

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    icons = Object.keys(iconComponents).filter((name) => name.toLowerCase().includes(query))
  }

  return icons
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
  updateDropdownPosition()

  nextTick(() => {
    searchInputRef.value?.focus()
  })
}

// 关闭下拉
const closeDropdown = () => {
  isOpen.value = false
  searchQuery.value = ''
}

// 更新下拉位置
const updateDropdownPosition = () => {
  if (!pickerRef.value) return

  const rect = pickerRef.value.getBoundingClientRect()
  const spaceBelow = window.innerHeight - rect.bottom
  const dropdownHeight = 400

  const showAbove = spaceBelow < dropdownHeight && rect.top > spaceBelow

  dropdownStyle.value = {
    position: 'fixed',
    left: `${rect.left}px`,
    width: '320px',
    zIndex: '9999',
    ...(showAbove
      ? { bottom: `${window.innerHeight - rect.top + 4}px` }
      : { top: `${rect.bottom + 4}px` }),
  }
}

// 选择图标
const selectIcon = (iconName: string) => {
  emit('update:modelValue', iconName)
  emit('change', iconName)
  closeDropdown()
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
.icon-picker-wrapper {
  position: relative;
  display: inline-flex;
  width: 100%;
}

/* 触发器 */
.icon-picker-trigger {
  display: flex;
  align-items: center;
  width: 100%;
  height: 36px;
  padding: 0 12px;
  background: white;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  gap: 8px;
}

.icon-picker-trigger:hover {
  border-color: #9ca3af;
}

.icon-picker-wrapper:has(.icon-picker-dropdown) .icon-picker-trigger {
  border-color: #22c55e;
  box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.1);
}

/* 预览 */
.icon-picker-preview {
  flex: 1;
  display: flex;
  align-items: center;
}

.icon-picker-selected {
  width: 20px;
  height: 20px;
  color: #374151;
}

.icon-picker-placeholder {
  color: #9ca3af;
  font-size: 14px;
}

/* 清除按钮 */
.icon-picker-clear {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: #9ca3af;
  cursor: pointer;
  opacity: 0;
  transition: all 0.2s;
}

.icon-picker-wrapper:hover .icon-picker-clear {
  opacity: 1;
}

.icon-picker-clear:hover {
  color: #6b7280;
}

/* 箭头 */
.icon-picker-arrow {
  display: inline-flex;
  align-items: center;
  color: #9ca3af;
  transition: transform 0.2s ease;
}

.icon-picker-arrow-active {
  transform: rotate(180deg);
}

/* 下拉面板 */
.icon-picker-dropdown {
  background: white;
  border-radius: 12px;
  box-shadow:
    0 10px 40px rgba(0, 0, 0, 0.15),
    0 0 0 1px rgba(0, 0, 0, 0.05);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  max-height: 400px;
}

/* 搜索框 */
.icon-picker-search {
  display: flex;
  align-items: center;
  padding: 12px;
  border-bottom: 1px solid #e5e7eb;
  gap: 8px;
}

.icon-picker-search-icon {
  color: #9ca3af;
  flex-shrink: 0;
}

.icon-picker-search-input {
  flex: 1;
  border: none;
  outline: none;
  font-size: 14px;
  color: #1f2937;
  background: transparent;
}

.icon-picker-search-input::placeholder {
  color: #9ca3af;
}

.icon-picker-search-clear {
  display: flex;
  align-items: center;
  justify-content: center;
  color: #9ca3af;
  background: none;
  border: none;
  cursor: pointer;
  padding: 2px;
  border-radius: 4px;
}

.icon-picker-search-clear:hover {
  background: #f3f4f6;
  color: #6b7280;
}

/* 分类标签 */
.icon-picker-categories {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  padding: 8px 12px;
  border-bottom: 1px solid #e5e7eb;
  background: #f9fafb;
}

.icon-picker-category {
  padding: 4px 10px;
  font-size: 12px;
  color: #6b7280;
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 16px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.icon-picker-category:hover {
  border-color: #d1d5db;
  color: #374151;
}

.icon-picker-category-active {
  background: #22c55e;
  border-color: #22c55e;
  color: white;
}

.icon-picker-category-active:hover {
  background: #16a34a;
  border-color: #16a34a;
  color: white;
}

/* 图标网格 */
.icon-picker-grid {
  display: grid;
  grid-template-columns: repeat(8, 1fr);
  gap: 4px;
  padding: 12px;
  overflow-y: auto;
  flex: 1;
}

.icon-picker-item {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 6px;
  cursor: pointer;
  color: #6b7280;
  transition: all 0.15s ease;
}

.icon-picker-item:hover {
  background: #f3f4f6;
  color: #1f2937;
}

.icon-picker-item-selected {
  background: #f0fdf4;
  color: #22c55e;
}

.icon-picker-item-selected:hover {
  background: #dcfce7;
}

/* 空状态 */
.icon-picker-empty {
  grid-column: 1 / -1;
  padding: 24px;
  text-align: center;
  color: #9ca3af;
  font-size: 14px;
}

/* 底部信息 */
.icon-picker-footer {
  padding: 8px 12px;
  border-top: 1px solid #e5e7eb;
  background: #f9fafb;
}

.icon-picker-footer-text {
  font-size: 12px;
  color: #6b7280;
}

/* 下拉动画 */
.icon-picker-dropdown-enter-active,
.icon-picker-dropdown-leave-active {
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.icon-picker-dropdown-enter-from,
.icon-picker-dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.95);
}

/* 滚动条 */
.icon-picker-grid::-webkit-scrollbar {
  width: 6px;
}

.icon-picker-grid::-webkit-scrollbar-track {
  background: transparent;
}

.icon-picker-grid::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 3px;
}

.icon-picker-grid::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}
</style>
