import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { settingsApi } from '../api/note'
import i18n from '../i18n'

// 专业默认值（基于护眼人体工学建议）
// 内部全部使用秒为单位
// 提示文字默认值根据语言环境动态获取
function getDefaultText(): string {
  return i18n.global.t('screenSaver.defaultText')
}

const DEFAULTS = {
  enabled: true, // 默认开启护眼屏保
  idleTimeout: 3600, // 3600 秒 = 60 分钟空闲触发
  duration: 300, // 300 秒 = 5 分钟屏保持续
  bgColor: '#1a1a2e', // 深蓝灰底色（低蓝光、舒缓）
  bgImage: '', // 默认无背景图
  textColor: '#e0e0e0', // 柔和暖白文字
  fontSize: 48, // 大字号，远距离可读
}

// 检测是否运行在屏保独立窗口中
// 屏保窗口的 URL 为 index.html?mode=screensaver
const isStandaloneWindow = new URLSearchParams(window.location.search).get('mode') === 'screensaver'

// 模块级单例状态
// 屏保窗口中默认 active（窗口仅在屏保激活时显示）
const isScreenSaverActive = ref(isStandaloneWindow)
const enabled = ref(DEFAULTS.enabled)
const idleTimeout = ref(DEFAULTS.idleTimeout) // 秒
const duration = ref(DEFAULTS.duration) // 秒
const bgColor = ref(DEFAULTS.bgColor)
const bgImage = ref(DEFAULTS.bgImage)
const text = ref(getDefaultText())
const textColor = ref(DEFAULTS.textColor)
const fontSize = ref(DEFAULTS.fontSize)

// Rust 后端推送的计时器状态
const timerState = ref<'running' | 'paused' | 'screensaver' | 'disabled'>('disabled')
const idleRemaining = ref(0)
const durationRemaining = ref(0)

// 防止 deactivate() 重复调用
let isDeactivating = false

const unlistenFns: UnlistenFn[] = []
let listenersInitialized = false

/** 格式化秒为 mm:ss */
function formatTime(seconds: number): string {
  const m = Math.floor(seconds / 60)
  const s = seconds % 60
  return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`
}

const idleDisplay = computed(() => formatTime(idleRemaining.value))
const durationDisplay = computed(() => formatTime(durationRemaining.value))

/** 初始化事件监听（只执行一次） */
async function initListeners() {
  if (listenersInitialized) return
  listenersInitialized = true

  unlistenFns.push(
    await listen('ss-activate', () => {
      isScreenSaverActive.value = true
    }),
  )

  unlistenFns.push(
    await listen('ss-deactivate', () => {
      isScreenSaverActive.value = false
    }),
  )

  unlistenFns.push(
    await listen('ss-tick', (event) => {
      const payload = event.payload as {
        timerState: string
        idleRemaining: number
        idleTimeout: number
        duration: number
        durationRemaining: number
      }
      timerState.value = payload.timerState as typeof timerState.value
      idleRemaining.value = payload.idleRemaining
      durationRemaining.value = payload.durationRemaining
    }),
  )

  unlistenFns.push(
    await listen('ss-disabled', () => {
      enabled.value = false
      isScreenSaverActive.value = false
      timerState.value = 'disabled'
    }),
  )
}

/** 从 DB 加载配置（DB 中存储的是分钟，需转换为秒） */
async function loadSettings() {
  try {
    const settings = await settingsApi.getAll()
    enabled.value = settings.screenSaverEnabled !== '0' // 默认开启，仅 '0' 表示禁用
    // DB 存储分钟，转换为秒
    const idleMinutes = parseInt(settings.screenSaverIdleTimeout || '60')
    const durationMinutes = parseInt(settings.screenSaverDuration || '5')
    idleTimeout.value = idleMinutes * 60
    duration.value = durationMinutes * 60
    bgColor.value = settings.screenSaverBgColor || DEFAULTS.bgColor
    bgImage.value = settings.screenSaverBgImage || DEFAULTS.bgImage
    text.value = settings.screenSaverText ?? getDefaultText()
    textColor.value = settings.screenSaverTextColor || DEFAULTS.textColor
    fontSize.value = parseInt(settings.screenSaverFontSize || String(DEFAULTS.fontSize))
  } catch {
    // 使用默认值
  }
}

/** 应用启动时初始化 */
async function checkStartup() {
  await initListeners()
  await loadSettings()
  // 屏保独立窗口：查询后端当前状态，立即同步倒计时
  if (isStandaloneWindow) {
    try {
      const state = await invoke<{
        timerState: string
        idleRemaining: number
        idleTimeout: number
        duration: number
        durationRemaining: number
      }>('ss_get_state')
      timerState.value = state.timerState as typeof timerState.value
      idleRemaining.value = state.idleRemaining
      durationRemaining.value = state.durationRemaining
    } catch {
      // 查询失败时保持默认值，ss-tick 事件会随后更新
    }
    return
  }
  if (enabled.value) {
    await invoke('ss_start', {
      idleTimeout: idleTimeout.value,
      duration: duration.value,
    })
  }
}

/** 启用/禁用屏保 */
async function setEnabled(value: boolean) {
  enabled.value = value
  if (value) {
    await invoke('ss_start', {
      idleTimeout: idleTimeout.value,
      duration: duration.value,
    })
  } else {
    await invoke('ss_stop')
  }
}

/** 更新计时器设置（参数为分钟，内部转秒） */
async function updateTimerSettings(idleTimeoutMinutes: number, durationMinutes: number) {
  idleTimeout.value = idleTimeoutMinutes * 60
  duration.value = durationMinutes * 60
  await invoke('ss_update_settings', {
    idleTimeout: idleTimeout.value,
    duration: duration.value,
  })
}

/** 暂停空闲倒计时 */
async function pause() {
  await invoke('ss_pause')
}

/** 继续空闲倒计时 */
async function resume() {
  await invoke('ss_resume')
}

/** 重置空闲倒计时 */
async function reset() {
  await invoke('ss_reset')
}

/** 退出屏保，重新开始空闲倒计时（防重入） */
async function deactivate() {
  if (isDeactivating) return
  isDeactivating = true
  try {
    await invoke('ss_exit')
  } finally {
    isDeactivating = false
  }
}

/** 重启屏保（设置变更后调用） */
async function restart() {
  if (enabled.value) {
    await invoke('ss_start', {
      idleTimeout: idleTimeout.value,
      duration: duration.value,
    })
  } else {
    await invoke('ss_stop')
  }
}

export function useScreenSaver() {
  return {
    // 状态
    isScreenSaverActive,
    isStandaloneWindow,
    enabled,
    idleTimeout,
    duration,
    bgColor,
    bgImage,
    text,
    textColor,
    fontSize,
    // Rust 推送的倒计时
    timerState,
    idleRemaining,
    durationRemaining,
    // 计算属性
    idleDisplay,
    durationDisplay,
    // 方法
    checkStartup,
    loadSettings,
    setEnabled,
    updateTimerSettings,
    pause,
    resume,
    reset,
    deactivate,
    restart,
  }
}
