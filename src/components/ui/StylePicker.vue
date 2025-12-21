<template>
  <div ref="pickerRef" class="style-picker-wrapper">
    <!-- 触发器 -->
    <div class="style-picker-trigger" @click="toggleDropdown">
      <!-- 颜色预览 -->
      <div class="style-picker-preview">
        <span v-if="modelValue" :class="['style-picker-dot', modelValue]">●</span>
        <span class="style-picker-value">{{ displayValue }}</span>
      </div>

      <!-- 清除按钮 -->
      <span v-if="clearable && modelValue" class="style-picker-clear" @click.stop="handleClear">
        <X :size="14" />
      </span>

      <!-- 箭头 -->
      <span :class="['style-picker-arrow', { 'style-picker-arrow-active': isOpen }]">
        <ChevronDown :size="16" />
      </span>
    </div>

    <!-- 下拉面板 -->
    <Teleport to="body">
      <Transition name="style-picker-dropdown">
        <div v-if="isOpen" ref="dropdownRef" class="style-picker-dropdown" :style="dropdownStyle">
          <!-- 预设颜色 -->
          <div class="style-picker-section">
            <div class="style-picker-section-title">预设颜色</div>
            <div class="style-picker-colors">
              <div
                v-for="color in presetColors"
                :key="color.value"
                :class="[
                  'style-picker-color-item',
                  { 'style-picker-color-selected': modelValue === color.value },
                ]"
                :title="color.label"
                @click="selectColor(color.value)"
              >
                <span :class="['style-picker-color-dot', color.value]">●</span>
              </div>
            </div>
          </div>

          <!-- 色阶选择 -->
          <div v-if="selectedColorFamily" class="style-picker-section">
            <div class="style-picker-section-title">{{ selectedColorFamily.label }} 色阶</div>
            <div class="style-picker-shades">
              <div
                v-for="shade in selectedColorFamily.shades"
                :key="shade.value"
                :class="[
                  'style-picker-shade-item',
                  { 'style-picker-shade-selected': modelValue === shade.value },
                ]"
                :title="shade.label"
                @click="selectColor(shade.value)"
              >
                <span :class="['style-picker-shade-dot', shade.value]">●</span>
                <span class="style-picker-shade-label">{{ shade.name }}</span>
              </div>
            </div>
          </div>

          <!-- 自定义输入 -->
          <div class="style-picker-section">
            <div class="style-picker-section-title">自定义样式</div>
            <div class="style-picker-custom">
              <input
                v-model="customInput"
                type="text"
                placeholder="输入 Tailwind 类名，如 text-red-500"
                class="style-picker-custom-input"
                @keydown.enter="applyCustom"
              />
              <button
                class="style-picker-custom-btn"
                :disabled="!customInput.trim()"
                @click="applyCustom"
              >
                应用
              </button>
            </div>
            <div v-if="customInput" class="style-picker-custom-preview">
              <span>预览:</span>
              <span :class="customInput.trim()">●</span>
            </div>
          </div>

          <!-- 已选信息 -->
          <div v-if="modelValue" class="style-picker-footer">
            <span class="style-picker-footer-text">当前: {{ modelValue }}</span>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { X, ChevronDown } from 'lucide-vue-next'

interface ColorOption {
  label: string
  value: string
  family?: string
}

interface ColorFamily {
  label: string
  key: string
  shades: { name: string; value: string; label: string }[]
}

interface Props {
  modelValue?: string | null
  placeholder?: string
  clearable?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: null,
  placeholder: '选择样式',
  clearable: true,
})

const emit = defineEmits<{
  'update:modelValue': [value: string | null]
  change: [value: string | null]
}>()

