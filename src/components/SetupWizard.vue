<template>
  <div class="fixed inset-0 flex items-center justify-center bg-surface-alt">
    <div class="w-[680px] bg-surface rounded-2xl shadow-2xl border border-edge overflow-hidden">
      <!-- 标题 + 操作按钮 -->
      <div class="px-8 pt-8 pb-4 flex items-start justify-between">
        <div>
          <h1 class="text-2xl font-bold text-content">{{ t('setup.title') }}</h1>
          <p class="text-sm text-content-secondary mt-1">{{ t('setup.subtitle') }}</p>
        </div>
        <div class="flex items-center gap-2 flex-shrink-0">
          <!-- 返回配置列表 -->
          <button
            v-if="props.showBack"
            @click="emit('back')"
            class="flex items-center gap-1.5 px-3 py-1.5 text-sm text-content-secondary hover:text-content bg-surface-dim rounded-lg hover:bg-slate-200 transition-colors"
          >
            <ArrowLeft class="w-4 h-4" />
            <span>{{ t('profile.selectTitle') }}</span>
          </button>
          <!-- 语言切换 -->
          <button
            @click="toggleLang"
            class="flex items-center gap-1.5 px-3 py-1.5 text-sm text-content-secondary hover:text-content bg-surface-dim rounded-lg hover:bg-slate-200 transition-colors"
            :title="nextLocaleLabel"
          >
            <Languages class="w-4 h-4" />
            <span>{{ currentLocaleLabel }}</span>
          </button>
        </div>
      </div>

      <!-- 步骤指示器 -->
      <div class="px-8 pb-4">
        <div class="flex items-center gap-2">
          <template v-for="(step, idx) in steps" :key="idx">
            <div
              class="flex items-center gap-1.5 text-sm"
              :class="idx <= currentStep ? 'text-indigo-600 font-medium' : 'text-content-tertiary'"
            >
              <div
                class="w-6 h-6 rounded-full flex items-center justify-center text-xs font-bold"
                :class="
                  idx < currentStep
                    ? 'bg-indigo-600 text-white'
                    : idx === currentStep
                      ? 'bg-indigo-100 text-indigo-600 border-2 border-indigo-600'
                      : 'bg-surface-dim text-content-tertiary'
                "
              >
                <Check v-if="idx < currentStep" class="w-3.5 h-3.5" />
                <span v-else>{{ idx + 1 }}</span>
              </div>
              <span class="hidden sm:inline">{{ step }}</span>
            </div>
            <div
              v-if="idx < steps.length - 1"
              class="flex-1 h-px"
              :class="idx < currentStep ? 'bg-indigo-600' : 'bg-edge'"
            />
          </template>
        </div>
      </div>

      <!-- 步骤内容 -->
      <div class="px-8 py-4 min-h-[340px]">
        <!-- Step 0: 数据库类型 -->
        <div v-if="currentStep === 0" class="space-y-3">
          <div
            v-for="dbOpt in dbTypeOptions"
            :key="dbOpt.value"
            @click="form.datasource.type = dbOpt.value"
            class="flex items-center gap-4 p-4 rounded-xl border-2 cursor-pointer transition-all"
            :class="
              form.datasource.type === dbOpt.value
                ? 'border-indigo-600 bg-indigo-50/50'
                : 'border-edge hover:border-slate-300'
            "
          >
            <div
              class="w-10 h-10 rounded-lg flex items-center justify-center"
              :class="
                form.datasource.type === dbOpt.value
                  ? 'bg-indigo-100 text-indigo-600'
                  : 'bg-surface-dim text-content-secondary'
              "
            >
              <component :is="dbOpt.icon" class="w-5 h-5" />
            </div>
            <div class="flex-1">
              <div class="font-medium text-content">{{ dbOpt.label }}</div>
              <div class="text-sm text-content-secondary">{{ dbOpt.desc }}</div>
            </div>
            <div
              class="w-5 h-5 rounded-full border-2 flex items-center justify-center"
              :class="
                form.datasource.type === dbOpt.value ? 'border-indigo-600' : 'border-slate-300'
              "
            >
              <div
                v-if="form.datasource.type === dbOpt.value"
                class="w-2.5 h-2.5 rounded-full bg-indigo-600"
              />
            </div>
          </div>
        </div>

        <!-- Step 1: 连接配置 -->
        <div v-if="currentStep === 1" class="space-y-4">
          <!-- Profile 名称 -->
          <div>
            <label class="block text-sm font-medium text-content-secondary mb-1">
              {{ t('setup.profileName') }}
            </label>
            <input
              v-model="form.name"
              type="text"
              :placeholder="t('setup.profileNamePlaceholder')"
              class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
            />
          </div>

          <!-- SQLite 配置 -->
          <template v-if="form.datasource.type === 'sqlite'">
            <div>
              <label class="block text-sm font-medium text-content-secondary mb-1">
                {{ t('setup.dbPath') }}
              </label>
              <div class="flex gap-2">
                <input
                  v-model="form.datasource.path"
                  type="text"
                  :placeholder="t('setup.dbPathPlaceholder')"
                  class="flex-1 px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                />
                <button
                  @click="selectDbPathNew"
                  class="px-3 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors text-sm"
                  :title="t('setup.createNewDb')"
                >
                  {{ t('setup.createNewDb') }}
                </button>
                <button
                  @click="selectDbPathExisting"
                  class="px-3 py-2 bg-surface-dim text-content-secondary rounded-lg hover:bg-slate-200 transition-colors text-sm"
                  :title="t('setup.openExistingDb')"
                >
                  {{ t('setup.openExistingDb') }}
                </button>
              </div>
              <p class="text-xs text-content-tertiary mt-1">{{ t('setup.defaultPath') }}</p>
            </div>
          </template>

          <!-- MySQL / PostgreSQL 配置 -->
          <template v-if="form.datasource.type === 'mysql' || form.datasource.type === 'postgres'">
            <div class="grid grid-cols-3 gap-3">
              <div class="col-span-2">
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.host') }}
                </label>
                <input
                  v-model="form.datasource.host"
                  :placeholder="t('setup.hostPlaceholder')"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.port') }}
                </label>
                <input
                  v-model.number="form.datasource.port"
                  type="number"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-content-secondary mb-1">
                {{ t('setup.database') }}
              </label>
              <input
                v-model="form.datasource.database"
                :placeholder="t('setup.databasePlaceholder')"
                class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
              />
            </div>

            <!-- 认证方式 -->
            <div>
              <label class="block text-sm font-medium text-content-secondary mb-1">
                {{ t('setup.authMethod') }}
              </label>
              <div class="flex gap-1 bg-surface-dim rounded-lg p-1">
                <button
                  @click="form.datasource.authMethod = 'password'"
                  class="flex-1 px-3 py-1.5 text-sm rounded-md transition-colors"
                  :class="
                    form.datasource.authMethod === 'password'
                      ? 'bg-surface text-indigo-600 shadow-sm'
                      : 'text-content-secondary hover:text-content'
                  "
                >
                  {{ t('setup.authPassword') }}
                </button>
                <button
                  @click="form.datasource.authMethod = 'certificate'"
                  class="flex-1 px-3 py-1.5 text-sm rounded-md transition-colors"
                  :class="
                    form.datasource.authMethod === 'certificate'
                      ? 'bg-surface text-indigo-600 shadow-sm'
                      : 'text-content-secondary hover:text-content'
                  "
                >
                  {{ t('setup.authCertificate') }}
                </button>
              </div>
            </div>

            <!-- 密码认证 -->
            <template v-if="form.datasource.authMethod === 'password'">
              <div class="grid grid-cols-2 gap-3">
                <div>
                  <label class="block text-sm font-medium text-content-secondary mb-1">
                    {{ t('setup.username') }}
                  </label>
                  <input
                    v-model="form.datasource.username"
                    :placeholder="t('setup.usernamePlaceholder')"
                    class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-content-secondary mb-1">
                    {{ t('setup.password') }}
                  </label>
                  <input
                    v-model="dbPassword"
                    type="password"
                    :placeholder="t('setup.passwordPlaceholder')"
                    class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  />
                </div>
              </div>
            </template>

            <!-- 证书认证 -->
            <template v-if="form.datasource.authMethod === 'certificate'">
              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.username') }}
                </label>
                <input
                  v-model="form.datasource.username"
                  :placeholder="t('setup.usernamePlaceholder')"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                />
              </div>

              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.sslMode') }}
                </label>
                <select
                  v-model="form.datasource.ssl.mode"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                >
                  <option v-for="opt in sslModeOptions" :key="opt.value" :value="opt.value">
                    {{ opt.label }}
                  </option>
                </select>
              </div>

              <div class="space-y-2">
                <FileInput :label="t('setup.caCert')" v-model="form.datasource.ssl.caCert" />
                <FileInput
                  :label="t('setup.clientCert')"
                  v-model="form.datasource.ssl.clientCert"
                />
                <FileInput :label="t('setup.clientKey')" v-model="form.datasource.ssl.clientKey" />
              </div>
            </template>
          </template>
        </div>

        <!-- Step 2: 安全设置 -->
        <div v-if="currentStep === 2" class="space-y-6">
          <div
            class="p-5 rounded-xl border-2 transition-all"
            :class="
              form.security.contentEncryption ? 'border-indigo-600 bg-indigo-50/50' : 'border-edge'
            "
          >
            <div class="flex items-center justify-between mb-2">
              <div class="flex items-center gap-2">
                <Shield class="w-5 h-5 text-indigo-600" />
                <span class="font-medium text-content">{{ t('setup.contentEncryption') }}</span>
              </div>
              <button
                @click="toggleEncryption"
                class="relative w-11 h-6 rounded-full transition-colors"
                :class="form.security.contentEncryption ? 'bg-indigo-600' : 'bg-slate-300'"
              >
                <div
                  class="absolute top-0.5 w-5 h-5 bg-white rounded-full shadow-sm transition-transform"
                  :class="form.security.contentEncryption ? 'translate-x-5.5' : 'translate-x-0.5'"
                />
              </button>
            </div>
            <p class="text-sm text-content-secondary">{{ t('setup.contentEncryptionDesc') }}</p>

            <div v-if="form.security.contentEncryption" class="mt-4 space-y-3">
              <div
                class="flex items-center gap-2 p-3 bg-amber-50 text-amber-700 rounded-lg text-sm"
              >
                <AlertTriangle class="w-4 h-4 flex-shrink-0" />
                <span>{{ t('setup.contentEncryptionWarning') }}</span>
              </div>

              <div
                v-if="encryptionKeyGenerated"
                class="flex items-center gap-2 p-3 bg-green-50 text-green-700 rounded-lg text-sm"
              >
                <Check class="w-4 h-4 flex-shrink-0" />
                <span>{{ t('setup.keyGenerated') }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 3: 完成 -->
        <div v-if="currentStep === 3" class="flex flex-col items-center justify-center py-8">
          <div v-if="saving" class="flex flex-col items-center gap-4">
            <Loader2 class="w-8 h-8 text-indigo-600 animate-spin" />
            <p class="text-content-secondary">{{ t('setup.saving') }}</p>
          </div>
          <div v-else-if="saveError" class="flex flex-col items-center gap-4 text-center">
            <div class="w-12 h-12 bg-red-100 rounded-full flex items-center justify-center">
              <X class="w-6 h-6 text-red-600" />
            </div>
            <p class="text-red-600">{{ saveError }}</p>
            <button
              @click="currentStep = 1"
              class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
            >
              {{ t('setup.prev') }}
            </button>
          </div>
          <div v-else class="flex flex-col items-center gap-4">
            <div class="w-12 h-12 bg-green-100 rounded-full flex items-center justify-center">
              <Check class="w-6 h-6 text-green-600" />
            </div>
            <p class="text-content font-medium">{{ t('setup.testSuccess') }}</p>
          </div>
        </div>
      </div>

      <!-- 底部按钮 -->
      <div class="px-8 py-4 border-t border-edge flex justify-between">
        <button
          v-if="currentStep > 0 && currentStep < 3"
          @click="currentStep--"
          class="px-4 py-2 text-content-secondary hover:text-content transition-colors"
        >
          {{ t('setup.prev') }}
        </button>
        <div v-else />

        <div class="flex gap-2">
          <button
            v-if="currentStep === 1 && form.datasource.type !== 'sqlite'"
            @click="testConn"
            :disabled="testing"
            class="px-4 py-2 border border-edge text-content-secondary rounded-lg hover:bg-surface-dim transition-colors disabled:opacity-50"
          >
            {{ testing ? t('profile.connecting') : t('setup.testConnection') }}
          </button>
          <button
            v-if="currentStep < 2"
            @click="nextStep"
            :disabled="!canNext"
            class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50"
          >
            {{ t('setup.next') }}
          </button>
          <button
            v-if="currentStep === 2"
            @click="saveAndConnect"
            :disabled="saving"
            class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50"
          >
            {{ t('setup.saveAndConnect') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive } from 'vue'
import { useI18n } from 'vue-i18n'
import { open, save } from '@tauri-apps/plugin-dialog'
import { profileApi } from '../api/note'
import { showNotification } from './ui/notification'
import { parseError } from '../utils/errorHandler'
import { availableLocales, setLocale, getCurrentLocale } from '../i18n'
import type { LocaleType } from '../i18n'
import type { ProfileConfig } from '../types'
import {
  ArrowLeft,
  Check,
  Shield,
  AlertTriangle,
  X,
  Loader2,
  HardDrive,
  Database,
  Server,
  Languages,
} from 'lucide-vue-next'

const emit = defineEmits<{
  (e: 'complete'): void
  (e: 'back'): void
}>()

const props = defineProps<{
  editProfile?: ProfileConfig & { id?: string }
  /** 是否显示返回按钮（当已有 profile 时显示） */
  showBack?: boolean
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

const currentStep = ref(0)
const testing = ref(false)
const saving = ref(false)
const saveError = ref('')
const encryptionKeyGenerated = ref(false)
const encryptionKey = ref('')
const dbPassword = ref('')

const steps = computed(() => [
  t('setup.step.dbType'),
  t('setup.step.connection'),
  t('setup.step.security'),
  t('setup.step.finish'),
])

const form = reactive<ProfileConfig>({
  name: props.editProfile?.name || '',
  icon: props.editProfile?.icon || '',
  datasource: {
    type: props.editProfile?.datasource?.type || 'sqlite',
    path: props.editProfile?.datasource?.path || '',
    host: props.editProfile?.datasource?.host || 'localhost',
    port: props.editProfile?.datasource?.port || 3306,
    database: props.editProfile?.datasource?.database || '',
    username: props.editProfile?.datasource?.username || '',
    authMethod: props.editProfile?.datasource?.authMethod || 'password',
    ssl: {
      mode: props.editProfile?.datasource?.ssl?.mode || 'disabled',
      caCert: props.editProfile?.datasource?.ssl?.caCert || '',
      clientCert: props.editProfile?.datasource?.ssl?.clientCert || '',
      clientKey: props.editProfile?.datasource?.ssl?.clientKey || '',
    },
  },
  security: {
    contentEncryption: props.editProfile?.security?.contentEncryption || false,
  },
})

const dbTypeOptions = computed(() => [
  {
    value: 'sqlite',
    label: t('setup.dbType.sqlite'),
    desc: t('setup.dbType.sqliteDesc'),
    icon: HardDrive,
  },
  {
    value: 'mysql',
    label: t('setup.dbType.mysql'),
    desc: t('setup.dbType.mysqlDesc'),
    icon: Database,
  },
  {
    value: 'postgres',
    label: t('setup.dbType.postgres'),
    desc: t('setup.dbType.postgresDesc'),
    icon: Server,
  },
])

const sslModeOptions = computed(() => {
  const base = [
    { value: 'disabled', label: t('setup.sslModeDisabled') },
    { value: 'preferred', label: t('setup.sslModePreferred') },
    { value: 'required', label: t('setup.sslModeRequired') },
    { value: 'verify-ca', label: t('setup.sslModeVerifyCa') },
  ]
  if (form.datasource.type === 'mysql') {
    base.push({ value: 'verify-identity', label: t('setup.sslModeVerifyIdentity') })
  } else {
    base.push({ value: 'verify-full', label: t('setup.sslModeVerifyFull') })
  }
  return base
})

const canNext = computed(() => {
  if (currentStep.value === 0) return true
  if (currentStep.value === 1) {
    if (!form.name.trim()) return false
    if (form.datasource.type === 'sqlite') return true
    return !!(form.datasource.host && form.datasource.port && form.datasource.database)
  }
  return true
})

const nextStep = () => {
  if (currentStep.value === 0) {
    // 设置默认端口
    if (form.datasource.type === 'mysql' && form.datasource.port === 5432) {
      form.datasource.port = 3306
    } else if (form.datasource.type === 'postgres' && form.datasource.port === 3306) {
      form.datasource.port = 5432
    }
  }
  currentStep.value++
}

/** 新建数据库文件（save 对话框，选择保存位置） */
const selectDbPathNew = async () => {
  const path = await save({
    title: t('setup.createNewDb'),
    defaultPath: 'enote.db',
    filters: [{ name: 'SQLite', extensions: ['db'] }],
  })
  if (path) {
    form.datasource.path = path as string
  }
}

/** 打开已有数据库文件（open 对话框） */
const selectDbPathExisting = async () => {
  const path = await open({
    title: t('setup.openExistingDb'),
    filters: [{ name: 'SQLite', extensions: ['db', 'sqlite', 'sqlite3'] }],
  })
  if (path) {
    form.datasource.path = path as string
  }
}

const toggleEncryption = async () => {
  form.security.contentEncryption = !form.security.contentEncryption
  if (form.security.contentEncryption && !encryptionKeyGenerated.value) {
    try {
      encryptionKey.value = await profileApi.generateEncryptionKey()
      encryptionKeyGenerated.value = true
    } catch {
      form.security.contentEncryption = false
    }
  }
}

const testConn = async () => {
  testing.value = true
  try {
    await profileApi.testConnection(form, dbPassword.value || undefined)
    showNotification({ type: 'success', message: t('setup.testSuccess') })
  } catch (e: unknown) {
    showNotification({ type: 'error', message: `${t('setup.testFailed')}: ${parseError(e)}` })
  } finally {
    testing.value = false
  }
}

const saveAndConnect = async () => {
  saving.value = true
  saveError.value = ''
  currentStep.value = 3

  try {
    const profileId = props.editProfile?.id || nameToId(form.name) || `profile-${Date.now()}`

    await profileApi.saveProfile(
      profileId,
      form,
      dbPassword.value || undefined,
      form.security.contentEncryption ? encryptionKey.value : undefined,
    )

    // 热连接新 profile（不重启进程）
    await profileApi.reconnectProfile(profileId)

    emit('complete')
  } catch (e: unknown) {
    saveError.value = parseError(e)
    saving.value = false
  }
}

const nameToId = (name: string) =>
  name
    .toLowerCase()
    .replace(/[^a-z0-9-_]/g, '-')
    .replace(/-+/g, '-')
    .replace(/^-|-$/g, '')

// 文件输入辅助组件（内联）
const FileInput = {
  props: ['label', 'modelValue'],
  emits: ['update:modelValue'],
  template: `
    <div class="flex items-center gap-2">
      <label class="text-sm text-content-secondary w-24 flex-shrink-0">{{ label }}</label>
      <input
        :value="modelValue"
        @input="$emit('update:modelValue', $event.target.value)"
        class="flex-1 px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
        placeholder="..."
      />
      <button
        @click="selectFile"
        class="px-2 py-1.5 text-xs bg-surface-dim text-content-secondary rounded hover:bg-slate-200 transition-colors"
      >
        ...
      </button>
    </div>
  `,
  setup(
    props: { label: string; modelValue: string },
    { emit }: { emit: (event: string, ...args: unknown[]) => void },
  ) {
    const selectFile = async () => {
      const path = await open({
        title: props.label,
        filters: [{ name: 'Certificate', extensions: ['pem', 'crt', 'key'] }],
      })
      if (path) {
        emit('update:modelValue', path)
      }
    }
    return { selectFile }
  },
}
</script>
