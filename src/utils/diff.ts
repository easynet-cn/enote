import { diffArrays, diffChars } from 'diff'

/**
 * 将 HTML 拆成「结构标签」与「文本片段」token 数组。
 * 文本片段已经是经过转义的 HTML 内容（如 &amp;、&lt;），因此不做二次转义，
 * 直接透传，与原 htmldiff-js 的行为保持一致。
 */
function tokenizeHtml(html: string): string[] {
  return html.match(/<[^>]+>|[^<]+/g) ?? []
}

/**
 * HTML 内容差异对比
 * 用 diff 包对 token 数组做 diff，生成带 <ins>/<del> 标签的合并 HTML，
 * 保留原始富文本样式（标签作为独立 token 原样保留）。
 */
export function diffHtml(oldHtml: string, newHtml: string): string {
  if (!oldHtml && !newHtml) return ''
  if (!oldHtml) return `<ins class="diff-ins">${newHtml}</ins>`
  if (!newHtml) return `<del class="diff-del">${oldHtml}</del>`

  const changes = diffArrays(tokenizeHtml(oldHtml), tokenizeHtml(newHtml))

  return changes
    .map((change) => {
      const content = change.value.join('')
      if (change.added) return `<ins class="diff-ins">${content}</ins>`
      if (change.removed) return `<del class="diff-del">${content}</del>`
      return content
    })
    .join('')
}

/**
 * Markdown/纯文本差异对比
 * 用 diff 包做字符级 diff，生成带高亮标记的 HTML。
 */
export function diffText(oldText: string, newText: string): string {
  if (!oldText && !newText) return ''
  if (!oldText)
    return `<pre class="diff-pre"><ins class="diff-ins">${escapeHtml(newText)}</ins></pre>`
  if (!newText)
    return `<pre class="diff-pre"><del class="diff-del">${escapeHtml(oldText)}</del></pre>`

  const html = diffChars(oldText, newText)
    .map((part) => {
      const escaped = escapeHtml(part.value)
      if (part.removed) return `<del class="diff-del">${escaped}</del>`
      if (part.added) return `<ins class="diff-ins">${escaped}</ins>`
      return `<span>${escaped}</span>`
    })
    .join('')

  return `<pre class="diff-pre">${html}</pre>`
}

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
}
