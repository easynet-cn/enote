import { save } from '@tauri-apps/plugin-dialog'
import { writeTextFile, writeFile } from '@tauri-apps/plugin-fs'
import JSZip from 'jszip'
import type { ShowNote } from '../types'
import { ContentType } from '../types'
import { markdownToHtml } from './markdown'

export type ExportFormat = 'json' | 'xml' | 'word' | 'enex' | 'markdown'

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
 * 格式化时间为 ENEX 格式 (YYYYMMDDTHHmmssZ)
 */
function formatEnexTime(dateStr: string | null): string {
  if (!dateStr) return ''
  try {
    const date = new Date(dateStr)
    const year = date.getFullYear()
    const month = String(date.getMonth() + 1).padStart(2, '0')
    const day = String(date.getDate()).padStart(2, '0')
    const hours = String(date.getHours()).padStart(2, '0')
    const minutes = String(date.getMinutes()).padStart(2, '0')
    const seconds = String(date.getSeconds()).padStart(2, '0')
    return `${year}${month}${day}T${hours}${minutes}${seconds}Z`
  } catch {
    return ''
  }
}

/**
 * 导出为印象笔记 ENEX 格式
 */
function exportToEnex(note: ShowNote): string {
  const escapeXml = (str: string): string => {
    return str
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&apos;')
  }

  const htmlContent = getHtmlContent(note)
  const created = formatEnexTime(note.createTime)
  const updated = formatEnexTime(note.updateTime)
  const tags = note.tags?.map((t) => `    <tag>${escapeXml(t.name)}</tag>`).join('\n') || ''
  const exportDate = formatEnexTime(new Date().toISOString())

  // ENML 内容格式
  const enmlContent = `<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE en-note SYSTEM "http://xml.evernote.com/pub/enml2.dtd">
<en-note>${htmlContent}</en-note>`

  return `<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE en-export SYSTEM "http://xml.evernote.com/pub/evernote-export4.dtd">
<en-export export-date="${exportDate}" application="enote">
  <note>
    <title>${escapeXml(note.title)}</title>
    <content><![CDATA[${enmlContent}]]></content>
    <created>${created}</created>
    <updated>${updated}</updated>
${tags}
  </note>
</en-export>`
}

/**
 * 导出为 Markdown 格式
 */
function exportToMarkdown(note: ShowNote): string {
  // 如果已经是 Markdown 格式，直接返回
  if (note.contentType === ContentType.Markdown) {
    return note.content
  }

  // HTML 转 Markdown（简单转换）
  let content = note.content

  // 转换标题
  content = content.replace(/<h1[^>]*>(.*?)<\/h1>/gi, '# $1\n\n')
  content = content.replace(/<h2[^>]*>(.*?)<\/h2>/gi, '## $1\n\n')
  content = content.replace(/<h3[^>]*>(.*?)<\/h3>/gi, '### $1\n\n')
  content = content.replace(/<h4[^>]*>(.*?)<\/h4>/gi, '#### $1\n\n')
  content = content.replace(/<h5[^>]*>(.*?)<\/h5>/gi, '##### $1\n\n')
  content = content.replace(/<h6[^>]*>(.*?)<\/h6>/gi, '###### $1\n\n')

  // 转换格式
  content = content.replace(/<strong[^>]*>(.*?)<\/strong>/gi, '**$1**')
  content = content.replace(/<b[^>]*>(.*?)<\/b>/gi, '**$1**')
  content = content.replace(/<em[^>]*>(.*?)<\/em>/gi, '*$1*')
  content = content.replace(/<i[^>]*>(.*?)<\/i>/gi, '*$1*')
  content = content.replace(/<code[^>]*>(.*?)<\/code>/gi, '`$1`')
  content = content.replace(/<del[^>]*>(.*?)<\/del>/gi, '~~$1~~')

  // 转换链接和图片
  content = content.replace(/<a[^>]*href=["']([^"']+)["'][^>]*>(.*?)<\/a>/gi, '[$2]($1)')
  content = content.replace(
    /<img[^>]*src=["']([^"']+)["'][^>]*alt=["']([^"']*)["'][^>]*\/?>/gi,
    '![$2]($1)',
  )
  content = content.replace(/<img[^>]*src=["']([^"']+)["'][^>]*\/?>/gi, '![]($1)')

  // 转换列表
  content = content.replace(/<li[^>]*>(.*?)<\/li>/gi, '- $1\n')
  content = content.replace(/<\/?[uo]l[^>]*>/gi, '\n')

  // 转换段落和换行
  content = content.replace(/<p[^>]*>(.*?)<\/p>/gi, '$1\n\n')
  content = content.replace(/<br[^>]*\/?>/gi, '\n')

  // 转换引用
  content = content.replace(/<blockquote[^>]*>(.*?)<\/blockquote>/gis, (_, text) => {
    return (
      text
        .split('\n')
        .map((line: string) => `> ${line}`)
        .join('\n') + '\n\n'
    )
  })

  // 转换水平线
  content = content.replace(/<hr[^>]*\/?>/gi, '\n---\n\n')

  // 移除其他 HTML 标签
  content = content.replace(/<[^>]+>/g, '')

  // 解码 HTML 实体
  content = content
    .replace(/&nbsp;/g, ' ')
    .replace(/&amp;/g, '&')
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'")

  // 清理多余空行
  content = content.replace(/\n{3,}/g, '\n\n')

  return content.trim()
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
    case 'enex':
      return 'enex'
    case 'markdown':
      return 'md'
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
    case 'enex':
      return [{ name: '印象笔记', extensions: ['enex'] }]
    case 'markdown':
      return [{ name: 'Markdown', extensions: ['md'] }]
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
    case 'enex':
      content = exportToEnex(note)
      break
    case 'markdown':
      content = exportToMarkdown(note)
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
    { value: 'markdown', label: 'Markdown', description: '导出为 .md 格式' },
    { value: 'enex', label: '印象笔记', description: '导出为 .enex 格式，可导入印象笔记' },
    { value: 'json', label: 'JSON', description: '导出为 JSON 数据格式' },
    { value: 'xml', label: 'XML', description: '导出为 XML 数据格式' },
  ]
}

