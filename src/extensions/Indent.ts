import { Extension } from '@tiptap/core'

export interface IndentOptions {
  types: string[]
  minIndent: number
  maxIndent: number
  indentUnit: string
}

declare module '@tiptap/core' {
  interface Commands<ReturnType> {
    indent: {
      indent: () => ReturnType
      outdent: () => ReturnType
    }
  }
}

export const Indent = Extension.create<IndentOptions>({
  name: 'indent',

  addOptions() {
    return {
      types: ['paragraph', 'heading', 'blockquote'],
      minIndent: 0,
      maxIndent: 10,
      indentUnit: '2em',
    }
  },

  addGlobalAttributes() {
    return [
      {
        types: this.options.types,
        attributes: {
          indent: {
            default: 0,
            parseHTML: (element) => {
              const indent = element.style.marginLeft || element.style.paddingLeft
              if (!indent) return 0
              const match = indent.match(/(\d+)/)
              return match ? parseInt(match[1], 10) / 2 : 0
            },
            renderHTML: (attributes) => {
              if (!attributes.indent || attributes.indent <= 0) {
                return {}
              }
              return {
                style: `margin-left: ${attributes.indent * 2}em`,
              }
            },
          },
        },
      },
    ]
  },

  addCommands() {
    return {
      indent:
        () =>
        ({ tr, state, dispatch }) => {
          const { selection } = state
          const { from, to } = selection

          let changed = false

          state.doc.nodesBetween(from, to, (node, pos) => {
            if (this.options.types.includes(node.type.name)) {
              const currentIndent = node.attrs.indent || 0
              if (currentIndent < this.options.maxIndent) {
                if (dispatch) {
                  tr.setNodeMarkup(pos, undefined, {
                    ...node.attrs,
                    indent: currentIndent + 1,
                  })
                }
                changed = true
              }
            }
          })

          return changed
        },

      outdent:
        () =>
        ({ tr, state, dispatch }) => {
          const { selection } = state
          const { from, to } = selection

          let changed = false

          state.doc.nodesBetween(from, to, (node, pos) => {
            if (this.options.types.includes(node.type.name)) {
              const currentIndent = node.attrs.indent || 0
              if (currentIndent > this.options.minIndent) {
                if (dispatch) {
                  tr.setNodeMarkup(pos, undefined, {
                    ...node.attrs,
                    indent: currentIndent - 1,
                  })
                }
                changed = true
              }
            }
          })

          return changed
        },
    }
  },

  addKeyboardShortcuts() {
    return {
      Tab: () => this.editor.commands.indent(),
      'Shift-Tab': () => this.editor.commands.outdent(),
    }
  },
})

export default Indent
