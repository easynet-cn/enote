export {}

declare module 'vue-i18n' {
  export interface DefineLocaleMessage {
    // 通用
    common: {
      save: string
      cancel: string
      delete: string
      edit: string
      confirm: string
      search: string
      name: string
      description: string
      sort: string
      icon: string
      create: string
      add: string
      loading: string
      empty: string
      required: string
      yes: string
      no: string
      close: string
      apply: string
      reset: string
      export: string
      import: string
      settings: string
      more: string
    }

    // 侧边栏
    sidebar: {
      collapse: string
      expand: string
      notebooks: string
      tags: string
      createNotebook: string
      editNotebook: string
      deleteNotebook: string
      createTag: string
      editTag: string
      deleteTag: string
      importNotes: string
      selectIcon: string
      selectColor: string
      notesCount: string
      notebookForm: {
        title: string
        nameLabel: string
        namePlaceholder: string
        descriptionLabel: string
        descriptionPlaceholder: string
        sortOrderLabel: string
        sortOrderPlaceholder: string
        createTitle: string
        editTitle: string
      }
      tagForm: {
        title: string
        nameLabel: string
        namePlaceholder: string
        iconLabel: string
        iconPlaceholder: string
        colorLabel: string
        colorPlaceholder: string
        sortOrderLabel: string
        sortOrderPlaceholder: string
        createTitle: string
        editTitle: string
      }
      deleteNotebookConfirm: {
        title: string
        message: string
        confirmText: string
      }
      deleteTagConfirm: {
        title: string
        message: string
        confirmText: string
      }
    }

    // 笔记列表
    noteList: {
      searchPlaceholder: string
      clearSearch: string
      collapseList: string
      expandList: string
      empty: string
      noResults: string
      noTitle: string
      noContent: string
      updateTime: string
    }

    // 编辑器
    editor: {
      toggleEditMode: string
      toggleViewMode: string
      newNote: string
      saveNote: string
      cancelEdit: string
      deleteNote: string
      noteSettings: string
      history: string
      wordCount: string
      export: string
      noteTitle: string
      noteTitlePlaceholder: string
      noteContentPlaceholder: string
      editorPlaceholder: string
      selectNotebook: string
      selectTags: string
      noTags: string
      deleteNoteConfirm: {
        title: string
        message: string
        confirmText: string
      }
      unsavedChanges: {
        title: string
        message: string
        confirmText: string
        cancelText: string
      }
      toolbar: {
        heading: string
        paragraph: string
        bold: string
        italic: string
        underline: string
        strike: string
        code: string
        codeBlock: string
        link: string
        image: string
        table: string
        bulletList: string
        orderedList: string
        taskList: string
        quote: string
        divider: string
        textAlignLeft: string
        textAlignCenter: string
        textAlignRight: string
        highlight: string
        textColor: string
        bgColor: string
        fontFamily: string
        fontSize: string
        superscript: string
        subscript: string
        undo: string
        redo: string
      }
      history: {
        title: string
        empty: string
        version: string
        time: string
        preview: string
        restore: string
        restoreConfirm: {
          title: string
          message: string
          confirmText: string
        }
      }
    }

    // 导出
    export: {
      title: string
      format: string
      formatOptions: {
        markdown: string
        html: string
        pdf: string
        txt: string
      }
      includeOptions: string
      includeMeta: string
      includeTags: string
      filename: string
      filenamePlaceholder: string
      exportButton: string
      exporting: string
      success: string
      error: string
    }

    // 导入
    import: {
      title: string
      format: string
      formatOptions: {
        markdown: string
        html: string
        txt: string
        zip: string
      }
      selectFile: string
      orDragAndDrop: string
      supportedFormats: string
      selectNotebook: string
      selectNotebookPlaceholder: string
      importButton: string
      importing: string
      success: string
      successMessage: string
      error: string
      errorFileFormat: string
      errorFileContent: string
      confirmOverwrite: {
        title: string
        message: string
        confirmText: string
        cancelText: string
      }
    }

    // 键盘快捷键
    shortcuts: {
      save: string
      newNote: string
      editNote: string
      cancelEdit: string
      toggleSidebar: string
    }

    // 验证
    validation: {
      required: string
      invalidFormat: string
      tooLong: string
      tooShort: string
      invalidUrl: string
    }

    // 日期时间
    datetime: {
      today: string
      yesterday: string
      justNow: string
      minutesAgo: string
      hoursAgo: string
      daysAgo: string
      weeksAgo: string
      monthsAgo: string
      yearsAgo: string
    }

    // 通知
    notification: {
      success: string
      error: string
      warning: string
      info: string
      saved: string
      deleted: string
      updated: string
      created: string
      networkError: string
      serverError: string
    }
  }
}
