<template>
  <Teleport to="body">
    <Transition name="lock-screen">
      <div v-if="visible" class="lock-screen-overlay">
        <div class="lock-screen-card" :class="{ shake: shaking }">
          <!-- App icon -->
          <div
            class="w-16 h-16 mx-auto mb-4 bg-indigo-100 rounded-2xl flex items-center justify-center"
          >
            <Lock class="w-8 h-8 text-indigo-600" />
          </div>

          <!-- Title -->
          <h2 class="text-xl font-bold text-content mb-1">{{ t('lockScreen.title') }}</h2>
          <p class="text-sm text-content-tertiary mb-6">{{ t('lockScreen.subtitle') }}</p>

          <!-- Password input -->
          <form @submit.prevent="handleUnlock" class="space-y-4">
            <div class="relative">
              <input
                ref="passwordInput"
                v-model="password"
                type="password"
                :placeholder="t('lockScreen.placeholder')"
                class="w-full px-4 py-3 text-sm border border-edge rounded-xl bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent text-center"
                :disabled="loading"
                autocomplete="off"
              />
            </div>

            <!-- Error message -->
            <p v-if="errorMsg" class="text-sm text-red-500">{{ errorMsg }}</p>

            <!-- Unlock button -->
            <button
              type="submit"
              :disabled="!password.trim() || loading"
              class="w-full py-3 text-sm font-medium bg-indigo-600 text-white rounded-xl hover:bg-indigo-700 transition-colors disabled:opacity-50"
            >
              {{ loading ? '...' : t('lockScreen.unlock') }}
            </button>
          </form>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { Lock } from 'lucide-vue-next'
import { authApi } from '../api/note'

const { t } = useI18n()

const props = defineProps<{
  visible: boolean
}>()

const emit = defineEmits<{
  unlocked: []
}>()

const password = ref('')
const errorMsg = ref('')
const loading = ref(false)
const shaking = ref(false)
const passwordInput = ref<HTMLInputElement>()

const handleUnlock = async () => {
  if (!password.value.trim() || loading.value) return

  loading.value = true
  errorMsg.value = ''

  try {
    const success = await authApi.verifyLockPassword(password.value)
    if (success) {
      password.value = ''
      emit('unlocked')
    } else {
      errorMsg.value = t('lockScreen.incorrectPassword')
      password.value = ''
      // Shake animation
      shaking.value = true
      setTimeout(() => {
        shaking.value = false
      }, 500)
      nextTick(() => passwordInput.value?.focus())
    }
  } catch {
    errorMsg.value = t('lockScreen.incorrectPassword')
  } finally {
    loading.value = false
  }
}

// Focus input when visible
watch(
  () => props.visible,
  (val) => {
    if (val) {
      password.value = ''
      errorMsg.value = ''
      nextTick(() => passwordInput.value?.focus())
    }
  },
)
</script>

<style scoped>
.lock-screen-overlay {
  position: fixed;
  inset: 0;
  z-index: 99999;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(20px);
}

.lock-screen-card {
  width: 340px;
  padding: 2.5rem 2rem;
  background: var(--color-bg-primary);
  border-radius: 1.25rem;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.4);
  text-align: center;
}

/* Shake animation */
.shake {
  animation: shake 0.5s ease-in-out;
}

@keyframes shake {
  0%,
  100% {
    transform: translateX(0);
  }
  10%,
  30%,
  50%,
  70%,
  90% {
    transform: translateX(-6px);
  }
  20%,
  40%,
  60%,
  80% {
    transform: translateX(6px);
  }
}

/* Transition */
.lock-screen-enter-active,
.lock-screen-leave-active {
  transition: opacity 0.3s ease;
}

.lock-screen-enter-active .lock-screen-card,
.lock-screen-leave-active .lock-screen-card {
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.lock-screen-enter-from,
.lock-screen-leave-to {
  opacity: 0;
}

.lock-screen-enter-from .lock-screen-card {
  opacity: 0;
  transform: scale(0.9);
}

.lock-screen-leave-to .lock-screen-card {
  opacity: 0;
  transform: scale(0.95);
}
</style>
