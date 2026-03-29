import { invoke } from '@tauri-apps/api/core'
import { parseErrorToAppError } from './errorHandler'

/**
 * 带自动重试的 Tauri invoke 封装
 *
 * 对可重试错误（数据库、网络、超时）自动进行指数退避重试。
 * 业务错误和验证错误不会重试。
 */
export async function invokeWithRetry<T>(
  cmd: string,
  args?: Record<string, unknown>,
  options: { maxRetries?: number; baseDelay?: number } = {},
): Promise<T> {
  const { maxRetries = 2, baseDelay = 500 } = options
  let lastError: unknown

  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      return await invoke<T>(cmd, args)
    } catch (error) {
      lastError = error

      // 最后一次重试也失败了，直接抛出
      if (attempt === maxRetries) break

      // 仅对可重试错误进行重试
      const appError = parseErrorToAppError(error)
      if (!appError.retryable) break

      // 指数退避：500ms → 1000ms → 2000ms
      await new Promise((r) => setTimeout(r, baseDelay * Math.pow(2, attempt)))
    }
  }

  throw lastError
}
