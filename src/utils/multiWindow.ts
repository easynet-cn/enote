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

/**
 * 在新窗口中打开帮助手册
 *
 * 窗口为非模态、可拖拽、可最大化/最小化的悬浮窗口
 * 设置 alwaysOnTop 使其可悬浮在主窗口之上
 */
export async function openHelpInNewWindow() {
  const label = `help-manual-${Date.now()}`

  const webview = new WebviewWindow(label, {
    url: 'index.html?helpWindow=true',
    title: 'ENote - Help Manual',
    width: 1000,
    height: 800,
    minWidth: 700,
    minHeight: 500,
    center: true,
    resizable: true,
    decorations: true,
    maximizable: true,
    minimizable: true,
  })

  webview.once('tauri://error', (e) => {
    console.error('Failed to open help window:', e)
  })
}
