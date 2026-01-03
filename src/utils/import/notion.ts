/**
 * Notion Markdown ZIP 格式导入解析器
 *
 * Notion 导出的 ZIP 结构:
 * - Page 1 abc123.md
 * - Page 2 def456/
 *   - Page 2 def456.md
 *   - image.png
 * - ...
 *
 * 特点:
 * - 文件名包含 Notion 页面 ID 后缀（16位字母数字）
 * - 图片存放在同名文件夹中
 * - 支持嵌套子页面
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
  }
  return mimeTypes[ext] || 'application/octet-stream'
}

/** 从 Notion 文件名提取标题（移除 ID 后缀） */
function extractTitleFromNotionPath(path: string): string {
  // 获取文件名（不含扩展名）
  const filename = path.split('/').pop() || ''
  const nameWithoutExt = filename.replace(/\.(md|markdown)$/i, '')

  // Notion 文件名格式: "标题 abc123def456789" (ID 通常是 32 位)
  // 移除末尾的 ID 后缀（空格 + 16-32 位字母数字）
  const titleMatch = nameWithoutExt.match(/^(.+?)\s+[a-f0-9]{16,32}$/i)
  if (titleMatch) {
    return titleMatch[1].trim()
  }

  return nameWithoutExt || '未命名笔记'
}

/** 判断是否为 Markdown 笔记文件 */
function isNotionNoteFile(filename: string): boolean {
  // 只处理 .md 文件，跳过隐藏文件和 CSV
  if (filename.startsWith('.') || filename.startsWith('_')) return false
  return /\.(md|markdown)$/i.test(filename)
}

/** 获取文件所在目录 */
function getDirectory(filepath: string): string {
  const parts = filepath.split('/')
  parts.pop()
  return parts.join('/')
}

/** 将 Markdown 中的图片路径替换为 base64 内嵌 */
async function embedNotionImages(content: string, basePath: string, zip: JSZip): Promise<string> {
  // Notion 导出的 Markdown 图片格式: ![description](path%20to%20image.png)
  // 路径可能被 URL 编码
  const imgRegex = /!\[([^\]]*)\]\(([^)]+)\)/g
  let result = content
  const matches: Array<{ fullMatch: string; alt: string; src: string }> = []

  let match
  while ((match = imgRegex.exec(content)) !== null) {
    matches.push({
      fullMatch: match[0],
      alt: match[1],
      src: match[2],
    })
  }

  for (const { fullMatch, alt, src } of matches) {
    // 跳过外部 URL 和已经是 base64 的图片
    if (src.startsWith('http') || src.startsWith('data:')) continue

    // URL 解码路径
    let imagePath = decodeURIComponent(src)

    // 构建完整路径
    if (!imagePath.startsWith('/')) {
      imagePath = basePath ? `${basePath}/${imagePath}` : imagePath
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

/** 处理 Notion 特有的 Markdown 语法 */
function processNotionMarkdown(content: string): string {
  let result = content

  // 处理 Notion 的 callout 块 (> 开头带 emoji)
  // > 💡 This is a callout
  // 保持原样，TipTap 可以渲染为引用块

  // 处理 Notion 的 toggle 块
  // <details><summary>Toggle title</summary>content</details>
  // 保持 HTML 格式

  // 处理 Notion 的数据库链接引用
  // [Link to page](Page%20Name%20abc123.md)
  // 转换为普通文本
  result = result.replace(/\[([^\]]+)\]\(([^)]+\.md)\)/g, (_match, text) => `**${text}**`)

  // 处理 Notion 的 @mention
  // @Person Name -> **@Person Name**
  result = result.replace(/@(\w+(?:\s+\w+)*)/g, '**@$1**')

  return result
}

/**
 * 解析 Notion 导出的 ZIP 文件
 * @param data ZIP 文件的二进制数据
 * @param onProgress 进度回调
 * @returns 导入结果
 */
export async function parseNotionZip(
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

    // 查找所有 Markdown 笔记文件
    const noteFiles: string[] = []
    zip.forEach((relativePath) => {
      if (isNotionNoteFile(relativePath)) {
        noteFiles.push(relativePath)
      }
    })

    if (noteFiles.length === 0) {
      throw new Error('ZIP 文件中没有找到 Notion 笔记文件 (.md)')
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
          currentTitle: extractTitleFromNotionPath(filepath),
          phase: 'parsing',
        })
      }

      try {
        const file = zip.file(filepath)
        if (!file) continue

        let content = await file.async('string')
        const basePath = getDirectory(filepath)

        // 处理 Notion 特有语法
        content = processNotionMarkdown(content)

        // 处理图片内嵌
        content = await embedNotionImages(content, basePath, zip)

        const note: ImportedNote = {
          title: extractTitleFromNotionPath(filepath),
          content,
          contentType: 'markdown',
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
