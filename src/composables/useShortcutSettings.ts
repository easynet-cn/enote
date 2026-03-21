import { ref, computed } from 'vue'
import {
  SHORTCUT_DEFS,
  bindingsEqual,
  bindingToText,
  type KeyBinding,
  type ShortcutDef,
} from '../config/shortcuts'
import { settingsApi } from '../api/note'
import type { ShortcutHandler } from './useKeyboardShortcuts'

const SETTINGS_KEY = 'shortcuts'

// 单例共享的用户自定义绑定（只存储与默认不同的项）
const customBindings = ref<Record<string, KeyBinding>>({})
const loaded = ref(false)

export function useShortcutSettings() {
  /** 获取某个快捷键的最终绑定（用户覆盖 > 默认值） */
  const getBinding = (id: string): KeyBinding => {
    if (customBindings.value[id]) return customBindings.value[id]
    const def = SHORTCUT_DEFS.find((d) => d.id === id)
    return def ? { ...def.defaultBinding } : { key: '' }
  }

  /** 获取某个快捷键的显示文本 */
  const getBindingText = (id: string): string => {
    return bindingToText(getBinding(id))
  }

  /** 所有快捷键的最终绑定（响应式） */
  const allBindings = computed(() => {
    const result: Record<string, KeyBinding> = {}
    for (const def of SHORTCUT_DEFS) {
      result[def.id] = customBindings.value[def.id] ?? { ...def.defaultBinding }
    }
    return result
  })

  /** 检测冲突：返回冲突的快捷键定义，无冲突返回 null */
  const checkConflict = (id: string, binding: KeyBinding): ShortcutDef | null => {
    for (const def of SHORTCUT_DEFS) {
      if (def.id === id) continue
      const existing = customBindings.value[def.id] ?? def.defaultBinding
      if (bindingsEqual(existing, binding)) {
        return def
      }
    }
    return null
  }

  /** 修改快捷键 */
  const setBinding = (id: string, binding: KeyBinding) => {
    const def = SHORTCUT_DEFS.find((d) => d.id === id)
    if (!def || def.system) return

    // 如果与默认值相同，删除自定义（等于重置）
    if (bindingsEqual(binding, def.defaultBinding)) {
      const updated = { ...customBindings.value }
      delete updated[id]
      customBindings.value = updated
    } else {
      customBindings.value = { ...customBindings.value, [id]: binding }
    }
  }

  /** 重置单个快捷键 */
  const resetBinding = (id: string) => {
    const updated = { ...customBindings.value }
    delete updated[id]
    customBindings.value = updated
  }

  /** 重置所有快捷键 */
  const resetAll = () => {
    customBindings.value = {}
  }

  /** 判断某个快捷键是否被修改过 */
  const isCustomized = (id: string): boolean => {
    return id in customBindings.value
  }

  /**
   * 生成 useKeyboardShortcuts 所需的 ShortcutHandler 数组
   * handlerMap: { 'save-note': () => { ... }, 'new-note': () => { ... }, ... }
   */
  const buildHandlers = (handlerMap: Record<string, () => void>): ShortcutHandler[] => {
    const handlers: ShortcutHandler[] = []
    for (const def of SHORTCUT_DEFS) {
      const handler = handlerMap[def.id]
      if (!handler) continue
      const binding = customBindings.value[def.id] ?? def.defaultBinding
      handlers.push({
        key: binding.key,
        ctrl: binding.ctrl,
        shift: binding.shift,
        alt: binding.alt,
        handler,
        description: def.descriptionKey,
      })
    }
    return handlers
  }

  /** 从后端加载用户自定义快捷键 */
  const load = async () => {
    if (loaded.value) return
    try {
      const settings = await settingsApi.getAll()
      if (settings[SETTINGS_KEY]) {
        const parsed = JSON.parse(settings[SETTINGS_KEY])
        if (typeof parsed === 'object' && parsed !== null) {
          customBindings.value = parsed
        }
      }
    } catch {
      // 使用默认值
    }
    loaded.value = true
  }

  /** 保存用户自定义快捷键到后端 */
  const save = async () => {
    try {
      const json = JSON.stringify(customBindings.value)
      await settingsApi.save({ [SETTINGS_KEY]: json })
    } catch {
      // 静默失败
    }
  }

  return {
    customBindings,
    allBindings,
    getBinding,
    getBindingText,
    checkConflict,
    setBinding,
    resetBinding,
    resetAll,
    isCustomized,
    buildHandlers,
    load,
    save,
  }
}
