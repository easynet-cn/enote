<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { authApi } from '../../api/note'
import { useLockScreen } from '../../composables/useLockScreen'
import { showNotification } from '../ui/notification'

const { t } = useI18n()
const { lockMode, lockOnMinimize } = useLockScreen()

const currentLockMode = defineModel<'none' | 'password' | 'biometric'>('lockMode', {
  required: true,
})
const lockTimeoutValue = defineModel<string>('lockTimeout', { required: true })
const lockOnMinimizeEnabled = defineModel<boolean>('lockOnMinimize', { required: true })

const emit = defineEmits<{
  (e: 'save'): void
}>()

const hasPassword = ref(false)
const showPasswordForm = ref(false)
const passwordError = ref('')
const savingPassword = ref(false)
const passwordForm = ref({ current: '', newPwd: '', confirm: '' })

const lockModeOptions = computed(() => [
  { value: 'none' as const, label: t('settings.lockModeNone') },
  { value: 'password' as const, label: t('settings.lockModePassword') },
])

const setLockMode = async (mode: 'none' | 'password' | 'biometric') => {
  if (mode === 'password' && !hasPassword.value) {
    currentLockMode.value = 'password'
    showPasswordForm.value = true
    lockMode.value = mode
    emit('save')
    return
  }
  if (mode === 'none' && hasPassword.value) {
    await authApi.clearLockPassword()
    hasPassword.value = false
  }
  currentLockMode.value = mode
  lockMode.value = mode
  showPasswordForm.value = false
  emit('save')
}

const handleSavePassword = async () => {
  passwordError.value = ''

  if (hasPassword.value && !passwordForm.value.current) {
    passwordError.value = t('settings.currentPassword')
    return
  }
  if (passwordForm.value.newPwd.length < 4) {
    passwordError.value = t('settings.passwordTooShort')
    return
  }
  if (passwordForm.value.newPwd !== passwordForm.value.confirm) {
    passwordError.value = t('settings.passwordMismatch')
    return
  }

  savingPassword.value = true
  try {
    if (hasPassword.value) {
      const valid = await authApi.verifyLockPassword(passwordForm.value.current)
      if (!valid) {
        passwordError.value = t('settings.passwordIncorrect')
        savingPassword.value = false
        return
      }
    }

    await authApi.setLockPassword(passwordForm.value.newPwd)
    hasPassword.value = true
    showPasswordForm.value = false
    passwordForm.value = { current: '', newPwd: '', confirm: '' }
    showNotification({
      type: 'success',
      message: t(hasPassword.value ? 'settings.passwordChanged' : 'settings.passwordSet'),
    })
  } catch {
    showNotification({ type: 'error', message: t('settings.saveFailed') })
  } finally {
    savingPassword.value = false
  }
}

const handleRemovePassword = async () => {
  try {
    await authApi.clearLockPassword()
    hasPassword.value = false
    currentLockMode.value = 'none'
    lockMode.value = 'none'
    emit('save')
    showNotification({ type: 'success', message: t('settings.passwordRemoved') })
  } catch {
    showNotification({ type: 'error', message: t('settings.saveFailed') })
  }
}

const toggleLockOnMinimize = () => {
  lockOnMinimizeEnabled.value = !lockOnMinimizeEnabled.value
  lockOnMinimize.value = lockOnMinimizeEnabled.value
  emit('save')
}

const initHasPassword = (value: boolean) => {
  hasPassword.value = value
}

defineExpose({ initHasPassword })
</script>

