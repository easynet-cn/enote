import { invoke } from '@tauri-apps/api/core'
import type { AppLog, AppLogSearchParam, LogFileInfo, PageResult } from '../types'

export const appLogApi = {
  /** 分页搜索应用日志 */
  async searchPage(param: AppLogSearchParam): Promise<PageResult<AppLog>> {
    return await invoke('search_page_app_logs', { param })
  },

  /** 删除单条日志 */
  async deleteById(id: number): Promise<void> {
    return await invoke('delete_app_log', { id })
  },

  /** 批量删除日志 */
  async deleteBatch(ids: number[]): Promise<number> {
    return await invoke('delete_batch_app_logs', { ids })
  },

  /** 清空所有日志 */
  async clearAll(): Promise<number> {
    return await invoke('clear_app_logs')
  },

  /** 按时间清理日志 */
  async cleanupBefore(before: string): Promise<number> {
    return await invoke('cleanup_app_logs_before', { before })
  },

  /** 列出日志文件 */
  async listLogFiles(): Promise<LogFileInfo[]> {
    return await invoke('list_log_files')
  },

  /** 读取日志文件内容 */
  async readLogFile(filename: string): Promise<string> {
    return await invoke('read_log_file', { filename })
  },

  /** 删除日志文件 */
  async deleteLogFiles(filenames: string[]): Promise<number> {
    return await invoke('delete_log_files', { filenames })
  },

  /** 清理旧日志文件 */
  async cleanupOldLogFiles(retentionDays: number): Promise<number> {
    return await invoke('cleanup_old_log_files', { retentionDays })
  },
}
