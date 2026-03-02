import {
  MAX_NOTE_CONTENT_SIZE,
  MAX_NOTE_TITLE_LENGTH,
  MAX_NOTEBOOK_NAME_LENGTH,
  MAX_TAG_NAME_LENGTH,
} from '../config/constants'
import i18n from '../i18n'

/**
 * 验证结果类型
 */
export interface ValidationResult {
  valid: boolean
  error?: string
}

const t = i18n.global.t

/**
 * 验证笔记本名称
 */
export const validateNotebookName = (name: string | undefined): ValidationResult => {
  if (!name?.trim()) {
    return { valid: false, error: t('validationError.notebookNameRequired') }
  }
  if (name.length > MAX_NOTEBOOK_NAME_LENGTH) {
    return {
      valid: false,
      error: t('validationError.notebookNameTooLong', { max: MAX_NOTEBOOK_NAME_LENGTH }),
    }
  }
  return { valid: true }
}

/**
 * 验证标签名称
 */
export const validateTagName = (name: string | undefined): ValidationResult => {
  if (!name?.trim()) {
    return { valid: false, error: t('validationError.tagNameRequired') }
  }
  if (name.length > MAX_TAG_NAME_LENGTH) {
    return {
      valid: false,
      error: t('validationError.tagNameTooLong', { max: MAX_TAG_NAME_LENGTH }),
    }
  }
  return { valid: true }
}

/**
 * 验证笔记标题
 */
export const validateNoteTitle = (title: string | undefined): ValidationResult => {
  if (!title?.trim()) {
    return { valid: false, error: t('validationError.noteTitleRequired') }
  }
  if (title.length > MAX_NOTE_TITLE_LENGTH) {
    return {
      valid: false,
      error: t('validationError.noteTitleTooLong', { max: MAX_NOTE_TITLE_LENGTH }),
    }
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
    return { valid: false, error: t('validationError.urlRequired') }
  }
  try {
    new URL(url)
    return { valid: true }
  } catch {
    return { valid: false, error: t('validationError.urlInvalid') }
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
      error: t('validationError.noteContentTooLarge', { size: sizeMB }),
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
