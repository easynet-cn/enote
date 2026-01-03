/**
 * 统一导入接口
 */

import { open } from '@tauri-apps/plugin-dialog'
import { readFile, readTextFile } from '@tauri-apps/plugin-fs'
import { parseEvernoteEnex } from './evernote'
import { parseYoudaoZip } from './youdao'
import { parseNotionZip } from './notion'
import type { ImportSource, ImportResult, ImportProgressCallback } from './types'

export * from './types'

/**
 * 打开文件选择对话框
 * @param source 导入来源
 * @returns 选中的文件路径，取消返回 null
 */
export async function selectImportFile(source: ImportSource): Promise<string | null> {
  const sourceConfig = (await import('./types')).IMPORT_SOURCES.find((s) => s.id === source)
  if (!sourceConfig) return null

  const result = await open({
    title: `选择 ${sourceConfig.name} 导出文件`,
    filters: [
      {
        name: sourceConfig.name,
        extensions: sourceConfig.fileTypes,
      },
    ],
    multiple: false,
  })

  return result as string | null
}

/**
 * 解析导入文件
 * @param source 导入来源
 * @param filePath 文件路径
 * @param onProgress 进度回调
 * @returns 导入结果
 */
export async function parseImportFile(
  source: ImportSource,
  filePath: string,
  onProgress?: ImportProgressCallback,
): Promise<ImportResult> {
  switch (source) {
    case 'evernote': {
      // ENEX 是 XML 文本文件
      const content = await readTextFile(filePath)
      return parseEvernoteEnex(content, onProgress)
    }

    case 'youdao': {
      // 有道笔记是 ZIP 文件
      const data = await readFile(filePath)
      return parseYoudaoZip(data.buffer as ArrayBuffer, onProgress)
    }

    case 'notion': {
      // Notion 是 ZIP 文件
      const data = await readFile(filePath)
      return parseNotionZip(data.buffer as ArrayBuffer, onProgress)
    }

    default:
      return {
        success: 0,
        failed: 0,
        errors: [`不支持的导入来源: ${source}`],
        notes: [],
      }
  }
}

/**
 * 一站式导入流程
 * @param source 导入来源
 * @param onProgress 进度回调
 * @returns 导入结果，用户取消返回 null
 */
export async function importFromSource(
  source: ImportSource,
  onProgress?: ImportProgressCallback,
): Promise<ImportResult | null> {
  // 1. 选择文件
  const filePath = await selectImportFile(source)
  if (!filePath) return null

  // 2. 解析文件
  return parseImportFile(source, filePath, onProgress)
}

/**
 * 获取导入来源的显示信息
 */
export { IMPORT_SOURCES } from './types'