// 预设颜色（主要使用 500 色阶）
const presetColors: ColorOption[] = [
  { label: '红色', value: 'text-red-500', family: 'red' },
  { label: '橙色', value: 'text-orange-500', family: 'orange' },
  { label: '琥珀色', value: 'text-amber-500', family: 'amber' },
  { label: '黄色', value: 'text-yellow-500', family: 'yellow' },
  { label: '青柠色', value: 'text-lime-500', family: 'lime' },
  { label: '绿色', value: 'text-green-500', family: 'green' },
  { label: '翠绿色', value: 'text-emerald-500', family: 'emerald' },
  { label: '青色', value: 'text-teal-500', family: 'teal' },
  { label: '青蓝色', value: 'text-cyan-500', family: 'cyan' },
  { label: '天蓝色', value: 'text-sky-500', family: 'sky' },
  { label: '蓝色', value: 'text-blue-500', family: 'blue' },
  { label: '靛蓝色', value: 'text-indigo-500', family: 'indigo' },
  { label: '紫罗兰', value: 'text-violet-500', family: 'violet' },
  { label: '紫色', value: 'text-purple-500', family: 'purple' },
  { label: '品红色', value: 'text-fuchsia-500', family: 'fuchsia' },
  { label: '粉色', value: 'text-pink-500', family: 'pink' },
  { label: '玫瑰色', value: 'text-rose-500', family: 'rose' },
  { label: '石板灰', value: 'text-slate-500', family: 'slate' },
  { label: '灰色', value: 'text-gray-500', family: 'gray' },
  { label: '锌灰色', value: 'text-zinc-500', family: 'zinc' },
]

