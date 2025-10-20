import { invoke } from '@tauri-apps/api/core'

import { Notebook } from '../types'

export const noteApi = {
    async getNotebooks(): Promise<Notebook[]> {
        return await invoke('find_all_notebooks')
    },
}