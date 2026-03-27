/**
 * Tailwind CSS 颜色配置
 * 用于 StylePicker 组件的预设颜色和色阶选择
 */

export interface ColorOption {
  label: string
  value: string
  family?: string
}

export interface ColorShade {
  name: string
  value: string
  label: string
}

export interface ColorFamily {
  label: string
  key: string
  shades: ColorShade[]
}

// 色阶列表
const SHADES = ['50', '100', '200', '300', '400', '500', '600', '700', '800', '900', '950']

// 颜色族 key 列表
const COLOR_KEYS = [
  'red',
  'orange',
  'amber',
  'yellow',
  'lime',
  'green',
  'emerald',
  'teal',
  'cyan',
  'sky',
  'blue',
  'indigo',
  'violet',
  'purple',
  'fuchsia',
  'pink',
  'rose',
  'slate',
  'gray',
  'zinc',
]

type TranslateFn = (key: string) => string

// 生成颜色族
function buildColorFamilies(t: TranslateFn): Record<string, ColorFamily> {
  const families: Record<string, ColorFamily> = {}
  for (const key of COLOR_KEYS) {
    const label = t(`colors.${key}`)
    families[key] = {
      label,
      key,
      shades: SHADES.map((shade) => ({
        name: shade,
        value: `text-${key}-${shade}`,
        label: `${label} ${shade}`,
      })),
    }
  }
  return families
}

// 预设颜色（主要使用 500 色阶）
export function getPresetColors(t: TranslateFn): ColorOption[] {
  return COLOR_KEYS.map((key) => ({
    label: t(`colors.${key}`),
    value: `text-${key}-500`,
    family: key,
  }))
}

// 颜色族（用于色阶选择）
export function getColorFamilies(t: TranslateFn): Record<string, ColorFamily> {
  return buildColorFamilies(t)
}
