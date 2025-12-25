import { tags, state } from './store'
import { noteApi } from '../api/note'
import { parseId, validateTagName } from '../utils/validation'
import { showError, withNotification } from '../utils/errorHandler'
import type { ShowTag } from '../types'

export function useTagManager() {
  // 获取标签列表
  const getTags = async () => {
    const result = await withNotification(
      async () => {
        const data = (await noteApi.geTags()).map(
          (tag): ShowTag => ({
            id: String(tag.id),
            name: tag.name,
            icon: tag.icon,
            cls: tag.cls,
            createTime: tag.createTime ?? '',
            updateTime: tag.updateTime ?? '',
          }),
        )
        return data
      },
      { loading: '正在加载标签', error: '加载标签失败' },
    )

    if (result) {
      tags.value = [tags.value[0], ...result]
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
        loading: '正在保存标签',
        success: '标签保存成功',
        error: '保存标签失败',
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
        loading: '正在删除标签',
        success: '标签已删除',
        error: '删除标签失败',
      },
    )
  }

  // 设置活动标签
  const setActiveTag = async (tagId: string, searchNotes: () => Promise<void>) => {
    state.activeTag = tagId
    state.activeNote = null
    state.noteSearchPageParam.tagId = parseId(tagId)
    await searchNotes()
  }

  // 根据ID获取标签
  const getTagById = (tagId: string) => {
    return tags.value.find((tag) => tag.id === tagId)
  }

  return {
    tags,
    getTags,
    saveTag,
    deleteTag,
    setActiveTag,
    getTagById,
  }
}
