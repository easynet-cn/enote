import { invoke } from '@tauri-apps/api/core'

import { Note, NotebookResult, NoteSearchPageParam, PageResult, Tag } from '../types'

export const noteApi = {
    async getNotebooks(): Promise<NotebookResult> {
        return await invoke('find_all_notebooks')
    },

    async createNote(notebookId: number, title: string, content: string, tags: Tag[]): Promise<Note> {
        const note: Note = { id: 0, notebookId: notebookId, title: title, content: content, tags: [], createTime: null, updateTime: null }

        return await invoke('create_note', { note })
    },

    async searchPageNotes(searchParam: NoteSearchPageParam): Promise<PageResult<Note>> {
        return await invoke('search_page_notes', { searchParam })
    },
}