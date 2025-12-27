import { describe, it, expect } from 'vitest'
import {
  showTagToTag,
  tagToShowTag,
  showTagsToTags,
  tagsToShowTags,
  noteToShowNote,
} from '../utils/converters'
import { ContentType } from '../types'
import type { Tag, ShowTag, Note } from '../types'

describe('converters utils', () => {
  describe('tag conversions', () => {
    const showTag: ShowTag = {
      id: '123',
      name: 'Test Tag',
      icon: 'tag-icon',
      cls: 'tag-class',
      sortOrder: 1,
    }

    const tag: Tag = {
      id: 123,
      name: 'Test Tag',
      icon: 'tag-icon',
      cls: 'tag-class',
      sortOrder: 1,
      createTime: '2024-01-01 00:00:00',
      updateTime: '2024-01-01 00:00:00',
    }

    it('should convert ShowTag to Tag', () => {
      const result = showTagToTag(showTag)

      expect(result.id).toBe(123)
      expect(result.name).toBe('Test Tag')
      expect(result.icon).toBe('tag-icon')
    })

    it('should convert Tag to ShowTag', () => {
      const result = tagToShowTag(tag)

      expect(result.id).toBe('123')
      expect(result.name).toBe('Test Tag')
      expect(result.icon).toBe('tag-icon')
    })

    it('should batch convert ShowTag[] to Tag[]', () => {
      const showTags = [showTag, { ...showTag, id: '456', name: 'Tag 2' }]
      const result = showTagsToTags(showTags)

      expect(result).toHaveLength(2)
      expect(result[0].id).toBe(123)
      expect(result[1].id).toBe(456)
    })

    it('should batch convert Tag[] to ShowTag[]', () => {
      const tags = [tag, { ...tag, id: 456, name: 'Tag 2' }]
      const result = tagsToShowTags(tags)

      expect(result).toHaveLength(2)
      expect(result[0].id).toBe('123')
      expect(result[1].id).toBe('456')
    })
  })

  describe('note conversions', () => {
    const note: Note = {
      id: 1,
      notebookId: 10,
      notebookName: 'My Notebook',
      title: 'Test Note',
      content: '<p>Hello World</p>',
      contentType: ContentType.Html,
      tags: [
        {
          id: 1,
          name: 'Tag1',
          createTime: null,
          updateTime: null,
        },
      ],
      createTime: '2024-01-01 00:00:00',
      updateTime: '2024-01-01 00:00:00',
    }

    it('should convert Note to ShowNote', () => {
      const result = noteToShowNote(note)

      expect(result.id).toBe('1')
      expect(result.notebookId).toBe('10')
      expect(result.notebookName).toBe('My Notebook')
      expect(result.title).toBe('Test Note')
      expect(result.content).toBe('<p>Hello World</p>')
      expect(result.contentType).toBe(ContentType.Html)
      expect(result.tags).toHaveLength(1)
      expect(result.tags[0].id).toBe('1')
    })

    it('should handle note without notebookName', () => {
      const noteWithoutName = { ...note, notebookName: undefined }
      const result = noteToShowNote(noteWithoutName)

      expect(result.notebookName).toBeUndefined()
    })
  })
})
