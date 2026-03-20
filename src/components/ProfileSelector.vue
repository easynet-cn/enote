<template>
  <div class="fixed inset-0 flex items-center justify-center bg-surface-alt">
    <div class="w-[560px] bg-surface rounded-2xl shadow-2xl border border-edge overflow-hidden">
      <!-- 标题 + 操作按钮 -->
      <div class="px-8 pt-8 pb-2 flex items-center justify-between">
        <h1 class="text-2xl font-bold text-content">{{ t('profile.selectTitle') }}</h1>
        <div class="flex items-center gap-2">
          <button
            @click="toggleLang"
            class="flex items-center gap-1.5 px-3 py-1.5 text-sm text-content-secondary hover:text-content bg-surface-dim rounded-lg hover:bg-slate-200 transition-colors"
            :title="nextLocaleLabel"
          >
            <Languages class="w-4 h-4" />
            <span>{{ currentLocaleLabel }}</span>
          </button>
          <button
            v-if="props.showClose"
            @click="emit('close')"
            class="p-1.5 text-content-tertiary hover:text-content hover:bg-surface-dim rounded-lg transition-colors"
            :title="t('common.close')"
          >
            <X class="w-5 h-5" />
          </button>
        </div>
      </div>

      <!-- Profile 列表 -->
      <div class="px-8 py-4 space-y-3 max-h-[400px] overflow-y-auto">
        <div
          v-for="profile in profiles"
          :key="profile.id"
          @click="selectedId = profile.id"
          @dblclick="connectProfile(profile.id)"
          class="flex items-center gap-4 p-4 rounded-xl border-2 cursor-pointer transition-all group"
          :class="
            selectedId === profile.id
              ? 'border-indigo-600 bg-indigo-50/50'
              : 'border-edge hover:border-slate-300'
          "
        >
          <!-- 图标 -->
          <div
            class="w-10 h-10 rounded-lg flex items-center justify-center"
            :class="
              selectedId === profile.id
                ? 'bg-indigo-100 text-indigo-600'
                : 'bg-surface-dim text-content-secondary'
            "
          >
            <component :is="getDbIcon(profile.dbType)" class="w-5 h-5" />
          </div>

          <!-- 信息 -->
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2">
              <span class="font-medium text-content truncate">{{ profile.name }}</span>
              <span
                v-if="profile.isActive"
                class="text-xs px-1.5 py-0.5 bg-indigo-100 text-indigo-600 rounded"
              >
                {{ t('profile.lastUsed') }}
              </span>
              <Shield v-if="profile.contentEncryption" class="w-3.5 h-3.5 text-green-600" />
            </div>
            <div class="text-sm text-content-tertiary truncate">
              {{ profile.dbType.toUpperCase() }} · {{ profile.connectionInfo }}
            </div>
          </div>

          <!-- 操作菜单 -->
          <div class="opacity-0 group-hover:opacity-100 flex gap-1">
            <button
              @click.stop="editProfile(profile.id)"
              class="p-1.5 text-content-tertiary hover:text-indigo-600 hover:bg-indigo-50 rounded transition-colors"
              :title="t('profile.editProfile')"
            >
              <Pencil class="w-3.5 h-3.5" />
            </button>
            <button
              @click.stop="confirmDeleteProfile(profile)"
              class="p-1.5 text-content-tertiary hover:text-red-600 hover:bg-red-50 rounded transition-colors"
              :title="t('profile.deleteProfile')"
            >
              <Trash2 class="w-3.5 h-3.5" />
            </button>
          </div>
        </div>

        <!-- 新建配置按钮 -->
        <button
          @click="$emit('create')"
          class="w-full flex items-center justify-center gap-2 p-4 rounded-xl border-2 border-dashed border-edge text-content-secondary hover:border-indigo-400 hover:text-indigo-600 hover:bg-indigo-50/30 transition-all"
        >
          <Plus class="w-4 h-4" />
          <span>{{ t('profile.newProfile') }}</span>
        </button>
      </div>

      <!-- 底部 -->
      <div class="px-8 py-4 border-t border-edge flex items-center justify-between">
        <label class="flex items-center gap-2 text-sm text-content-secondary cursor-pointer">
          <input
            type="checkbox"
            v-model="autoConnect"
            @change="handleAutoConnectChange"
            class="w-4 h-4 rounded border-slate-300 text-indigo-600 focus:ring-indigo-500"
          />
          {{ t('profile.autoConnect') }}
        </label>

        <button
          @click="connectProfile(selectedId)"
          :disabled="!selectedId || connecting"
          class="px-5 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50"
        >
          {{ connecting ? t('profile.connecting') : t('profile.connect') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { ask } from '@tauri-apps/plugin-dialog'
import { profileApi } from '../api/note'
import { showNotification } from './ui/notification'
import { parseError } from '../utils/errorHandler'
import { availableLocales, setLocale, getCurrentLocale } from '../i18n'
import type { LocaleType } from '../i18n'
import type { ProfileSummary } from '../types'
import {
  Plus,
  Pencil,
  Trash2,
  Shield,
  HardDrive,
  Database,
  Server,
  Languages,
  X,
} from 'lucide-vue-next'

const props = defineProps<{
  /** 是否显示关闭按钮（从主界面进入时显示，首次启动时不显示） */
  showClose?: boolean
}>()

const emit = defineEmits<{
  (e: 'create'): void
  (e: 'edit', profileId: string): void
  (e: 'connected'): void
  (e: 'close'): void
}>()

const { t } = useI18n()

const currentLocale = ref<LocaleType>(getCurrentLocale())
const currentLocaleLabel = computed(
  () => availableLocales.find((l) => l.value === currentLocale.value)?.label ?? '',
)
const nextLocaleLabel = computed(() => {
  const next = availableLocales.find((l) => l.value !== currentLocale.value)
  return next?.label ?? ''
})
const toggleLang = () => {
  const next = currentLocale.value === 'zh-CN' ? 'en-US' : 'zh-CN'
  currentLocale.value = next as LocaleType
  setLocale(next as LocaleType)
}

const profiles = ref<ProfileSummary[]>([])
const selectedId = ref('')
const connecting = ref(false)
const autoConnect = ref(false)

const getDbIcon = (dbType: string) => {
  switch (dbType) {
    case 'mysql':
      return Database
    case 'postgres':
      return Server
    default:
      return HardDrive
  }
}

const loadProfiles = async () => {
  try {
    profiles.value = await profileApi.listProfiles()
    const index = await profileApi.getProfileIndex()
    autoConnect.value = index.autoConnect

    // 默认选中活跃的 profile
    const active = profiles.value.find((p) => p.isActive)
    if (active) {
      selectedId.value = active.id
    } else if (profiles.value.length > 0) {
      selectedId.value = profiles.value[0].id
    }
  } catch {
    // 静默
  }
}

const connectProfile = async (profileId: string) => {
  if (!profileId) return
  connecting.value = true
  try {
    // 标记本次为用户主动选择，重启后跳过选择页面
    localStorage.setItem('enote-skip-profile-select', '1')
    await profileApi.restartWithProfile(profileId)
    emit('connected')
  } catch (e: unknown) {
    localStorage.removeItem('enote-skip-profile-select')
    showNotification({ type: 'error', message: parseError(e) })
    connecting.value = false
  }
}

const editProfile = (profileId: string) => {
  emit('edit', profileId)
}

const confirmDeleteProfile = async (profile: ProfileSummary) => {
  const confirmed = await ask(t('profile.deleteConfirm', { name: profile.name }), {
    title: t('profile.deleteProfile'),
    kind: 'warning',
  })
  if (!confirmed) return

  try {
    await profileApi.deleteProfile(profile.id)
    await loadProfiles()
  } catch (e: unknown) {
    showNotification({ type: 'error', message: parseError(e) })
  }
}

const handleAutoConnectChange = async () => {
  try {
    await profileApi.setAutoConnect(autoConnect.value)
  } catch {
    // 静默
  }
}

onMounted(loadProfiles)
</script>
