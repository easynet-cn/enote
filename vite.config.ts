import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'
import { resolve } from 'path'

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [vue(), tailwindcss()],

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
    // Chunk splitting for better caching
    rollupOptions: {
      output: {
        manualChunks: {
          tiptap: [
            '@tiptap/vue-3',
            '@tiptap/starter-kit',
            'tiptap-markdown',
          ],
          vendor: ['vue', 'pinia', 'marked'],
        },
      },
    },
    // Faster builds by not reporting compressed size
    reportCompressedSize: false,
    // Target modern browsers for smaller bundle
    target: 'esnext',
    // Minification settings
    minify: 'esbuild',
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
