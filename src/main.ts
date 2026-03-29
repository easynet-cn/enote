import { createApp } from 'vue'
import { createPinia } from 'pinia'
import i18n from './i18n'
import './styles/main.css'

// ============================================================================
// Chunk 加载失败防白屏机制
// ============================================================================

const RELOAD_KEY = 'enote_chunk_reload'
const RELOAD_MAX = 2

/**
 * 检测是否为 chunk 加载失败错误（动态 import 失败、CSS 加载失败等）
 * 这是导致打包后白屏的主要原因
 */
function isChunkLoadError(error: unknown): boolean {
  if (error instanceof Error) {
    const msg = error.message.toLowerCase()
    return (
      msg.includes('failed to fetch dynamically imported module') ||
      msg.includes('loading chunk') ||
      msg.includes('loading css chunk') ||
      msg.includes('dynamically imported module') ||
      msg.includes('importing a module script failed') ||
      error.name === 'ChunkLoadError'
    )
  }
  return false
}

/**
 * 处理 chunk 加载失败：自动重载页面（最多重试 RELOAD_MAX 次）
 */
function handleChunkLoadError(): boolean {
  const reloadCount = Number(sessionStorage.getItem(RELOAD_KEY) || '0')
  if (reloadCount < RELOAD_MAX) {
    sessionStorage.setItem(RELOAD_KEY, String(reloadCount + 1))
    console.warn(
      `[ENote] Chunk load failed, reloading... (attempt ${reloadCount + 1}/${RELOAD_MAX})`,
    )
    window.location.reload()
    return true
  }
  console.error(`[ENote] Chunk load failed after ${RELOAD_MAX} retries, giving up`)
  return false
}

// 成功加载后清除重试计数
window.addEventListener('load', () => {
  sessionStorage.removeItem(RELOAD_KEY)
})

// ============================================================================
// 应用初始化
// ============================================================================

// 检查是否为帮助手册独立窗口
const params = new URLSearchParams(window.location.search)
const isHelpWindow = params.get('helpWindow') === 'true'

let RootComponent: typeof import('./App.vue').default
if (isHelpWindow) {
  RootComponent = (await import('./components/HelpWindow.vue')).default
} else {
  RootComponent = (await import('./App.vue')).default
}

const app = createApp(RootComponent)
const pinia = createPinia()

// 全局错误处理
app.config.errorHandler = (err, _instance, info) => {
  // Chunk 加载失败时自动重载
  if (isChunkLoadError(err)) {
    handleChunkLoadError()
    return
  }
  console.error(`[Vue Error] ${info}:`, err)
}

window.addEventListener('unhandledrejection', (event) => {
  // Chunk 加载失败时自动重载
  if (isChunkLoadError(event.reason)) {
    event.preventDefault()
    handleChunkLoadError()
    return
  }
  console.error('[Unhandled Promise Rejection]:', event.reason)
})

app.use(pinia)
app.use(i18n)
app.mount('#app')
