import { createI18n } from 'vue-i18n'
import zhCN from './locales/zh-CN'
import enUS from './locales/en-US'

export type LocaleType = 'zh-CN' | 'en-US'

const messages = {
  'zh-CN': zhCN,
  'en-US': enUS,
}

// 默认语言设置为中文
const savedLocale = (localStorage.getItem('app-locale') as LocaleType) || 'zh-CN'

const i18n = createI18n({
  legacy: false,
  locale: savedLocale,
  fallbackLocale: 'zh-CN',
  messages,
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
