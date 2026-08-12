<script setup lang="ts">
import { onMounted, onUnmounted, computed } from 'vue'
import { useScreenSaver } from '../composables/useScreenSaver'
import { useI18n } from 'vue-i18n'
import { convertFileSrc } from '@tauri-apps/api/core'

const { t } = useI18n()
const {
  isScreenSaverActive,
  bgColor,
  bgImage,
  text,
  textColor,
  fontSize,
  durationDisplay,
  duration,
  deactivate,
} = useScreenSaver()

const bgStyle = computed(() => {
  const style: Record<string, string> = { backgroundColor: bgColor.value }
  if (bgImage.value) {
    style.backgroundImage = `url(${convertFileSrc(bgImage.value)})`
    style.backgroundSize = 'cover'
    style.backgroundPosition = 'center'
    style.backgroundRepeat = 'no-repeat'
  }
  return style
})

const showCountdown = computed(() => duration.value > 0)

const handleKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape' && isScreenSaverActive.value) {
    deactivate()
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <Teleport to="body">
    <Transition name="screensaver-fade">
      <div
        v-if="isScreenSaverActive"
        class="screensaver-overlay"
        :style="bgStyle"
        @click="deactivate"
      >
        <!-- 背景遮罩（有背景图时叠加半透明层确保文字可读） -->
        <div v-if="bgImage" class="screensaver-dim" />

        <!-- 居中文字 -->
        <div
          v-if="text"
          class="screensaver-text"
          :style="{ color: textColor, fontSize: `${fontSize}px` }"
        >
          {{ text }}
        </div>

        <!-- 倒计时 -->
        <div v-if="showCountdown" class="screensaver-countdown" :style="{ color: textColor }">
          {{ t('screenSaver.remaining') }}: {{ durationDisplay }}
        </div>

        <!-- 退出提示 -->
        <div class="screensaver-tip" :style="{ color: textColor }">
          {{ t('screenSaver.tipExit') }}
        </div>

        <!-- 退出按钮 -->
        <button class="screensaver-exit-btn" @click.stop="deactivate">
          {{ t('screenSaver.exit') }}
        </button>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.screensaver-overlay {
  position: fixed;
  inset: 0;
  z-index: 99998;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  user-select: none;
}

.screensaver-dim {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
}

.screensaver-text {
  position: relative;
  z-index: 1;
  text-align: center;
  font-weight: 500;
  letter-spacing: 2px;
  padding: 0 40px;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  animation: ss-breathe 4s ease-in-out infinite;
}

.screensaver-countdown {
  position: relative;
  z-index: 1;
  margin-top: 24px;
  font-size: 14px;
  opacity: 0.7;
  letter-spacing: 1px;
}

.screensaver-tip {
  position: relative;
  z-index: 1;
  margin-top: 16px;
  font-size: 12px;
  opacity: 0.5;
}

.screensaver-exit-btn {
  position: fixed;
  bottom: 32px;
  right: 32px;
  z-index: 2;
  padding: 8px 20px;
  font-size: 13px;
  color: #fff;
  background: rgba(255, 255, 255, 0.15);
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: 8px;
  cursor: pointer;
  backdrop-filter: blur(8px);
  transition: background 0.2s;
}

.screensaver-exit-btn:hover {
  background: rgba(255, 255, 255, 0.25);
}

@keyframes ss-breathe {
  0%,
  100% {
    opacity: 0.85;
  }
  50% {
    opacity: 1;
  }
}

@media (prefers-reduced-motion: reduce) {
  .screensaver-text {
    animation: none;
  }
}

.screensaver-fade-enter-active,
.screensaver-fade-leave-active {
  transition: opacity 0.5s ease;
}

.screensaver-fade-enter-from,
.screensaver-fade-leave-to {
  opacity: 0;
}
</style>
