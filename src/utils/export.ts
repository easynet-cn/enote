import { save } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import type { ShowNote } from '../types'
import { ContentType } from '../types'
import { markdownToHtml } from './markdown'

export type ExportFormat = 'json' | 'xml' | 'word'

interface ExportOptions {
  note: ShowNote
  format: ExportFormat
}

/**
 * 将笔记内容转换为 HTML
 */
function getHtmlContent(note: ShowNote): string {
  if (note.contentType === ContentType.Markdown) {
    return markdownToHtml(note.content)
  }
  return note.content
}

/**
 * 导出为 JSON 格式
 */
function exportToJson(note: ShowNote): string {
  const exportData = {
    title: note.title,
    content: note.content,
    contentType: note.contentType === ContentType.Markdown ? 'markdown' : 'html',
    tags: note.tags?.map((t) => t.name) || [],
    createTime: note.createTime,
    updateTime: note.updateTime,
  }
  return JSON.stringify(exportData, null, 2)
}

/**
 * 导出为 XML 格式
 */
function exportToXml(note: ShowNote): string {
  const escapeXml = (str: string): string => {
    return str
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&apos;')
  }

  const tags = note.tags?.map((t) => `    <tag>${escapeXml(t.name)}</tag>`).join('\n') || ''

  return `<?xml version="1.0" encoding="UTF-8"?>
<note>
  <title>${escapeXml(note.title)}</title>
  <contentType>${note.contentType === ContentType.Markdown ? 'markdown' : 'html'}</contentType>
  <content><![CDATA[${note.content}]]></content>
  <tags>
${tags}
  </tags>
  <createTime>${note.createTime}</createTime>
  <updateTime>${note.updateTime}</updateTime>
</note>`
}

/**
 * 导出为 Word 格式（HTML 格式，.doc 扩展名）
 */
function exportToWord(note: ShowNote): string {
  const htmlContent = getHtmlContent(note)
  const tags = note.tags?.map((t) => t.name).join(', ') || ''

  return `<!DOCTYPE html>
<html xmlns:o="urn:schemas-microsoft-com:office:office"
      xmlns:w="urn:schemas-microsoft-com:office:word"
      xmlns="http://www.w3.org/TR/REC-html40">
<head>
  <meta charset="UTF-8">
  <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
  <title>${note.title}</title>
  <!--[if gte mso 9]>
  <xml>
    <w:WordDocument>
      <w:View>Print</w:View>
      <w:Zoom>100</w:Zoom>
      <w:DoNotOptimizeForBrowser/>
    </w:WordDocument>
  </xml>
  <![endif]-->
  <style>
    body {
      font-family: 'Microsoft YaHei', Arial, sans-serif;
      font-size: 12pt;
      line-height: 1.6;
      margin: 2cm;
    }
    h1 { font-size: 24pt; margin-bottom: 12pt; color: #1f2937; }
    h2 { font-size: 18pt; margin-top: 18pt; margin-bottom: 10pt; color: #374151; }
    h3 { font-size: 14pt; margin-top: 14pt; margin-bottom: 8pt; color: #4b5563; }
    p { margin-bottom: 8pt; }
    pre { background-color: #f3f4f6; padding: 10pt; font-family: Consolas, monospace; }
    code { background-color: #f3f4f6; padding: 2pt 4pt; font-family: Consolas, monospace; }
    blockquote { border-left: 3pt solid #e5e7eb; padding-left: 10pt; color: #6b7280; font-style: italic; }
    table { border-collapse: collapse; width: 100%; margin: 10pt 0; }
    th, td { border: 1pt solid #d1d5db; padding: 6pt; text-align: left; }
    th { background-color: #f9fafb; }
    .meta { color: #9ca3af; font-size: 10pt; margin-bottom: 20pt; }
    .tags { margin-top: 5pt; }
    .tag { display: inline-block; background: #e0e7ff; color: #4338ca; padding: 2pt 8pt; border-radius: 10pt; margin-right: 5pt; font-size: 9pt; }
  </style>
</head>
<body>
  <h1>${note.title || '无标题'}</h1>
  <div class="meta">
    <div>创建时间: ${note.createTime}</div>
    <div>更新时间: ${note.updateTime}</div>
    ${tags ? `<div class="tags">标签: ${note.tags?.map((t) => `<span class="tag">${t.name}</span>`).join(' ') || ''}</div>` : ''}
  </div>
  <hr>
  <div class="content">
    ${htmlContent}
  </div>
</body>
</html>`
}

/**
 * 获取文件扩展名
 */
function getFileExtension(format: ExportFormat): string {
  switch (format) {
    case 'json':
      return 'json'
    case 'xml':
      return 'xml'
    case 'word':
      return 'doc'
  }
}

/**
 * 获取文件过滤器
 */
function getFileFilters(format: ExportFormat) {
  switch (format) {
    case 'json':
      return [{ name: 'JSON', extensions: ['json'] }]
    case 'xml':
      return [{ name: 'XML', extensions: ['xml'] }]
    case 'word':
      return [{ name: 'Word 文档', extensions: ['doc'] }]
  }
}

/**
 * 导出笔记
 */
export async function exportNote(options: ExportOptions): Promise<boolean> {
  const { note, format } = options

  // 生成文件内容
  let content: string
  switch (format) {
    case 'json':
      content = exportToJson(note)
      break
    case 'xml':
      content = exportToXml(note)
      break
    case 'word':
      content = exportToWord(note)
      break
  }

  // 生成默认文件名
  const fileName = `${note.title || '未命名笔记'}.${getFileExtension(format)}`

  // 打开保存文件对话框
  const filePath = await save({
    defaultPath: fileName,
    filters: getFileFilters(format),
    title: '导出笔记',
  })

  if (!filePath) {
    return false // 用户取消
  }

  // 写入文件
  await writeTextFile(filePath, content)
  return true
}

/**
 * 获取支持的导出格式
 */
export function getExportFormats(): { value: ExportFormat; label: string; description: string }[] {
  return [
    { value: 'word', label: 'Word 文档', description: '导出为 .doc 格式，可用 Word 打开' },
    { value: 'json', label: 'JSON', description: '导出为 JSON 数据格式' },
    { value: 'xml', label: 'XML', description: '导出为 XML 数据格式' },
  ]
}
