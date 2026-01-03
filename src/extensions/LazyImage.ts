import Image from '@tiptap/extension-image'
import { Plugin, PluginKey } from '@tiptap/pm/state'

export interface LazyImageOptions {
  rootMargin?: string
  threshold?: number
}

export const lazyImagePluginKey = new PluginKey('lazyImage')

/**
 * 图片懒加载扩展
 * 基于 IntersectionObserver 实现，只有当图片进入视口时才加载
 */
export const LazyImage = Image.extend<LazyImageOptions>({
  name: 'lazyImage',

  addOptions() {
    return {
      ...this.parent?.(),
      rootMargin: '100px',
      threshold: 0.1,
    }
  },

  addProseMirrorPlugins() {
    const options = this.options

    return [
      new Plugin({
        key: lazyImagePluginKey,
        view: (view) => {
          let observer: IntersectionObserver | null = null

          const setupObserver = () => {
            if (observer) {
              observer.disconnect()
            }

            observer = new IntersectionObserver(
              (entries) => {
                entries.forEach((entry) => {
                  if (entry.isIntersecting) {
                    const img = entry.target as HTMLImageElement
                    const dataSrc = img.getAttribute('data-src')

                    if (dataSrc && !img.src) {
                      img.src = dataSrc
                      img.removeAttribute('data-src')
                      observer?.unobserve(img)
                    }
                  }
                })
              },
              {
                root: view.dom.closest('.tiptap-editor, .tiptap-editor-edit'),
                rootMargin: options.rootMargin,
                threshold: options.threshold,
              },
            )

            // 观察所有带有 data-src 的图片
            const images = view.dom.querySelectorAll('img[data-src]')
            images.forEach((img) => observer?.observe(img))
          }

          // 初始设置
          setupObserver()

          return {
            update: () => {
              // 编辑器更新后重新观察新图片
              setTimeout(() => {
                const images = view.dom.querySelectorAll('img[data-src]:not([src])')
                images.forEach((img) => observer?.observe(img))
              }, 100)
            },
            destroy: () => {
              observer?.disconnect()
              observer = null
            },
          }
        },
      }),
    ]
  },

  // 重写 renderHTML 将 src 移到 data-src（仅对外部图片）
  renderHTML({ HTMLAttributes }) {
    const src = HTMLAttributes.src

    // 只对外部 HTTP(S) 图片启用懒加载，Base64 图片直接加载
    if (src && src.startsWith('http') && !src.startsWith('data:')) {
      return [
        'img',
        {
          ...HTMLAttributes,
          src: '', // 清空 src
          'data-src': src, // 保存到 data-src
          loading: 'lazy', // 原生懒加载作为后备
          class: (HTMLAttributes.class || '') + ' lazy-image',
        },
      ]
    }

    return ['img', { ...HTMLAttributes, loading: 'lazy' }]
  },
})

export default LazyImage
