import { invoke } from '@tauri-apps/api/core'

// ============================================================================
// Help Manual API
// ============================================================================

export const helpApi = {
  async readManual(lang: string): Promise<string> {
    return await invoke('read_help_manual', { lang })
  },
}
