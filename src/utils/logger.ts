/**
 * 前端日志工具
 *
 * 替代 console.log/warn/error，根据配置的日志级别过滤，
 * 同时将日志通过 invoke 发送到后端数据库记录。
 *
 * 使用方法：
 *   import { logger } from '@/utils/logger'
 *   logger.info('module', 'action', 'message')
 *   logger.error('module', 'action', 'message', detail?)
 */

import { invoke } from '@tauri-apps/api/core'

/** 日志级别优先级 */
const LOG_LEVELS = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3,
  none: 4,
} as const

type LogLevel = keyof typeof LOG_LEVELS

/** 当前日志级别，默认 info，可通过 setLevel 动态修改 */
let currentLevel: LogLevel = 'info'

/** 是否已初始化（避免在 DB 未连接时发送日志） */
let initialized = false

/** 设置日志级别 */
export function setLogLevel(level: string) {
  if (level in LOG_LEVELS) {
    currentLevel = level as LogLevel
  }
}

/** 标记已初始化（DB 已连接后调用） */
export function initLogger(level?: string) {
  initialized = true
  if (level) {
    setLogLevel(level)
  }
}

/** 检查是否应该记录该级别日志 */
function shouldLog(level: LogLevel): boolean {
  return LOG_LEVELS[level] >= LOG_LEVELS[currentLevel]
}

/** 发送日志到后端 */
async function sendLog(
  level: string,
  module: string,
  action: string,
  message: string,
  detail?: string,
) {
  if (!initialized) return
  try {
    await invoke('write_frontend_log', {
      log: {
        id: 0,
        level: level.toUpperCase(),
        module,
        action,
        targetId: null,
        targetName: null,
        message,
        detail: detail ?? null,
        createTime: null,
      },
    })
  } catch {
    // 日志写入失败不应影响业务
  }
}

export const logger = {
  debug(module: string, action: string, message: string, detail?: string) {
    if (!shouldLog('debug')) return
    sendLog('DEBUG', module, action, message, detail)
  },

  info(module: string, action: string, message: string, detail?: string) {
    if (!shouldLog('info')) return
    sendLog('INFO', module, action, message, detail)
  },

  warn(module: string, action: string, message: string, detail?: string) {
    if (!shouldLog('warn')) return
    sendLog('WARN', module, action, message, detail)
  },

  error(module: string, action: string, message: string, detail?: string) {
    if (!shouldLog('error')) return
    sendLog('ERROR', module, action, message, detail)
  },
}

export default logger
