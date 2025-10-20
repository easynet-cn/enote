import { ref, reactive, computed, onMounted } from 'vue';
import type { Notebook, Tag, Note, AppState } from '../types';
import { noteApi } from '../api/note';

const notebooks = ref<Notebook[]>([
    { id: 0, name: '全部', count: 5, icon: '📒' },
]);

const tags = ref<Tag[]>([
    { id: 1, name: '重要', color: 'text-red-500' },
    { id: 2, name: '待办', color: 'text-yellow-500' },
    { id: 3, name: '已完成', color: 'text-green-500' },
    { id: 4, name: '灵感', color: 'text-blue-500' }
]);

const notes = ref<Note[]>([
    {
        id: 1,
        notebookId: 1,
        title: '项目会议记录',
        content: '今天与团队讨论了下一阶段的开发计划，确定了主要功能和开发时间表。需要在下周五前完成原型设计。',
        tags: [1, 2],
        createdAt: '2023-04-15 10:30',
        updatedAt: '2023-04-15 14:20'
    },
    {
        id: 2,
        notebookId: 1,
        title: '产品需求文档',
        content: '用户希望能够通过手机应用快速创建和编辑笔记，并支持图片和附件上传功能。界面需要简洁直观。',
        tags: [1],
        createdAt: '2023-04-14 09:15',
        updatedAt: '2023-04-15 11:45'
    },
    {
        id: 3,
        notebookId: 2,
        title: '读书笔记 - 《深度工作》',
        content: '深度工作是指在无干扰的状态下进行专注的职业活动，这种能力将使你能够快速掌握复杂信息并产出更好的成果。',
        tags: [3, 4],
        createdAt: '2023-04-13 16:40',
        updatedAt: '2023-04-13 16:40'
    },
    {
        id: 4,
        notebookId: 3,
        title: 'Vue 3 学习要点',
        content: 'Composition API 提供了更好的逻辑复用和类型推导。响应式系统使用 Proxy 重构，性能更好。',
        tags: [2],
        createdAt: '2023-04-12 14:20',
        updatedAt: '2023-04-14 10:15'
    },
    {
        id: 5,
        notebookId: 4,
        title: '日本旅行计划',
        content: '计划在秋季前往日本关西地区，主要游览京都、大阪和奈良。需要提前预订住宿和购买JR Pass。',
        tags: [2],
        createdAt: '2023-04-10 11:30',
        updatedAt: '2023-04-11 09:45'
    }
]);

// 状态管理
const state = reactive<AppState>({
    activeNotebook: 1,
    activeNote: null,
    searchQuery: '',
    editMode: false,
    loading: false
});

export function useNotes() {
    // 计算属性
    const filteredNotes = computed(() => {
        let filtered = notes.value.filter(note => note.notebookId === state.activeNotebook);

        if (state.searchQuery) {
            const query = state.searchQuery.toLowerCase();
            filtered = filtered.filter(note =>
                note.title.toLowerCase().includes(query) ||
                note.content.toLowerCase().includes(query)
            );
        }

        return filtered;
    });

    const activeNoteData = computed(() => {
        return notes.value.find(note => note.id === state.activeNote) || null;
    });

    // 方法
    const setActiveNotebook = (notebookId: number) => {
        state.activeNotebook = notebookId;
        state.activeNote = null;
    };

    const setActiveNote = (noteId: number) => {
        state.activeNote = noteId;
        state.editMode = false;
    };

    const createNewNote = () => {
        const newNote: Note = {
            id: Date.now(),
            notebookId: state.activeNotebook,
            title: '新笔记',
            content: '开始记录你的想法...',
            tags: [],
            createdAt: new Date().toISOString().split('T')[0] + ' ' +
                new Date().toTimeString().split(' ')[0].substring(0, 5),
            updatedAt: new Date().toISOString().split('T')[0] + ' ' +
                new Date().toTimeString().split(' ')[0].substring(0, 5)
        };

        notes.value.unshift(newNote);
        state.activeNote = newNote.id;
        state.editMode = true;
    };

    const saveNote = () => {
        if (!state.activeNote) return;

        const noteIndex = notes.value.findIndex(note => note.id === state.activeNote);
        if (noteIndex !== -1) {
            notes.value[noteIndex].updatedAt = new Date().toISOString().split('T')[0] + ' ' +
                new Date().toTimeString().split(' ')[0].substring(0, 5);
            state.editMode = false;
        }
    };

    const cancelEdit = () => {
        state.editMode = false;
    };

    const deleteNote = () => {
        if (!state.activeNote) return;

        const noteIndex = notes.value.findIndex(note => note.id === state.activeNote);
        if (noteIndex !== -1) {
            notes.value.splice(noteIndex, 1);
            state.activeNote = null;
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

    const formatDate = (dateStr: string) => {
        const date = new Date(dateStr);
        const now = new Date();
        const diffTime = Math.abs(now.getTime() - date.getTime());
        const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

        if (diffDays === 1) return '昨天';
        if (diffDays === 2) return '前天';
        if (diffDays <= 7) return `${diffDays}天前`;

        return dateStr.split(' ')[0];
    };

    const getTagById = (tagId: number) => {
        return tags.value.find(tag => tag.id === tagId);
    };

    // 初始化
    const initialize = async () => {
        state.loading = true

        try {
            const data = await noteApi.getNotebooks();

            notebooks.value = [...notebooks.value, ...data];

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
        filteredNotes,
        activeNoteData,
        setActiveNotebook,
        setActiveNote,
        createNewNote,
        saveNote,
        cancelEdit,
        deleteNote,
        updateNoteTitle,
        updateNoteContent,
        formatDate,
        getTagById
    };
}