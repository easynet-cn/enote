import { Extension } from '@tiptap/core'
import { Plugin, PluginKey } from '@tiptap/pm/state'
import { convertFileSrc } from '@tauri-apps/api/core'
import { imageApi } from '../api/note'

/**
 * 清理粘贴的 HTML 内容
 * - 移除 style/script/meta 标签
 * - 清理 MS Office/Google Docs 特有标记
 * - 移除 class 属性
 * - 清理多余的 inline style（只保留基本格式）
 * - 将 div 转换为 p
 * - 去除空段落
 */
function cleanPastedHtml(html: string): string {
  const parser = new DOMParser()
  const doc = parser.parseFromString(html, 'text/html')

  // 移除 style, script, meta, link, comments
  const removeTags = doc.querySelectorAll('style, script, meta, link, title, head')
  removeTags.forEach((el) => el.remove())

  // 递归清理节点
  cleanNode(doc.body)

  return doc.body.innerHTML
}

/** 保留的 inline style 属性 */
const ALLOWED_STYLE_PROPS = new Set([
  'font-weight',
  'font-style',
  'text-decoration',
  'text-decoration-line',
  'text-align',
  'color',
  'background-color',
])

function cleanNode(node: Node): void {
  const children = Array.from(node.childNodes)

  for (const child of children) {
    if (child.nodeType === Node.COMMENT_NODE) {
      child.remove()
      continue
    }

    if (child.nodeType === Node.ELEMENT_NODE) {
      const el = child as HTMLElement
      const tagName = el.tagName.toLowerCase()

      // 移除隐藏元素和 MS Office 特有标记
      if (
        tagName === 'o:p' ||
        tagName === 'xml' ||
        tagName.startsWith('o:') ||
        tagName.startsWith('v:') ||
        tagName.startsWith('w:')
      ) {
        // 保留文本内容
        const text = document.createTextNode(el.textContent || '')
        node.replaceChild(text, el)
        continue
      }

      // 将 div 转换为 p（如果内部没有块级元素）
      if (tagName === 'div' && !hasBlockChildren(el)) {
        const p = document.createElement('p')
        p.innerHTML = el.innerHTML
        node.replaceChild(p, el)
        cleanNode(p)
        continue
      }

      // 移除 class 属性
      el.removeAttribute('class')
      el.removeAttribute('id')
      el.removeAttribute('name')

      // 移除以 data- 开头的属性（TipTap 自用的除外）
      const attrs = Array.from(el.attributes)
      for (const attr of attrs) {
        if (attr.name.startsWith('data-') && !isTiptapDataAttr(attr.name)) {
          el.removeAttribute(attr.name)
        }
        // 移除 MS Office 特有属性
        if (attr.name.startsWith('mso-') || attr.name.startsWith('xmlns')) {
          el.removeAttribute(attr.name)
        }
      }

      // 清理 inline style
      if (el.hasAttribute('style')) {
        const cleanedStyle = cleanInlineStyle(el.style)
        if (cleanedStyle) {
          el.setAttribute('style', cleanedStyle)
        } else {
          el.removeAttribute('style')
        }
      }

      // 移除空的 span（没有有意义的属性）
      if (tagName === 'span' && el.attributes.length === 0) {
        const fragment = document.createDocumentFragment()
        while (el.firstChild) {
          fragment.appendChild(el.firstChild)
        }
        node.replaceChild(fragment, el)
        continue
      }

      // 递归处理子节点
      cleanNode(el)

      // 移除空的块级元素（但保留 br, hr, img）
      if (isBlockElement(tagName) && !el.textContent?.trim() && !el.querySelector('img, br, hr')) {
        el.remove()
      }
    }
  }
}

function isTiptapDataAttr(name: string): boolean {
  return name === 'data-type' || name === 'data-checked'
}

function hasBlockChildren(el: HTMLElement): boolean {
  const blockTags = new Set([
    'p',
    'div',
    'h1',
    'h2',
    'h3',
    'h4',
    'h5',
    'h6',
    'ul',
    'ol',
    'li',
    'blockquote',
    'pre',
    'table',
    'hr',
  ])
  for (const child of el.children) {
    if (blockTags.has(child.tagName.toLowerCase())) {
      return true
    }
  }
  return false
}

