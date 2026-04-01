import { describe, it, expect } from 'vitest'
import { parseError, parseErrorToAppError, AppErrorCode } from '../utils/errorHandler'

describe('errorHandler utils', () => {
  describe('parseErrorToAppError', () => {
    it('should parse network errors', () => {
      const error = new Error('Failed to fetch data')
      const result = parseErrorToAppError(error)

      expect(result.code).toBe(AppErrorCode.NETWORK_ERROR)
      expect(result.retryable).toBe(true)
    })

    it('should parse timeout errors', () => {
      const error = new Error('Request timeout')
      const result = parseErrorToAppError(error)

      expect(result.code).toBe(AppErrorCode.TIMEOUT_ERROR)
      expect(result.retryable).toBe(true)
    })

    it('should parse database errors', () => {
      const error = new Error('SQL query failed in database')
      const result = parseErrorToAppError(error)

      expect(result.code).toBe(AppErrorCode.DATABASE_ERROR)
      expect(result.retryable).toBe(true)
    })

    it('should handle structured errors from backend', () => {
      const error = { code: 'NOT_FOUND', message: 'Note not found' }
      const result = parseErrorToAppError(error)

      expect(result.code).toBe(AppErrorCode.NOT_FOUND)
      // message 会被 i18n 本地化，只需确保非空
      expect(result.message).toBeTruthy()
    })

    it('should handle string errors', () => {
      const result = parseErrorToAppError('Something went wrong')

      expect(result.code).toBe(AppErrorCode.UNKNOWN_ERROR)
      expect(result.message).toBe('Something went wrong')
      expect(result.retryable).toBe(false)
    })

    it('should hide path information for security', () => {
      const error = new Error('File not found: /usr/local/data/secret.txt')
      const result = parseErrorToAppError(error)

      expect(result.code).toBe(AppErrorCode.UNKNOWN_ERROR)
      expect(result.message).not.toContain('/usr/local')
    })
  })

  describe('parseError', () => {
    it('should return string message for errors', () => {
      expect(parseError(new Error('Test error'))).toBe('Test error')
      expect(parseError('Simple error')).toBe('Simple error')
    })

    it('should return default message for unknown types', () => {
      // 默认消息取决于 i18n 语言，只需确保返回非空字符串
      expect(parseError(null)).toBeTruthy()
      expect(parseError(undefined)).toBeTruthy()
      expect(parseError(123)).toBeTruthy()
      // 三者返回相同的默认消息
      expect(parseError(null)).toBe(parseError(undefined))
      expect(parseError(null)).toBe(parseError(123))
    })
  })
})
