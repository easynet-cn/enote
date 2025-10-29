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
    updateTime?: String | null;
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
    updateTime?: String | null;
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
    notebookId?: String;
    title: string;
    content: string;
    tags?: ShowTag[];
    createTime?: string | null;
    updateTime?: string | null;
}
export interface NotePageResult extends PageResult<Note> {
    notebookCounts: Map<number, number>
}

export interface NoteHistory {
    id: string;
    noteId: number;
    oldContent: string;
    newConmtent: string;
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

export interface AppState {
    activeNotebook: string;
    activeNote: string | null;
    noteSearchPageParam: NoteSearchPageParam;
    editMode: boolean;
    loading: boolean;
}