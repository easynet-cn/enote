<template>
  <BaseDialog v-model="visible" :title="t('help.title')" :width="960" @open="loadManual">
    <div class="help-manual">
      <!-- 工具栏 -->
      <div class="help-toolbar">
        <div class="help-search">
          <Search class="w-4 h-4 text-content-tertiary" />
          <input
            v-model="searchQuery"
            type="text"
            :placeholder="t('help.searchPlaceholder')"
            class="help-search-input"
          />
          <button
            v-if="searchQuery"
            class="help-search-clear"
            @click="searchQuery = ''"
            :title="t('common.clear')"
            :aria-label="t('common.clear')"
          >
            <X class="w-3.5 h-3.5" />
          </button>
        </div>
        <button
          class="help-toolbar-btn"
          :title="t('help.openInNewWindow')"
          @click="openInNewWindow"
        >
          <ExternalLink class="w-4 h-4" />
        </button>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="help-loading">
        <div class="help-spinner"></div>
        <span class="text-sm text-content-secondary mt-2">{{ t('help.loading') }}</span>
      </div>

      <!-- 错误状态 -->
      <div v-else-if="error" class="help-error">
        <AlertCircle class="w-8 h-8 text-red-400" />
        <span class="text-sm text-content-secondary mt-2">{{ t('help.loadFailed') }}</span>
      </div>

      <!-- 内容区 -->
      <div v-else class="help-content-wrapper">
        <!-- 左侧目录导航 -->
        <nav class="help-toc">
          <ul class="help-toc-list">
            <li
              v-for="item in tocItems"
              :key="item.id"
              :class="[
                'help-toc-item',
                `help-toc-level-${item.level}`,
                { active: activeHeading === item.id },
              ]"
            >
              <a :href="`#${item.id}`" @click.prevent="scrollToHeading(item.id)">
                {{ item.text }}
              </a>
            </li>
          </ul>
        </nav>

        <!-- 右侧内容 -->
        <div ref="contentRef" class="help-content" @scroll="handleScroll">
          <div v-if="filteredHtml" class="help-markdown" v-html="filteredHtml"></div>
          <div v-else class="help-no-results">
            <SearchX class="w-8 h-8 text-content-tertiary" />
            <span class="text-sm text-content-secondary mt-2">{{ t('help.noResults') }}</span>
          </div>

          <!-- 返回顶部 -->
          <button
            v-show="showBackToTop"
            class="help-back-to-top"
            :title="t('help.backToTop')"
            @click="scrollToTop"
          >
            <ArrowUp class="w-4 h-4" />
          </button>
        </div>
      </div>
    </div>
  </BaseDialog>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { Search, X, ExternalLink, AlertCircle, ArrowUp, SearchX } from 'lucide-vue-next'
import { marked } from 'marked'
import DOMPurify from 'dompurify'
import BaseDialog from './ui/BaseDialog.vue'
import { helpApi } from '../api/help'
import { openHelpInNewWindow } from '../utils/multiWindow'

const visible = defineModel<boolean>({ default: false })

const { t, locale } = useI18n()

const loading = ref(false)
const error = ref(false)
const rawMarkdown = ref('')
const searchQuery = ref('')
const activeHeading = ref('')
const showBackToTop = ref(false)
const contentRef = ref<HTMLElement | null>(null)

interface TocItem {
  id: string
  text: string
  level: number
}

const renderedHtml = computed(() => {
  if (!rawMarkdown.value) return ''
  const html = marked.parse(rawMarkdown.value, { async: false }) as string
  return DOMPurify.sanitize(html)
})

