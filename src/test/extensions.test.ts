import { describe, it, expect } from 'vitest'
import { FontSize } from '../extensions/FontSize'
import { Indent } from '../extensions/Indent'
import { SearchAndReplace } from '../extensions/SearchAndReplace'
import { DragHandle } from '../extensions/DragHandle'
import { TableOfContents } from '../extensions/TableOfContents'
import { LazyImage } from '../extensions/LazyImage'

describe('TipTap Extensions', () => {
  describe('FontSize', () => {
    it('should have correct name', () => {
      expect(FontSize.name).toBe('fontSize')
    })

    it('should have default options', () => {
      const extension = FontSize.configure({})
      expect(extension.options).toBeDefined()
    })
  })

  describe('Indent', () => {
    it('should have correct name', () => {
      expect(Indent.name).toBe('indent')
    })

    it('should have default options', () => {
      const extension = Indent.configure({})
      expect(extension.options.indentUnit).toBeDefined()
      expect(extension.options.maxIndent).toBeDefined()
    })

    it('should allow custom indent unit', () => {
      const extension = Indent.configure({ indentUnit: '3em' })
      expect(extension.options.indentUnit).toBe('3em')
    })

    it('should allow custom max indent', () => {
      const extension = Indent.configure({ maxIndent: 10 })
      expect(extension.options.maxIndent).toBe(10)
    })
  })

  describe('SearchAndReplace', () => {
    it('should have correct name', () => {
      expect(SearchAndReplace.name).toBe('searchAndReplace')
    })

    it('should have storage with default values', () => {
      const extension = SearchAndReplace.configure({})
      // Storage is initialized when extension is added to editor
      expect(extension.storage).toBeDefined()
    })
  })

  describe('DragHandle', () => {
    it('should have correct name', () => {
      expect(DragHandle.name).toBe('dragHandle')
    })

    it('should have default options', () => {
      const extension = DragHandle.configure({})
      expect(extension.options.dragHandleClass).toBe('drag-handle')
      expect(extension.options.draggingClass).toBe('is-dragging')
    })
  })

  describe('TableOfContents', () => {
    it('should have correct name', () => {
      expect(TableOfContents.name).toBe('tableOfContents')
    })

    it('should have storage for items', () => {
      const extension = TableOfContents.configure({})
      expect(extension.storage).toBeDefined()
    })

    it('should accept onUpdate callback option', () => {
      const onUpdate = () => {}
      const extension = TableOfContents.configure({ onUpdate })
      expect(extension.options.onUpdate).toBe(onUpdate)
    })
  })

  describe('LazyImage', () => {
    it('should have correct name', () => {
      expect(LazyImage.name).toBe('lazyImage')
    })

    it('should have default options', () => {
      const extension = LazyImage.configure({})
      expect(extension.options.rootMargin).toBe('100px')
      expect(extension.options.threshold).toBe(0.1)
    })

    it('should allow custom rootMargin', () => {
      const extension = LazyImage.configure({ rootMargin: '200px' })
      expect(extension.options.rootMargin).toBe('200px')
    })

    it('should allow custom threshold', () => {
      const extension = LazyImage.configure({ threshold: 0.5 })
      expect(extension.options.threshold).toBe(0.5)
    })
  })
})
