<template>
  <Dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    :width="400"
  >
    <div class="flex items-start gap-4">
      <div
        :class="[
          'flex-shrink-0 w-10 h-10 rounded-full flex items-center justify-center',
          type === 'danger' ? 'bg-red-100' : type === 'warning' ? 'bg-yellow-100' : 'bg-blue-100',
        ]"
      >
        <AlertTriangle
          v-if="type === 'danger' || type === 'warning'"
          :class="['w-5 h-5', type === 'danger' ? 'text-red-600' : 'text-yellow-600']"
        />
        <HelpCircle v-else class="w-5 h-5 text-blue-600" />
      </div>
      <div class="flex-1">
        <h3 class="text-lg font-semibold text-gray-900 mb-2">{{ title }}</h3>
        <p class="text-gray-600">{{ message }}</p>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-end gap-3">
        <button
          @click="handleCancel"
          class="px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
        >
          {{ cancelText }}
        </button>
        <button
          @click="handleConfirm"
          :class="[
            'px-4 py-2 text-white rounded-md transition-colors',
            type === 'danger'
              ? 'bg-red-500 hover:bg-red-600'
              : type === 'warning'
                ? 'bg-yellow-500 hover:bg-yellow-600'
                : 'bg-green-500 hover:bg-green-600',
          ]"
        >
          {{ confirmText }}
        </button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { AlertTriangle, HelpCircle } from 'lucide-vue-next'
import Dialog from './Dialog.vue'

interface Props {
  modelValue: boolean
  title?: string
  message: string
  type?: 'info' | 'warning' | 'danger'
  confirmText?: string
  cancelText?: string
}

withDefaults(defineProps<Props>(), {
  title: '确认操作',
  type: 'info',
  confirmText: '确定',
  cancelText: '取消',
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  confirm: []
  cancel: []
}>()

const handleConfirm = () => {
  emit('update:modelValue', false)
  emit('confirm')
}

const handleCancel = () => {
  emit('update:modelValue', false)
  emit('cancel')
}
</script>
