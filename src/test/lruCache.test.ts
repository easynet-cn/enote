import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { LRUCache } from '../utils/lruCache'

describe('LRUCache', () => {
  beforeEach(() => {
    vi.useFakeTimers()
  })

  afterEach(() => {
    vi.useRealTimers()
  })

  describe('basic operations', () => {
    it('should set and get values', () => {
      const cache = new LRUCache<string, number>(3)

      cache.set('a', 1)
      cache.set('b', 2)

      expect(cache.get('a')).toBe(1)
      expect(cache.get('b')).toBe(2)
    })

    it('should return undefined for non-existent keys', () => {
      const cache = new LRUCache<string, number>(3)

      expect(cache.get('nonexistent')).toBeUndefined()
    })

    it('should update existing values', () => {
      const cache = new LRUCache<string, number>(3)

      cache.set('a', 1)
      cache.set('a', 2)

      expect(cache.get('a')).toBe(2)
      expect(cache.size).toBe(1)
    })

    it('should delete values', () => {
      const cache = new LRUCache<string, number>(3)

      cache.set('a', 1)
      cache.delete('a')

      expect(cache.get('a')).toBeUndefined()
      expect(cache.has('a')).toBe(false)
    })

    it('should clear all values', () => {
      const cache = new LRUCache<string, number>(3)

      cache.set('a', 1)
      cache.set('b', 2)
      cache.clear()

      expect(cache.size).toBe(0)
      expect(cache.get('a')).toBeUndefined()
    })
  })

  describe('LRU eviction', () => {
    it('should evict least recently used item when full', () => {
      const cache = new LRUCache<string, number>(3)

      cache.set('a', 1)
      cache.set('b', 2)
      cache.set('c', 3)
      cache.set('d', 4) // This should evict 'a'

      expect(cache.get('a')).toBeUndefined()
      expect(cache.get('b')).toBe(2)
      expect(cache.get('c')).toBe(3)
      expect(cache.get('d')).toBe(4)
    })

    it('should update access order on get', () => {
      const cache = new LRUCache<string, number>(3)

      cache.set('a', 1)
      cache.set('b', 2)
      cache.set('c', 3)

      // Access 'a' to make it most recently used
      cache.get('a')

      // Adding 'd' should now evict 'b' instead of 'a'
      cache.set('d', 4)

      expect(cache.get('a')).toBe(1)
      expect(cache.get('b')).toBeUndefined()
      expect(cache.get('c')).toBe(3)
      expect(cache.get('d')).toBe(4)
    })

    it('should update access order on set', () => {
      const cache = new LRUCache<string, number>(3)

      cache.set('a', 1)
      cache.set('b', 2)
      cache.set('c', 3)

      // Update 'a' to make it most recently used
      cache.set('a', 10)

      // Adding 'd' should evict 'b'
      cache.set('d', 4)

      expect(cache.get('a')).toBe(10)
      expect(cache.get('b')).toBeUndefined()
    })
  })

  describe('TTL (Time To Live)', () => {
    it('should expire items after TTL', () => {
      const cache = new LRUCache<string, number>(3, 1000) // 1 second TTL

      cache.set('a', 1)

      // Before TTL expires
      expect(cache.get('a')).toBe(1)

      // Advance time past TTL
      vi.advanceTimersByTime(1500)

      // After TTL expires
      expect(cache.get('a')).toBeUndefined()
    })

    it('should refresh TTL on get', () => {
      const cache = new LRUCache<string, number>(3, 1000)

      cache.set('a', 1)

      // Advance time but not past TTL
      vi.advanceTimersByTime(500)
      expect(cache.get('a')).toBe(1) // This refreshes the timestamp

      // Advance another 800ms (total 1300ms from original set)
      vi.advanceTimersByTime(800)

      // Should still be valid because we refreshed at 500ms
      expect(cache.get('a')).toBe(1)
    })

    it('should not expire with TTL of 0', () => {
      const cache = new LRUCache<string, number>(3, 0)

      cache.set('a', 1)

      vi.advanceTimersByTime(10000000)

      expect(cache.get('a')).toBe(1)
    })

    it('should cleanup expired items', () => {
      const cache = new LRUCache<string, number>(10, 1000)

      cache.set('a', 1)
      cache.set('b', 2)

      vi.advanceTimersByTime(500)
      cache.set('c', 3) // Added later

      vi.advanceTimersByTime(600) // Now 'a' and 'b' are expired, 'c' is not

      cache.cleanup()

      expect(cache.has('a')).toBe(false)
      expect(cache.has('b')).toBe(false)
      expect(cache.has('c')).toBe(true)
    })
  })

  describe('has method', () => {
    it('should return true for existing keys', () => {
      const cache = new LRUCache<string, number>(3)

      cache.set('a', 1)

      expect(cache.has('a')).toBe(true)
    })

    it('should return false for non-existent keys', () => {
      const cache = new LRUCache<string, number>(3)

      expect(cache.has('nonexistent')).toBe(false)
    })

    it('should return false for expired keys', () => {
      const cache = new LRUCache<string, number>(3, 1000)

      cache.set('a', 1)

      vi.advanceTimersByTime(1500)

      expect(cache.has('a')).toBe(false)
    })
  })
})
