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

// 颜色族定义：[key, label]
const COLOR_FAMILY_DEFS: [string, string][] = [
  ['red', '红色'],
  ['orange', '橙色'],
  ['amber', '琥珀色'],
  ['yellow', '黄色'],
  ['lime', '青柠色'],
  ['green', '绿色'],
  ['emerald', '翠绿色'],
  ['teal', '青色'],
  ['cyan', '青蓝色'],
  ['sky', '天蓝色'],
  ['blue', '蓝色'],
  ['indigo', '靛蓝色'],
  ['violet', '紫罗兰'],
  ['purple', '紫色'],
  ['fuchsia', '品红色'],
  ['pink', '粉色'],
  ['rose', '玫瑰色'],
  ['slate', '石板灰'],
  ['gray', '灰色'],
  ['zinc', '锌灰色'],
]

// 生成颜色族
function buildColorFamilies(): Record<string, ColorFamily> {
  const families: Record<string, ColorFamily> = {}
  for (const [key, label] of COLOR_FAMILY_DEFS) {
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
export const presetColors: ColorOption[] = COLOR_FAMILY_DEFS.map(([key, label]) => ({
  label,
  value: `text-${key}-500`,
  family: key,
}))

// 颜色族（用于色阶选择）
export const colorFamilies: Record<string, ColorFamily> = buildColorFamilies()
