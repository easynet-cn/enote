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
  if (name.length > 50) {
    return { valid: false, error: '笔记本名称不能超过50个字符' }
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
  if (name.length > 30) {
    return { valid: false, error: '标签名称不能超过30个字符' }
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
  if (title.length > 200) {
    return { valid: false, error: '笔记标题不能超过200个字符' }
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
    console.warn(`Invalid ID: ${id}`)
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
