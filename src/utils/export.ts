import { save } from '@tauri-apps/plugin-dialog'
import { writeTextFile, writeFile } from '@tauri-apps/plugin-fs'
import JSZip from 'jszip'
import type { ShowNote } from '../types'
import { ContentType } from '../types'
import { markdownToHtml } from './markdown'
import i18n from '../i18n'

const t = i18n.global.t

export type ExportFormat = 'json' | 'xml' | 'word' | 'enex' | 'markdown' | 'pdf' | 'html'

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
  <h1>${note.title || t('export.noTitle')}</h1>
  <div class="meta">
    <div>${t('export.createTime')}: ${note.createTime}</div>
    <div>${t('export.updateTime')}: ${note.updateTime}</div>
    ${tags ? `<div class="tags">${t('export.tags')}: ${note.tags?.map((tag) => `<span class="tag">${tag.name}</span>`).join(' ') || ''}</div>` : ''}
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
 * HTML 特殊字符转义
 */
function escapeHtml(str: string): string {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;')
}

/**
 * 导出为 PDF（通过浏览器打印对话框）
 */
export async function exportAsPdf(title: string, content: string): Promise<void> {
  const printWindow = window.open('', '_blank', 'width=800,height=600')
  if (!printWindow) return

  const html = `<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>${escapeHtml(title)}</title>
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Microsoft YaHei', sans-serif;
      max-width: 800px;
      margin: 0 auto;
      padding: 40px 20px;
      color: #1a1a1a;
      line-height: 1.6;
    }
    h1 { font-size: 24px; margin-bottom: 24px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
    h2 { font-size: 20px; }
    h3 { font-size: 17px; }
    img { max-width: 100%; height: auto; }
    pre { background: #f5f5f5; padding: 12px; border-radius: 6px; overflow-x: auto; }
    code { background: #f5f5f5; padding: 2px 6px; border-radius: 3px; font-size: 0.9em; }
    pre code { background: none; padding: 0; }
    blockquote { border-left: 3px solid #ddd; margin-left: 0; padding-left: 16px; color: #666; }
    table { border-collapse: collapse; width: 100%; }
    th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
    th { background: #f5f5f5; }
    ul[data-type="taskList"] { list-style: none; padding-left: 0; }
    ul[data-type="taskList"] li { display: flex; align-items: flex-start; gap: 8px; }
    @media print {
      body { padding: 0; }
    }
  </style>
</head>
<body>
  <h1>${escapeHtml(title)}</h1>
  ${content}
</body>
</html>`

  printWindow.document.write(html)
  printWindow.document.close()

  // Wait for images to load then print
  printWindow.onload = () => {
    printWindow.print()
    printWindow.close()
  }
  // Fallback if onload doesn't fire
  setTimeout(() => {
    try {
      printWindow.print()
      printWindow.close()
    } catch {
      // Window may already be closed
    }
  }, 1000)
}

/**
 * 导出为独立 HTML 文件
 */
