<template>
  <select
    :value="modelValue"
    @change="handleChange"
    :disabled="disabled"
    :class="selectClasses"
    :aria-label="ariaLabel"
  >
    <option v-if="placeholder" value="" disabled :selected="!modelValue && modelValue !== 0">
      {{ placeholder }}
    </option>
    <template v-for="opt in options" :key="opt.group ?? opt.value">
      <optgroup v-if="opt.group" :label="opt.group">
        <option
          v-for="child in opt.children"
          :key="child.value"
          :value="child.value"
          :disabled="child.disabled"
        >
          {{ child.label }}
        </option>
      </optgroup>
      <option v-else :value="opt.value" :disabled="opt.disabled">
        {{ opt.label }}
      </option>
    </template>
  </select>
</template>

<script setup lang="ts">
import { computed } from 'vue'

export interface AppSelectOption {
  label: string
  value: string | number
  disabled?: boolean
  /** 如果存在则作为 optgroup，children 为分组内的选项 */
  group?: string
  children?: AppSelectOption[]
}

type SelectSize = 'xs' | 'sm' | 'md' | 'lg'

interface Props {
  modelValue?: string | number | null
  options?: AppSelectOption[]
  placeholder?: string
  disabled?: boolean
  size?: SelectSize
  ariaLabel?: string
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: null,
  options: () => [],
  placeholder: '',
  disabled: false,
  size: 'md',
  ariaLabel: '',
})

const emit = defineEmits<{
  'update:modelValue': [value: string | number]
  change: [value: string | number]
}>()

const selectClasses = computed(() => {
  const base =
    'border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-colors'
  const sizeMap: Record<SelectSize, string> = {
    xs: 'h-7 px-1.5 text-xs',
    sm: 'h-8 px-2 text-sm',
    md: 'h-10 px-3 py-2 text-sm',
    lg: 'h-12 px-4 py-3 text-base',
  }
  const disabled = props.disabled ? 'opacity-50 cursor-not-allowed' : ''
  return `${base} ${sizeMap[props.size]} ${disabled}`
})

const handleChange = (e: Event) => {
  const target = e.target as HTMLSelectElement
  const raw = target.value
  // 尝试保持原始类型：如果 options 中对应值是 number，则转为 number
  const matchedOpt = findOption(raw)
  const value = matchedOpt && typeof matchedOpt.value === 'number' ? Number(raw) : raw
  emit('update:modelValue', value)
  emit('change', value)
}

const findOption = (val: string): AppSelectOption | undefined => {
  for (const opt of props.options) {
    if (opt.group && opt.children) {
      const child = opt.children.find((c) => String(c.value) === val)
      if (child) return child
    } else if (String(opt.value) === val) {
      return opt
    }
  }
  return undefined
}
</script>
