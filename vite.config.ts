import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'
import { resolve } from 'path'

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST
// @ts-expect-error process is a nodejs global
const isMobileBuild = ['android', 'ios'].includes(process.env.TAURI_ENV_PLATFORM ?? '')

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [vue(), tailwindcss()],

  // 编译时注入平台标记（Tauri 构建移动端时 TAURI_ENV_PLATFORM = 'android' | 'ios'）
  define: {
    __IS_MOBILE__: JSON.stringify(isMobileBuild),
  },

  // Use relative paths for Tauri production build
  base: './',

  // Path aliases (must match tsconfig.json paths)
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
      '@components': resolve(__dirname, 'src/components'),
      '@composables': resolve(__dirname, 'src/composables'),
      '@utils': resolve(__dirname, 'src/utils'),
      '@stores': resolve(__dirname, 'src/stores'),
      '@types': resolve(__dirname, 'src/types'),
      '@config': resolve(__dirname, 'src/config'),
      '@api': resolve(__dirname, 'src/api'),
    },
  },

  // Optimize dependencies
  optimizeDeps: {
    include: [
      '@tiptap/pm/state',
      '@tiptap/pm/view',
      '@tiptap/pm/model',
      '@tiptap/pm/transform',
      '@tiptap/pm/commands',
      '@tiptap/pm/schema-list',
      '@tiptap/pm/tables',
      '@tiptap/pm/gapcursor',
      '@tiptap/pm/history',
      '@tiptap/pm/keymap',
      '@tiptap/pm/inputrules',
      '@tiptap/pm/collab',
      '@tiptap/pm/dropcursor',
      '@tiptap/pm/markdown',
    ],
  },

  // Build optimizations
  build: {
    // Faster builds by not reporting compressed size
    reportCompressedSize: false,
    // Target modern browsers for smaller bundle
    target: 'esnext',
    // Minification settings
    minify: 'esbuild',
    // Increase warning limit since Tauri apps load locally
    chunkSizeWarningLimit: 1000,
    // 生成稳定的 chunk 文件名，避免缓存导致白屏
    rollupOptions: {
      output: {
        // 使用内容哈希确保文件名与内容一致
        chunkFileNames: 'assets/[name]-[hash].js',
        entryFileNames: 'assets/[name]-[hash].js',
        assetFileNames: 'assets/[name]-[hash][extname]',
        manualChunks(id) {
          if (id.includes('node_modules')) {
            // TipTap 编辑器核心（最大 chunk）
            if (
              id.includes('@tiptap') ||
              id.includes('prosemirror') ||
              id.includes('tiptap-markdown')
            ) {
              return 'vendor-editor'
            }
            // 代码高亮（按需加载，不在首屏）
            if (id.includes('lowlight') || id.includes('highlight.js')) {
              return 'vendor-highlight'
            }
            // KaTeX 数学公式（按需加载）
            if (id.includes('katex')) {
              return 'vendor-katex'
            }
            // 图标库
            if (id.includes('lucide-vue-next')) {
              return 'vendor-icons'
            }
            // Vue 生态核心（稳定，缓存友好）
            if (
              id.includes('vue') ||
              id.includes('pinia') ||
              id.includes('vue-i18n') ||
              id.includes('@vueuse')
            ) {
              return 'vendor-vue'
            }
            // Tauri API 层
            if (id.includes('@tauri-apps')) {
              return 'vendor-tauri'
            }
          }
        },
      },
    },
    // 启用 CSS 代码分割
    cssCodeSplit: true,
    // 启用 source map 便于调试白屏问题（release 可关闭）
    sourcemap: !!host,
  },

  // Vite options tailored for Tauri development
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
}))
