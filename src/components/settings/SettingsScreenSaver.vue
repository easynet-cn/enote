<script setup lang="ts">
import { ref, computed, watchEffect } from 'vue'
import { useI18n } from 'vue-i18n'
import { open } from '@tauri-apps/plugin-dialog'
import { readFile } from '@tauri-apps/plugin-fs'
import { imageApi } from '../../api/note'
import { useScreenSaver } from '../../composables/useScreenSaver'
import { showNotification } from '../ui/notification'
import { AppSelect } from '../ui'
import type { AppSelectOption } from '../ui'

const { t } = useI18n()
const { setEnabled, updateTimerSettings } = useScreenSaver()

// v-model 绑定
const enabledModel = defineModel<boolean>('enabled', { required: true })
const idleTimeoutModel = defineModel<string>('idleTimeout', { required: true })
const durationModel = defineModel<string>('duration', { required: true })

// 本地同步缓存：defineModel 在 @change 中读取时可能因 prop 传播延迟返回旧值，
// 用 watchEffect 同步追踪，确保 handler 中总能读到最新值
const currentIdleTimeout = ref(idleTimeoutModel.value)
const currentDuration = ref(durationModel.value)
watchEffect(() => {
  currentIdleTimeout.value = idleTimeoutModel.value
  currentDuration.value = durationModel.value
})
const bgColorModel = defineModel<string>('bgColor', { required: true })
const bgImageModel = defineModel<string>('bgImage', { required: true })
const textModel = defineModel<string>('text', { required: true })
const textColorModel = defineModel<string>('textColor', { required: true })
const fontSizeModel = defineModel<string>('fontSize', { required: true })

const emit = defineEmits<{
  (e: 'save'): void
}>()

const selectingImage = ref(false)

const idleTimeoutOptions = computed<AppSelectOption[]>(() => [
  { value: '30', label: t('settings.screenSaverMinutes', { n: 30 }) },
  { value: '45', label: t('settings.screenSaverMinutes', { n: 45 }) },
  { value: '60', label: t('settings.screenSaverMinutes', { n: 60 }) },
  { value: '90', label: t('settings.screenSaverMinutes', { n: 90 }) },
  { value: '120', label: t('settings.screenSaverMinutes', { n: 120 }) },
])

const durationOptions = computed<AppSelectOption[]>(() => [
  { value: '3', label: t('settings.screenSaverMinutes', { n: 3 }) },
  { value: '5', label: t('settings.screenSaverMinutes', { n: 5 }) },
  { value: '10', label: t('settings.screenSaverMinutes', { n: 10 }) },
  { value: '15', label: t('settings.screenSaverMinutes', { n: 15 }) },
  { value: '0', label: t('settings.screenSaverDurationUnlimited') },
])

const toggleEnabled = async () => {
  enabledModel.value = !enabledModel.value
  await setEnabled(enabledModel.value)
  emit('save')
}

// 空闲触发时间变更：事件值用于本字段，本地缓存读另一字段，避免 defineModel 传播延迟
const handleIdleTimeoutChange = async (value: string | number) => {
  const minutes = parseInt(String(value))
  currentIdleTimeout.value = String(value) // 立即同步，供 handleDurationChange 读取
  const durationMinutes = parseInt(currentDuration.value)
  await updateTimerSettings(minutes, durationMinutes)
  emit('save')
}

// 持续时长变更：事件值用于本字段，本地缓存读另一字段，避免 defineModel 传播延迟
const handleDurationChange = async (value: string | number) => {
  const minutes = parseInt(currentIdleTimeout.value)
  const durationMinutes = parseInt(String(value))
  currentDuration.value = String(value) // 立即同步，供 handleIdleTimeoutChange 读取
  await updateTimerSettings(minutes, durationMinutes)
  emit('save')
}

const handleSelectImage = async () => {
  if (selectingImage.value) return
  selectingImage.value = true
  try {
    const selected = await open({
      filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif', 'bmp'] }],
      multiple: false,
    })
    if (!selected) return

    // 读取文件并保存到应用数据目录
    const fileBytes = await readFile(selected)
    const base64 = btoa(String.fromCharCode(...fileBytes))
    const savedPath = await imageApi.saveImage(base64)
    bgImageModel.value = savedPath
    emit('save')
    showNotification({ type: 'success', message: t('settings.screenSaverImageSet') })
  } catch {
    showNotification({ type: 'error', message: t('settings.screenSaverImageFailed') })
  } finally {
    selectingImage.value = false
  }
}

