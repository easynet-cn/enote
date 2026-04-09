/// <reference types="vite/client" />

declare module 'htmldiff-js' {
  interface HtmlDiffModule {
    execute(oldHtml: string, newHtml: string): string
  }
  const HtmlDiff: { default: HtmlDiffModule }
  export default HtmlDiff
}

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  // eslint-disable-next-line @typescript-eslint/no-empty-object-type, @typescript-eslint/no-explicit-any
  const component: DefineComponent<{}, {}, any>
  export default component
}
