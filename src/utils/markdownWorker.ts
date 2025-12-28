/**
 * Markdown Worker 管理器
 * 使用 Web Worker 在后台线程处理 Markdown 预处理
 */

let worker: Worker | null = null
let requestId = 0
const pendingRequests = new Map<number, (result: string) => void>()

// 大文件阈值（字节），超过此大小使用 Worker
const LARGE_FILE_THRESHOLD = 50 * 1024 // 50KB

// 预处理 Markdown 内容（同步版本，用于小文件）
const preprocessMarkdownSync = (content: string): string => {
  return content.replace(/\n{3,}/g, (match) => {
    const extraLines = match.length - 2
    return '\n\n' + '<br>\n'.repeat(extraLines)
  })
}

// 初始化 Worker
const initWorker = () => {
  if (worker) return worker

  try {
    worker = new Worker(new URL('../workers/markdown.worker.ts', import.meta.url), {
      type: 'module',
    })

    worker.onmessage = (event: MessageEvent<{ type: string; result: string; id: number }>) => {
      const { type, result, id } = event.data
      if (type === 'result') {
        const resolve = pendingRequests.get(id)
        if (resolve) {
          resolve(result)
          pendingRequests.delete(id)
        }
      }
    }

    worker.onerror = (error) => {
      console.error('Markdown Worker error:', error)
      // 清理所有待处理请求
      pendingRequests.clear()
    }
  } catch {
    // Worker 创建失败，回退到同步处理
    worker = null
  }

  return worker
}

/**
 * 预处理 Markdown 内容
 * 小文件同步处理，大文件使用 Worker 异步处理
 */
export const preprocessMarkdown = async (content: string): Promise<string> => {
  // 计算内容大小
  const contentSize = new Blob([content]).size

  // 小文件直接同步处理
  if (contentSize < LARGE_FILE_THRESHOLD) {
    return preprocessMarkdownSync(content)
  }

  // 大文件使用 Worker
  const w = initWorker()

  if (!w) {
    // Worker 不可用，回退到同步处理
    return preprocessMarkdownSync(content)
  }

  return new Promise((resolve) => {
    const id = ++requestId
    pendingRequests.set(id, resolve)
    w.postMessage({ type: 'preprocess', content, id })

    // 超时处理（2秒）
    setTimeout(() => {
      if (pendingRequests.has(id)) {
        pendingRequests.delete(id)
        resolve(preprocessMarkdownSync(content))
      }
    }, 2000)
  })
}

/**
 * 同步预处理（用于不需要异步的场景）
 */
export const preprocessMarkdownSync_ = preprocessMarkdownSync

/**
 * 销毁 Worker
 */
export const terminateMarkdownWorker = () => {
  if (worker) {
    worker.terminate()
    worker = null
    pendingRequests.clear()
  }
}
