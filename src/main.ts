import { createApp } from 'vue'
import { createPinia } from 'pinia'
import i18n from './i18n'
import './styles/main.css'
import App from './App.vue'

const app = createApp(App)
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
