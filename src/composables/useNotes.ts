import { ref, reactive, computed, onMounted } from 'vue';
import { useDateFormat, useNow } from '@vueuse/core'
import type { AppState, ShowNotebook, ShowTag, ShowNote, NotePageResult } from '../types';
import { ElNotification } from 'element-plus';
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

const query = ref<string>('');

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
        const notification = ElNotification({
            title: '',
            message: '正在加载笔记本',
            type: 'success',
            duration: 0,
        })

        try {
            const data = (await noteApi.getNotebooks()).map((notebook): ShowNotebook => (
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
        } catch (error) {
            ElNotification({
                title: '',
                message: String(error),
                type: 'error',
                duration: 0,
            })
        }

        notification.close();
    }

    // 获取笔记
    const searchNotes = async () => {
        const notification = ElNotification({
            title: '',
            message: '正在加载笔记',
            type: 'success',
            duration: 0,
        })

        try {
            const pageResult: NotePageResult = await noteApi.searchPageNotes(state.noteSearchPageParam);

            let countMap = new Map<number, number>();
            let totalCount = 0;

            Object.entries(pageResult.notebookCounts).forEach(([k, v]) => {
                countMap.set(Number.parseInt(k) ?? 0, v);
                totalCount += v;
            })

            notebooks.value.forEach(e => {
                if (e.id == '0') {
                    e.count = totalCount;
                } else {
                    const id = Number.parseInt(e.id) ?? 0;

                    e.count = countMap.get(id) || 0;
                }
            });

            return pageResult.data.map((note): ShowNote => (
                {
                    id: String(note.id),
                    notebookId: String(note.notebookId),
                    title: note.title,
                    content: note.content,
                    createTime: note.createTime,
                    updateTime: note.updateTime,
                }
            ));
        } catch (error) {
            ElNotification({
                title: '',
                message: String(error),
                type: 'error',
                duration: 0,
            })
        } finally {
            notification.close();
        }

        return new Array();
    }

    const activeNoteData = computed(() => {
        return notes.value.find(note => note.id === state.activeNote) || null;
    });

    // 方法
    const setActiveNotebook = async (notebookId: string) => {
        state.activeNotebook = notebookId;
        state.activeNote = null;
        state.noteSearchPageParam.notebookId = Number.parseInt(notebookId);

        notes.value = await searchNotes();
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

        const notification = ElNotification({
            title: '',
            message: '正在保存笔记',
            type: 'success',
            duration: 0,
        })

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

            notes.value = await searchNotes();

            setActiveNote(newNoteId);
        } catch (error) {
            ElNotification({
                title: '',
                message: String(error),
                type: 'error',
                duration: 0,
            })
        } finally {
            notification.close();
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
            const notification = ElNotification({
                title: '',
                message: '正在删除笔记',
                type: 'success',
                duration: 0,
            })


            try {
                await noteApi.deleteNote(Number.parseInt(noteId));

                state.noteSearchPageParam.pageIndex = 1;
            } catch (error) {
                ElNotification({
                    title: '',
                    message: String(error),
                    type: 'error',
                    duration: 0,
                })
            } finally {
                notification.close();
            }

            notes.value = await searchNotes();
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

    const handleUpdateSearchQuery = async () => {
        state.noteSearchPageParam.keyword = query.value;

        notes.value = await searchNotes();
    }

    // 初始化
    const initialize = async () => {
        state.loading = true;

        const notification = ElNotification({
            title: '',
            message: '正在加载',
            type: 'success',
            duration: 0,
        })


        try {
            await getNotebookResult();

            if (notebooks.value.length > 0) {
                await setActiveNotebook(notebooks.value[0].id)
            }


            notes.value = await searchNotes();

        } catch (error) {
            ElNotification({
                title: '错误信息',
                type: 'error',
                message: String(error),
                duration: 0,
            })
        } finally {
            state.loading = false
            notification.close();
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
        query,
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
        getTagById,
        handleUpdateSearchQuery
    };
}