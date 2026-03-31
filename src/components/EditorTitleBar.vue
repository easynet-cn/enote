<template>
  <div v-if="activeNote" class="title-area">
    <div class="flex items-center">
      <button
        v-if="props.layout === 'mobile'"
        @click="emit('back')"
        class="p-1.5 -ml-1.5 mr-1 text-content-secondary hover:text-content hover:bg-surface-dim rounded-lg transition-colors shrink-0"
        :aria-label="t('common.close')"
      >
        <ArrowLeft class="w-5 h-5" />
      </button>
      <div class="flex-1">
        <input
          ref="titleInput"
          :value="activeNote.title"
          @input="emit('updateTitle', ($event.target as HTMLInputElement).value)"
          :placeholder="t('editor.noteTitlePlaceholder')"
          :readonly="!editMode"
          :aria-label="t('editor.noteTitle')"
          class="w-full text-xl font-bold border-0 outline-none bg-transparent focus:ring-0"
          :class="{ 'cursor-default': !editMode }"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { ArrowLeft } from 'lucide-vue-next'
import type { ShowNote } from '../types'
import type { LayoutMode } from '../composables/usePlatform'

const { t } = useI18n()

interface Props {
  activeNote: ShowNote | null
  editMode: boolean
  layout?: LayoutMode
}

const props = defineProps<Props>()

const emit = defineEmits<{
  updateTitle: [title: string]
  back: []
}>()

const titleInput = ref<HTMLInputElement | null>(null)

const focus = () => {
  titleInput.value?.focus()
  titleInput.value?.setSelectionRange(0, 0)
}

defineExpose({ focus })
</script>

<style scoped>
.title-area {
  padding-top: 1rem;
  padding-bottom: 0.75rem;
  margin-bottom: 0.25rem;
  border-bottom: 1px solid var(--color-border);
}
</style>