// 颜色族定义（用于色阶选择）
const colorFamilies: Record<string, ColorFamily> = {
  red: {
    label: '红色',
    key: 'red',
    shades: [
      { name: '50', value: 'text-red-50', label: '红色 50' },
      { name: '100', value: 'text-red-100', label: '红色 100' },
      { name: '200', value: 'text-red-200', label: '红色 200' },
      { name: '300', value: 'text-red-300', label: '红色 300' },
      { name: '400', value: 'text-red-400', label: '红色 400' },
      { name: '500', value: 'text-red-500', label: '红色 500' },
      { name: '600', value: 'text-red-600', label: '红色 600' },
      { name: '700', value: 'text-red-700', label: '红色 700' },
      { name: '800', value: 'text-red-800', label: '红色 800' },
      { name: '900', value: 'text-red-900', label: '红色 900' },
      { name: '950', value: 'text-red-950', label: '红色 950' },
    ],
  },
  orange: {
    label: '橙色',
    key: 'orange',
    shades: [
      { name: '50', value: 'text-orange-50', label: '橙色 50' },
      { name: '100', value: 'text-orange-100', label: '橙色 100' },
      { name: '200', value: 'text-orange-200', label: '橙色 200' },
      { name: '300', value: 'text-orange-300', label: '橙色 300' },
      { name: '400', value: 'text-orange-400', label: '橙色 400' },
      { name: '500', value: 'text-orange-500', label: '橙色 500' },
      { name: '600', value: 'text-orange-600', label: '橙色 600' },
      { name: '700', value: 'text-orange-700', label: '橙色 700' },
      { name: '800', value: 'text-orange-800', label: '橙色 800' },
      { name: '900', value: 'text-orange-900', label: '橙色 900' },
      { name: '950', value: 'text-orange-950', label: '橙色 950' },
    ],
  },
  amber: {
    label: '琥珀色',
    key: 'amber',
    shades: [
      { name: '50', value: 'text-amber-50', label: '琥珀色 50' },
      { name: '100', value: 'text-amber-100', label: '琥珀色 100' },
      { name: '200', value: 'text-amber-200', label: '琥珀色 200' },
      { name: '300', value: 'text-amber-300', label: '琥珀色 300' },
      { name: '400', value: 'text-amber-400', label: '琥珀色 400' },
      { name: '500', value: 'text-amber-500', label: '琥珀色 500' },
      { name: '600', value: 'text-amber-600', label: '琥珀色 600' },
      { name: '700', value: 'text-amber-700', label: '琥珀色 700' },
      { name: '800', value: 'text-amber-800', label: '琥珀色 800' },
      { name: '900', value: 'text-amber-900', label: '琥珀色 900' },
      { name: '950', value: 'text-amber-950', label: '琥珀色 950' },
    ],
  },
  yellow: {
    label: '黄色',
    key: 'yellow',
    shades: [
      { name: '50', value: 'text-yellow-50', label: '黄色 50' },
      { name: '100', value: 'text-yellow-100', label: '黄色 100' },
      { name: '200', value: 'text-yellow-200', label: '黄色 200' },
      { name: '300', value: 'text-yellow-300', label: '黄色 300' },
      { name: '400', value: 'text-yellow-400', label: '黄色 400' },
      { name: '500', value: 'text-yellow-500', label: '黄色 500' },
      { name: '600', value: 'text-yellow-600', label: '黄色 600' },
      { name: '700', value: 'text-yellow-700', label: '黄色 700' },
      { name: '800', value: 'text-yellow-800', label: '黄色 800' },
      { name: '900', value: 'text-yellow-900', label: '黄色 900' },
      { name: '950', value: 'text-yellow-950', label: '黄色 950' },
    ],
  },
  lime: {
    label: '青柠色',
    key: 'lime',
    shades: [
      { name: '50', value: 'text-lime-50', label: '青柠色 50' },
      { name: '100', value: 'text-lime-100', label: '青柠色 100' },
      { name: '200', value: 'text-lime-200', label: '青柠色 200' },
      { name: '300', value: 'text-lime-300', label: '青柠色 300' },
      { name: '400', value: 'text-lime-400', label: '青柠色 400' },
      { name: '500', value: 'text-lime-500', label: '青柠色 500' },
      { name: '600', value: 'text-lime-600', label: '青柠色 600' },
      { name: '700', value: 'text-lime-700', label: '青柠色 700' },
      { name: '800', value: 'text-lime-800', label: '青柠色 800' },
      { name: '900', value: 'text-lime-900', label: '青柠色 900' },
      { name: '950', value: 'text-lime-950', label: '青柠色 950' },
    ],
  },
  green: {
    label: '绿色',
    key: 'green',
    shades: [
      { name: '50', value: 'text-green-50', label: '绿色 50' },
      { name: '100', value: 'text-green-100', label: '绿色 100' },
      { name: '200', value: 'text-green-200', label: '绿色 200' },
      { name: '300', value: 'text-green-300', label: '绿色 300' },
      { name: '400', value: 'text-green-400', label: '绿色 400' },
      { name: '500', value: 'text-green-500', label: '绿色 500' },
      { name: '600', value: 'text-green-600', label: '绿色 600' },
      { name: '700', value: 'text-green-700', label: '绿色 700' },
      { name: '800', value: 'text-green-800', label: '绿色 800' },
      { name: '900', value: 'text-green-900', label: '绿色 900' },
      { name: '950', value: 'text-green-950', label: '绿色 950' },
    ],
  },
  emerald: {
    label: '翠绿色',
    key: 'emerald',
    shades: [
      { name: '50', value: 'text-emerald-50', label: '翠绿色 50' },
      { name: '100', value: 'text-emerald-100', label: '翠绿色 100' },
      { name: '200', value: 'text-emerald-200', label: '翠绿色 200' },
      { name: '300', value: 'text-emerald-300', label: '翠绿色 300' },
      { name: '400', value: 'text-emerald-400', label: '翠绿色 400' },
      { name: '500', value: 'text-emerald-500', label: '翠绿色 500' },
      { name: '600', value: 'text-emerald-600', label: '翠绿色 600' },
      { name: '700', value: 'text-emerald-700', label: '翠绿色 700' },
      { name: '800', value: 'text-emerald-800', label: '翠绿色 800' },
      { name: '900', value: 'text-emerald-900', label: '翠绿色 900' },
      { name: '950', value: 'text-emerald-950', label: '翠绿色 950' },
    ],
  },
  teal: {
    label: '青色',
    key: 'teal',
    shades: [
      { name: '50', value: 'text-teal-50', label: '青色 50' },
      { name: '100', value: 'text-teal-100', label: '青色 100' },
      { name: '200', value: 'text-teal-200', label: '青色 200' },
      { name: '300', value: 'text-teal-300', label: '青色 300' },
      { name: '400', value: 'text-teal-400', label: '青色 400' },
      { name: '500', value: 'text-teal-500', label: '青色 500' },
      { name: '600', value: 'text-teal-600', label: '青色 600' },
      { name: '700', value: 'text-teal-700', label: '青色 700' },
      { name: '800', value: 'text-teal-800', label: '青色 800' },
      { name: '900', value: 'text-teal-900', label: '青色 900' },
      { name: '950', value: 'text-teal-950', label: '青色 950' },
    ],
  },
  cyan: {
    label: '青蓝色',
    key: 'cyan',
    shades: [
      { name: '50', value: 'text-cyan-50', label: '青蓝色 50' },
      { name: '100', value: 'text-cyan-100', label: '青蓝色 100' },
      { name: '200', value: 'text-cyan-200', label: '青蓝色 200' },
      { name: '300', value: 'text-cyan-300', label: '青蓝色 300' },
      { name: '400', value: 'text-cyan-400', label: '青蓝色 400' },
      { name: '500', value: 'text-cyan-500', label: '青蓝色 500' },
      { name: '600', value: 'text-cyan-600', label: '青蓝色 600' },
      { name: '700', value: 'text-cyan-700', label: '青蓝色 700' },
      { name: '800', value: 'text-cyan-800', label: '青蓝色 800' },
      { name: '900', value: 'text-cyan-900', label: '青蓝色 900' },
      { name: '950', value: 'text-cyan-950', label: '青蓝色 950' },
    ],
  },
  sky: {
    label: '天蓝色',
    key: 'sky',
    shades: [
      { name: '50', value: 'text-sky-50', label: '天蓝色 50' },
      { name: '100', value: 'text-sky-100', label: '天蓝色 100' },
      { name: '200', value: 'text-sky-200', label: '天蓝色 200' },
      { name: '300', value: 'text-sky-300', label: '天蓝色 300' },
      { name: '400', value: 'text-sky-400', label: '天蓝色 400' },
      { name: '500', value: 'text-sky-500', label: '天蓝色 500' },
      { name: '600', value: 'text-sky-600', label: '天蓝色 600' },
      { name: '700', value: 'text-sky-700', label: '天蓝色 700' },
      { name: '800', value: 'text-sky-800', label: '天蓝色 800' },
      { name: '900', value: 'text-sky-900', label: '天蓝色 900' },
      { name: '950', value: 'text-sky-950', label: '天蓝色 950' },
    ],
  },
  blue: {
    label: '蓝色',
    key: 'blue',
    shades: [
      { name: '50', value: 'text-blue-50', label: '蓝色 50' },
      { name: '100', value: 'text-blue-100', label: '蓝色 100' },
      { name: '200', value: 'text-blue-200', label: '蓝色 200' },
      { name: '300', value: 'text-blue-300', label: '蓝色 300' },
      { name: '400', value: 'text-blue-400', label: '蓝色 400' },
      { name: '500', value: 'text-blue-500', label: '蓝色 500' },
      { name: '600', value: 'text-blue-600', label: '蓝色 600' },
      { name: '700', value: 'text-blue-700', label: '蓝色 700' },
      { name: '800', value: 'text-blue-800', label: '蓝色 800' },
      { name: '900', value: 'text-blue-900', label: '蓝色 900' },
      { name: '950', value: 'text-blue-950', label: '蓝色 950' },
    ],
  },
  indigo: {
    label: '靛蓝色',
    key: 'indigo',
    shades: [
      { name: '50', value: 'text-indigo-50', label: '靛蓝色 50' },
      { name: '100', value: 'text-indigo-100', label: '靛蓝色 100' },
      { name: '200', value: 'text-indigo-200', label: '靛蓝色 200' },
      { name: '300', value: 'text-indigo-300', label: '靛蓝色 300' },
      { name: '400', value: 'text-indigo-400', label: '靛蓝色 400' },
      { name: '500', value: 'text-indigo-500', label: '靛蓝色 500' },
      { name: '600', value: 'text-indigo-600', label: '靛蓝色 600' },
      { name: '700', value: 'text-indigo-700', label: '靛蓝色 700' },
      { name: '800', value: 'text-indigo-800', label: '靛蓝色 800' },
      { name: '900', value: 'text-indigo-900', label: '靛蓝色 900' },
      { name: '950', value: 'text-indigo-950', label: '靛蓝色 950' },
    ],
  },
  violet: {
    label: '紫罗兰',
    key: 'violet',
    shades: [
      { name: '50', value: 'text-violet-50', label: '紫罗兰 50' },
      { name: '100', value: 'text-violet-100', label: '紫罗兰 100' },
      { name: '200', value: 'text-violet-200', label: '紫罗兰 200' },
      { name: '300', value: 'text-violet-300', label: '紫罗兰 300' },
      { name: '400', value: 'text-violet-400', label: '紫罗兰 400' },
      { name: '500', value: 'text-violet-500', label: '紫罗兰 500' },
      { name: '600', value: 'text-violet-600', label: '紫罗兰 600' },
      { name: '700', value: 'text-violet-700', label: '紫罗兰 700' },
      { name: '800', value: 'text-violet-800', label: '紫罗兰 800' },
      { name: '900', value: 'text-violet-900', label: '紫罗兰 900' },
      { name: '950', value: 'text-violet-950', label: '紫罗兰 950' },
    ],
  },
  purple: {
    label: '紫色',
    key: 'purple',
    shades: [
      { name: '50', value: 'text-purple-50', label: '紫色 50' },
      { name: '100', value: 'text-purple-100', label: '紫色 100' },
      { name: '200', value: 'text-purple-200', label: '紫色 200' },
      { name: '300', value: 'text-purple-300', label: '紫色 300' },
      { name: '400', value: 'text-purple-400', label: '紫色 400' },
      { name: '500', value: 'text-purple-500', label: '紫色 500' },
      { name: '600', value: 'text-purple-600', label: '紫色 600' },
      { name: '700', value: 'text-purple-700', label: '紫色 700' },
      { name: '800', value: 'text-purple-800', label: '紫色 800' },
      { name: '900', value: 'text-purple-900', label: '紫色 900' },
      { name: '950', value: 'text-purple-950', label: '紫色 950' },
    ],
  },
  fuchsia: {
    label: '品红色',
    key: 'fuchsia',
    shades: [
      { name: '50', value: 'text-fuchsia-50', label: '品红色 50' },
      { name: '100', value: 'text-fuchsia-100', label: '品红色 100' },
      { name: '200', value: 'text-fuchsia-200', label: '品红色 200' },
      { name: '300', value: 'text-fuchsia-300', label: '品红色 300' },
      { name: '400', value: 'text-fuchsia-400', label: '品红色 400' },
      { name: '500', value: 'text-fuchsia-500', label: '品红色 500' },
      { name: '600', value: 'text-fuchsia-600', label: '品红色 600' },
      { name: '700', value: 'text-fuchsia-700', label: '品红色 700' },
      { name: '800', value: 'text-fuchsia-800', label: '品红色 800' },
      { name: '900', value: 'text-fuchsia-900', label: '品红色 900' },
      { name: '950', value: 'text-fuchsia-950', label: '品红色 950' },
    ],
  },
  pink: {
    label: '粉色',
    key: 'pink',
    shades: [
      { name: '50', value: 'text-pink-50', label: '粉色 50' },
      { name: '100', value: 'text-pink-100', label: '粉色 100' },
      { name: '200', value: 'text-pink-200', label: '粉色 200' },
      { name: '300', value: 'text-pink-300', label: '粉色 300' },
      { name: '400', value: 'text-pink-400', label: '粉色 400' },
      { name: '500', value: 'text-pink-500', label: '粉色 500' },
      { name: '600', value: 'text-pink-600', label: '粉色 600' },
      { name: '700', value: 'text-pink-700', label: '粉色 700' },
      { name: '800', value: 'text-pink-800', label: '粉色 800' },
      { name: '900', value: 'text-pink-900', label: '粉色 900' },
      { name: '950', value: 'text-pink-950', label: '粉色 950' },
    ],
  },
  rose: {
    label: '玫瑰色',
    key: 'rose',
    shades: [
      { name: '50', value: 'text-rose-50', label: '玫瑰色 50' },
      { name: '100', value: 'text-rose-100', label: '玫瑰色 100' },
      { name: '200', value: 'text-rose-200', label: '玫瑰色 200' },
      { name: '300', value: 'text-rose-300', label: '玫瑰色 300' },
      { name: '400', value: 'text-rose-400', label: '玫瑰色 400' },
      { name: '500', value: 'text-rose-500', label: '玫瑰色 500' },
      { name: '600', value: 'text-rose-600', label: '玫瑰色 600' },
      { name: '700', value: 'text-rose-700', label: '玫瑰色 700' },
      { name: '800', value: 'text-rose-800', label: '玫瑰色 800' },
      { name: '900', value: 'text-rose-900', label: '玫瑰色 900' },
      { name: '950', value: 'text-rose-950', label: '玫瑰色 950' },
    ],
  },
  slate: {
    label: '石板灰',
    key: 'slate',
    shades: [
      { name: '50', value: 'text-slate-50', label: '石板灰 50' },
      { name: '100', value: 'text-slate-100', label: '石板灰 100' },
      { name: '200', value: 'text-slate-200', label: '石板灰 200' },
      { name: '300', value: 'text-slate-300', label: '石板灰 300' },
      { name: '400', value: 'text-slate-400', label: '石板灰 400' },
      { name: '500', value: 'text-slate-500', label: '石板灰 500' },
      { name: '600', value: 'text-slate-600', label: '石板灰 600' },
      { name: '700', value: 'text-slate-700', label: '石板灰 700' },
      { name: '800', value: 'text-slate-800', label: '石板灰 800' },
      { name: '900', value: 'text-slate-900', label: '石板灰 900' },
      { name: '950', value: 'text-slate-950', label: '石板灰 950' },
    ],
  },
  gray: {
    label: '灰色',
    key: 'gray',
    shades: [
      { name: '50', value: 'text-gray-50', label: '灰色 50' },
      { name: '100', value: 'text-gray-100', label: '灰色 100' },
      { name: '200', value: 'text-gray-200', label: '灰色 200' },
      { name: '300', value: 'text-gray-300', label: '灰色 300' },
      { name: '400', value: 'text-gray-400', label: '灰色 400' },
      { name: '500', value: 'text-gray-500', label: '灰色 500' },
      { name: '600', value: 'text-gray-600', label: '灰色 600' },
      { name: '700', value: 'text-gray-700', label: '灰色 700' },
      { name: '800', value: 'text-gray-800', label: '灰色 800' },
      { name: '900', value: 'text-gray-900', label: '灰色 900' },
      { name: '950', value: 'text-gray-950', label: '灰色 950' },
    ],
  },
  zinc: {
    label: '锌灰色',
    key: 'zinc',
    shades: [
      { name: '50', value: 'text-zinc-50', label: '锌灰色 50' },
      { name: '100', value: 'text-zinc-100', label: '锌灰色 100' },
      { name: '200', value: 'text-zinc-200', label: '锌灰色 200' },
      { name: '300', value: 'text-zinc-300', label: '锌灰色 300' },
      { name: '400', value: 'text-zinc-400', label: '锌灰色 400' },
      { name: '500', value: 'text-zinc-500', label: '锌灰色 500' },
      { name: '600', value: 'text-zinc-600', label: '锌灰色 600' },
      { name: '700', value: 'text-zinc-700', label: '锌灰色 700' },
      { name: '800', value: 'text-zinc-800', label: '锌灰色 800' },
      { name: '900', value: 'text-zinc-900', label: '锌灰色 900' },
      { name: '950', value: 'text-zinc-950', label: '锌灰色 950' },
    ],
  },
}

