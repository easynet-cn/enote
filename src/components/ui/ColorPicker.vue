<template>
  <div class="relative inline-block" ref="pickerRef">
    <button
      @click="toggle"
      :disabled="disabled"
      class="w-7 h-7 rounded-lg border border-slate-300 transition-colors"
      :class="disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer hover:border-slate-400'"
      :style="{ backgroundColor: modelValue || '#ffffff' }"
    ></button>

    <Transition name="picker">
      <div
        v-if="visible"
        class="absolute top-full left-0 mt-1 p-3 bg-white rounded-xl shadow-lg border border-slate-200 z-50"
      >
        <div class="grid grid-cols-5 gap-1 mb-2">
          <button
            v-for="color in predefineColors"
            :key="color"
            @click="selectColor(color)"
            class="w-6 h-6 rounded-lg border border-slate-200 cursor-pointer hover:scale-110 transition-transform"
            :style="{ backgroundColor: color }"
            :class="{ 'ring-2 ring-indigo-500 ring-offset-1': modelValue === color }"
          ></button>
        </div>
        <div class="flex items-center gap-2 pt-2 border-t border-slate-200">
          <input
            type="color"
            :value="modelValue || '#000000'"
            @input="handleInput"
            class="w-8 h-8 cursor-pointer border-0 p-0"
          />
          <input
            type="text"
            :value="modelValue"
            @input="handleTextInput"
            class="flex-1 h-8 px-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
            placeholder="#000000"
          />
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

interface Props {
  modelValue: string
  predefine?: string[]
  disabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
  predefine: () => [
    '#000000',
    '#ffffff',
    '#ff4500',
    '#ff8c00',
    '#ffd700',
    '#90ee90',
    '#00ced1',
    '#1e90ff',
    '#c71585',
    '#8b4513',
  ],
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
  change: [value: string]
}>()

const visible = ref(false)
const pickerRef = ref<HTMLElement>()

const predefineColors = props.predefine

const toggle = () => {
  if (props.disabled) return
  visible.value = !visible.value
}

const selectColor = (color: string) => {
  emit('update:modelValue', color)
  emit('change', color)
  visible.value = false
}

const handleInput = (event: Event) => {
  const value = (event.target as HTMLInputElement).value
  emit('update:modelValue', value)
  emit('change', value)
}

const handleTextInput = (event: Event) => {
  const value = (event.target as HTMLInputElement).value
  emit('update:modelValue', value)
  emit('change', value)
}

const handleClickOutside = (event: MouseEvent) => {
  if (pickerRef.value && !pickerRef.value.contains(event.target as Node)) {
    visible.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.picker-enter-active,
.picker-leave-active {
  transition: all 0.15s ease;
}

.picker-enter-from,
.picker-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

input[type='color'] {
  -webkit-appearance: none;
  appearance: none;
}

input[type='color']::-webkit-color-swatch-wrapper {
  padding: 0;
}

input[type='color']::-webkit-color-swatch {
  border: 1px solid #e2e8f0;
  border-radius: 6px;
}
</style>
