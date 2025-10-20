export interface Category {
    id: number;
    parent_id: number;
    name: string;
    description: string;
    icon: string;
    cls: string;
    sort_order: number;
    create_time: string;
    update_time: string;
}

export interface Notebook {
    id: number;
    name: string;
    count: number;
    icon: string;
}

export interface Tag {
    id: number;
    name: string;
    color: string;
}

export interface Note {
    id: number;
    notebookId: number;
    title: string;
    content: string;
    tags: number[];
    createdAt: string;
    updatedAt: string;
}

export interface AppState {
    activeNotebook: number;
    activeNote: number | null;
    searchQuery: string;
    editMode: boolean;
    loading: boolean;
}