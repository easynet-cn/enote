import { invoke } from '@tauri-apps/api/core'

import {
  ContentType,
  Note,
  Notebook,
  NoteHistory,
  NoteHistorySearchPageParam,
  NoteSearchPageParam,
  NoteStatsResult,
  PageResult,
  Tag,
} from '../types'

export const noteApi = {
  async getNotebooks(): Promise<Notebook[]> {
    return await invoke('find_all_notebooks')
  },

  async createNotebook(notebook: Notebook): Promise<Notebook> {
    return await invoke('create_notebook', { notebook })
  },

  async updateNotebook(notebook: Notebook): Promise<Notebook> {
    return await invoke('update_notebook', { notebook })
  },

  async deleteNotebook(id: number): Promise<void> {
    return await invoke('delete_notebook_by_id', { id })
  },

  async createNote(
    notebookId: number,
    title: string,
    content: string,
    contentType: ContentType,
    tags: Tag[],
  ): Promise<Note> {
    const note: Note = {
      id: 0,
      notebookId: notebookId,
      title: title,
      content: content,
      contentType: contentType,
      tags: tags,
      createTime: null,
      updateTime: null,
    }

    return await invoke('create_note', { note })
  },

  async updateNote(
    id: number,
    notebookId: number,
    title: string,
    content: string,
    contentType: ContentType,
    tags: Tag[],
  ): Promise<Note> {
    const note: Note = {
      id: id,
      notebookId: notebookId,
      title: title,
      content: content,
      contentType: contentType,
      tags: tags,
      createTime: null,
      updateTime: null,
    }

    return await invoke('update_note', { note })
  },

  async deleteNote(id: number): Promise<void> {
    return await invoke('delete_note_by_id', { id })
  },

  async searchPageNotes(searchParam: NoteSearchPageParam): Promise<PageResult<Note>> {
    return await invoke('search_page_notes', { searchParam })
  },

  async noteStats(searchParam: NoteSearchPageParam): Promise<NoteStatsResult> {
    return await invoke('note_stats', { searchParam })
  },

  async geTags(): Promise<Tag[]> {
    return await invoke('find_all_tags')
  },

  async createTag(tag: Tag): Promise<Tag> {
    return await invoke('create_tag', { tag })
  },

  async updateTag(tag: Tag): Promise<Tag> {
    return await invoke('update_tag', { tag })
  },

  async deleteTag(id: number): Promise<void> {
    return await invoke('delete_tag_by_id', { id })
  },

  async searchPageNoteHistories(
    searchParam: NoteHistorySearchPageParam,
  ): Promise<PageResult<NoteHistory>> {
    return await invoke('search_page_note_histories', { searchParam })
  },
}
