import { defineStore } from 'pinia'
import { useNotebookStore } from './notebooks'
import { useTagStore } from './tags'
import { useNoteStore } from './notes'
import { useEditorStore } from './editor'
import { useUiStore } from './ui'

/**
 * 统一应用 Store（门面模式）
 *
 * 内部委托给拆分后的子 store，对外保持统一的 API，
 * 已有组件无需修改导入。新代码建议直接使用子 store。
 */
export const useAppStore = defineStore('app', () => {
  const notebookStore = useNotebookStore()
  const tagStore = useTagStore()
  const noteStore = useNoteStore()
  const editorStore = useEditorStore()
  const uiStore = useUiStore()

  // 语言切换时更新默认项
  const updateDefaultItems = () => {
    notebookStore.updateDefaultNotebook()
    tagStore.updateDefaultTag()
  }

  return {
    // ==================== 笔记本 ====================
    notebooks: notebookStore.notebooks,
    notebookTree: notebookStore.notebookTree,
    expandedNotebooks: notebookStore.expandedNotebooks,
    setNotebooks: notebookStore.setNotebooks,
    updateNotebook: notebookStore.updateNotebook,
    removeNotebook: notebookStore.removeNotebook,
    getNotebookById: notebookStore.getNotebookById,
    toggleNotebookExpand: notebookStore.toggleNotebookExpand,
    initExpandedNotebooks: notebookStore.initExpandedNotebooks,

    // ==================== 标签 ====================
    tags: tagStore.tags,
    setTags: tagStore.setTags,
    updateTag: tagStore.updateTag,
    removeTag: tagStore.removeTag,
    getTagById: tagStore.getTagById,

    // ==================== 笔记列表 ====================
    notes: noteStore.notes,
    query: noteStore.query,
    notePageIndex: noteStore.notePageIndex,
    notePageSize: noteStore.notePageSize,
    noteTotal: noteStore.noteTotal,
    noteSortField: noteStore.noteSortField,
    noteSortOrder: noteStore.noteSortOrder,
    noteSearchPageParam: noteStore.noteSearchPageParam,
    notesLoading: noteStore.notesLoading,
    setNotes: noteStore.setNotes,
    prependNote: noteStore.prependNote,
    updateNote: noteStore.updateNote,
    removeNote: noteStore.removeNote,
    getNoteById: noteStore.getNoteById,
    resetSearchParam: noteStore.resetSearchParam,

    // ==================== 多选 ====================
    isSelectMode: noteStore.isSelectMode,
    selectedNotes: noteStore.selectedNotes,
    toggleSelectMode: noteStore.toggleSelectMode,
    toggleNoteSelection: noteStore.toggleNoteSelection,
    selectAllNotes: noteStore.selectAllNotes,
    clearSelection: noteStore.clearSelection,

    // ==================== 编辑器 ====================
    editingNote: editorStore.editingNote,
    editMode: editorStore.editMode,
    activeNote: editorStore.activeNote,
    activeNoteData: editorStore.activeNoteData,
    isDirty: editorStore.isDirty,
    setActiveNote: editorStore.setActiveNote,
    createDefaultNote: editorStore.createDefaultNote,
    startEdit: editorStore.startEdit,
    cancelEdit: editorStore.cancelEdit,

    // ==================== 历史记录 ====================
    histories: editorStore.histories,
    historyPageIndex: editorStore.historyPageIndex,
    historyPageSize: editorStore.historyPageSize,
    historyTotal: editorStore.historyTotal,
    historyLoading: editorStore.historyLoading,

    // ==================== UI ====================
    activeNotebook: uiStore.activeNotebook,
    activeTag: uiStore.activeTag,
    loading: uiStore.loading,

    // ==================== 国际化 ====================
    updateDefaultItems,
  }
})
