import { invoke } from '@tauri-apps/api/core'

import { Note, Notebook, NotePageResult, NoteSearchPageParam, Tag } from '../types'

export const noteApi = {
    async getNotebooks(): Promise<Notebook[]> {
        return await invoke('find_all_notebooks')
    },

    async createNotebook(notebook: Notebook): Promise<Notebook> {
        return await invoke('create_notebook', { notebook })
    },

    async createNote(notebookId: number, title: string, content: string, tags: Tag[]): Promise<Note> {
        const note: Note = { id: 0, notebookId: notebookId, title: title, content: content, tags: tags, createTime: null, updateTime: null }

        return await invoke('create_note', { note })
    },

    async updateNote(id: number, notebookId: number, title: string, content: string, tags: Tag[]): Promise<Note> {
        const note: Note = { id: id, notebookId: notebookId, title: title, content: content, tags: tags, createTime: null, updateTime: null }

        return await invoke('update_note', { note })
    },

    async deleteNote(id: number): Promise<Note> {
        return await invoke('delete_note_by_id', { id })
    },

    async searchPageNotes(searchParam: NoteSearchPageParam): Promise<NotePageResult> {
        return await invoke('search_page_notes', { searchParam })
    },


    async createTag(tag: Tag): Promise<Tag> {
        return await invoke('create_tag', { tag })
    },
}