const pickerRef = ref<HTMLElement>()
const dropdownRef = ref<HTMLElement>()
const isOpen = ref(false)
const customInput = ref('')
const dropdownStyle = ref<Record<string, string>>({})

// 显示值
const displayValue = computed(() => {
  if (!props.modelValue) return props.placeholder
  const preset = presetColors.find((c) => c.value === props.modelValue)
  if (preset) return preset.label
  return props.modelValue
})

// 当前选中的颜色族
const selectedColorFamily = computed(() => {
  if (!props.modelValue) return null
  // 从值中提取颜色族，如 text-red-500 -> red
  const match = props.modelValue.match(/text-(\w+)-\d+/)
  if (match && colorFamilies[match[1]]) {
    return colorFamilies[match[1]]
  }
  return null
})

// 切换下拉
const toggleDropdown = () => {
  if (isOpen.value) {
    closeDropdown()
  } else {
    openDropdown()
  }
}

// 打开下拉
const openDropdown = () => {
  isOpen.value = true
  updateDropdownPosition()
}

// 关闭下拉
const closeDropdown = () => {
  isOpen.value = false
  customInput.value = ''
}

// 更新下拉位置
const updateDropdownPosition = () => {
  if (!pickerRef.value) return

  const rect = pickerRef.value.getBoundingClientRect()
  const spaceBelow = window.innerHeight - rect.bottom
  const dropdownHeight = 450

  const showAbove = spaceBelow < dropdownHeight && rect.top > spaceBelow

  dropdownStyle.value = {
    position: 'fixed',
    left: `${Math.max(8, rect.left)}px`,
    width: '300px',
    zIndex: '9999',
    ...(showAbove
      ? { bottom: `${window.innerHeight - rect.top + 4}px` }
      : { top: `${rect.bottom + 4}px` }),
  }
}

