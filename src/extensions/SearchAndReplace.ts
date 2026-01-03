import { Extension } from '@tiptap/core'
import { Node as ProseMirrorNode } from '@tiptap/pm/model'
import { Plugin, PluginKey } from '@tiptap/pm/state'
import { Decoration, DecorationSet } from '@tiptap/pm/view'

export interface SearchAndReplaceOptions {
  searchResultClass: string
  currentResultClass: string
  caseSensitive: boolean
}

export interface SearchAndReplaceStorage {
  searchTerm: string
  replaceTerm: string
  results: { from: number; to: number }[]
  currentIndex: number
  caseSensitive: boolean
}

declare module '@tiptap/core' {
  interface Commands<ReturnType> {
    searchAndReplace: {
      setSearchTerm: (searchTerm: string) => ReturnType
      setReplaceTerm: (replaceTerm: string) => ReturnType
      setCaseSensitive: (caseSensitive: boolean) => ReturnType
      nextSearchResult: () => ReturnType
      previousSearchResult: () => ReturnType
      replaceCurrentResult: () => ReturnType
      replaceAllResults: () => ReturnType
      clearSearch: () => ReturnType
    }
  }
}

export const searchAndReplacePluginKey = new PluginKey('searchAndReplace')

// Helper function to find all search results
function findSearchResults(
  doc: ProseMirrorNode,
  searchTerm: string,
  caseSensitive: boolean,
): { from: number; to: number }[] {
  const results: { from: number; to: number }[] = []

  if (!searchTerm) return results

  doc.descendants((node: ProseMirrorNode, pos: number) => {
    if (!node.isText) return

    const text = node.text || ''
    const term = caseSensitive ? searchTerm : searchTerm.toLowerCase()
    const nodeText = caseSensitive ? text : text.toLowerCase()

    let index = nodeText.indexOf(term)
    while (index !== -1) {
      results.push({
        from: pos + index,
        to: pos + index + term.length,
      })
      index = nodeText.indexOf(term, index + 1)
    }
  })

  return results
}

export const SearchAndReplace = Extension.create<SearchAndReplaceOptions, SearchAndReplaceStorage>({
  name: 'searchAndReplace',

  addOptions() {
    return {
      searchResultClass: 'search-result',
      currentResultClass: 'search-result-current',
      caseSensitive: false,
    }
  },

  addStorage() {
    return {
      searchTerm: '',
      replaceTerm: '',
      results: [],
      currentIndex: 0,
      caseSensitive: this.options.caseSensitive,
    }
  },

  addCommands() {
    return {
      setSearchTerm:
        (searchTerm: string) =>
        ({ editor }) => {
          this.storage.searchTerm = searchTerm
          this.storage.currentIndex = 0
          this.storage.results = findSearchResults(
            editor.state.doc,
            searchTerm,
            this.storage.caseSensitive,
          )
          editor.view.dispatch(editor.state.tr)
          return true
        },

      setReplaceTerm: (replaceTerm: string) => () => {
        this.storage.replaceTerm = replaceTerm
        return true
      },

      setCaseSensitive:
        (caseSensitive: boolean) =>
        ({ editor }) => {
          this.storage.caseSensitive = caseSensitive
          this.storage.results = findSearchResults(
            editor.state.doc,
            this.storage.searchTerm,
            caseSensitive,
          )
          editor.view.dispatch(editor.state.tr)
          return true
        },

      nextSearchResult:
        () =>
        ({ editor }) => {
          if (this.storage.results.length === 0) return false

          this.storage.currentIndex = (this.storage.currentIndex + 1) % this.storage.results.length

          const result = this.storage.results[this.storage.currentIndex]
          if (result) {
            editor.commands.setTextSelection({ from: result.from, to: result.to })
          }

          editor.view.dispatch(editor.state.tr)
          return true
        },

      previousSearchResult:
        () =>
        ({ editor }) => {
          if (this.storage.results.length === 0) return false

          this.storage.currentIndex =
            (this.storage.currentIndex - 1 + this.storage.results.length) %
            this.storage.results.length

          const result = this.storage.results[this.storage.currentIndex]
          if (result) {
            editor.commands.setTextSelection({ from: result.from, to: result.to })
          }

          editor.view.dispatch(editor.state.tr)
          return true
        },

      replaceCurrentResult:
        () =>
        ({ editor, tr }) => {
          if (this.storage.results.length === 0) return false

          const result = this.storage.results[this.storage.currentIndex]
          if (!result) return false

          tr.insertText(this.storage.replaceTerm, result.from, result.to)
          editor.view.dispatch(tr)

          // Update results after replacement
          setTimeout(() => {
            this.storage.results = findSearchResults(
              editor.state.doc,
              this.storage.searchTerm,
              this.storage.caseSensitive,
            )
            if (this.storage.currentIndex >= this.storage.results.length) {
              this.storage.currentIndex = Math.max(0, this.storage.results.length - 1)
            }
            editor.view.dispatch(editor.state.tr)
          }, 0)

          return true
        },

      replaceAllResults:
        () =>
        ({ editor, tr }) => {
          if (this.storage.results.length === 0) return false

          // Replace from end to start to maintain positions
          const results = [...this.storage.results].reverse()
          for (const result of results) {
            tr.insertText(this.storage.replaceTerm, result.from, result.to)
          }
          editor.view.dispatch(tr)

          // Clear results
          this.storage.results = []
          this.storage.currentIndex = 0

          return true
        },

      clearSearch:
        () =>
        ({ editor }) => {
          this.storage.searchTerm = ''
          this.storage.replaceTerm = ''
          this.storage.results = []
          this.storage.currentIndex = 0
          editor.view.dispatch(editor.state.tr)
          return true
        },
    }
  },

  addProseMirrorPlugins() {
    const { storage, options } = this

    return [
      new Plugin({
        key: searchAndReplacePluginKey,
        state: {
          init() {
            return DecorationSet.empty
          },
          apply(_tr, _oldState, _oldEditorState, newEditorState) {
            if (!storage.searchTerm || storage.results.length === 0) {
              return DecorationSet.empty
            }

            const decorations: Decoration[] = []

            for (let i = 0; i < storage.results.length; i++) {
              const result = storage.results[i]
              const isCurrent = i === storage.currentIndex

              decorations.push(
                Decoration.inline(result.from, result.to, {
                  class: isCurrent
                    ? `${options.searchResultClass} ${options.currentResultClass}`
                    : options.searchResultClass,
                }),
              )
            }

            return DecorationSet.create(newEditorState.doc, decorations)
          },
        },
        props: {
          decorations(state) {
            return this.getState(state)
          },
        },
      }),
    ]
  },
})

export default SearchAndReplace
