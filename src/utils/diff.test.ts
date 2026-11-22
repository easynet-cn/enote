import { describe, it, expect } from 'vitest'
import { diffHtml, diffText } from './diff'

describe('diffHtml', () => {
  it('两边都为空时返回空串', () => {
    expect(diffHtml('', '')).toBe('')
  })

  it('只有新内容时全部标记为新增', () => {
    expect(diffHtml('', '<p>new</p>')).toBe('<ins class="diff-ins"><p>new</p></ins>')
  })

  it('只有旧内容时全部标记为删除', () => {
    expect(diffHtml('<p>old</p>', '')).toBe('<del class="diff-del"><p>old</p></del>')
  })

  it('内容相同时不产生 ins/del 标记', () => {
    const html = '<p>Hello <strong>world</strong></p>'
    const out = diffHtml(html, html)
    expect(out).toBe(html)
    expect(out).not.toContain('diff-ins')
    expect(out).not.toContain('diff-del')
  })

  it('保留富文本标签并标记变化的文本片段', () => {
    const out = diffHtml(
      '<p>Hello <strong>world</strong>, welcome!</p>',
      '<p>Hello <strong>there</strong>, welcome!</p>',
    )
    expect(out).toContain('<strong>')
    expect(out).toContain('diff-del')
    expect(out).toContain('world')
    expect(out).toContain('diff-ins')
    expect(out).toContain('there')
  })

  it('HTML 实体不被二次转义', () => {
    const out = diffHtml('<p>A &amp; B &lt;tag&gt;</p>', '<p>A &amp; C &lt;tag&gt;</p>')
    expect(out).not.toContain('&amp;amp;')
    expect(out).toContain('&amp;')
    expect(out).toContain('&lt;tag&gt;')
  })
})

describe('diffText', () => {
  it('两边都为空时返回空串', () => {
    expect(diffText('', '')).toBe('')
  })

  it('只有新内容时用 pre + ins 包裹且做 HTML 转义', () => {
    expect(diffText('', '<b>x</b>')).toBe(
      '<pre class="diff-pre"><ins class="diff-ins">&lt;b&gt;x&lt;/b&gt;</ins></pre>',
    )
  })

  it('只有旧内容时用 pre + del 包裹且做 HTML 转义', () => {
    expect(diffText('<b>x</b>', '')).toBe(
      '<pre class="diff-pre"><del class="diff-del">&lt;b&gt;x&lt;/b&gt;</del></pre>',
    )
  })

  it('字符级 diff 输出 ins/del 并用 pre 包裹', () => {
    const out = diffText('the quick brown fox', 'the slow brown fox')
    expect(out.startsWith('<pre class="diff-pre">')).toBe(true)
    expect(out.endsWith('</pre>')).toBe(true)
    expect(out).toContain('diff-del')
    expect(out).toContain('diff-ins')
    expect(out).toContain('quick')
    expect(out).toContain('slow')
  })

  it('特殊字符被转义而非当作 HTML 输出', () => {
    expect(diffText('a < b', 'a > b')).toBe(
      '<pre class="diff-pre"><span>a </span><del class="diff-del">&lt;</del><ins class="diff-ins">&gt;</ins><span> b</span></pre>',
    )
  })
})
