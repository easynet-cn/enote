<script setup lang="ts">
import { ref, onErrorCaptured } from 'vue'
import { useI18n } from 'vue-i18n'
import { RotateCcw } from 'lucide-vue-next'

const { t } = useI18n()
const error = ref<Error | null>(null)

onErrorCaptured((err: Error) => {
  error.value = err
  console.error('[ErrorBoundary]', err)
  return false
})

const handleRetry = () => {
  error.value = null
}
</script>

<template>
  <slot v-if="!error" />
  <div v-else class="error-boundary">
    <div class="error-boundary-content">
      <p class="error-boundary-message">{{ t('common.errorBoundaryMessage') }}</p>
      <button class="error-boundary-retry" @click="handleRetry">
        <RotateCcw class="w-4 h-4" />
        {{ t('common.retry') }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.error-boundary {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  min-height: 200px;
  background: var(--color-bg-secondary);
}

.error-boundary-content {
  text-align: center;
}

.error-boundary-message {
  color: var(--color-text-secondary);
  margin-bottom: 12px;
  font-size: 14px;
}

.error-boundary-retry {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  font-size: 14px;
  transition: background var(--transition-fast) var(--ease-default);
}

.error-boundary-retry:hover {
  background: var(--color-primary-hover);
}
</style>