// 选择颜色
const selectColor = (value: string) => {
  emit('update:modelValue', value)
  emit('change', value)
}

// 应用自定义样式
const applyCustom = () => {
  const value = customInput.value.trim()
  if (value) {
    emit('update:modelValue', value)
    emit('change', value)
    closeDropdown()
  }
}

// 清除选择
const handleClear = () => {
  emit('update:modelValue', null)
  emit('change', null)
}

// 点击外部关闭
const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as Node
  if (
    pickerRef.value &&
    !pickerRef.value.contains(target) &&
    dropdownRef.value &&
    !dropdownRef.value.contains(target)
  ) {
    closeDropdown()
  }
}

// 窗口事件
const handleScroll = () => {
  if (isOpen.value) {
    updateDropdownPosition()
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  window.addEventListener('scroll', handleScroll, true)
  window.addEventListener('resize', handleScroll)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  window.removeEventListener('scroll', handleScroll, true)
  window.removeEventListener('resize', handleScroll)
})

// 暴露方法
defineExpose({
  open: openDropdown,
  close: closeDropdown,
})
</script>

<style scoped>
.style-picker-wrapper {
  position: relative;
  display: inline-flex;
  width: 100%;
}

/* 触发器 */
.style-picker-trigger {
  display: flex;
  align-items: center;
  width: 100%;
  height: 36px;
  padding: 0 12px;
  background: white;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  gap: 8px;
}

