import { ref, onUnmounted } from 'vue'
import { settingsApi } from '../api/note'
import { throttle } from '../utils/debounce'

const isLocked = ref(false)
const lockMode = ref<'none' | 'password' | 'biometric'>('none')
const lockTimeout = ref(0) // minutes, 0 = only on startup
const lockOnMinimize = ref(false)

let activityTimer: ReturnType<typeof setTimeout> | null = null
let activityListenersAttached = false

const resetActivityTimer = () => {
  if (activityTimer) {
    clearTimeout(activityTimer)
    activityTimer = null
  }
  if (lockTimeout.value > 0 && lockMode.value !== 'none' && !isLocked.value) {
    activityTimer = setTimeout(
      () => {
        lock()
      },
      lockTimeout.value * 60 * 1000,
    )
  }
}

const onActivity = throttle(() => {
  resetActivityTimer()
}, 1000)

const startActivityMonitoring = () => {
  if (activityListenersAttached) return
  activityListenersAttached = true
  window.addEventListener('mousemove', onActivity)
  window.addEventListener('keydown', onActivity)
  window.addEventListener('click', onActivity)
  window.addEventListener('scroll', onActivity)
  resetActivityTimer()
}

const stopActivityMonitoring = () => {
  if (!activityListenersAttached) return
  activityListenersAttached = false
  window.removeEventListener('mousemove', onActivity)
  window.removeEventListener('keydown', onActivity)
  window.removeEventListener('click', onActivity)
  window.removeEventListener('scroll', onActivity)
  if (activityTimer) {
    clearTimeout(activityTimer)
    activityTimer = null
  }
}

const lock = () => {
  if (lockMode.value !== 'none') {
    isLocked.value = true
    stopActivityMonitoring()
  }
}

const unlock = () => {
  isLocked.value = false
  startActivityMonitoring()
}

const loadLockSettings = async () => {
  try {
    const settings = await settingsApi.getAll()
    lockMode.value = (settings.lockMode as 'none' | 'password' | 'biometric') || 'none'
    lockTimeout.value = parseInt(settings.lockTimeout || '0')
    lockOnMinimize.value = settings.lockOnMinimize === '1'
  } catch {
    // Use defaults
  }
}

const checkStartupLock = async () => {
  await loadLockSettings()
  if (lockMode.value !== 'none') {
    isLocked.value = true
  } else {
    startActivityMonitoring()
  }
}

export function useLockScreen() {
  // Window visibility change for lockOnMinimize
  const handleVisibilityChange = () => {
    if (
      document.visibilityState === 'visible' &&
      lockOnMinimize.value &&
      lockMode.value !== 'none'
    ) {
      lock()
    }
  }

  const setupMinimizeListener = () => {
    document.addEventListener('visibilitychange', handleVisibilityChange)
  }

  onUnmounted(() => {
    document.removeEventListener('visibilitychange', handleVisibilityChange)
    stopActivityMonitoring()
  })

  return {
    isLocked,
    lockMode,
    lockTimeout,
    lockOnMinimize,
    lock,
    unlock,
    checkStartupLock,
    loadLockSettings,
    resetActivityTimer,
    startActivityMonitoring,
    setupMinimizeListener,
  }
}
