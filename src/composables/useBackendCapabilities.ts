import { computed, ref } from 'vue'

/** 当前活跃 profile 的后端类型 */
const activeBackendType = ref<string>('database')

/**
 * 后端能力感知 composable
 *
 * 根据当前 profile 的后端类型（database / server），
 * 判断哪些功能可用，供 UI 组件动态显示/隐藏功能。
 */
export function useBackendCapabilities() {
  const isServerBackend = computed(() => activeBackendType.value === 'server')

  /** 是否支持数据库备份/恢复 */
  const supportsBackup = computed(() => !isServerBackend.value)

  /** 是否支持跨 Profile 同步 */
  const supportsSync = computed(() => !isServerBackend.value)

  /** 是否支持客户端内容加密 */
  const supportsClientEncryption = computed(() => !isServerBackend.value)

  /** 是否支持 FTS 搜索配置 */
  const supportsFtsConfig = computed(() => !isServerBackend.value)

  /** 设置当前后端类型（在 profile 切换时调用） */
  function setBackendType(type: string) {
    activeBackendType.value = type || 'database'
  }

  return {
    isServerBackend,
    supportsBackup,
    supportsSync,
    supportsClientEncryption,
    supportsFtsConfig,
    setBackendType,
  }
}