.style-picker-trigger:hover {
  border-color: #9ca3af;
}

/* 预览 */
.style-picker-preview {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 8px;
}

.style-picker-dot {
  font-size: 16px;
}

.style-picker-value {
  font-size: 14px;
  color: #374151;
}

/* 清除按钮 */
.style-picker-clear {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: #9ca3af;
  cursor: pointer;
  opacity: 0;
  transition: all 0.2s;
}

.style-picker-wrapper:hover .style-picker-clear {
  opacity: 1;
}

.style-picker-clear:hover {
  color: #6b7280;
}

/* 箭头 */
.style-picker-arrow {
  display: inline-flex;
  align-items: center;
  color: #9ca3af;
  transition: transform 0.2s ease;
}

.style-picker-arrow-active {
  transform: rotate(180deg);
}

/* 下拉面板 */
.style-picker-dropdown {
  background: white;
  border-radius: 12px;
  box-shadow:
    0 10px 40px rgba(0, 0, 0, 0.15),
    0 0 0 1px rgba(0, 0, 0, 0.05);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  max-height: 450px;
  overflow-y: auto;
}

/* 区块 */
.style-picker-section {
  padding: 12px;
  border-bottom: 1px solid #e5e7eb;
}

.style-picker-section:last-child {
  border-bottom: none;
}

