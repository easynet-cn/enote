/**
 * LRU (Least Recently Used) 缓存实现
 * 当缓存满时，自动淘汰最久未使用的项
 */
export class LRUCache<K, V> {
  private cache: Map<K, { value: V; timestamp: number }>
  private maxSize: number
  private ttl: number // Time to live in milliseconds

  /**
   * @param maxSize 最大缓存条目数
   * @param ttl 缓存过期时间（毫秒），0 表示永不过期
   */
  constructor(maxSize: number = 10, ttl: number = 0) {
    this.cache = new Map()
    this.maxSize = maxSize
    this.ttl = ttl
  }

  /**
   * 获取缓存值
   * 如果存在且未过期，会更新访问顺序
   */
  get(key: K): V | undefined {
    const entry = this.cache.get(key)

    if (!entry) {
      return undefined
    }

    // 检查是否过期
    if (this.ttl > 0 && Date.now() - entry.timestamp > this.ttl) {
      this.cache.delete(key)
      return undefined
    }

    // 更新访问顺序：删除后重新插入（Map 保持插入顺序）
    this.cache.delete(key)
    this.cache.set(key, { value: entry.value, timestamp: Date.now() })

    return entry.value
  }

  /**
   * 设置缓存值
   * 如果缓存已满，会淘汰最久未使用的项
   */
  set(key: K, value: V): void {
    // 如果 key 已存在，先删除（更新访问顺序）
    if (this.cache.has(key)) {
      this.cache.delete(key)
    }
    // 如果达到最大容量，删除最旧的项（Map 迭代器的第一个）
    else if (this.cache.size >= this.maxSize) {
      const oldestKey = this.cache.keys().next().value
      if (oldestKey !== undefined) {
        this.cache.delete(oldestKey)
      }
    }

    this.cache.set(key, { value, timestamp: Date.now() })
  }

  /**
   * 检查 key 是否存在且未过期
   */
  has(key: K): boolean {
    const entry = this.cache.get(key)

    if (!entry) {
      return false
    }

    // 检查是否过期
    if (this.ttl > 0 && Date.now() - entry.timestamp > this.ttl) {
      this.cache.delete(key)
      return false
    }

    return true
  }

  /**
   * 删除指定 key
   */
  delete(key: K): boolean {
    return this.cache.delete(key)
  }

  /**
   * 清空缓存
   */
  clear(): void {
    this.cache.clear()
  }

  /**
   * 获取当前缓存大小
   */
  get size(): number {
    return this.cache.size
  }

  /**
   * 清理所有过期项
   */
  cleanup(): void {
    if (this.ttl <= 0) return

    const now = Date.now()
    for (const [key, entry] of this.cache) {
      if (now - entry.timestamp > this.ttl) {
        this.cache.delete(key)
      }
    }
  }
}
