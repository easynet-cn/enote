import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import i18n from '../i18n'
import type { ShowTag } from '../types'

export const useTagStore = defineStore('tags', () => {
  const t = i18n.global.t

  const tagsMap = ref<Map<string, ShowTag>>(new Map())

  const defaultTag = computed<ShowTag>(() => ({
    id: '0',
    name: t('tag.all'),
    icon: 'Tags',
  }))

  // 初始化默认项
  tagsMap.value.set('0', defaultTag.value)

  // ==================== Getters ====================
  const tags = computed<ShowTag[]>(() => {
    return Array.from(tagsMap.value.values())
  })

  // ==================== Actions ====================
  const setTags = (items: ShowTag[]) => {
    tagsMap.value.clear()
    tagsMap.value.set('0', defaultTag.value)

    for (const item of items) {
      tagsMap.value.set(item.id, item)
    }
  }

  const updateTag = (tag: ShowTag) => {
    tagsMap.value.set(tag.id, tag)
  }

  const removeTag = (id: string) => {
    tagsMap.value.delete(id)
  }

  const getTagById = (id: string): ShowTag | undefined => {
    return tagsMap.value.get(id)
  }

  const updateDefaultTag = () => {
    tagsMap.value.set('0', defaultTag.value)
  }

  return {
    tags,
    setTags,
    updateTag,
    removeTag,
    getTagById,
    updateDefaultTag,
  }
})
