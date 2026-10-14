import { computed, ref } from 'vue'

/** 当前活跃 profile 的后端类型 */
const activeBackendType = ref<string>('database')

/**
 * 后端能力感知 composable
 *
 * 根据当前 profile 的后端类型（database / server / git），
 * 判断哪些功能可用，供 UI 组件动态显示/隐藏功能。
 *
 * 能力对照表：
 * | 功能                  | database | server | git  |
 * |-----------------------|----------|--------|------|
 * | 三格式备份/恢复        |    ✓    |   ✗   |  ✗  |
 * | 跨 Profile 同步       |    ✓    |   ✗   |  ✗  |
 * | 客户端内容加密         |    ✓    |   ✗   |  ✓  |
 * | FTS 搜索配置          |    ✓    |   ✗   |  ✗  |
 * | 云备份                |    ✓    |   ✗   |  ✓  |
 */
export function useBackendCapabilities() {
  const isServerBackend = computed(() => activeBackendType.value === 'server')
  const isGitBackend = computed(() => activeBackendType.value === 'git')
  const isDatabaseBackend = computed(() => activeBackendType.value === 'database')

  /** 是否支持数据库备份/恢复（三格式 SQL/Excel/CSV） */
  const supportsBackup = computed(() => isDatabaseBackend.value)

  /** 是否支持跨 Profile 同步 */
  const supportsSync = computed(() => isDatabaseBackend.value)

  /** 是否支持客户端内容加密 */
  const supportsClientEncryption = computed(() => !isServerBackend.value)

  /** 是否支持 FTS 搜索配置 */
  const supportsFtsConfig = computed(() => isDatabaseBackend.value)

  /** 设置当前后端类型（在 profile 切换时调用） */
  function setBackendType(type: string) {
    activeBackendType.value = type || 'database'
  }

  return {
    isServerBackend,
    isGitBackend,
    isDatabaseBackend,
    supportsBackup,
    supportsSync,
    supportsClientEncryption,
    supportsFtsConfig,
    setBackendType,
  }
}
