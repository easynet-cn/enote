import { ref, reactive, computed, onMounted, watchEffect } from 'vue';
import { useDateFormat, useNow } from '@vueuse/core'
import type { AppState, ShowNotebook, ShowTag, ShowNote } from '../types';
import { noteApi } from '../api/note';

const notebooks = ref<ShowNotebook[]>([
    { id: '0', name: '全部', count: 0, icon: '📒' },
]);

const tags = ref<ShowTag[]>([
    { id: '1', name: '重要', cls: 'text-red-500' },
    { id: '2', name: '待办', cls: 'text-yellow-500' },
    { id: '3', name: '已完成', cls: 'text-green-500' },
    { id: '4', name: '灵感', cls: 'text-blue-500' }
]);

const notes = ref<ShowNote[]>([]);

// 状态管理
const state = reactive<AppState>({
    activeNotebook: '',
    activeNote: null,
    noteSearchPageParam: { pageIndex: 1, pageSize: 50, notebookId: 0, tagId: 0, keyword: '' },
    editMode: false,
    loading: false,
});

export function useNotes() {
    // 获取笔记本
    const getNotebookResult = async () => {
        const notebookResult = await noteApi.getNotebooks();
        const data = notebookResult.notebooks.map((notebook): ShowNotebook => (
            {
                id: String(notebook.id),
                parentId: notebook.parentId,
                name: notebook.name,
                description: notebook.description,
                icon: notebook.icon,
                cls: notebook.cls,
                count: notebook.count,
                createTime: notebook.createTime,
                updateTime: notebook.updateTime,
            }
        ));

        notebooks.value = [...[notebooks.value[0]], ...data];

        notebooks.value[0].count = notebookResult.totalCount;
    }

    // 获取笔记
    const searchNotes = async () => {
        return (await noteApi.searchPageNotes(state.noteSearchPageParam)).data.map((note): ShowNote => (
            {
                id: String(note.id),
                notebookId: String(note.notebookId),
                title: note.title,
                content: note.content,
                createTime: note.createTime,
                updateTime: note.updateTime,
            }
        ));
    }

    // 计算属性
    watchEffect(async () => {
        notes.value = await searchNotes();
    });

    const activeNoteData = computed(() => {
        return notes.value.find(note => note.id === state.activeNote) || null;
    });

    // 方法
    const setActiveNotebook = (notebookId: string) => {
        state.activeNotebook = notebookId;
        state.activeNote = null;
        state.noteSearchPageParam.notebookId = Number.parseInt(notebookId);
    };

    const setActiveNote = (noteId: string) => {
        state.activeNote = noteId;
        state.editMode = false;
    };

    const createNewNote = () => {
        const now = useNow();
        const nowStr = useDateFormat(now, 'YYYY-MM-DD HH:mm:ss').value;

        const newNote: ShowNote = {
            id: 0 + '-' + now.value.getTime(),
            notebookId: state.activeNotebook,
            title: '',
            content: '',
            tags: [],
            createTime: nowStr,
            updateTime: nowStr
        };

        notes.value.unshift(newNote);
        state.activeNote = newNote.id;
        state.editMode = true;
    };

    const saveNote = async () => {
        if (!state.activeNote || !activeNoteData.value) return

        try {
            const noteId = state.activeNote;
            let newNoteId = noteId;

            if (noteId.indexOf('-') < 0) {
                await noteApi.updateNote(
                    Number.parseInt(noteId),
                    Number.parseInt(state.activeNotebook),
                    activeNoteData.value.title,
                    activeNoteData.value.content,
                    []
                )
            } else {
                let newNote = await noteApi.createNote(
                    Number.parseInt(state.activeNotebook),
                    activeNoteData.value.title,
                    activeNoteData.value.content,
                    []
                )

                newNoteId = String(newNote.id);
            }

            await Promise.all([getNotebookResult(), searchNotes()]);

            setActiveNote(newNoteId);
        } catch (error) {
            console.error('Failed to save note:', error)
        }
    };

    const cancelEdit = () => {
        state.editMode = false;
    };

    const deleteNote = async () => {
        if (!state.activeNote) return;

        const noteId = state.activeNote;

        if (noteId.indexOf('-') > -1) {
            const noteIndex = notes.value.findIndex(note => note.id === state.activeNote);

            if (noteIndex !== -1) {
                notes.value.splice(noteIndex, 1);
                state.activeNote = null;
            }
        } else {
            await noteApi.deleteNote(Number.parseInt(noteId));

            state.noteSearchPageParam.pageIndex = 1;

            await Promise.all([getNotebookResult(), searchNotes()]);
        }
    };

    const updateNoteTitle = (title: string) => {
        if (!state.activeNote) return;

        const noteIndex = notes.value.findIndex(note => note.id === state.activeNote);

        if (noteIndex !== -1) {
            notes.value[noteIndex].title = title;
        }
    };

    const updateNoteContent = (content: string) => {
        if (!state.activeNote) return;

        const noteIndex = notes.value.findIndex(note => note.id === state.activeNote);

        if (noteIndex !== -1) {
            notes.value[noteIndex].content = content;
        }
    };

    const getTagById = (tagId: string) => {
        return tags.value.find(tag => tag.id === tagId);
    };

    // 初始化
    const initialize = async () => {
        state.loading = true

        try {
            await getNotebookResult();

            if (notebooks.value.length > 0) {
                await setActiveNotebook(notebooks.value[0].id)
            }

        } catch (error) {
            console.error('Failed to initialize:', error)
        } finally {
            state.loading = false
        }
    }

    // 初始化
    onMounted(() => {
        initialize();
    });

    return {
        notebooks,
        tags,
        notes,
        state,
        activeNoteData,
        setActiveNotebook,
        setActiveNote,
        createNewNote,
        saveNote,
        cancelEdit,
        deleteNote,
        updateNoteTitle,
        updateNoteContent,
        getTagById
    };
}