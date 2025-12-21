import { showNotification } from '../components/ui/notification'

/**
 * 错误消息映射
 */
const ErrorMessages: Record<string, string> = {
  NETWORK_ERROR: '网络连接失败，请检查您的网络',
  TIMEOUT_ERROR: '请求超时，请稍后重试',
  NOT_FOUND: '请求的资源不存在',
  PERMISSION_DENIED: '没有权限执行此操作',
  VALIDATION_ERROR: '输入数据验证失败',
  UNKNOWN_ERROR: '操作失败，请稍后重试',
}

/**
 * 解析错误信息
 */
export const parseError = (error: unknown): string => {
  if (error instanceof Error) {
    // 检查是否为网络错误
    if (error.message.includes('fetch') || error.message.includes('network')) {
      return ErrorMessages.NETWORK_ERROR
    }
    // 检查是否为超时
    if (error.message.includes('timeout')) {
      return ErrorMessages.TIMEOUT_ERROR
    }
    // 不暴露包含路径信息的错误
    if (error.message.includes('/') || error.message.includes('\\')) {
      return ErrorMessages.UNKNOWN_ERROR
    }
    return error.message
  }

  if (typeof error === 'string') {
    return error
  }

  return ErrorMessages.UNKNOWN_ERROR
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
 * 带通知的异步操作包装器
 */
export const withNotification = async <T>(
  fn: () => Promise<T>,
  options: {
    loading?: string
    success?: string
    error?: string
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
    const result = await fn()

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