function exportToStandaloneHtml(note: ShowNote): string {
  const htmlContent = getHtmlContent(note)
  const title = escapeHtml(note.title || t('export.noTitle'))
  const tags = note.tags?.map((t) => t.name).join(', ') || ''

  return `<!DOCTYPE html>
<html lang="zh">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>${title}</title>
  <style>
    *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Microsoft YaHei', 'Helvetica Neue', Arial, sans-serif;
      max-width: 800px;
      margin: 0 auto;
      padding: 2rem 1.5rem;
      color: #1a1a2e;
      line-height: 1.8;
      background: #ffffff;
    }
    h1.note-title {
      font-size: 2rem;
      font-weight: 700;
      margin-bottom: 0.5rem;
      color: #1a1a2e;
      border-bottom: 2px solid #e2e8f0;
      padding-bottom: 0.75rem;
    }
    .note-meta {
      font-size: 0.8rem;
      color: #64748b;
      margin-bottom: 1.5rem;
      padding-bottom: 1rem;
      border-bottom: 1px solid #f1f5f9;
    }
    .note-meta .tag {
      display: inline-block;
      background: #e0e7ff;
      color: #4338ca;
      padding: 2px 8px;
      border-radius: 10px;
      font-size: 0.75rem;
      margin-right: 4px;
    }
    .note-content h1 { font-size: 1.75rem; font-weight: 700; margin: 1.5rem 0 0.75rem; color: #1e293b; }
    .note-content h2 { font-size: 1.5rem; font-weight: 600; margin: 1.25rem 0 0.625rem; color: #1e293b; }
    .note-content h3 { font-size: 1.25rem; font-weight: 600; margin: 1rem 0 0.5rem; color: #334155; }
    .note-content h4 { font-size: 1.125rem; font-weight: 600; margin: 0.875rem 0 0.5rem; color: #334155; }
    .note-content p { margin-bottom: 0.75rem; }
    .note-content a { color: #4f46e5; text-decoration: underline; }
    .note-content a:hover { color: #3730a3; }
    .note-content img { max-width: 100%; height: auto; border-radius: 8px; margin: 1rem 0; }
    .note-content ul, .note-content ol { padding-left: 1.5rem; margin-bottom: 0.75rem; }
    .note-content li { margin-bottom: 0.25rem; }
    .note-content blockquote {
      border-left: 4px solid #4f46e5;
      padding: 0.5rem 1rem;
      margin: 1rem 0;
      background: #f8fafc;
      color: #475569;
      font-style: italic;
      border-radius: 0 8px 8px 0;
    }
    .note-content code {
      background: #f1f5f9;
      padding: 0.15rem 0.4rem;
      border-radius: 4px;
      font-family: 'SF Mono', Consolas, 'Liberation Mono', Menlo, monospace;
      font-size: 0.9em;
      color: #e11d48;
    }
    .note-content pre {
      background: #1e293b;
      color: #e2e8f0;
      padding: 1rem 1.25rem;
      border-radius: 8px;
      overflow-x: auto;
      margin: 1rem 0;
      line-height: 1.6;
    }
    .note-content pre code {
      background: none;
      color: inherit;
      padding: 0;
      font-size: 0.875rem;
    }
    .note-content table { border-collapse: collapse; width: 100%; margin: 1rem 0; }
    .note-content th, .note-content td { border: 1px solid #e2e8f0; padding: 0.5rem 0.75rem; text-align: left; }
    .note-content th { background: #f8fafc; font-weight: 600; }
    .note-content hr { border: none; border-top: 1px solid #e2e8f0; margin: 1.5rem 0; }
    .note-content mark { background: #fef08a; padding: 0.1rem 0.2rem; border-radius: 2px; }
    .note-content ul[data-type="taskList"] { list-style: none; padding-left: 0; }
    .note-content ul[data-type="taskList"] li { display: flex; align-items: flex-start; gap: 0.5rem; }
    .note-content ul[data-type="taskList"] li[data-checked="true"] > div { text-decoration: line-through; color: #94a3b8; }
    .note-footer {
      margin-top: 3rem;
      padding-top: 1rem;
      border-top: 1px solid #e2e8f0;
      font-size: 0.75rem;
      color: #94a3b8;
      text-align: center;
    }
    @media (prefers-color-scheme: dark) {
      body { background: #0f172a; color: #e2e8f0; }
      h1.note-title { color: #f1f5f9; border-bottom-color: #334155; }
      .note-meta { color: #94a3b8; border-bottom-color: #1e293b; }
      .note-meta .tag { background: #312e81; color: #a5b4fc; }
      .note-content h1, .note-content h2 { color: #f1f5f9; }
      .note-content h3, .note-content h4 { color: #cbd5e1; }
      .note-content a { color: #818cf8; }
      .note-content a:hover { color: #a5b4fc; }
      .note-content blockquote { background: #1e293b; color: #94a3b8; border-left-color: #818cf8; }
      .note-content code { background: #1e293b; color: #fb7185; }
      .note-content pre { background: #0f172a; border: 1px solid #334155; }
      .note-content th { background: #1e293b; }
      .note-content th, .note-content td { border-color: #334155; }
      .note-content hr { border-top-color: #334155; }
      .note-content mark { background: #854d0e; color: #fef08a; }
      .note-footer { border-top-color: #334155; color: #475569; }
    }
    @media print {
      body { padding: 0; max-width: 100%; }
      .note-footer { display: none; }
    }
    @media (max-width: 600px) {
      body { padding: 1rem; }
      h1.note-title { font-size: 1.5rem; }
    }
  </style>
</head>
<body>
  <h1 class="note-title">${title}</h1>
  <div class="note-meta">
    <div>${t('export.createTime')}: ${note.createTime || ''} &nbsp;&middot;&nbsp; ${t('export.updateTime')}: ${note.updateTime || ''}</div>
    ${tags ? `<div style="margin-top:4px">${t('export.tags')}: ${note.tags?.map((tag) => `<span class="tag">${escapeHtml(tag.name)}</span>`).join(' ') || ''}</div>` : ''}
  </div>
  <div class="note-content">
    ${htmlContent}
  </div>
  <div class="note-footer">
    Exported from eNote
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
    case 'enex':
      return 'enex'
    case 'markdown':
      return 'md'
    case 'pdf':
      return 'pdf'
    case 'html':
      return 'html'
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
      return [{ name: t('export.wordDoc'), extensions: ['doc'] }]
    case 'enex':
      return [{ name: t('export.evernote'), extensions: ['enex'] }]
    case 'markdown':
      return [{ name: 'Markdown', extensions: ['md'] }]
    case 'pdf':
      return [{ name: 'PDF', extensions: ['pdf'] }]
    case 'html':
      return [{ name: 'HTML', extensions: ['html'] }]
  }
}

/**
 * 导出笔记
 */
export async function exportNote(options: ExportOptions): Promise<boolean> {
  const { note, format } = options

  // PDF 使用浏览器打印对话框，不走文件保存流程
  if (format === 'pdf') {
    const htmlContent = getHtmlContent(note)
    await exportAsPdf(note.title || t('export.untitledNote'), htmlContent)
    return true
  }

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
    case 'html':
      content = exportToStandaloneHtml(note)
      break
  }

  // 生成默认文件名
  const fileName = `${note.title || t('export.untitledNote')}.${getFileExtension(format)}`

  // 打开保存文件对话框
  const filePath = await save({
    defaultPath: fileName,
    filters: getFileFilters(format),
    title: t('export.exportNote'),
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
    { value: 'pdf', label: 'PDF', description: t('export.pdfDesc') },
    { value: 'html', label: 'HTML', description: t('export.htmlDesc') },
    { value: 'word', label: t('export.wordDoc'), description: t('export.wordDocDesc') },
    { value: 'markdown', label: 'Markdown', description: t('export.markdownDesc') },
    { value: 'enex', label: t('export.evernote'), description: t('export.evernoteDesc') },
    { value: 'json', label: 'JSON', description: t('export.jsonDesc') },
    { value: 'xml', label: 'XML', description: t('export.xmlDesc') },
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
    const filename = `${sanitizeFilename(note.title || t('export.untitledNote'))}.md`
    zip.file(filename, content)
  }

  // 生成 ZIP 文件
  const zipContent = await zip.generateAsync({ type: 'uint8array' })

  // 打开保存文件对话框
  const filePath = await save({
    defaultPath: 'notes_export.zip',
    filters: [{ name: t('export.zipArchive'), extensions: ['zip'] }],
    title: t('export.batchExport'),
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
    filters: [{ name: t('export.evernote'), extensions: ['enex'] }],
    title: t('export.batchExport'),
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
