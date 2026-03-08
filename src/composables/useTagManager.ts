import { useAppStore } from '../stores/app'
import { noteApi } from '../api/note'
import { parseId, validateTagName } from '../utils/validation'
import { tagToShowTag } from '../utils/converters'
import { showError, withNotification } from '../utils/errorHandler'
import i18n from '../i18n'
import type { ShowTag } from '../types'

export function useTagManager() {
  const store = useAppStore()

  // 获取标签列表
  const getTags = async () => {
    const result = await withNotification(
      async () => {
        const data = (await noteApi.getTags()).map(tagToShowTag)
        return data
      },
      {
        loading: i18n.global.t('composable.loadingTags'),
        error: i18n.global.t('composable.loadTagsFailed'),
      },
    )

    if (result) {
      store.setTags(result)
    }
  }

  // 保存标签（创建或更新）
  const saveTag = async (showTag: ShowTag, refreshData: () => Promise<void>) => {
    const validation = validateTagName(showTag.name)
    if (!validation.valid) {
      showError(null, validation.error)
      return
    }

    const id = parseId(showTag.id)

    await withNotification(
      async () => {
        const tagData = {
          id,
          name: showTag.name ?? '',
          icon: showTag.icon,
          cls: showTag.cls,
        }

        if (id === 0) {
          await noteApi.createTag(tagData)
        } else {
          await noteApi.updateTag(tagData)
        }

        await refreshData()
      },
      {
        loading: i18n.global.t('composable.savingTag'),
        success: i18n.global.t('composable.tagSaved'),
        error: i18n.global.t('composable.saveTagFailed'),
      },
    )
  }

  // 删除标签
  const deleteTag = async (id: string, refreshData: () => Promise<void>) => {
    const tagId = parseId(id)
    if (tagId === 0) return

    await withNotification(
      async () => {
        await noteApi.deleteTag(tagId)
        await refreshData()
      },
      {
        loading: i18n.global.t('composable.deletingTag'),
        success: i18n.global.t('composable.tagDeleted'),
        error: i18n.global.t('composable.deleteTagFailed'),
      },
    )
  }

  // 设置活动标签
  const setActiveTag = async (tagId: string, searchNotes: () => Promise<void>) => {
    store.activeTag = tagId
    store.activeNote = null
    store.notePageIndex = 1
    store.noteSearchPageParam.tagId = parseId(tagId)
    await searchNotes()
  }

  return {
    getTags,
    saveTag,
    deleteTag,
    setActiveTag,
    getTagById: store.getTagById,
  }
}
