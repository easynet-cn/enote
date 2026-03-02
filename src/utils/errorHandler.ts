import { showNotification } from '../components/ui/notification'
import i18n from '../i18n'

/**
 * 应用错误类型
 */
export enum AppErrorCode {
  NETWORK_ERROR = 'NETWORK_ERROR',
  TIMEOUT_ERROR = 'TIMEOUT_ERROR',
  NOT_FOUND = 'NOT_FOUND',
  PERMISSION_DENIED = 'PERMISSION_DENIED',
  VALIDATION_ERROR = 'VALIDATION_ERROR',
  DATABASE_ERROR = 'DATABASE_ERROR',
  UNKNOWN_ERROR = 'UNKNOWN_ERROR',
  BUSINESS_ERROR = 'BUSINESS_ERROR',
  CONFIG_ERROR = 'CONFIG_ERROR',
  INTERNAL_ERROR = 'INTERNAL_ERROR',
}

/**
 * 结构化应用错误
 */
export interface AppError {
  code: AppErrorCode
  message: string
  details?: string
  retryable: boolean
}

/**
 * 后端错误响应结构
 */
export interface BackendErrorResponse {
  code: string
  message: string
  details?: string
}

/**
 * 可重试的错误类型
 */
const RETRYABLE_ERRORS = new Set([
  AppErrorCode.NETWORK_ERROR,
  AppErrorCode.TIMEOUT_ERROR,
  AppErrorCode.DATABASE_ERROR,
])

/**
 * 将后端错误码映射到前端错误码
 */
const mapBackendErrorCode = (backendCode: string): AppErrorCode => {
  const codeMap: Record<string, AppErrorCode> = {
    DATABASE_ERROR: AppErrorCode.DATABASE_ERROR,
    NOT_FOUND: AppErrorCode.NOT_FOUND,
    VALIDATION_ERROR: AppErrorCode.VALIDATION_ERROR,
    BUSINESS_ERROR: AppErrorCode.BUSINESS_ERROR,
    CONFIG_ERROR: AppErrorCode.CONFIG_ERROR,
    INTERNAL_ERROR: AppErrorCode.INTERNAL_ERROR,
  }
  return codeMap[backendCode] || AppErrorCode.UNKNOWN_ERROR
}

/**
 * 根据错误码获取本地化的错误消息
 */
const getLocalizedErrorMessage = (code: AppErrorCode, fallbackMessage?: string): string => {
  // 优先使用后端消息（作为备选）
  if (fallbackMessage) {
    return fallbackMessage
  }

  // 使用 i18n 实例获取翻译
  const locale = i18n.global.locale.value
  const messages = i18n.global.messages.value[locale]
  const errorMessages = (messages?.errorCodes ?? {}) as Record<string, string>
  return errorMessages[code] || errorMessages.UNKNOWN_ERROR || 'Unknown error'
}

/**
 * 解析错误为结构化类型
 */
export const parseErrorToAppError = (error: unknown): AppError => {
  // 处理 Tauri 后端返回的结构化错误
  if (error && typeof error === 'object' && 'code' in error) {
    const structuredError = error as BackendErrorResponse
    const frontendCode = mapBackendErrorCode(structuredError.code)
    return {
      code: frontendCode,
      message: getLocalizedErrorMessage(frontendCode, structuredError.message),
      details: structuredError.details,
      retryable: RETRYABLE_ERRORS.has(frontendCode),
    }
  }

  if (error instanceof Error) {
    const msg = error.message.toLowerCase()

    // 检查网络错误
    if (msg.includes('fetch') || msg.includes('network') || msg.includes('connection')) {
      return {
        code: AppErrorCode.NETWORK_ERROR,
        message: getLocalizedErrorMessage(AppErrorCode.NETWORK_ERROR),
        details: error.message,
        retryable: true,
      }
    }

    // 检查超时
    if (msg.includes('timeout')) {
      return {
        code: AppErrorCode.TIMEOUT_ERROR,
        message: getLocalizedErrorMessage(AppErrorCode.TIMEOUT_ERROR),
        details: error.message,
        retryable: true,
      }
    }

    // 检查数据库错误
    if (msg.includes('database') || msg.includes('sql') || msg.includes('db')) {
      return {
        code: AppErrorCode.DATABASE_ERROR,
        message: getLocalizedErrorMessage(AppErrorCode.DATABASE_ERROR),
        details: error.message,
        retryable: true,
      }
    }

    // 不暴露包含路径信息的错误
    if (msg.includes('/') || msg.includes('\\')) {
      return {
        code: AppErrorCode.UNKNOWN_ERROR,
        message: getLocalizedErrorMessage(AppErrorCode.UNKNOWN_ERROR),
        retryable: false,
      }
    }

    return {
      code: AppErrorCode.UNKNOWN_ERROR,
      message: error.message,
      retryable: false,
    }
  }

  if (typeof error === 'string') {
    return {
      code: AppErrorCode.UNKNOWN_ERROR,
      message: error,
      retryable: false,
    }
  }

  return {
    code: AppErrorCode.UNKNOWN_ERROR,
    message: getLocalizedErrorMessage(AppErrorCode.UNKNOWN_ERROR),
    retryable: false,
  }
}

/**
 * 解析错误信息（简化版，返回字符串）
 */
export const parseError = (error: unknown): string => {
  return parseErrorToAppError(error).message
}

/**
 * 显示错误通知
 */
export const showError = (error: unknown, fallbackMessage?: string): void => {
  const message = fallbackMessage || parseError(error)
  showNotification({
    message,
    type: 'error',
    duration: 5000,
  })
}

/**
 * 显示成功通知
 */
export const showSuccess = (message: string): void => {
  showNotification({
    message,
    type: 'success',
    duration: 3000,
  })
}

/**
 * 延迟函数
 */
const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

/**
 * 带自动重试的异步操作
 */
export const withRetry = async <T>(
  fn: () => Promise<T>,
  options: {
    maxRetries?: number
    delayMs?: number
    shouldRetry?: (error: AppError) => boolean
  } = {},
): Promise<T> => {
  const { maxRetries = 3, delayMs = 1000, shouldRetry } = options

  let lastError: unknown

  for (let attempt = 0; attempt < maxRetries; attempt++) {
    try {
      return await fn()
    } catch (error) {
      lastError = error
      const appError = parseErrorToAppError(error)

      // 检查是否应该重试
      const canRetry = shouldRetry ? shouldRetry(appError) : appError.retryable

      if (!canRetry || attempt === maxRetries - 1) {
        throw error
      }

      // 指数退避
      await delay(delayMs * Math.pow(2, attempt))
    }
  }

  throw lastError
}

/**
 * 带通知的异步操作包装器
 */
export const withNotification = async <T>(
  fn: () => Promise<T>,
  options: {
    loading?: string
    success?: string
    error?: string
    retry?: boolean
    maxRetries?: number
  } = {},
): Promise<T | null> => {
  let loadingNotification: { close: () => void } | null = null

  if (options.loading) {
    loadingNotification = showNotification({
      message: options.loading,
      type: 'success',
      duration: 0,
    })
  }

  try {
    let result: T

    if (options.retry) {
      result = await withRetry(fn, { maxRetries: options.maxRetries })
    } else {
      result = await fn()
    }

    if (options.success) {
      showSuccess(options.success)
    }

    return result
  } catch (error) {
    showError(error, options.error)
    return null
  } finally {
    loadingNotification?.close()
  }
}
