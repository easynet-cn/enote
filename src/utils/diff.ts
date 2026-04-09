import DiffMatchPatch from 'diff-match-patch'
import HtmlDiffModule from 'htmldiff-js'

const HtmlDiff = HtmlDiffModule.default

/**
 * HTML 内容差异对比
 * 使用 htmldiff-js 对比两段 HTML，生成带 <ins>/<del> 标签的合并 HTML
 * 保留原始富文本样式
 */
export function diffHtml(oldHtml: string, newHtml: string): string {
  if (!oldHtml && !newHtml) return ''
  if (!oldHtml) return `<ins class="diff-ins">${newHtml}</ins>`
  if (!newHtml) return `<del class="diff-del">${oldHtml}</del>`

  return HtmlDiff.execute(oldHtml, newHtml)
}

/**
 * Markdown/纯文本差异对比
 * 使用 diff-match-patch 做字符级 diff，生成带高亮标记的 HTML
 */
export function diffText(oldText: string, newText: string): string {
  if (!oldText && !newText) return ''
  if (!oldText)
    return `<pre class="diff-pre"><ins class="diff-ins">${escapeHtml(newText)}</ins></pre>`
  if (!newText)
    return `<pre class="diff-pre"><del class="diff-del">${escapeHtml(oldText)}</del></pre>`

  const dmp = new DiffMatchPatch()
  const diffs = dmp.diff_main(oldText, newText)
  dmp.diff_cleanupSemantic(diffs)

  const html = diffs
    .map(([op, text]) => {
      const escaped = escapeHtml(text)
      switch (op) {
        case DiffMatchPatch.DIFF_DELETE:
          return `<del class="diff-del">${escaped}</del>`
        case DiffMatchPatch.DIFF_INSERT:
          return `<ins class="diff-ins">${escaped}</ins>`
        default:
          return `<span>${escaped}</span>`
      }
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