/**
 * 批量导出笔记为 Markdown ZIP
 */
export async function exportNotesToMarkdownZip(notes: ShowNote[]): Promise<boolean> {
  if (notes.length === 0) return false

  const zip = new JSZip()

  // 为每个笔记创建 Markdown 文件
  for (const note of notes) {
    const content = exportToMarkdown(note)
    const filename = `${sanitizeFilename(note.title || '未命名笔记')}.md`
    zip.file(filename, content)
  }

  // 生成 ZIP 文件
  const zipContent = await zip.generateAsync({ type: 'uint8array' })

  // 打开保存文件对话框
  const filePath = await save({
    defaultPath: 'notes_export.zip',
    filters: [{ name: 'ZIP 压缩包', extensions: ['zip'] }],
    title: '批量导出笔记',
  })

  if (!filePath) {
    return false
  }

  // 写入文件
  await writeFile(filePath, zipContent)
  return true
}

/**
 * 批量导出笔记为 ENEX 格式
 */
export async function exportNotesToEnex(notes: ShowNote[]): Promise<boolean> {
  if (notes.length === 0) return false

  const escapeXml = (str: string): string => {
    return str
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&apos;')
  }

  const exportDate = formatEnexTime(new Date().toISOString())

  // 生成所有笔记的 XML
  const notesXml = notes
    .map((note) => {
      const htmlContent = getHtmlContent(note)
      const created = formatEnexTime(note.createTime)
      const updated = formatEnexTime(note.updateTime)
      const tags = note.tags?.map((t) => `    <tag>${escapeXml(t.name)}</tag>`).join('\n') || ''

      const enmlContent = `<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE en-note SYSTEM "http://xml.evernote.com/pub/enml2.dtd">
<en-note>${htmlContent}</en-note>`

      return `  <note>
    <title>${escapeXml(note.title)}</title>
    <content><![CDATA[${enmlContent}]]></content>
    <created>${created}</created>
    <updated>${updated}</updated>
${tags}
  </note>`
    })
    .join('\n')

  const content = `<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE en-export SYSTEM "http://xml.evernote.com/pub/evernote-export4.dtd">
<en-export export-date="${exportDate}" application="enote">
${notesXml}
</en-export>`

  // 打开保存文件对话框
  const filePath = await save({
    defaultPath: 'notes_export.enex',
    filters: [{ name: '印象笔记', extensions: ['enex'] }],
    title: '批量导出笔记',
  })

  if (!filePath) {
    return false
  }

  // 写入文件
  await writeTextFile(filePath, content)
  return true
}

/**
 * 清理文件名中的非法字符
 */
function sanitizeFilename(filename: string): string {
  return filename
    .replace(/[<>:"/\\|?*]/g, '_')
    .replace(/\s+/g, ' ')
    .trim()
    .slice(0, 200)
}
