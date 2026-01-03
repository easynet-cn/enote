import { Extension } from '@tiptap/core'
import { Plugin, PluginKey } from '@tiptap/pm/state'

export interface DragHandleOptions {
  dragHandleClass: string
  draggingClass: string
}

export const dragHandlePluginKey = new PluginKey('dragHandle')

export const DragHandle = Extension.create<DragHandleOptions>({
  name: 'dragHandle',

  addOptions() {
    return {
      dragHandleClass: 'drag-handle',
      draggingClass: 'is-dragging',
    }
  },

  addProseMirrorPlugins() {
    const options = this.options
    let draggedNode: HTMLElement | null = null
    let draggedPos: number | null = null

    return [
      new Plugin({
        key: dragHandlePluginKey,
        props: {
          handleDOMEvents: {
            mousedown(view, event) {
              const target = event.target as HTMLElement
              if (!target.classList.contains(options.dragHandleClass)) {
                return false
              }

              event.preventDefault()

              // Find the parent block node
              const pos = view.posAtDOM(target.parentElement!, 0)
              const $pos = view.state.doc.resolve(pos)
              const node = $pos.nodeAfter

              if (!node) return false

              draggedPos = pos
              draggedNode = target.parentElement as HTMLElement
              draggedNode.classList.add(options.draggingClass)

              return true
            },
            mousemove(view, event) {
              if (!draggedNode || draggedPos === null) return false

              // Update drag position indicator
              const coords = { left: event.clientX, top: event.clientY }
              const pos = view.posAtCoords(coords)

              if (pos) {
                // Show drop indicator
                view.dom.classList.add('drag-active')
              }

              return true
            },
            mouseup(view, event) {
              if (!draggedNode || draggedPos === null) return false

              draggedNode.classList.remove(options.draggingClass)
              view.dom.classList.remove('drag-active')

              const coords = { left: event.clientX, top: event.clientY }
              const dropPos = view.posAtCoords(coords)

              if (dropPos && dropPos.pos !== draggedPos) {
                const { state, dispatch } = view
                const $from = state.doc.resolve(draggedPos)
                const nodeToMove = $from.nodeAfter

                if (nodeToMove) {
                  const nodeSize = nodeToMove.nodeSize
                  let targetPos = dropPos.pos

                  // Adjust target position
                  if (targetPos > draggedPos) {
                    targetPos -= nodeSize
                  }

                  // Create transaction to move the node
                  const tr = state.tr

                  // Delete from original position
                  tr.delete(draggedPos, draggedPos + nodeSize)

                  // Insert at new position
                  const $targetPos = tr.doc.resolve(targetPos)
                  const insertPos = $targetPos.before($targetPos.depth) || targetPos

                  tr.insert(insertPos, nodeToMove)

                  dispatch(tr)
                }
              }

              draggedNode = null
              draggedPos = null

              return true
            },
          },
        },
      }),
    ]
  },
})

export default DragHandle
