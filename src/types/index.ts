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
}