function isBlockElement(tagName: string): boolean {
  return ['p', 'div', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'li'].includes(tagName)
}

function cleanInlineStyle(style: CSSStyleDeclaration): string {
  const result: string[] = []
  for (const prop of ALLOWED_STYLE_PROPS) {
    const value = style.getPropertyValue(prop)
    if (value) {
      // 跳过 MS Office 默认值
      if (prop === 'color' && value === 'windowtext') continue
      if (prop === 'background-color' && (value === 'transparent' || value === 'white')) continue
      result.push(`${prop}: ${value}`)
    }
  }
  return result.join('; ')
}

export const pasteHandlerPluginKey = new PluginKey('pasteHandler')

/**
 * PasteHandler 扩展
 * - 清理从网页/Office 粘贴的 HTML 格式
 * - 处理截图/剪贴板图片粘贴
 */
export const PasteHandler = Extension.create({
  name: 'pasteHandler',

  addProseMirrorPlugins() {
    const editor = this.editor

    return [
      new Plugin({
        key: pasteHandlerPluginKey,

        props: {
          handlePaste(_view, event) {
            const clipboardData = event.clipboardData
            if (!clipboardData) return false

            // 处理剪贴板图片（截图粘贴）
            const imageItems = Array.from(clipboardData.items).filter((item) =>
              item.type.startsWith('image/'),
            )

            if (imageItems.length > 0) {
              event.preventDefault()

              for (const item of imageItems) {
                const file = item.getAsFile()
                if (!file) continue

                const reader = new FileReader()
                reader.onload = async (e) => {
                  const base64 = e.target?.result as string
                  if (base64) {
                    try {
                      const filePath = await imageApi.saveImage(base64)
                      const assetUrl = convertFileSrc(filePath)
                      editor.chain().focus().setImage({ src: assetUrl }).run()
                    } catch {
                      // 保存失败时回退到 Base64 内联
                      editor.chain().focus().setImage({ src: base64 }).run()
                    }
                  }
                }
                reader.readAsDataURL(file)
              }
              return true
            }

            // 处理 HTML 粘贴内容
            const html = clipboardData.getData('text/html')
            if (html) {
              // 检测是否来自外部来源（非本编辑器）
              if (isExternalHtml(html)) {
                event.preventDefault()
                const cleaned = cleanPastedHtml(html)
                editor.commands.insertContent(cleaned, {
                  parseOptions: { preserveWhitespace: false },
                })
                return true
              }
            }

            // 其他情况走默认处理
            return false
          },

          // 处理拖拽图片
          handleDrop(view, event) {
            const dataTransfer = event.dataTransfer
            if (!dataTransfer) return false

            const files = Array.from(dataTransfer.files).filter((f) => f.type.startsWith('image/'))
            if (files.length === 0) return false

            event.preventDefault()

            for (const file of files) {
              const reader = new FileReader()
              reader.onload = async (e) => {
                const base64 = e.target?.result as string
                if (base64) {
                  let src = base64
                  try {
                    const filePath = await imageApi.saveImage(base64)
                    src = convertFileSrc(filePath)
                  } catch {
                    // 保存失败时回退到 Base64 内联
                  }
                  // 获取拖拽位置
                  const pos = view.posAtCoords({ left: event.clientX, top: event.clientY })
                  if (pos) {
                    editor.chain().focus().setTextSelection(pos.pos).setImage({ src }).run()
                  } else {
                    editor.chain().focus().setImage({ src }).run()
                  }
                }
              }
              reader.readAsDataURL(file)
            }
            return true
          },
        },
      }),
    ]
  },
})

/**
 * 检测 HTML 是否来自外部来源
 * TipTap 自身的复制会有特定的 data 属性
 */
function isExternalHtml(html: string): boolean {
  // 来自 MS Office
  if (html.includes('urn:schemas-microsoft-com:office') || html.includes('mso-')) return true
  // 来自 Google Docs
  if (html.includes('docs-internal-guid') || html.includes('google-docs')) return true
  // 来自网页（有大量 class 或 style）
  if ((html.match(/class="/g) || []).length > 3) return true
  if ((html.match(/style="/g) || []).length > 5) return true
  return false
}
