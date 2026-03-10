<template>
  <Dialog
    v-model="visible"
    :title="isDecryptMode ? t('encryption.decrypt') : t('encryption.encrypt')"
    :width="400"
  >
    <div class="space-y-4">
      <div>
        <label class="block text-sm text-content-secondary mb-1">{{
          t('encryption.enterPassword')
        }}</label>
        <input
          ref="passwordInput"
          v-model="password"
          type="password"
          class="w-full px-3 py-2 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
          @keyup.enter="isDecryptMode ? handleDecrypt() : null"
        />
      </div>

      <div v-if="!isDecryptMode">
        <label class="block text-sm text-content-secondary mb-1">{{
          t('encryption.confirmPassword')
        }}</label>
        <input
          v-model="confirmPassword"
          type="password"
          class="w-full px-3 py-2 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
          @keyup.enter="handleEncrypt"
        />
      </div>

      <p v-if="errorMsg" class="text-xs text-red-500">{{ errorMsg }}</p>
    </div>

    <template #footer>
      <div class="flex justify-end gap-3">
        <Button type="secondary" @click="visible = false">{{ t('common.cancel') }}</Button>
        <Button v-if="isDecryptMode" @click="handleDecrypt">{{ t('encryption.decrypt') }}</Button>
        <Button v-else @click="handleEncrypt">{{ t('encryption.encrypt') }}</Button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { Dialog, Button } from './ui'
import { cryptoApi } from '../api/note'
import { showNotification } from './ui/notification'

const { t } = useI18n()

const visible = defineModel<boolean>({ default: false })

const props = defineProps<{
  content: string
  isDecryptMode: boolean
}>()

const emit = defineEmits<{
  result: [content: string]
}>()

const password = ref('')
const confirmPassword = ref('')
const errorMsg = ref('')
const passwordInput = ref<HTMLInputElement>()

const handleEncrypt = async () => {
  errorMsg.value = ''
  if (!password.value) {
    errorMsg.value = t('encryption.passwordRequired')
    return
  }
  if (password.value !== confirmPassword.value) {
    errorMsg.value = t('encryption.passwordMismatch')
    return
  }
  try {
    const encrypted = await cryptoApi.encrypt(props.content, password.value)
    emit('result', encrypted)
    showNotification({ type: 'success', message: t('encryption.encryptSuccess') })
    visible.value = false
  } catch {
    showNotification({ type: 'error', message: t('encryption.encryptFailed') })
  }
}

const handleDecrypt = async () => {
  errorMsg.value = ''
  if (!password.value) {
    errorMsg.value = t('encryption.passwordRequired')
    return
  }
  try {
    const decrypted = await cryptoApi.decrypt(props.content, password.value)
    emit('result', decrypted)
    showNotification({ type: 'success', message: t('encryption.decryptSuccess') })
    visible.value = false
  } catch {
    errorMsg.value = t('encryption.decryptFailed')
  }
}

watch(visible, (val) => {
  if (val) {
    password.value = ''
    confirmPassword.value = ''
    errorMsg.value = ''
    nextTick(() => passwordInput.value?.focus())
  }
})
</script>
