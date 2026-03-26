import { createApp } from 'vue'
import { createPinia } from 'pinia'
import i18n from './i18n'
import './styles/main.css'

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
  console.error(`[Vue Error] ${info}:`, err)
}

window.addEventListener('unhandledrejection', (event) => {
  console.error('[Unhandled Promise Rejection]:', event.reason)
})

app.use(pinia)
app.use(i18n)
app.mount('#app')
