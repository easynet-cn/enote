import { computed } from 'vue'

// Vite 编译时注入的平台标记（由 TAURI_ENV_PLATFORM 决定）
// 桌面端构建: false，Android/iOS 构建: true
declare const __IS_MOBILE__: boolean

const mobile = __IS_MOBILE__

export function usePlatform() {
  const isMobile = computed(() => mobile)
  const isDesktop = computed(() => !mobile)
  const isAndroid = computed(() => mobile && /Android/i.test(navigator.userAgent))
  const isIOS = computed(() => mobile && /iPhone|iPad|iPod/i.test(navigator.userAgent))

  return { isMobile, isDesktop, isAndroid, isIOS }
}
