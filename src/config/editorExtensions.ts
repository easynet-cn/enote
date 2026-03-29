import type { AnyExtension } from '@tiptap/core'
import StarterKit from '@tiptap/starter-kit'
import TextAlign from '@tiptap/extension-text-align'
import { TextStyle } from '@tiptap/extension-text-style'
import Color from '@tiptap/extension-color'
import Highlight from '@tiptap/extension-highlight'
import Underline from '@tiptap/extension-underline'
import Link from '@tiptap/extension-link'
import Image from '@tiptap/extension-image'
import { Table } from '@tiptap/extension-table'
import { TableRow } from '@tiptap/extension-table-row'
import { TableCell } from '@tiptap/extension-table-cell'
import { TableHeader } from '@tiptap/extension-table-header'
import FontFamily from '@tiptap/extension-font-family'
import TaskList from '@tiptap/extension-task-list'
import TaskItem from '@tiptap/extension-task-item'
import { Markdown } from 'tiptap-markdown'
import Subscript from '@tiptap/extension-subscript'
import Superscript from '@tiptap/extension-superscript'
import Placeholder from '@tiptap/extension-placeholder'
import CharacterCount from '@tiptap/extension-character-count'
import {
  FontSize,
  Indent,
  SearchAndReplace,
  DragHandle,
  TableOfContents,
  PasteHandler,
} from '../extensions'

// ============================================================================
// 重型依赖延迟加载（lowlight ~100KB, KaTeX ~200KB）
// 首次调用 getRichTextExtensions/getMarkdownExtensions 时异步初始化
// ============================================================================

let _lowlightInstance: unknown = null
let _codeBlockLowlightExt: AnyExtension | null = null
let _mathematicsExt: AnyExtension | null = null
let _heavyDepsLoaded = false

/**
 * 异步加载重型扩展（代码高亮 + 数学公式）
 * 仅在首次创建编辑器时触发，后续使用缓存
 */
async function loadHeavyExtensions(): Promise<void> {
  if (_heavyDepsLoaded) return

  const [lowlightModule, codeBlockModule, mathModule] = await Promise.all([
    import('lowlight'),
    import('@tiptap/extension-code-block-lowlight'),
    import('@tiptap/extension-mathematics'),
  ])

  // KaTeX CSS
  await import('katex/dist/katex.min.css')

  _lowlightInstance = lowlightModule.createLowlight(lowlightModule.common)
  _codeBlockLowlightExt = codeBlockModule.default.configure({
    lowlight: _lowlightInstance,
    defaultLanguage: 'plaintext',
  })
  _mathematicsExt = mathModule.default

  _heavyDepsLoaded = true
}

/**
 * 构建基础扩展（不含重型依赖）
 */
function buildCoreExtensions(): AnyExtension[] {
  const extensions: AnyExtension[] = [
    StarterKit.configure({
      codeBlock: false, // 禁用默认代码块，使用带高亮的版本
    }),
    TextAlign.configure({
      types: ['heading', 'paragraph'],
    }),
    TextStyle,
    Link.configure({
      openOnClick: false,
      HTMLAttributes: {
        class: 'text-blue-500 underline cursor-pointer',
      },
    }),
    CharacterCount,
    DragHandle,
    TableOfContents,
    PasteHandler,
  ]

  // 代码块高亮（延迟加载后可用）
  if (_codeBlockLowlightExt) {
    extensions.push(_codeBlockLowlightExt)
  }

  return extensions
}

/**
 * 格式化扩展 - 文本格式相关
 */
const formattingExtensions: AnyExtension[] = [
  Color,
  Highlight.configure({ multicolor: true }),
  Underline,
  FontFamily,
  FontSize,
  Subscript,
  Superscript,
  Indent,
  SearchAndReplace,
]

/**
 * 构建媒体扩展（含延迟加载的数学公式）
 */
function buildMediaExtensions(): AnyExtension[] {
  const extensions: AnyExtension[] = [
    Image.configure({
      inline: true,
      allowBase64: true,
    }),
    Table.configure({
      resizable: true,
    }),
    TableRow,
    TableCell,
    TableHeader,
  ]

  // 数学公式 (KaTeX)（延迟加载后可用）
  if (_mathematicsExt) {
    extensions.push(_mathematicsExt)
  }

  return extensions
}

/**
 * 任务列表扩展
 */
const taskExtensions: AnyExtension[] = [
  TaskList,
  TaskItem.configure({
    nested: true,
  }),
]

/**
 * Markdown 扩展
 */
const markdownExtension = Markdown.configure({
  html: true,
  tightLists: true,
  tightListClass: 'tight',
  bulletListMarker: '-',
  linkify: true,
  breaks: true,
  transformPastedText: true,
  transformCopiedText: true,
})

// 缓存扩展数组，避免每次创建编辑器时重复构建
let _richTextExtensionsCache: AnyExtension[] | null = null
let _markdownExtensionsCache: AnyExtension[] | null = null

function buildPlaceholder(placeholder?: string): AnyExtension {
  return Placeholder.configure({
    placeholder: placeholder || '',
    emptyEditorClass: 'is-editor-empty',
    emptyNodeClass: 'is-empty',
  })
}

/**
 * 获取富文本编辑器扩展（异步 - 首次调用时加载重型依赖）
 */
export async function getRichTextExtensions(placeholder?: string): Promise<AnyExtension[]> {
  await loadHeavyExtensions()

  if (!placeholder && _richTextExtensionsCache) {
    return _richTextExtensionsCache
  }
  const extensions = [
    ...buildCoreExtensions(),
    buildPlaceholder(placeholder),
    ...formattingExtensions,
    ...buildMediaExtensions(),
    ...taskExtensions,
  ]
  if (!placeholder) {
    _richTextExtensionsCache = extensions
  }
  return extensions
}

/**
 * 获取 Markdown 编辑器扩展（异步 - 首次调用时加载重型依赖）
 */
export async function getMarkdownExtensions(placeholder?: string): Promise<AnyExtension[]> {
  await loadHeavyExtensions()

  if (!placeholder && _markdownExtensionsCache) {
    return _markdownExtensionsCache
  }
  const extensions = [
    ...buildCoreExtensions(),
    buildPlaceholder(placeholder),
    ...formattingExtensions,
    ...buildMediaExtensions(),
    ...taskExtensions,
    markdownExtension,
  ]
  if (!placeholder) {
    _markdownExtensionsCache = extensions
  }
  return extensions
}
