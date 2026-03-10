import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

/**
 * 在新窗口中打开笔记
 *
 * 使用 Tauri WebviewWindow API 创建新的编辑器窗口
 * 通过 URL 查询参数传递笔记 ID
 */
export async function openNoteInNewWindow(noteId: number | string) {
  const label = `note-${noteId}-${Date.now()}`

  const webview = new WebviewWindow(label, {
    url: `index.html?noteWindow=${noteId}`,
    title: 'ENote',
    width: 900,
    height: 700,
    minWidth: 600,
    minHeight: 400,
    center: true,
    resizable: true,
    decorations: true,
  })

  // 监听创建错误
  webview.once('tauri://error', (e) => {
    console.error('Failed to open note window:', e)
  })
}
