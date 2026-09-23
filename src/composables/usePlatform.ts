import { computed, ref } from 'vue'

// Vite 编译时注入的平台标记（由 TAURI_ENV_PLATFORM 决定）
// 桌面端构建: false，Android/iOS 构建: true
declare const __IS_MOBILE__: boolean

const tauriMobile = __IS_MOBILE__

// 运行时窗口尺寸响应式检测（单例共享）
const windowWidth = ref(window.innerWidth)
const windowHeight = ref(window.innerHeight)

/**
 * 将实测视口高度同步到 CSS 变量 `--app-height`。
 * Tauri/WebView 在窗口最大化时 `100vh` 可能滞后于实际高度，
 * 导致主界面底部留白，因此用 JS 实测值兜底。
 */
const syncAppHeight = () => {
  document.documentElement.style.setProperty('--app-height', `${window.innerHeight}px`)
}

let resizeTimer: ReturnType<typeof setTimeout> | null = null
const handleResize = () => {
  if (resizeTimer) clearTimeout(resizeTimer)
  resizeTimer = setTimeout(() => {
    windowWidth.value = window.innerWidth
    windowHeight.value = window.innerHeight
    syncAppHeight()
  }, 100)
}
window.addEventListener('resize', handleResize)
// 初始化时立即同步，避免首屏或最大化后未触发 resize 时高度为 0
syncAppHeight()

export type LayoutMode = 'mobile' | 'tablet' | 'desktop'

// 用户手动覆盖的布局模式（'auto' 表示自动检测）
const layoutOverride = ref<LayoutMode | 'auto'>('auto')

export function usePlatform() {
  // Tauri 平台特性判断（系统托盘、窗口管理等）
  const isTauriMobile = computed(() => tauriMobile)
  const isAndroid = computed(() => tauriMobile && /Android/i.test(navigator.userAgent))
  const isIOS = computed(() => tauriMobile && /iPhone|iPad|iPod/i.test(navigator.userAgent))

  // 自动检测的布局模式
  const autoLayout = computed<LayoutMode>(() => {
    if (tauriMobile) return 'mobile'
    if (windowWidth.value < 640) return 'mobile'
    if (windowWidth.value < 1024) return 'tablet'
    return 'desktop'
  })

  // 最终布局模式（手动覆盖 > 自动检测）
  const layout = computed<LayoutMode>(() => {
    if (layoutOverride.value !== 'auto') return layoutOverride.value
    return autoLayout.value
  })

  const isMobileLayout = computed(() => layout.value === 'mobile')
  const isTabletLayout = computed(() => layout.value === 'tablet')
  const isDesktopLayout = computed(() => layout.value === 'desktop')

  // 保留向后兼容
  const isMobile = computed(() => layout.value === 'mobile')
  const isDesktop = computed(() => layout.value === 'desktop')

  // 设置布局模式覆盖
  const setLayoutOverride = (mode: LayoutMode | 'auto') => {
    layoutOverride.value = mode
  }

  return {
    isMobile,
    isDesktop,
    isTauriMobile,
    isAndroid,
    isIOS,
    layout,
    autoLayout,
    layoutOverride,
    isMobileLayout,
    isTabletLayout,
    isDesktopLayout,
    windowWidth,
    windowHeight,
    setLayoutOverride,
  }
}
