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

/**
 * 基础扩展 - 所有编辑器都需要的核心扩展
 */
export const coreExtensions: AnyExtension[] = [
  StarterKit,
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
]

/**
 * 格式化扩展 - 文本格式相关
 */
export const formattingExtensions: AnyExtension[] = [
  Color,
  Highlight.configure({ multicolor: true }),
  Underline,
  FontFamily,
]

/**
 * 媒体扩展 - 图片、表格等
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
