/**
 * Vitest 测试配置文件
 *
 * 在每个测试文件运行前执行，用于：
 * - 配置全局测试环境
 * - Mock Tauri API
 * - 配置 Vue Test Utils
 * - 设置测试工具函数
 */

import { vi } from 'vitest'
import { config } from '@vue/test-utils'

// 配置 Vue Test Utils
config.global.stubs = {
  // 禁用 teleport 以避免测试中的问题
  teleport: true,
  // 禁用过渡动画以加快测试速度
  transition: false,
  'transition-group': false,
}

// Mock Tauri invoke API
// @ts-expect-error - Mock module
globalThis.__TAURI__ = {
  invoke: vi.fn().mockResolvedValue(undefined),
}

// Mock matchMedia for components that use it
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: (query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: () => {},
    removeListener: () => {},
    addEventListener: () => {},
    removeEventListener: () => {},
    dispatchEvent: () => false,
  }),
})

// Mock ResizeObserver
class MockResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
}
globalThis.ResizeObserver = MockResizeObserver

// Mock IntersectionObserver
class MockIntersectionObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
}
// @ts-expect-error - Mock class
globalThis.IntersectionObserver = MockIntersectionObserver

export {}
