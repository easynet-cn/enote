/**
 * 有道笔记 ZIP 格式导入解析器
 *
 * 有道笔记导出为 ZIP 压缩包，包含 HTML 或 Markdown 文件
 * 目录结构:
 * - note1/
 *   - note.html 或 note.md
 *   - attachments/
 *     - image.png
 * - note2/
 *   - ...
 */

import JSZip from 'jszip'
import type { ImportedNote, ImportResult, ImportProgressCallback } from './types'

/** 获取文件的 MIME 类型 */
function getMimeType(filename: string): string {
  const ext = filename.split('.').pop()?.toLowerCase() || ''
  const mimeTypes: Record<string, string> = {
    png: 'image/png',
    jpg: 'image/jpeg',
    jpeg: 'image/jpeg',
    gif: 'image/gif',
    webp: 'image/webp',
    svg: 'image/svg+xml',
    pdf: 'application/pdf',
    doc: 'application/msword',
    docx: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  }
  return mimeTypes[ext] || 'application/octet-stream'
}

/** 将图片路径替换为 base64 内嵌 */
async function embedImages(content: string, basePath: string, zip: JSZip): Promise<string> {
  // 匹配 HTML 中的图片标签
  const imgRegex = /<img[^>]*src=["']([^"']+)["'][^>]*>/gi
  let result = content
  let match

  while ((match = imgRegex.exec(content)) !== null) {
    const [, src] = match

    // 跳过已经是 base64 的图片
    if (src.startsWith('data:')) continue

    // 构建完整路径
    let imagePath = src
    if (!src.startsWith('/')) {
      imagePath = basePath + '/' + src
    }
    // 移除开头的斜杠
    imagePath = imagePath.replace(/^\/+/, '')

    // 尝试从 ZIP 中读取图片
    const imageFile = zip.file(imagePath)
    if (imageFile) {
      try {
        const imageData = await imageFile.async('base64')
        const mimeType = getMimeType(imagePath)
        const newSrc = `data:${mimeType};base64,${imageData}`
        result = result.replace(src, newSrc)
      } catch (error) {
        console.warn(`无法读取图片: ${imagePath}`, error)
      }
    }
  }

  return result
}

/** 将 Markdown 中的图片路径替换为 base64 内嵌 */
async function embedMarkdownImages(content: string, basePath: string, zip: JSZip): Promise<string> {
  // 匹配 Markdown 中的图片语法 ![alt](path)
  const imgRegex = /!\[([^\]]*)\]\(([^)]+)\)/g
  let result = content
  let match

  while ((match = imgRegex.exec(content)) !== null) {
    const [fullMatch, alt, src] = match

    // 跳过已经是 base64 或 URL 的图片
    if (src.startsWith('data:') || src.startsWith('http')) continue

    // 构建完整路径
    let imagePath = src
    if (!src.startsWith('/')) {
      imagePath = basePath + '/' + src
    }
    imagePath = imagePath.replace(/^\/+/, '')

    // 尝试从 ZIP 中读取图片
    const imageFile = zip.file(imagePath)
    if (imageFile) {
      try {
        const imageData = await imageFile.async('base64')
        const mimeType = getMimeType(imagePath)
        const newSrc = `data:${mimeType};base64,${imageData}`
        result = result.replace(fullMatch, `![${alt}](${newSrc})`)
      } catch (error) {
        console.warn(`无法读取图片: ${imagePath}`, error)
      }
    }
  }

  return result
}

/** 从文件名提取笔记标题 */
function extractTitleFromPath(path: string): string {
  // 获取文件名（不含扩展名）
  const filename = path.split('/').pop() || ''
  const title = filename.replace(/\.(html?|md|markdown)$/i, '')
  return title || '未命名笔记'
}

/** 判断是否为笔记文件 */
function isNoteFile(filename: string): boolean {
  return /\.(html?|md|markdown)$/i.test(filename)
}

/** 获取文件的基础目录 */
function getBasePath(filepath: string): string {
  const parts = filepath.split('/')
  parts.pop() // 移除文件名
  return parts.join('/')
}

/**
 * 解析有道笔记 ZIP 文件
 * @param data ZIP 文件的二进制数据
 * @param onProgress 进度回调
 * @returns 导入结果
 */
export async function parseYoudaoZip(
  data: ArrayBuffer,
  onProgress?: ImportProgressCallback,
): Promise<ImportResult> {
  const result: ImportResult = {
    success: 0,
    failed: 0,
    errors: [],
    notes: [],
  }

  try {
    // 加载 ZIP 文件
    const zip = await JSZip.loadAsync(data)

    // 查找所有笔记文件
    const noteFiles: string[] = []
    zip.forEach((relativePath) => {
      if (isNoteFile(relativePath)) {
        noteFiles.push(relativePath)
      }
    })

    if (noteFiles.length === 0) {
      throw new Error('ZIP 文件中没有找到笔记文件 (.html, .md)')
    }

    const total = noteFiles.length

    // 解析每个笔记文件
    for (let i = 0; i < noteFiles.length; i++) {
      const filepath = noteFiles[i]

      // 报告进度
      if (onProgress) {
        onProgress({
          current: i + 1,
          total,
          currentTitle: extractTitleFromPath(filepath),
          phase: 'parsing',
        })
      }

      try {
        const file = zip.file(filepath)
        if (!file) continue

        const content = await file.async('string')
        const basePath = getBasePath(filepath)
        const isMarkdown = /\.(md|markdown)$/i.test(filepath)

        // 处理图片内嵌
        let processedContent: string
        if (isMarkdown) {
          processedContent = await embedMarkdownImages(content, basePath, zip)
        } else {
          processedContent = await embedImages(content, basePath, zip)
        }

        const note: ImportedNote = {
          title: extractTitleFromPath(filepath),
          content: processedContent,
          contentType: isMarkdown ? 'markdown' : 'html',
          tags: [],
        }

        result.notes.push(note)
        result.success++
      } catch (error) {
        result.failed++
        result.errors.push(`解析 ${filepath} 失败: ${error}`)
      }
    }
  } catch (error) {
    result.errors.push(error instanceof Error ? error.message : '未知错误')
  }

  return result
}