.style-picker-section-title {
  font-size: 12px;
  font-weight: 600;
  color: #6b7280;
  margin-bottom: 8px;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

/* 预设颜色网格 */
.style-picker-colors {
  display: grid;
  grid-template-columns: repeat(10, 1fr);
  gap: 4px;
}

.style-picker-color-item {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
  border: 2px solid transparent;
}

.style-picker-color-item:hover {
  transform: scale(1.2);
}

.style-picker-color-selected {
  border-color: #1f2937;
  transform: scale(1.1);
}

.style-picker-color-dot {
  font-size: 18px;
}

/* 色阶选择 */
.style-picker-shades {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.style-picker-shade-item {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
  border: 1px solid #e5e7eb;
  font-size: 12px;
}

.style-picker-shade-item:hover {
  background: #f3f4f6;
  border-color: #d1d5db;
}

.style-picker-shade-selected {
  background: #f0fdf4;
  border-color: #22c55e;
}

.style-picker-shade-dot {
  font-size: 14px;
}

.style-picker-shade-label {
  color: #6b7280;
}

/* 自定义输入 */
.style-picker-custom {
  display: flex;
  gap: 8px;
}

.style-picker-custom-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 13px;
  outline: none;
  transition: all 0.2s ease;
}

.style-picker-custom-input:focus {
  border-color: #22c55e;
  box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.1);
}

.style-picker-custom-btn {
  padding: 8px 16px;
  background: #22c55e;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.style-picker-custom-btn:hover:not(:disabled) {
  background: #16a34a;
}

.style-picker-custom-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 自定义预览 */
.style-picker-custom-preview {
  margin-top: 8px;
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: #6b7280;
}

/* 底部信息 */
.style-picker-footer {
  padding: 8px 12px;
  border-top: 1px solid #e5e7eb;
  background: #f9fafb;
}

.style-picker-footer-text {
  font-size: 12px;
  color: #6b7280;
}

/* 下拉动画 */
.style-picker-dropdown-enter-active,
.style-picker-dropdown-leave-active {
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.style-picker-dropdown-enter-from,
.style-picker-dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.95);
}

/* 滚动条 */
.style-picker-dropdown::-webkit-scrollbar {
  width: 6px;
}

.style-picker-dropdown::-webkit-scrollbar-track {
  background: transparent;
}

.style-picker-dropdown::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 3px;
}

.style-picker-dropdown::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}
</style>
