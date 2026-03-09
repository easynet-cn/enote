<template>
  <Dialog v-model="visible" :title="t('editor.searchDialog.title')" :width="400">
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-1">{{
          t('editor.searchDialog.findLabel')
        }}</label>
        <input
          v-model="searchTerm"
          type="text"
          :placeholder="t('editor.linkDialog.findPlaceholder')"
          @input="handleSearchInput"
          @keydown.enter="findNext"
          class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
        />
      </div>
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-1">{{
          t('editor.searchDialog.replaceLabel')
        }}</label>
        <input
          v-model="replaceTerm"
          type="text"
          :placeholder="t('editor.linkDialog.replacePlaceholder')"
          @input="handleReplaceInput"
          class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
        />
      </div>
      <div class="flex items-center gap-4">
        <label class="flex items-center gap-2 text-sm text-slate-600 cursor-pointer">
          <input
            type="checkbox"
            v-model="caseSensitive"
            @change="handleCaseSensitiveChange"
            class="w-4 h-4 rounded border-slate-300 text-indigo-600 focus:ring-indigo-500"
          />
          {{ t('editor.searchDialog.caseSensitive') }}
        </label>
        <span v-if="searchResultCount > 0" class="text-sm text-slate-500">
          {{ currentSearchIndex + 1 }} / {{ searchResultCount }}
        </span>
        <span v-else-if="searchTerm" class="text-sm text-slate-400">
          {{ t('editor.searchDialog.noResults') }}
        </span>
      </div>
    </div>
    <template #footer>
      <div class="flex justify-between">
        <div class="flex gap-3">
          <Tooltip :content="t('editor.searchDialog.findPrevious')">
            <Button type="secondary" :disabled="searchResultCount === 0" @click="findPrevious">
              <template #icon><ChevronUp class="w-4 h-4" /></template>
            </Button>
          </Tooltip>
          <Tooltip :content="t('editor.searchDialog.findNext')">
            <Button type="secondary" :disabled="searchResultCount === 0" @click="findNext">
              <template #icon><ChevronDown class="w-4 h-4" /></template>
            </Button>
          </Tooltip>
        </div>
        <div class="flex gap-3">
          <Button type="secondary" :disabled="searchResultCount === 0" @click="replaceOne">
            {{ t('editor.searchDialog.replace') }}
          </Button>
          <Button type="primary" :disabled="searchResultCount === 0" @click="replaceAll">
            {{ t('editor.searchDialog.replaceAll') }}
          </Button>
        </div>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'
import { Dialog, Tooltip, Button } from '../ui'
import { ChevronUp, ChevronDown } from 'lucide-vue-next'

const { t } = useI18n()

const props = defineProps<{
  editor: Editor | null
}>()

const visible = defineModel<boolean>({ default: false })

const searchTerm = ref('')
const replaceTerm = ref('')
const caseSensitive = ref(false)
const searchResultCount = ref(0)
const currentSearchIndex = ref(0)

// 关闭时清除搜索状态
watch(visible, (val) => {
  if (!val && props.editor) {
    props.editor.commands.clearSearch()
    searchTerm.value = ''
    replaceTerm.value = ''
    searchResultCount.value = 0
    currentSearchIndex.value = 0
  }
})

const updateSearchState = () => {
  if (!props.editor) return
  const storage = (props.editor.storage as unknown as Record<string, unknown>).searchAndReplace as
    | { results: unknown[]; currentIndex: number }
    | undefined
  if (storage) {
    searchResultCount.value = storage.results.length
    currentSearchIndex.value = storage.currentIndex
  }
}

const handleSearchInput = () => {
  if (!props.editor) return
  props.editor.commands.setSearchTerm(searchTerm.value)
  updateSearchState()
}

const handleReplaceInput = () => {
  if (!props.editor) return
  props.editor.commands.setReplaceTerm(replaceTerm.value)
}

const handleCaseSensitiveChange = () => {
  if (!props.editor) return
  props.editor.commands.setCaseSensitive(caseSensitive.value)
  updateSearchState()
}

const findNext = () => {
  if (!props.editor) return
  props.editor.commands.nextSearchResult()
  updateSearchState()
}

const findPrevious = () => {
  if (!props.editor) return
  props.editor.commands.previousSearchResult()
  updateSearchState()
}

const replaceOne = () => {
  if (!props.editor) return
  props.editor.commands.replaceCurrentResult()
  setTimeout(updateSearchState, 10)
}

const replaceAll = () => {
  if (!props.editor) return
  props.editor.commands.replaceAllResults()
  setTimeout(updateSearchState, 10)
}
</script>
