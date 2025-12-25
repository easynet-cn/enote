/**
 * Vitest 测试配置文件
 *
 * 在每个测试文件运行前执行，用于：
 * - 配置全局测试环境
 * - Mock Tauri API
 * - 设置测试工具函数
 */

// Mock Tauri invoke API
// @ts-expect-error - Mock module
globalThis.__TAURI__ = {
  invoke: () => Promise.resolve(),
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
