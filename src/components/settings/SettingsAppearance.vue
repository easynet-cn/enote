<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { setLocale, type LocaleType } from '../../i18n'
import { useAppStore } from '../../stores/app'
import { usePlatform, type LayoutMode } from '../../composables/usePlatform'
import { AppSelect } from '../ui'
import type { AppSelectOption } from '../ui'

const { t, locale } = useI18n()
const appStore = useAppStore()
const { setLayoutOverride } = usePlatform()

const currentTheme = defineModel<string>('theme', { required: true })
const currentLayoutMode = defineModel<LayoutMode | 'auto'>('layoutMode', { required: true })
const currentEditorFontSize = defineModel<string>('editorFontSize', { required: true })

const emit = defineEmits<{
  (e: 'save'): void
}>()

const currentLocale = computed(() => locale.value as LocaleType)

const themeOptions = computed(() => [
  { value: 'light', label: t('settings.themeLight') },
  { value: 'dark', label: t('settings.themeDark') },
  { value: 'system', label: t('settings.themeSystem') },
])

const editorFontSizeOptions: AppSelectOption[] = [
  { value: '12', label: '12px' },
  { value: '14', label: '14px' },
  { value: '16', label: '16px' },
  { value: '18', label: '18px' },
  { value: '20', label: '20px' },
]

const applyEditorFontSize = (size: string) => {
  document.documentElement.style.setProperty('--editor-font-size', `${size}px`)
}

const setEditorFontSize = (size: string) => {
  currentEditorFontSize.value = size
  applyEditorFontSize(size)
  emit('save')
}

const layoutModeOptions = computed(() => [
  { value: 'auto' as const, label: t('settings.layoutAuto') },
  { value: 'desktop' as LayoutMode | 'auto', label: t('settings.layoutDesktop') },
  { value: 'tablet' as LayoutMode | 'auto', label: t('settings.layoutTablet') },
  { value: 'mobile' as LayoutMode | 'auto', label: t('settings.layoutMobile') },
])

const applyTheme = (theme: string) => {
  const root = document.documentElement
  if (theme === 'system') {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    root.setAttribute('data-theme', prefersDark ? 'dark' : 'light')
  } else {
    root.setAttribute('data-theme', theme)
  }
}

const setTheme = (theme: string) => {
  currentTheme.value = theme
  applyTheme(theme)
  emit('save')
}

const switchLocale = (newLocale: LocaleType) => {
  setLocale(newLocale)
  appStore.updateDefaultItems()
  emit('save')
}

const setLayoutMode = (mode: LayoutMode | 'auto') => {
  currentLayoutMode.value = mode
  setLayoutOverride(mode)
  emit('save')
}

// 监听系统主题变化
const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
mediaQuery.addEventListener('change', () => {
  if (currentTheme.value === 'system') {
    applyTheme('system')
  }
})

defineExpose({ applyTheme, applyEditorFontSize })
</script>

<template>
  <div>
    <h3 class="text-sm font-semibold text-content-secondary mb-3">
      {{ t('settings.appearance') }}
    </h3>
    <div class="space-y-4">
      <!-- 主题 -->
      <div class="flex items-center justify-between">
        <label class="text-sm text-content-secondary">{{ t('settings.theme') }}</label>
        <div class="flex gap-1 bg-surface-dim rounded-lg p-1">
          <button
            v-for="option in themeOptions"
            :key="option.value"
            @click="setTheme(option.value)"
            class="px-3 py-1.5 text-sm rounded-md transition-colors"
            :class="
              currentTheme === option.value
                ? 'bg-surface text-indigo-600 shadow-sm'
                : 'text-content-secondary hover:text-content'
            "
          >
            {{ option.label }}
          </button>
        </div>
      </div>

      <!-- 语言 -->
      <div class="flex items-center justify-between">
        <label class="text-sm text-content-secondary">{{ t('settings.language') }}</label>
        <div class="flex gap-1 bg-surface-dim rounded-lg p-1">
          <button
            @click="switchLocale('zh-CN')"
            class="px-3 py-1.5 text-sm rounded-md transition-colors"
            :class="
              currentLocale === 'zh-CN'
                ? 'bg-surface text-indigo-600 shadow-sm'
                : 'text-content-secondary hover:text-content'
            "
          >
            {{ t('settings.languageZh') }}
          </button>
          <button
            @click="switchLocale('en-US')"
            class="px-3 py-1.5 text-sm rounded-md transition-colors"
            :class="
              currentLocale === 'en-US'
                ? 'bg-surface text-indigo-600 shadow-sm'
                : 'text-content-secondary hover:text-content'
            "
          >
            {{ t('settings.languageEn') }}
          </button>
        </div>
      </div>

      <!-- 布局模式 -->
      <div class="flex items-center justify-between">
        <label class="text-sm text-content-secondary">{{ t('settings.layoutMode') }}</label>
        <div class="flex gap-1 bg-surface-dim rounded-lg p-1">
          <button
            v-for="option in layoutModeOptions"
            :key="option.value"
            @click="setLayoutMode(option.value)"
            class="px-3 py-1.5 text-sm rounded-md transition-colors"
            :class="
              currentLayoutMode === option.value
                ? 'bg-surface text-indigo-600 shadow-sm'
                : 'text-content-secondary hover:text-content'
            "
          >
            {{ option.label }}
          </button>
        </div>
      </div>

      <!-- 编辑器字体大小 -->
      <div class="flex items-center justify-between">
        <label class="text-sm text-content-secondary">{{ t('settings.editorFontSize') }}</label>
        <AppSelect
          :model-value="currentEditorFontSize"
          :options="editorFontSizeOptions"
          size="sm"
          @change="(val: string | number) => setEditorFontSize(String(val))"
        />
      </div>
    </div>
  </div>
</template>
