/**
 * 导入功能类型定义
 */
import i18n from '../../i18n'

/** 导入来源类型 */
export type ImportSource = 'evernote' | 'youdao' | 'notion'

/** 导入来源配置 */
export interface ImportSourceConfig {
  id: ImportSource
  name: () => string
  description: () => string
  icon: string
  fileTypes: string[]
  accept: string
}

/** 导入的附件 */
export interface ImportedAttachment {
  filename: string
  mimeType: string
  data: string // base64 编码
}

/** 导入的笔记 */
export interface ImportedNote {
  title: string
  content: string
  contentType: 'html' | 'markdown'
  tags: string[]
  createTime?: string
  updateTime?: string
  attachments?: ImportedAttachment[]
}

/** 导入进度回调参数 */
export interface ImportProgress {
  current: number
  total: number
  currentTitle: string
  phase: 'parsing' | 'importing'
}

/** 导入进度回调函数 */
export type ImportProgressCallback = (progress: ImportProgress) => void

/** 导入结果 */
export interface ImportResult {
  success: number
  failed: number
  errors: string[]
  notes: ImportedNote[]
}

/** 导入选项 */
export interface ImportOptions {
  notebookId: number
  createTags?: boolean // 是否自动创建不存在的标签
  skipDuplicates?: boolean // 是否跳过重复笔记
  onProgress?: ImportProgressCallback
}

/** 导入来源配置列表 */
export const IMPORT_SOURCES: ImportSourceConfig[] = [
  {
    id: 'evernote',
    name: () => i18n.global.t('importSource.evernoteName'),
    description: () => i18n.global.t('importSource.evernoteDescription'),
    icon: 'FileText',
    fileTypes: ['enex'],
    accept: '.enex',
  },
  {
    id: 'youdao',
    name: () => i18n.global.t('importSource.youdaoName'),
    description: () => i18n.global.t('importSource.youdaoDescription'),
    icon: 'FileArchive',
    fileTypes: ['zip'],
    accept: '.zip',
  },
  {
    id: 'notion',
    name: () => i18n.global.t('importSource.notionName'),
    description: () => i18n.global.t('importSource.notionDescription'),
    icon: 'FileCode',
    fileTypes: ['zip'],
    accept: '.zip',
  },
]
