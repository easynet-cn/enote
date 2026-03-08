<template>
  <Dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    :width="420"
  >
    <div class="confirm-content">
      <div :class="['confirm-icon', `confirm-icon-${type}`]">
        <AlertTriangle v-if="type === 'danger' || type === 'warning'" class="w-6 h-6" />
        <HelpCircle v-else class="w-6 h-6" />
      </div>
      <div class="confirm-body">
        <h3 class="confirm-title">{{ displayTitle }}</h3>
        <p class="confirm-message">{{ message }}</p>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-end gap-3">
        <Button type="secondary" @click="handleCancel">{{ displayCancelText }}</Button>
        <Button :type="confirmButtonType" @click="handleConfirm">{{ displayConfirmText }}</Button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { AlertTriangle, HelpCircle } from 'lucide-vue-next'
import Dialog from './BaseDialog.vue'
import Button from './BaseButton.vue'

const { t } = useI18n()

interface Props {
  modelValue: boolean
  title?: string
  message: string
  type?: 'info' | 'warning' | 'danger'
  confirmText?: string
  cancelText?: string
}

const props = withDefaults(defineProps<Props>(), {
  title: '',
  type: 'info',
  confirmText: '',
  cancelText: '',
})

const displayTitle = computed(() => props.title || t('common.confirmAction'))
const displayConfirmText = computed(() => props.confirmText || t('common.confirm'))
const displayCancelText = computed(() => props.cancelText || t('common.cancel'))

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  confirm: []
  cancel: []
}>()

const confirmButtonType = computed(() => {
  if (props.type === 'danger') return 'danger'
  if (props.type === 'warning') return 'warning'
  return 'primary'
})

const handleConfirm = () => {
  emit('update:modelValue', false)
  emit('confirm')
}

const handleCancel = () => {
  emit('update:modelValue', false)
  emit('cancel')
}
</script>

<style scoped>
.confirm-content {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
}

.confirm-icon {
  flex-shrink: 0;
  width: 48px;
  height: 48px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.confirm-icon-danger {
  background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
  color: var(--color-danger);
}

.confirm-icon-warning {
  background: linear-gradient(135deg, #fffbeb 0%, var(--color-warning-light) 100%);
  color: #d97706;
}

.confirm-icon-info {
  background: linear-gradient(135deg, #eff6ff 0%, var(--color-info-light) 100%);
  color: #2563eb;
}

.confirm-body {
  flex: 1;
  padding-top: 4px;
}

.confirm-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0 0 0.5rem 0;
}

.confirm-message {
  font-size: 0.9375rem;
  color: var(--color-text-secondary);
  line-height: 1.5;
  margin: 0;
}
</style>
