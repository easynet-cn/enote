/**
 * 印象笔记 ENEX 格式导入解析器
 *
 * ENEX 是 Evernote 的 XML 导出格式，包含笔记内容、标签、附件等信息
 */

import type {
  ImportedNote,
  ImportedAttachment,
  ImportResult,
  ImportProgressCallback,
} from './types'

/** 解析 ENEX 时间格式 (20240101T120000Z) */
function parseEnexTime(timeStr: string): string {
  if (!timeStr) return ''

  // 格式: YYYYMMDDTHHmmssZ
  const match = timeStr.match(/^(\d{4})(\d{2})(\d{2})T(\d{2})(\d{2})(\d{2})Z?$/)
  if (!match) return ''

  const [, year, month, day, hour, minute, second] = match
  return `${year}-${month}-${day} ${hour}:${minute}:${second}`
}

/** 从 en-note 中提取纯 HTML 内容 */
function extractHtmlFromEnml(enmlContent: string): string {
  // ENML 内容被包裹在 <en-note> 标签中
  const parser = new DOMParser()
  const doc = parser.parseFromString(enmlContent, 'text/html')

  // 查找 en-note 标签
  const enNote = doc.querySelector('en-note')
  if (enNote) {
    return enNote.innerHTML
  }

  // 如果没有 en-note 标签，直接返回 body 内容
  return doc.body?.innerHTML || enmlContent
}

/** 处理 ENML 中的媒体引用，将 en-media 替换为 img 标签 */
function processEnmlMedia(html: string, resources: Map<string, ImportedAttachment>): string {
  // 替换 en-media 标签为 img 标签
  // <en-media hash="abc123" type="image/png" />
  return html.replace(
    /<en-media[^>]*hash=["']([^"']+)["'][^>]*type=["']([^"']+)["'][^>]*\/?>/gi,
    (_match, hash, type) => {
      const resource = resources.get(hash)
      if (resource && type.startsWith('image/')) {
        return `<img src="data:${resource.mimeType};base64,${resource.data}" alt="${resource.filename}" />`
      }
      // 非图片类型，显示文件链接
      if (resource) {
        return `<a href="data:${resource.mimeType};base64,${resource.data}" download="${resource.filename}">${resource.filename}</a>`
      }
      return ''
    },
  )
}

/** 计算内容的 MD5 哈希 (用于附件映射) */
async function md5Hash(data: string): Promise<string> {
  // 浏览器环境使用 SubtleCrypto
  const encoder = new TextEncoder()
  const dataBuffer = encoder.encode(data)
  const hashBuffer = await crypto.subtle.digest('MD5', dataBuffer).catch(() => null)

  // 如果 MD5 不支持，尝试使用简单的哈希替代方案
  if (!hashBuffer) {
    // 简单的字符串哈希
    let hash = 0
    for (let i = 0; i < data.length; i++) {
      const char = data.charCodeAt(i)
      hash = (hash << 5) - hash + char
      hash = hash & hash
    }
    return Math.abs(hash).toString(16)
  }

  const hashArray = Array.from(new Uint8Array(hashBuffer))
  return hashArray.map((b) => b.toString(16).padStart(2, '0')).join('')
}

/** 解析单个 note 节点 */
async function parseNoteElement(noteElement: Element): Promise<ImportedNote | null> {
  try {
    // 提取基本信息
    const title = noteElement.querySelector('title')?.textContent || '未命名笔记'
    const contentElement = noteElement.querySelector('content')
    const created = noteElement.querySelector('created')?.textContent || ''
    const updated = noteElement.querySelector('updated')?.textContent || ''

    // 提取标签
    const tagElements = noteElement.querySelectorAll('tag')
    const tags: string[] = []
    tagElements.forEach((tag) => {
      if (tag.textContent) {
        tags.push(tag.textContent)
      }
    })

    // 提取附件资源
    const resources = new Map<string, ImportedAttachment>()
    const attachments: ImportedAttachment[] = []
    const resourceElements = noteElement.querySelectorAll('resource')

    for (const resourceElement of resourceElements) {
      const dataElement = resourceElement.querySelector('data')
      const mimeElement = resourceElement.querySelector('mime')
      const filenameElement = resourceElement.querySelector('resource-attributes > file-name')

      if (dataElement?.textContent && mimeElement?.textContent) {
        const data = dataElement.textContent.replace(/\s/g, '') // 移除空白字符
        const mimeType = mimeElement.textContent
        const filename =
          filenameElement?.textContent || `attachment.${mimeType.split('/')[1] || 'bin'}`

        // 计算哈希用于映射 en-media 引用
        const hash = await md5Hash(data).catch(() => '')

        const attachment: ImportedAttachment = {
          filename,
          mimeType,
          data,
        }

        if (hash) {
          resources.set(hash, attachment)
        }
        attachments.push(attachment)
      }
    }

    // 提取并处理内容
    let content = ''
    if (contentElement?.textContent) {
      // ENEX 中的内容是 CDATA 包裹的 ENML
      const enmlContent = contentElement.textContent
      content = extractHtmlFromEnml(enmlContent)
      content = processEnmlMedia(content, resources)
    }

    return {
      title,
      content,
      contentType: 'html',
      tags,
      createTime: parseEnexTime(created),
      updateTime: parseEnexTime(updated),
      attachments: attachments.length > 0 ? attachments : undefined,
    }
  } catch (error) {
    console.error('解析笔记失败:', error)
    return null
  }
}

/**
 * 解析印象笔记 ENEX 文件
 * @param content ENEX 文件内容 (XML 字符串)
 * @param onProgress 进度回调
 * @returns 导入结果
 */
export async function parseEvernoteEnex(
  content: string,
  onProgress?: ImportProgressCallback,
): Promise<ImportResult> {
  const result: ImportResult = {
    success: 0,
    failed: 0,
    errors: [],
    notes: [],
  }

  try {
    // 解析 XML
    const parser = new DOMParser()
    const doc = parser.parseFromString(content, 'text/xml')

    // 检查解析错误
    const parseError = doc.querySelector('parsererror')
    if (parseError) {
      throw new Error('ENEX 文件格式错误: ' + parseError.textContent)
    }

    // 获取所有笔记节点
    const noteElements = doc.querySelectorAll('note')
    const total = noteElements.length

    if (total === 0) {
      throw new Error('ENEX 文件中没有找到笔记')
    }

    // 逐个解析笔记
    for (let i = 0; i < noteElements.length; i++) {
      const noteElement = noteElements[i]

      // 报告进度
      if (onProgress) {
        const title = noteElement.querySelector('title')?.textContent || '未命名'
        onProgress({
          current: i + 1,
          total,
          currentTitle: title,
          phase: 'parsing',
        })
      }

      const note = await parseNoteElement(noteElement)
      if (note) {
        result.notes.push(note)
        result.success++
      } else {
        result.failed++
        result.errors.push(`解析第 ${i + 1} 个笔记失败`)
      }
    }
  } catch (error) {
    result.errors.push(error instanceof Error ? error.message : '未知错误')
  }

  return result
}