<template>
  <div>
    <h3 class="text-sm font-semibold text-content-secondary mb-3">
      {{ t('settings.security') }}
    </h3>
    <div class="space-y-4">
      <!-- 锁屏方式 -->
      <div class="flex items-center justify-between">
        <label class="text-sm text-content-secondary">{{ t('settings.lockMode') }}</label>
        <div class="flex gap-1 bg-surface-dim rounded-lg p-1">
          <button
            v-for="option in lockModeOptions"
            :key="option.value"
            @click="setLockMode(option.value)"
            class="px-3 py-1.5 text-sm rounded-md transition-colors"
            :class="
              currentLockMode === option.value
                ? 'bg-surface text-indigo-600 shadow-sm'
                : 'text-content-secondary hover:text-content'
            "
          >
            {{ option.label }}
          </button>
        </div>
      </div>

      <!-- 设置/修改密码 -->
      <div v-if="currentLockMode === 'password'">
        <!-- 密码表单 -->
        <div v-if="showPasswordForm" class="space-y-3 p-3 bg-surface-dim rounded-lg">
          <div v-if="hasPassword">
            <label class="block text-xs text-content-tertiary mb-1">{{
              t('settings.currentPassword')
            }}</label>
            <input
              v-model="passwordForm.current"
              type="password"
              class="w-full px-3 py-2 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
            />
          </div>
          <div>
            <label class="block text-xs text-content-tertiary mb-1">{{
              t('settings.newPassword')
            }}</label>
            <input
              v-model="passwordForm.newPwd"
              type="password"
              class="w-full px-3 py-2 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
            />
          </div>
          <div>
            <label class="block text-xs text-content-tertiary mb-1">{{
              t('settings.confirmPassword')
            }}</label>
            <input
              v-model="passwordForm.confirm"
              type="password"
              class="w-full px-3 py-2 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
            />
          </div>
          <p v-if="passwordError" class="text-xs text-red-500">{{ passwordError }}</p>
          <div class="flex justify-end gap-2">
            <button
              @click="showPasswordForm = false"
              class="px-3 py-1.5 text-sm text-content-secondary hover:text-content rounded-md"
            >
              {{ t('common.cancel') }}
            </button>
            <button
              @click="handleSavePassword"
              :disabled="savingPassword"
              class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50"
            >
              {{ t('common.save') }}
            </button>
          </div>
        </div>

        <!-- 密码操作按钮 -->
        <div v-else class="flex items-center justify-between">
          <span class="text-sm text-content-secondary">
            {{ hasPassword ? t('settings.passwordSet') : '' }}
          </span>
          <div class="flex gap-2">
            <button
              @click="showPasswordForm = true"
              class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
            >
              {{ hasPassword ? t('settings.changePassword') : t('settings.setPassword') }}
            </button>
            <button
              v-if="hasPassword"
              @click="handleRemovePassword"
              class="px-3 py-1.5 text-sm text-red-600 hover:bg-red-50 rounded-lg transition-colors"
            >
              {{ t('settings.removePassword') }}
            </button>
          </div>
        </div>
      </div>

      <!-- 自动锁定 -->
      <div v-if="currentLockMode !== 'none'" class="flex items-center justify-between">
        <label class="text-sm text-content-secondary">{{ t('settings.lockTimeout') }}</label>
        <select
          v-model="lockTimeoutValue"
          @change="emit('save')"
          class="px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
        >
          <option value="0">{{ t('settings.lockTimeoutNone') }}</option>
          <option value="5">{{ t('settings.lockTimeoutMinutes', { n: 5 }) }}</option>
          <option value="15">{{ t('settings.lockTimeoutMinutes', { n: 15 }) }}</option>
          <option value="30">{{ t('settings.lockTimeoutMinutes', { n: 30 }) }}</option>
        </select>
      </div>

      <!-- 最小化时锁定 -->
      <div v-if="currentLockMode !== 'none'" class="flex items-center justify-between">
        <label class="text-sm text-content-secondary">{{ t('settings.lockOnMinimize') }}</label>
        <button
          @click="toggleLockOnMinimize"
          class="relative w-10 h-5 rounded-full transition-colors"
          :class="lockOnMinimizeEnabled ? 'bg-indigo-600' : 'bg-slate-300'"
        >
          <span
            class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full transition-transform shadow-sm"
            :class="lockOnMinimizeEnabled ? 'translate-x-5' : ''"
          />
        </button>
      </div>
    </div>
  </div>
</template>
