/**
 * Markdown 预处理 Web Worker
 * 在后台线程中处理大文件的 Markdown 预处理，避免阻塞主线程
 */

// 预处理 Markdown 内容，保留多个连续空行
const preprocessMarkdown = (content: string): string => {
  // 将连续的空行（3个及以上换行）转换为带 <br> 的格式，保留空行效果
  return content.replace(/\n{3,}/g, (match) => {
    const extraLines = match.length - 2
    return '\n\n' + '<br>\n'.repeat(extraLines)
  })
}

// 监听主线程消息
self.onmessage = (event: MessageEvent<{ type: string; content: string; id: number }>) => {
  const { type, content, id } = event.data

  if (type === 'preprocess') {
    const result = preprocessMarkdown(content)
    self.postMessage({ type: 'result', result, id })
  }
}

export {}
