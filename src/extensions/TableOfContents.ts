import { Extension } from '@tiptap/core'
import { Plugin, PluginKey } from '@tiptap/pm/state'

export interface TocItem {
  id: string
  level: number
  text: string
  pos: number
}

export interface TableOfContentsOptions {
  onUpdate?: (items: TocItem[]) => void
}

export const tableOfContentsPluginKey = new PluginKey('tableOfContents')

export const TableOfContents = Extension.create<TableOfContentsOptions>({
  name: 'tableOfContents',

  addOptions() {
    return {
      onUpdate: undefined,
    }
  },

  addStorage() {
    return {
      items: [] as TocItem[],
    }
  },

  addProseMirrorPlugins() {
    const options = this.options
    const storage = this.storage

    return [
      new Plugin({
        key: tableOfContentsPluginKey,
        view: () => ({
          update: (view) => {
            const items: TocItem[] = []
            const { doc } = view.state

            doc.descendants((node, pos) => {
              if (node.type.name === 'heading') {
                const level = node.attrs.level as number
                const text = node.textContent
                const id = `heading-${pos}`

                items.push({
                  id,
                  level,
                  text,
                  pos,
                })
              }
            })

            // Update storage
            storage.items = items

            // Call callback if provided
            if (options.onUpdate) {
              options.onUpdate(items)
            }
          },
        }),
      }),
    ]
  },
})

export default TableOfContents