const tocItems = computed<TocItem[]>(() => {
  if (!rawMarkdown.value) return []
  const items: TocItem[] = []
  const headingRegex = /^(#{1,3})\s+(.+)$/gm
  let match
  while ((match = headingRegex.exec(rawMarkdown.value)) !== null) {
    const level = match[1].length
    const text = match[2].replace(/[*_`\[\]()]/g, '').trim()
    const id = text
      .toLowerCase()
      .replace(/[^\w\u4e00-\u9fff\s-]/g, '')
      .replace(/\s+/g, '-')
    items.push({ id, text, level })
  }
  return items
})

const filteredHtml = computed(() => {
  if (!searchQuery.value.trim()) return renderedHtml.value
  const query = searchQuery.value.trim().toLowerCase()
  // Simple: check if content contains query, highlight matches
  const html = renderedHtml.value
  if (!html.toLowerCase().includes(query)) return ''
  // Highlight matches (only in text nodes, not in tags)
  return html.replace(
    new RegExp(`(?<=>)([^<]*?)(${escapeRegExp(searchQuery.value.trim())})([^<]*?)(?=<)`, 'gi'),
    (_, before, match, after) => `${before}<mark class="help-highlight">${match}</mark>${after}`,
  )
})

function escapeRegExp(str: string) {
  return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}

async function loadManual() {
  if (rawMarkdown.value && !error.value) return
  loading.value = true
  error.value = false
  try {
    rawMarkdown.value = await helpApi.readManual(locale.value)
  } catch (e) {
    console.error('Failed to load help manual:', e)
    error.value = true
  } finally {
    loading.value = false
  }
}

// Reload when language changes
watch(locale, () => {
  if (visible.value) {
    rawMarkdown.value = ''
    loadManual()
  }
})

function scrollToHeading(id: string) {
  if (!contentRef.value) return
  const el = contentRef.value.querySelector(`#${CSS.escape(id)}`)
  if (el) {
    el.scrollIntoView({ behavior: 'smooth', block: 'start' })
    activeHeading.value = id
  }
}

function scrollToTop() {
  contentRef.value?.scrollTo({ top: 0, behavior: 'smooth' })
}

function handleScroll() {
  if (!contentRef.value) return
  showBackToTop.value = contentRef.value.scrollTop > 300

  // Update active heading based on scroll position
  const headings = contentRef.value.querySelectorAll('h1, h2, h3')
  let currentId = ''
  for (const heading of headings) {
    const rect = heading.getBoundingClientRect()
    const containerRect = contentRef.value.getBoundingClientRect()
    if (rect.top - containerRect.top <= 80) {
      currentId = heading.id
    }
  }
  if (currentId) activeHeading.value = currentId
}

async function openInNewWindow() {
  await openHelpInNewWindow()
  visible.value = false
}

// Add IDs to headings after render
watch(filteredHtml, async () => {
  await nextTick()
  if (!contentRef.value) return
  const headings = contentRef.value.querySelectorAll('h1, h2, h3')
  headings.forEach((heading) => {
    if (!heading.id) {
      const text = heading.textContent || ''
      heading.id = text
        .toLowerCase()
        .replace(/[^\w\u4e00-\u9fff\s-]/g, '')
        .replace(/\s+/g, '-')
    }
  })
})
</script>

<style scoped>
.help-manual {
  display: flex;
  flex-direction: column;
  height: 70vh;
  margin: -1.5rem;
}

/* 工具栏 */
.help-toolbar {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--color-border);
  background: var(--color-bg-secondary);
}

.help-search {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.375rem 0.75rem;
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  transition: border-color var(--transition-normal) var(--ease-default);
}

.help-search:focus-within {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}

.help-search-input {
  flex: 1;
  border: none;
  outline: none;
  background: transparent;
  font-size: 0.875rem;
  color: var(--color-text-primary);
}

.help-search-input::placeholder {
  color: var(--color-text-tertiary);
}

.help-search-clear {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border-radius: var(--radius-full);
  color: var(--color-text-tertiary);
  background: transparent;
  border: none;
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-default);
}

.help-search-clear:hover {
  background: var(--color-bg-tertiary);
  color: var(--color-text-secondary);
}

.help-toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: var(--radius-md);
  color: var(--color-text-secondary);
  background: transparent;
  border: none;
  cursor: pointer;
  transition: all var(--transition-normal) var(--ease-default);
}

.help-toolbar-btn:hover {
  background: var(--color-bg-tertiary);
  color: var(--color-primary);
}

/* 加载/错误状态 */
.help-loading,
.help-error,
.help-no-results {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  padding: 3rem;
}

.help-spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--color-border);
  border-top-color: var(--color-primary);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* 内容区 */
.help-content-wrapper {
  display: flex;
  flex: 1;
  overflow: hidden;
}

/* 目录导航 */
.help-toc {
  width: 240px;
  flex-shrink: 0;
  overflow-y: auto;
  border-right: 1px solid var(--color-border);
  background: var(--color-bg-secondary);
  padding: 0.75rem 0;
}

.help-toc-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.help-toc-item {
  margin: 0;
}

.help-toc-item a {
  display: block;
  padding: 0.375rem 1rem;
  font-size: 0.8125rem;
  color: var(--color-text-secondary);
  text-decoration: none;
  border-left: 2px solid transparent;
  transition: all var(--transition-fast) var(--ease-default);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.help-toc-item a:hover {
  color: var(--color-primary);
  background: var(--color-primary-lighter);
}

.help-toc-item.active a {
  color: var(--color-primary);
  border-left-color: var(--color-primary);
  background: var(--color-primary-lighter);
  font-weight: 500;
}

.help-toc-level-2 a {
  padding-left: 1.75rem;
}

.help-toc-level-3 a {
  padding-left: 2.5rem;
  font-size: 0.75rem;
}

/* 内容区 */
.help-content {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem 2rem;
  position: relative;
  scroll-behavior: smooth;
}

/* Markdown 样式 */
.help-markdown :deep(h1) {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 2rem 0 1rem;
  padding-bottom: 0.5rem;
  border-bottom: 2px solid var(--color-primary-light);
}

.help-markdown :deep(h1:first-child) {
  margin-top: 0;
}

.help-markdown :deep(h2) {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 1.75rem 0 0.75rem;
  padding-bottom: 0.375rem;
  border-bottom: 1px solid var(--color-border);
}

.help-markdown :deep(h3) {
  font-size: 1.0625rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 1.5rem 0 0.5rem;
}

.help-markdown :deep(h4) {
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-text-secondary);
  margin: 1.25rem 0 0.5rem;
}

.help-markdown :deep(p) {
  margin: 0.75rem 0;
  line-height: 1.7;
  color: var(--color-text-primary);
  font-size: 0.875rem;
}

.help-markdown :deep(ul),
.help-markdown :deep(ol) {
  margin: 0.5rem 0;
  padding-left: 1.5rem;
  font-size: 0.875rem;
  color: var(--color-text-primary);
}

.help-markdown :deep(li) {
  margin: 0.25rem 0;
  line-height: 1.6;
}

.help-markdown :deep(code) {
  padding: 0.125rem 0.375rem;
  border-radius: var(--radius-sm);
  background: var(--color-bg-tertiary);
  color: var(--color-primary);
  font-size: 0.8125rem;
  font-family: 'SF Mono', Monaco, Consolas, monospace;
}

.help-markdown :deep(pre) {
  margin: 0.75rem 0;
  padding: 1rem;
  border-radius: var(--radius-lg);
  background: var(--color-bg-tertiary);
  overflow-x: auto;
}

.help-markdown :deep(pre code) {
  padding: 0;
  background: transparent;
  color: var(--color-text-primary);
}

.help-markdown :deep(table) {
  width: 100%;
  border-collapse: collapse;
  margin: 0.75rem 0;
  font-size: 0.875rem;
}

.help-markdown :deep(th) {
  padding: 0.5rem 0.75rem;
  text-align: left;
  font-weight: 600;
  color: var(--color-text-primary);
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
}

.help-markdown :deep(td) {
  padding: 0.5rem 0.75rem;
  border: 1px solid var(--color-border);
  color: var(--color-text-primary);
}

.help-markdown :deep(blockquote) {
  margin: 0.75rem 0;
  padding: 0.5rem 1rem;
  border-left: 3px solid var(--color-primary);
  background: var(--color-primary-lighter);
  border-radius: 0 var(--radius-md) var(--radius-md) 0;
}

.help-markdown :deep(blockquote p) {
  margin: 0.25rem 0;
}

.help-markdown :deep(hr) {
  margin: 1.5rem 0;
  border: none;
  border-top: 1px solid var(--color-border);
}

.help-markdown :deep(strong) {
  font-weight: 600;
  color: var(--color-text-primary);
}

.help-markdown :deep(a) {
  color: var(--color-primary);
  text-decoration: none;
}

.help-markdown :deep(a:hover) {
  text-decoration: underline;
}

.help-markdown :deep(img) {
  max-width: 100%;
  border-radius: var(--radius-lg);
  margin: 0.75rem 0;
}

/* 搜索高亮 */
.help-markdown :deep(.help-highlight) {
  background: #fef08a;
  color: #854d0e;
  padding: 0.0625rem 0.125rem;
  border-radius: 2px;
}

/* 返回顶部 */
.help-back-to-top {
  position: sticky;
  bottom: 1rem;
  float: right;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: var(--radius-full);
  background: var(--color-primary);
  color: white;
  border: none;
  cursor: pointer;
  box-shadow: var(--shadow-md);
  transition: all var(--transition-normal) var(--ease-default);
  z-index: 10;
}

.help-back-to-top:hover {
  background: var(--color-primary-hover);
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

/* 移动端适配 */
@media (max-width: 768px) {
  .help-toc {
    display: none;
  }

  .help-content {
    padding: 1rem;
  }
}
</style>
