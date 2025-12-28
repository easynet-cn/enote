import {
  MAX_NOTE_CONTENT_SIZE,
  MAX_NOTE_TITLE_LENGTH,
  MAX_NOTEBOOK_NAME_LENGTH,
  MAX_TAG_NAME_LENGTH,
} from '../config/constants'

/**
 * 验证结果类型
 */
export interface ValidationResult {
  valid: boolean
  error?: string
}

/**
 * 验证笔记本名称
 */
export const validateNotebookName = (name: string | undefined): ValidationResult => {
  if (!name?.trim()) {
    return { valid: false, error: '笔记本名称不能为空' }
  }
  if (name.length > MAX_NOTEBOOK_NAME_LENGTH) {
    return { valid: false, error: `笔记本名称不能超过${MAX_NOTEBOOK_NAME_LENGTH}个字符` }
  }
  return { valid: true }
}

/**
 * 验证标签名称
 */
export const validateTagName = (name: string | undefined): ValidationResult => {
  if (!name?.trim()) {
    return { valid: false, error: '标签名称不能为空' }
  }
  if (name.length > MAX_TAG_NAME_LENGTH) {
    return { valid: false, error: `标签名称不能超过${MAX_TAG_NAME_LENGTH}个字符` }
  }
  return { valid: true }
}

/**
 * 验证笔记标题
 */
export const validateNoteTitle = (title: string | undefined): ValidationResult => {
  if (!title?.trim()) {
    return { valid: false, error: '笔记标题不能为空' }
  }
  if (title.length > MAX_NOTE_TITLE_LENGTH) {
    return { valid: false, error: `笔记标题不能超过${MAX_NOTE_TITLE_LENGTH}个字符` }
  }
  return { valid: true }
}

/**
 * 安全解析 ID
 */
export const parseId = (id: string | undefined): number => {
  if (!id || id === '0') return 0

  // 处理临时 ID（格式：0-timestamp）
  if (id.startsWith('0-')) return 0

  const parsed = Number.parseInt(id, 10)

  if (isNaN(parsed) || parsed < 0) {
    return 0
  }

  return parsed
}

/**
 * 检查是否为新建的临时 ID
 */
export const isTemporaryId = (id: string | undefined): boolean => {
  if (!id) return true
  return id === '0' || id.startsWith('0-')
}

/**
 * 验证 URL
 */
export const validateUrl = (url: string | undefined): ValidationResult => {
  if (!url?.trim()) {
    return { valid: false, error: 'URL 不能为空' }
  }
  try {
    new URL(url)
    return { valid: true }
  } catch {
    return { valid: false, error: '请输入有效的 URL' }
  }
}

// 重新导出 MAX_NOTE_CONTENT_SIZE 以保持向后兼容
export { MAX_NOTE_CONTENT_SIZE } from '../config/constants'

/**
 * 验证笔记内容大小
 */
export const validateNoteContent = (
  content: string | undefined,
  maxSize: number = MAX_NOTE_CONTENT_SIZE,
): ValidationResult => {
  if (!content) {
    return { valid: true }
  }

  // 计算字节大小（UTF-8 编码）
  const byteSize = new Blob([content]).size

  if (byteSize > maxSize) {
    const sizeMB = (maxSize / 1024 / 1024).toFixed(1)
    return {
      valid: false,
      error: `笔记内容超过大小限制（最大 ${sizeMB}MB）`,
    }
  }

  return { valid: true }
}

/**
 * 格式化字节大小为可读字符串
 */
export const formatBytes = (bytes: number): string => {
  if (bytes === 0) return '0 B'

  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))

  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}
