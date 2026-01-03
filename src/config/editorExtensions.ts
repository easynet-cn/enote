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
import CodeBlockLowlight from '@tiptap/extension-code-block-lowlight'
import Mathematics from '@tiptap/extension-mathematics'
import { common, createLowlight } from 'lowlight'
import { FontSize, Indent, SearchAndReplace, DragHandle, TableOfContents } from '../extensions'
import 'katex/dist/katex.min.css'

// 创建带有常用语言的 lowlight 实例
const lowlight = createLowlight(common)

/**
 * 基础扩展 - 所有编辑器都需要的核心扩展
 */
export const coreExtensions: AnyExtension[] = [
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
  // 代码块高亮
  CodeBlockLowlight.configure({
    lowlight,
    defaultLanguage: 'plaintext',
  }),
  // 占位符
  Placeholder.configure({
    placeholder: '开始编写笔记...',
    emptyEditorClass: 'is-editor-empty',
    emptyNodeClass: 'is-empty',
  }),
  // 字符统计
  CharacterCount,
  // 拖拽手柄
  DragHandle,
  // 目录生成
  TableOfContents,
]

/**
 * 格式化扩展 - 文本格式相关
 */
export const formattingExtensions: AnyExtension[] = [
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
 * 媒体扩展 - 图片、表格、数学公式等
 */
export const mediaExtensions: AnyExtension[] = [
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
  // 数学公式 (KaTeX)
  Mathematics,
]

/**
 * 任务列表扩展
 */
export const taskExtensions: AnyExtension[] = [
  TaskList,
  TaskItem.configure({
    nested: true,
  }),
]

/**
 * Markdown 扩展
 */
export const markdownExtension = Markdown.configure({
  html: true,
  tightLists: true,
  tightListClass: 'tight',
  bulletListMarker: '-',
  linkify: true,
  breaks: true,
  transformPastedText: true,
  transformCopiedText: true,
})

/**
 * 获取富文本编辑器扩展
 */
export function getRichTextExtensions(): AnyExtension[] {
  return [...coreExtensions, ...formattingExtensions, ...mediaExtensions, ...taskExtensions]
}

/**
 * 获取 Markdown 编辑器扩展
 */
export function getMarkdownExtensions(): AnyExtension[] {
  return [
    ...coreExtensions,
    ...formattingExtensions,
    ...mediaExtensions,
    ...taskExtensions,
    markdownExtension,
  ]
}

/**
 * 获取只读查看器扩展（最小化）
 */
export function getViewerExtensions(): AnyExtension[] {
  return [...coreExtensions, ...formattingExtensions, ...mediaExtensions, ...taskExtensions]
}
