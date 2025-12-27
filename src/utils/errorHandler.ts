import { showNotification } from '../components/ui/notification'

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
 * 错误消息映射
 */
const ErrorMessages: Record<AppErrorCode, string> = {
  [AppErrorCode.NETWORK_ERROR]: '网络连接失败，请检查您的网络',
  [AppErrorCode.TIMEOUT_ERROR]: '请求超时，请稍后重试',
  [AppErrorCode.NOT_FOUND]: '请求的资源不存在',
  [AppErrorCode.PERMISSION_DENIED]: '没有权限执行此操作',
  [AppErrorCode.VALIDATION_ERROR]: '输入数据验证失败',
  [AppErrorCode.DATABASE_ERROR]: '数据库操作失败',
  [AppErrorCode.UNKNOWN_ERROR]: '操作失败，请稍后重试',
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
 * 解析错误为结构化类型
 */
export const parseErrorToAppError = (error: unknown): AppError => {
  // 处理 Tauri 后端返回的结构化错误
  if (error && typeof error === 'object' && 'code' in error) {
    const structuredError = error as { code: string; message?: string }
    const code = (structuredError.code as AppErrorCode) || AppErrorCode.UNKNOWN_ERROR
    return {
      code,
      message: structuredError.message || ErrorMessages[code],
      retryable: RETRYABLE_ERRORS.has(code),
    }
  }

  if (error instanceof Error) {
    const msg = error.message.toLowerCase()

    // 检查网络错误
    if (msg.includes('fetch') || msg.includes('network') || msg.includes('connection')) {
      return {
        code: AppErrorCode.NETWORK_ERROR,
        message: ErrorMessages[AppErrorCode.NETWORK_ERROR],
        details: error.message,
        retryable: true,
      }
    }

    // 检查超时
    if (msg.includes('timeout')) {
      return {
        code: AppErrorCode.TIMEOUT_ERROR,
        message: ErrorMessages[AppErrorCode.TIMEOUT_ERROR],
        details: error.message,
        retryable: true,
      }
    }

    // 检查数据库错误
    if (msg.includes('database') || msg.includes('sql') || msg.includes('db')) {
      return {
        code: AppErrorCode.DATABASE_ERROR,
        message: ErrorMessages[AppErrorCode.DATABASE_ERROR],
        details: error.message,
        retryable: true,
      }
    }

    // 不暴露包含路径信息的错误
    if (msg.includes('/') || msg.includes('\\')) {
      return {
        code: AppErrorCode.UNKNOWN_ERROR,
        message: ErrorMessages[AppErrorCode.UNKNOWN_ERROR],
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
    message: ErrorMessages[AppErrorCode.UNKNOWN_ERROR],
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
