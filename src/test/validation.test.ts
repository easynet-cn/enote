import { describe, it, expect } from 'vitest'
import { parseId, isTemporaryId, validateNoteTitle } from '../utils/validation'

describe('validation utils', () => {
  describe('parseId', () => {
    it('should parse valid numeric string to number', () => {
      expect(parseId('123')).toBe(123)
      expect(parseId('0')).toBe(0)
      expect(parseId('999999')).toBe(999999)
    })

    it('should return 0 for undefined or empty string', () => {
      expect(parseId(undefined)).toBe(0)
      expect(parseId('')).toBe(0)
    })

    it('should return 0 for non-numeric strings', () => {
      expect(parseId('abc')).toBe(0)
      // parseInt('12abc') returns 12, which is valid behavior
      expect(parseId('12abc')).toBe(12)
    })

    it('should handle temporary IDs', () => {
      expect(parseId('0-1234567890')).toBe(0)
    })
  })

  describe('isTemporaryId', () => {
    it('should return true for temporary IDs starting with "0-"', () => {
      expect(isTemporaryId('0-1234567890')).toBe(true)
      expect(isTemporaryId('0-abc')).toBe(true)
    })

    it('should return false for regular IDs', () => {
      expect(isTemporaryId('123')).toBe(false)
      expect(isTemporaryId('1')).toBe(false)
    })

    it('should return true for "0" (treated as temporary)', () => {
      expect(isTemporaryId('0')).toBe(true)
    })

    it('should return true for undefined or null (no ID means new)', () => {
      expect(isTemporaryId(undefined)).toBe(true)
      // @ts-expect-error testing null
      expect(isTemporaryId(null)).toBe(true)
    })
  })

  describe('validateNoteTitle', () => {
    it('should return valid for non-empty titles', () => {
      expect(validateNoteTitle('My Note')).toEqual({ valid: true })
      expect(validateNoteTitle('测试笔记')).toEqual({ valid: true })
      expect(validateNoteTitle('a')).toEqual({ valid: true })
    })

    it('should return invalid for empty titles', () => {
      const result = validateNoteTitle('')
      expect(result.valid).toBe(false)
      expect(result.error).toBeDefined()
    })

    it('should return invalid for whitespace-only titles', () => {
      const result = validateNoteTitle('   ')
      expect(result.valid).toBe(false)
      expect(result.error).toBeDefined()
    })
  })
})
