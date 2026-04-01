import { defineStore, storeToRefs } from 'pinia'
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
 *
 * 注意：响应式属性必须通过 storeToRefs() 提取，
 * 否则 Pinia reactive proxy 会自动解包 ref 导致响应式丢失。
 */
export const useAppStore = defineStore('app', () => {
  const notebookStore = useNotebookStore()
  const tagStore = useTagStore()
  const noteStore = useNoteStore()
  const editorStore = useEditorStore()
  const uiStore = useUiStore()

  // 通过 storeToRefs 提取响应式引用（保持与子 store 的双向绑定）
  const { notebooks, notebookTree, expandedNotebooks } = storeToRefs(notebookStore)
  const { tags } = storeToRefs(tagStore)
  const {
    notes,
    query,
    notePageIndex,
    notePageSize,
    noteTotal,
    noteSortField,
    noteSortOrder,
    noteSearchPageParam,
    notesLoading,
    isSelectMode,
    selectedNotes,
  } = storeToRefs(noteStore)
  const {
    editingNote,
    editMode,
    activeNote,
    activeNoteData,
    isDirty,
    histories,
    historyPageIndex,
    historyPageSize,
    historyTotal,
    historyLoading,
  } = storeToRefs(editorStore)
  const { activeNotebook, activeTag, loading } = storeToRefs(uiStore)

  // 语言切换时更新默认项
  const updateDefaultItems = () => {
    notebookStore.updateDefaultNotebook()
    tagStore.updateDefaultTag()
  }

  return {
    // ==================== 笔记本（响应式属性） ====================
    notebooks,
    notebookTree,
    expandedNotebooks,
    // 笔记本（方法）
    setNotebooks: notebookStore.setNotebooks,
    updateNotebook: notebookStore.updateNotebook,
    removeNotebook: notebookStore.removeNotebook,
    getNotebookById: notebookStore.getNotebookById,
    toggleNotebookExpand: notebookStore.toggleNotebookExpand,
    initExpandedNotebooks: notebookStore.initExpandedNotebooks,

    // ==================== 标签（响应式属性） ====================
    tags,
    // 标签（方法）
    setTags: tagStore.setTags,
    updateTag: tagStore.updateTag,
    removeTag: tagStore.removeTag,
    getTagById: tagStore.getTagById,

    // ==================== 笔记列表（响应式属性） ====================
    notes,
    query,
    notePageIndex,
    notePageSize,
    noteTotal,
    noteSortField,
    noteSortOrder,
    noteSearchPageParam,
    notesLoading,
    // 笔记列表（方法）
    setNotes: noteStore.setNotes,
    prependNote: noteStore.prependNote,
    updateNote: noteStore.updateNote,
    removeNote: noteStore.removeNote,
    getNoteById: noteStore.getNoteById,
    resetSearchParam: noteStore.resetSearchParam,

    // ==================== 多选（响应式属性） ====================
    isSelectMode,
    selectedNotes,
    // 多选（方法）
    toggleSelectMode: noteStore.toggleSelectMode,
    toggleNoteSelection: noteStore.toggleNoteSelection,
    selectAllNotes: noteStore.selectAllNotes,
    clearSelection: noteStore.clearSelection,

    // ==================== 编辑器（响应式属性） ====================
    editingNote,
    editMode,
    activeNote,
    activeNoteData,
    isDirty,
    // 编辑器（方法）
    setActiveNote: editorStore.setActiveNote,
    createDefaultNote: editorStore.createDefaultNote,
    startEdit: editorStore.startEdit,
    cancelEdit: editorStore.cancelEdit,

    // ==================== 历史记录（响应式属性） ====================
    histories,
    historyPageIndex,
    historyPageSize,
    historyTotal,
    historyLoading,

    // ==================== UI（响应式属性） ====================
    activeNotebook,
    activeTag,
    loading,

    // ==================== 国际化 ====================
    updateDefaultItems,
  }
})
