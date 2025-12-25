import { marked } from 'marked'

// Configure marked for synchronous parsing
marked.use({
  async: false,
})

/**
 * Convert markdown to HTML
 */
export const markdownToHtml = (markdown: string): string => {
  return marked.parse(markdown) as string
}
