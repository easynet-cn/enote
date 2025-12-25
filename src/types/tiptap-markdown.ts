import type { Editor } from '@tiptap/core'

export interface MarkdownStorage {
  getMarkdown: () => string
}

interface EditorWithMarkdown extends Editor {
  storage: Editor['storage'] & {
    markdown?: MarkdownStorage
  }
}

export function getMarkdownFromEditor(editor: Editor): string {
  const editorWithMarkdown = editor as EditorWithMarkdown
  return editorWithMarkdown.storage.markdown?.getMarkdown() ?? ''
}
