export interface PageResult<T> {
    total: number;
    totalPages: number;
    data: T[];
}

export interface Notebook {
    id: number;
    parentId?: number;
    name: string;
    description?: string;
    icon?: string;
    cls?: string;
    sortOrder?: number;
    count?: number;
    createTime?: string | null;
    updateTime?: string | null;
}

export interface ShowNotebook {
    id: string;
    parentId?: number;
    name?: string;
    description?: string;
    icon?: string;
    cls?: string;
    sortOrder?: number;
    count?: number;
    createTime?: string | null;
    updateTime?: string | null;
}

export interface Tag {
    id: number;
    name: string;
    icon?: string;
    cls?: string;
    sortOrder?: number;
    createTime?: string | null;
    updateTime?: string | null;
}

export interface ShowTag {
    id: string;
    name: string;
    icon?: string;
    cls?: string;
    sortOrder?: number;
    createTime?: string;
    updateTime?: string;
}

export interface Note {
    id: number;
    notebookId: number;
    title: string;
    content: string;
    tags: Tag[];
    createTime: string | null;
    updateTime: string | null;
}

export interface ShowNote {
    id: string;
    notebookId?: string;
    title: string;
    content: string;
    tags?: ShowTag[];
    createTime?: string | null;
    updateTime?: string | null;
}
export interface NotePageResult extends PageResult<Note> {
    notebookCounts: Map<number, number>
}

export interface NoteHistoryExtra {
    notebookId: number;
    notebookName: string;
    noteId: number;
    title: string;
    tags: Tag[];
}

export interface NoteHistory {
    id: string;
    noteId: number;
    oldContent: string;
    newContent: string;
    extra: NoteHistoryExtra;
    operateType: number;
    operateTime: string;
    createTime: string;
}

export interface NoteSearchPageParam {
    pageIndex: number;
    pageSize: number;
    notebookId: number;
    tagId: number;
    keyword: string;
}

export interface NoteHistorySearchPageParam {
    pageIndex: number;
    pageSize: number;
    noteId: number;
}

export interface AppState {
    activeNotebook: string;
    activeTag: string;
    activeNote: string | null;
    noteSearchPageParam: NoteSearchPageParam;
    editMode: boolean;
    loading: boolean;
    historyPageIndex: number;
    historyPageSize: number;
    historyTotal: number;
}