<template>
  <div class="border-t border-edge">
    <!-- Header -->
    <button
      @click="expanded = !expanded"
      class="w-full flex items-center justify-between px-4 py-2 text-xs font-medium text-content-secondary hover:bg-surface-alt transition-colors"
    >
      <span class="flex items-center gap-1.5">
        <Link2 class="w-3.5 h-3.5" />
        {{ t('noteLink.title') }} ({{ links.length }})
      </span>
      <ChevronDown class="w-3.5 h-3.5 transition-transform" :class="expanded ? '' : '-rotate-90'" />
    </button>

    <div v-if="expanded" class="px-4 pb-3 space-y-2">
      <!-- Search to add -->
      <div class="relative" v-if="showSearch">
        <input
          ref="searchInput"
          v-model="searchKeyword"
          :placeholder="t('noteLink.searchPlaceholder')"
          class="w-full px-3 py-1.5 text-xs border border-edge rounded-md bg-surface text-content focus:outline-none focus:ring-1 focus:ring-indigo-500"
          @input="handleSearch"
          @keydown.escape="showSearch = false"
        />
        <div
          v-if="searchResults.length > 0"
          class="absolute z-10 mt-1 w-full bg-surface border border-edge rounded-lg shadow-lg max-h-40 overflow-y-auto"
        >
          <button
            v-for="result in searchResults"
            :key="result.noteId"
            @click="addLink(result.noteId)"
            class="w-full text-left px-3 py-2 text-xs text-content hover:bg-surface-alt transition-colors truncate"
          >
            {{ result.noteTitle }}
          </button>
        </div>
        <div
          v-if="searchKeyword && searchResults.length === 0 && !searching"
          class="absolute z-10 mt-1 w-full bg-surface border border-edge rounded-lg shadow-lg p-3 text-xs text-content-tertiary text-center"
        >
          {{ t('noteLink.noResults') }}
        </div>
      </div>

      <!-- Add button -->
      <button
        v-if="!showSearch"
        @click="openSearch"
        class="flex items-center gap-1 text-xs text-indigo-600 hover:text-indigo-700 transition-colors"
      >
        <Plus class="w-3 h-3" />
        {{ t('noteLink.addLink') }}
      </button>

      <!-- Link list -->
      <div v-if="links.length === 0 && !showSearch" class="text-xs text-content-tertiary py-1">
        {{ t('noteLink.noLinks') }}
      </div>
      <div v-for="link in links" :key="link.id" class="flex items-center justify-between group">
        <button
          @click="$emit('navigate-to-note', link.noteId)"
          class="text-xs text-indigo-600 hover:text-indigo-700 hover:underline truncate flex-1 text-left"
        >
          {{ link.noteTitle }}
        </button>
        <button
          @click="removeLink(link.id)"
          class="opacity-0 group-hover:opacity-100 p-0.5 text-content-tertiary hover:text-red-500 transition-all"
          :title="t('noteLink.removeLink')"
        >
          <X class="w-3 h-3" />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { Link2, ChevronDown, Plus, X } from 'lucide-vue-next'
import { noteLinkApi } from '../api/note'
import type { NoteLink } from '../types'

const { t } = useI18n()

const props = defineProps<{
  noteId: number
}>()

defineEmits<{
  'navigate-to-note': [noteId: number]
}>()

const expanded = ref(false)
const links = ref<NoteLink[]>([])
const showSearch = ref(false)
const searchKeyword = ref('')
const searchResults = ref<NoteLink[]>([])
const searching = ref(false)
const searchInput = ref<HTMLInputElement>()

let searchTimer: ReturnType<typeof setTimeout> | null = null

const loadLinks = async () => {
  if (!props.noteId || props.noteId <= 0) return
  try {
    links.value = await noteLinkApi.findLinks(props.noteId)
  } catch {
    // ignore
  }
}

const openSearch = () => {
  showSearch.value = true
  searchKeyword.value = ''
  searchResults.value = []
  nextTick(() => searchInput.value?.focus())
}

const handleSearch = () => {
  if (searchTimer) clearTimeout(searchTimer)
  searchTimer = setTimeout(async () => {
    if (!searchKeyword.value.trim()) {
      searchResults.value = []
      return
    }
    searching.value = true
    try {
      searchResults.value = await noteLinkApi.searchLinkable(props.noteId, searchKeyword.value)
    } catch {
      searchResults.value = []
    } finally {
      searching.value = false
    }
  }, 300)
}

const addLink = async (targetNoteId: number) => {
  try {
    await noteLinkApi.createLink(props.noteId, targetNoteId)
    showSearch.value = false
    searchKeyword.value = ''
    searchResults.value = []
    await loadLinks()
  } catch {
    // ignore
  }
}

const removeLink = async (linkId: number) => {
  try {
    await noteLinkApi.deleteLink(linkId)
    await loadLinks()
  } catch {
    // ignore
  }
}

watch(
  () => props.noteId,
  () => {
    loadLinks()
    showSearch.value = false
  },
  { immediate: true },
)
</script>
