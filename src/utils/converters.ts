import { parseId } from './validation'
import type { Tag, ShowTag, Notebook, ShowNotebook, Note, ShowNote } from '../types'
import { ContentType } from '../types'

/**
 * 类型转换工具函数
 * 统一处理前端展示类型与后端 API 类型之间的转换
 */

// ============ Tag 转换 ============

/**
 * ShowTag -> Tag (前端 -> API)
 */
export const showTagToTag = (showTag: ShowTag): Tag => ({
  id: parseId(showTag.id),
  name: showTag.name,
  icon: showTag.icon,
  cls: showTag.cls,
  sortOrder: showTag.sortOrder,
})

/**
 * Tag -> ShowTag (API -> 前端)
 */
export const tagToShowTag = (tag: Tag): ShowTag => ({
  id: String(tag.id),
  name: tag.name,
  icon: tag.icon,
  cls: tag.cls,
  sortOrder: tag.sortOrder,
  createTime: tag.createTime ?? undefined,
  updateTime: tag.updateTime ?? undefined,
})

/**
 * ShowTag[] -> Tag[] (批量转换)
 */
export const showTagsToTags = (showTags: ShowTag[]): Tag[] => showTags.map(showTagToTag)

/**
 * Tag[] -> ShowTag[] (批量转换)
 */
export const tagsToShowTags = (tags: Tag[]): ShowTag[] => tags.map(tagToShowTag)

// ============ Notebook 转换 ============

/**
 * ShowNotebook -> Notebook (前端 -> API)
 */
export const showNotebookToNotebook = (showNotebook: ShowNotebook): Notebook => ({
  id: parseId(showNotebook.id),
  parentId: showNotebook.parentId,
  name: showNotebook.name ?? '',
  description: showNotebook.description,
  icon: showNotebook.icon,
  cls: showNotebook.cls,
  sortOrder: showNotebook.sortOrder,
  count: showNotebook.count,
})

/**
 * Notebook -> ShowNotebook (API -> 前端)
 */
export const notebookToShowNotebook = (notebook: Notebook): ShowNotebook => ({
  id: String(notebook.id),
  parentId: notebook.parentId,
  name: notebook.name,
  description: notebook.description,
  icon: notebook.icon,
  cls: notebook.cls,
  sortOrder: notebook.sortOrder,
  count: notebook.count,
  createTime: notebook.createTime,
  updateTime: notebook.updateTime,
})

/**
 * Notebook[] -> ShowNotebook[] (批量转换)
 */
export const notebooksToShowNotebooks = (notebooks: Notebook[]): ShowNotebook[] =>
  notebooks.map(notebookToShowNotebook)

// ============ Note 转换 ============

/**
 * Note -> ShowNote (API -> 前端)
 */
export const noteToShowNote = (note: Note): ShowNote => ({
  id: String(note.id),
  notebookId: String(note.notebookId),
  notebookName: note.notebookName ?? undefined,
  title: note.title,
  content: note.content,
  contentType: note.contentType,
  tags: tagsToShowTags(note.tags),
  createTime: note.createTime,
  updateTime: note.updateTime,
})

/**
 * Note[] -> ShowNote[] (批量转换)
 */
export const notesToShowNotes = (notes: Note[]): ShowNote[] => notes.map(noteToShowNote)

/**
 * 创建用于 API 调用的笔记数据对象
 */
export const createNotePayload = (showNote: ShowNote) => ({
  notebookId: parseId(showNote.notebookId),
  title: showNote.title,
  content: showNote.content,
  contentType: showNote.contentType ?? ContentType.Html,
  tags: showNote.tags?.map(showTagToTag) ?? [],
})