const handleClearImage = () => {
  bgImageModel.value = ''
  emit('save')
}
</script>

<template>
  <div>
    <h3 class="text-sm font-semibold text-content-secondary mb-3">
      {{ t('settings.screenSaver') }}
    </h3>
    <div class="space-y-4">
      <!-- 启用开关 -->
      <div class="flex items-center justify-between">
        <div>
          <label class="text-sm text-content-secondary">{{
            t('settings.screenSaverEnabled')
          }}</label>
          <p class="text-xs text-content-tertiary mt-0.5">
            {{ t('settings.screenSaverEnabledDesc') }}
          </p>
        </div>
        <button
          @click="toggleEnabled"
          class="relative w-10 h-5 rounded-full transition-colors shrink-0"
          :class="enabledModel ? 'bg-indigo-600' : 'bg-slate-300'"
        >
          <span
            class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full transition-transform shadow-sm"
            :class="enabledModel ? 'translate-x-5' : ''"
          />
        </button>
      </div>

      <template v-if="enabledModel">
        <!-- 空闲触发时间 -->
        <div class="flex items-center justify-between">
          <label class="text-sm text-content-secondary">{{
            t('settings.screenSaverIdleTime')
          }}</label>
          <AppSelect
            v-model="idleTimeoutModel"
            :options="idleTimeoutOptions"
            size="sm"
            @change="handleIdleTimeoutChange"
          />
        </div>

        <!-- 持续时长 -->
        <div class="flex items-center justify-between">
          <label class="text-sm text-content-secondary">{{
            t('settings.screenSaverDuration')
          }}</label>
          <AppSelect
            v-model="durationModel"
            :options="durationOptions"
            size="sm"
            @change="handleDurationChange"
          />
        </div>

        <!-- 底色 -->
        <div class="flex items-center justify-between">
          <label class="text-sm text-content-secondary">{{
            t('settings.screenSaverBgColor')
          }}</label>
          <input
            v-model="bgColorModel"
            type="color"
            class="w-10 h-8 rounded-lg border border-edge cursor-pointer bg-surface"
            @change="emit('save')"
          />
        </div>

        <!-- 背景图片 -->
        <div class="flex items-center justify-between">
          <label class="text-sm text-content-secondary">{{
            t('settings.screenSaverBgImage')
          }}</label>
          <div class="flex gap-2">
            <button
              @click="handleSelectImage"
              :disabled="selectingImage"
              class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50"
            >
              {{ selectingImage ? t('common.loading') : t('settings.screenSaverSelectImage') }}
            </button>
            <button
              v-if="bgImageModel"
              @click="handleClearImage"
              class="px-3 py-1.5 text-sm text-red-600 hover:bg-red-50 rounded-lg transition-colors"
            >
              {{ t('settings.screenSaverClearImage') }}
            </button>
          </div>
        </div>

        <!-- 显示文字 -->
        <div>
          <label class="block text-xs text-content-tertiary mb-1">{{
            t('settings.screenSaverText')
          }}</label>
          <input
            v-model="textModel"
            type="text"
            class="w-full px-3 py-2 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
            @change="emit('save')"
          />
        </div>

        <!-- 文字颜色 -->
        <div class="flex items-center justify-between">
          <label class="text-sm text-content-secondary">{{
            t('settings.screenSaverTextColor')
          }}</label>
          <input
            v-model="textColorModel"
            type="color"
            class="w-10 h-8 rounded-lg border border-edge cursor-pointer bg-surface"
            @change="emit('save')"
          />
        </div>

        <!-- 文字字号 -->
        <div class="flex items-center justify-between">
          <label class="text-sm text-content-secondary">{{
            t('settings.screenSaverFontSize')
          }}</label>
          <div class="flex items-center gap-2">
            <input
              v-model="fontSizeModel"
              type="range"
              min="24"
              max="96"
              step="4"
              class="w-32 accent-indigo-600"
              @change="emit('save')"
            />
            <span class="text-xs text-content-tertiary w-10 text-right">{{ fontSizeModel }}px</span>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>
