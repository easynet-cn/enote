import { locale } from '@tauri-apps/plugin-os'
import { createI18n } from 'vue-i18n'
import zhCN from './locales/zh-CN'
import enUS from './locales/en-US'

export type LocaleType = 'zh-CN' | 'en-US'

const messages = {
  'zh-CN': zhCN,
  'en-US': enUS,
}

const supportedLocales: LocaleType[] = ['zh-CN', 'en-US']

function matchLocale(lang: string): LocaleType {
  // 精确匹配
  if (supportedLocales.includes(lang as LocaleType)) {
    return lang as LocaleType
  }
  // 前缀匹配（如 zh → zh-CN, en → en-US）
  const prefix = lang.split('-')[0]
  const matched = supportedLocales.find((l) => l.startsWith(prefix))
  if (matched) {
    return matched
  }
  // 不支持的语言，默认英文
  return 'en-US'
}

// 同步检测：先用 localStorage 或浏览器语言初始化，确保页面立即可用
function detectDefaultLocaleSync(): LocaleType {
  return matchLocale(navigator.language)
}

const savedLocale = (localStorage.getItem('app-locale') as LocaleType) || detectDefaultLocaleSync()

const i18n = createI18n({
  legacy: false,
  locale: savedLocale,
  fallbackLocale: 'en-US',
  messages,
})

// 异步获取系统语言，如果与当前不同则更新（仅当用户未手动设置过语言时）
locale().then((osLang) => {
  if (!osLang) return
  const manualSet = localStorage.getItem('app-locale')
  if (!manualSet) {
    i18n.global.locale.value = matchLocale(osLang)
  }
})

export const availableLocales: { value: LocaleType; label: string }[] = [
  { value: 'zh-CN', label: '简体中文' },
  { value: 'en-US', label: 'English' },
]

// 在 Composition API 模式下，locale 是一个 ref
export const setLocale = (locale: LocaleType) => {
  i18n.global.locale.value = locale
  localStorage.setItem('app-locale', locale)
}

export const getCurrentLocale = (): LocaleType => {
  return i18n.global.locale.value as LocaleType
}

// 导出 i18n 实例，用于在组件中使用 useI18n() 获取响应式的 locale
export default i18n
