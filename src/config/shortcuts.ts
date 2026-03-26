/**
 * 快捷键定义注册中心
 * 所有应用级快捷键在此集中定义
 */

export interface KeyBinding {
  key: string
  ctrl?: boolean
  shift?: boolean
  alt?: boolean
}

export interface ShortcutDef {
  /** 唯一标识 */
  id: string
  /** 默认按键绑定 */
  defaultBinding: KeyBinding
  /** i18n description key（如 'shortcuts.save'） */
  descriptionKey: string
  /** 是否为系统级快捷键（不可修改） */
  system: boolean
}

export const SHORTCUT_DEFS: ShortcutDef[] = [
  {
    id: 'save-note',
    defaultBinding: { key: 's', ctrl: true },
    descriptionKey: 'shortcuts.save',
    system: true,
  },
  {
    id: 'cancel-edit',
    defaultBinding: { key: 'Escape' },
    descriptionKey: 'shortcuts.cancelEdit',
    system: true,
  },
  {
    id: 'new-note',
    defaultBinding: { key: 'n', ctrl: true },
    descriptionKey: 'shortcuts.newNote',
    system: false,
  },
  {
    id: 'edit-note',
    defaultBinding: { key: 'e', ctrl: true },
    descriptionKey: 'shortcuts.editNote',
    system: false,
  },
  {
    id: 'toggle-sidebar',
    defaultBinding: { key: 'b', ctrl: true },
    descriptionKey: 'shortcuts.toggleSidebar',
    system: false,
  },
  {
    id: 'lock-app',
    defaultBinding: { key: 'l', ctrl: true },
    descriptionKey: 'shortcuts.lockApp',
    system: false,
  },
  {
    id: 'command-palette',
    defaultBinding: { key: 'p', ctrl: true },
    descriptionKey: 'shortcuts.commandPalette',
    system: false,
  },
  {
    id: 'switch-layout',
    defaultBinding: { key: 'm', ctrl: true, shift: true },
    descriptionKey: 'shortcuts.switchLayout',
    system: false,
  },
  {
    id: 'help-manual',
    defaultBinding: { key: 'F1' },
    descriptionKey: 'shortcuts.helpManual',
    system: false,
  },
]

/** 按 id 查找快捷键定义 */
export function getShortcutDef(id: string): ShortcutDef | undefined {
  return SHORTCUT_DEFS.find((d) => d.id === id)
}

/** 将 KeyBinding 转换为显示文本 */
export function bindingToText(binding: KeyBinding): string {
  const isMac =
    (navigator as unknown as { userAgentData?: { platform?: string } }).userAgentData?.platform
      ?.toUpperCase()
      ?.includes('MAC') ?? navigator.platform.toUpperCase().indexOf('MAC') >= 0
  const parts: string[] = []

  if (binding.ctrl) parts.push(isMac ? '⌘' : 'Ctrl')
  if (binding.shift) parts.push(isMac ? '⇧' : 'Shift')
  if (binding.alt) parts.push(isMac ? '⌥' : 'Alt')

  // 特殊键名友好显示
  const keyDisplay = binding.key === 'Escape' ? 'Esc' : binding.key.toUpperCase()
  parts.push(keyDisplay)

  return parts.join(isMac ? '' : '+')
}

/** 比较两个 KeyBinding 是否相同 */
export function bindingsEqual(a: KeyBinding, b: KeyBinding): boolean {
  return (
    a.key.toLowerCase() === b.key.toLowerCase() &&
    !!a.ctrl === !!b.ctrl &&
    !!a.shift === !!b.shift &&
    !!a.alt === !!b.alt
  )
}
