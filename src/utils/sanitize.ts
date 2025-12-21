import DOMPurify from 'dompurify'

/**
 * 净化 HTML 内容，防止 XSS 攻击
 */
export const sanitizeHtml = (html: string): string => {
  return DOMPurify.sanitize(html, {
    ALLOWED_TAGS: [
      'p',
      'br',
      'b',
      'i',
      'u',
      's',
      'em',
      'strong',
      'a',
      'ul',
      'ol',
      'li',
      'h1',
      'h2',
      'h3',
      'h4',
      'h5',
      'h6',
      'blockquote',
      'pre',
      'code',
      'table',
      'thead',
      'tbody',
      'tr',
      'th',
      'td',
      'img',
      'hr',
      'span',
      'div',
      'mark',
      'label',
      'input',
    ],
    ALLOWED_ATTR: [
      'href',
      'src',
      'alt',
      'title',
      'class',
      'style',
      'target',
      'rel',
      'data-type',
      'data-checked',
      'type',
      'checked',
      'colspan',
      'rowspan',
    ],
  })
}

/**
 * 从 HTML 中提取纯文本（用于预览）
 */
export const stripHtml = (html: string): string => {
  const doc = new DOMParser().parseFromString(html, 'text/html')
  return doc.body.textContent || ''
}

/**
 * 截断文本到指定长度
 */
export const truncateText = (text: string, maxLength: number = 100): string => {
  if (text.length <= maxLength) return text
  return text.slice(0, maxLength) + '...'
}
