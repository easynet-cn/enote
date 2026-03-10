import { invoke } from '@tauri-apps/api/core'

import {
  ContentType,
  Note,
  Notebook,
  NoteHistory,
  NoteHistorySearchPageParam,
  NoteSearchPageParam,
  NoteStatsResult,
  NoteLink,
  NoteTemplate,
  PageResult,
  Tag,
} from '../types'

export const settingsApi = {
  async getAll(): Promise<Record<string, string>> {
    return await invoke('get_all_settings')
  },

  async save(settings: Record<string, string>): Promise<void> {
    return await invoke('save_settings', { settings })
  },
}

export const trashApi = {
  async findDeletedNotes(): Promise<Note[]> {
    return await invoke('find_deleted_notes')
  },

  async restoreNote(id: number): Promise<void> {
    return await invoke('restore_note', { id })
  },

  async permanentDeleteNote(id: number): Promise<void> {
    return await invoke('permanent_delete_note', { id })
  },

  async emptyTrash(): Promise<void> {
    return await invoke('empty_trash')
  },
}

export const imageApi = {
  async saveImage(base64Data: string): Promise<string> {
    return await invoke('save_image', { base64Data })
  },

  async deleteImage(path: string): Promise<void> {
    return await invoke('delete_image', { path })
  },
}

export const backupApi = {
  async exportBackup(format: string, path: string): Promise<void> {
    return await invoke('export_backup', { format, path })
  },

  async importBackup(format: string, path: string): Promise<void> {
    return await invoke('import_backup', { format, path })
  },

  async autoBackup(): Promise<string> {
    return await invoke('auto_backup')
  },

  async cleanupOldBackups(maxCount: number): Promise<number> {
    return await invoke('cleanup_old_backups', { maxCount })
  },

  async listAutoBackups(): Promise<[string, number][]> {
    return await invoke('list_auto_backups')
  },
}

export const templateApi = {
  async findAll(): Promise<NoteTemplate[]> {
    return await invoke('find_all_templates')
  },
  async create(template: NoteTemplate): Promise<NoteTemplate> {
    return await invoke('create_template', { template })
  },
  async update(template: NoteTemplate): Promise<NoteTemplate> {
    return await invoke('update_template', { template })
  },
  async delete(id: number): Promise<void> {
    return await invoke('delete_template_by_id', { id })
  },
}

export const cryptoApi = {
  async encrypt(content: string, password: string): Promise<string> {
    return await invoke('encrypt_content', { content, password })
  },

  async decrypt(content: string, password: string): Promise<string> {
    return await invoke('decrypt_content', { content, password })
  },

  async isEncrypted(content: string): Promise<boolean> {
    return await invoke('is_content_encrypted', { content })
  },
}

export const noteLinkApi = {
  async findLinks(noteId: number): Promise<NoteLink[]> {
    return await invoke('find_note_links', { noteId })
  },

  async createLink(sourceNoteId: number, targetNoteId: number): Promise<void> {
    return await invoke('create_note_link', { sourceNoteId, targetNoteId })
  },

  async deleteLink(linkId: number): Promise<void> {
    return await invoke('delete_note_link', { linkId })
  },

  async searchLinkable(noteId: number, keyword: string): Promise<NoteLink[]> {
    return await invoke('search_linkable_notes', { noteId, keyword })
  },
}

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
    return await invoke('create_note', {
      note: {
        id: 0,
        notebookId,
        title,
        content,
        contentType,
        isPinned: 0,
        tags,
        createTime: null,
        updateTime: null,
        deletedAt: null,
      },
    })
  },

  async updateNote(
    id: number,
    notebookId: number,
    title: string,
    content: string,
    contentType: ContentType,
    tags: Tag[],
  ): Promise<Note> {
    return await invoke('update_note', {
      note: {
        id,
        notebookId,
        title,
        content,
        contentType,
        isPinned: 0,
        tags,
        createTime: null,
        updateTime: null,
        deletedAt: null,
      },
    })
  },

  async deleteNote(id: number): Promise<void> {
    return await invoke('delete_note_by_id', { id })
  },

  async toggleNotePin(id: number): Promise<Note> {
    return await invoke('toggle_note_pin', { id })
  },

  async reorderNotebooks(orders: [number, number][]): Promise<void> {
    return await invoke('reorder_notebooks', { orders })
  },

  async reorderTags(orders: [number, number][]): Promise<void> {
    return await invoke('reorder_tags', { orders })
  },

  async searchPageNotes(searchParam: NoteSearchPageParam): Promise<PageResult<Note>> {
    return await invoke('search_page_notes', { searchParam })
  },

  async noteStats(searchParam: NoteSearchPageParam): Promise<NoteStatsResult> {
    return await invoke('note_stats', { searchParam })
  },

  async getTags(): Promise<Tag[]> {
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
