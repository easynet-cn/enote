<template>
  <Dialog v-model="visible" :title="t('trash.title')" :width="640" @close="handleClose">
    <div class="space-y-3">
      <!-- 空状态 -->
      <div v-if="deletedNotes.length === 0" class="text-center py-8">
        <Trash2 class="w-12 h-12 mx-auto mb-3 text-content-disabled" />
        <p class="text-content-secondary">{{ t('trash.empty') }}</p>
      </div>

      <!-- 笔记列表 -->
      <div
        v-for="note in deletedNotes"
        :key="note.id"
        class="flex items-center gap-3 p-3 border border-edge rounded-lg hover:bg-surface-alt transition-colors"
      >
        <div class="flex-1 min-w-0">
          <div class="font-medium text-content truncate">
            {{ note.title || t('noteList.noTitle') }}
          </div>
          <div class="text-xs text-content-tertiary mt-1">
            {{ note.updateTime }}
          </div>
        </div>
        <div class="flex gap-2 shrink-0">
          <Button type="success" size="small" @click="handleRestore(note.id)">
            {{ t('trash.restore') }}
          </Button>
          <Button type="danger" size="small" @click="handlePermanentDelete(note.id)">
            {{ t('trash.permanentDelete') }}
          </Button>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-between items-center">
        <div>
          <Button v-if="deletedNotes.length > 0" type="danger" @click="confirmEmptyTrash = true">
            {{ t('trash.emptyTrash') }}
          </Button>
        </div>
        <div class="flex items-center gap-3">
          <Pagination
            v-if="total > pageSize"
            :current-page="pageIndex"
            :page-size="pageSize"
            :total="total"
            @current-change="handlePageChange"
          />
          <Button type="secondary" @click="visible = false">{{ t('common.close') }}</Button>
        </div>
      </div>
    </template>
  </Dialog>

  <!-- 清空确认 -->
  <ConfirmDialog
    v-model="confirmEmptyTrash"
    :title="t('trash.emptyConfirm.title')"
    :message="t('trash.emptyConfirm.message')"
    type="danger"
    :confirm-text="t('trash.emptyConfirm.confirmText')"
    @confirm="handleEmptyTrash"
  />

  <!-- 永久删除确认 -->
  <ConfirmDialog
    v-model="confirmDelete"
    :title="t('trash.deleteConfirm.title')"
    :message="t('trash.deleteConfirm.message')"
    type="danger"
    :confirm-text="t('trash.deleteConfirm.confirmText')"
    @confirm="handleConfirmDelete"
  />
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Dialog, Button, ConfirmDialog, Pagination } from './ui'
import { Trash2 } from 'lucide-vue-next'
import { trashApi } from '../api/note'
import { showNotification } from './ui/notification'
import type { Note } from '../types'

const { t } = useI18n()

const visible = defineModel<boolean>({ default: false })

const emit = defineEmits<{
  restored: []
}>()

const deletedNotes = ref<Note[]>([])
const confirmEmptyTrash = ref(false)
const confirmDelete = ref(false)
const deleteTargetId = ref<number>(0)
const pageIndex = ref(1)
const pageSize = ref(20)
const total = ref(0)

const loadDeletedNotes = async () => {
  try {
    const result = await trashApi.findDeletedNotes(pageIndex.value, pageSize.value)
    deletedNotes.value = result.data
    total.value = result.total
  } catch {
    showNotification({ type: 'error', message: t('trash.restoreFailed') })
  }
}

const handlePageChange = (page: number) => {
  pageIndex.value = page
  loadDeletedNotes()
}

const handleRestore = async (id: number) => {
  try {
    await trashApi.restoreNote(id)
    showNotification({ type: 'success', message: t('trash.restored') })
    await loadDeletedNotes()
    emit('restored')
  } catch {
    showNotification({ type: 'error', message: t('trash.restoreFailed') })
  }
}

const handlePermanentDelete = (id: number) => {
  deleteTargetId.value = id
  confirmDelete.value = true
}

const handleConfirmDelete = async () => {
  try {
    await trashApi.permanentDeleteNote(deleteTargetId.value)
    showNotification({ type: 'success', message: t('trash.deleted') })
    await loadDeletedNotes()
  } catch {
    showNotification({ type: 'error', message: t('trash.deleteFailed') })
  }
}

const handleEmptyTrash = async () => {
  try {
    await trashApi.emptyTrash()
    showNotification({ type: 'success', message: t('trash.emptied') })
    deletedNotes.value = []
  } catch {
    showNotification({ type: 'error', message: t('trash.emptyFailed') })
  }
}

const handleClose = () => {
  visible.value = false
}

watch(visible, (newVal) => {
  if (newVal) {
    loadDeletedNotes()
  }
})
</script>
