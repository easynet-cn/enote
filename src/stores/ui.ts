import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useUiStore = defineStore('ui', () => {
  const activeNotebook = ref<string>('')
  const activeTag = ref<string>('')
  const loading = ref<boolean>(false)

  return {
    activeNotebook,
    activeTag,
    loading,
  }
})
