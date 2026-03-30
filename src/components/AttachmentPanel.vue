<template>
  <div>
    <!-- Header -->
    <button
      @click="expanded = !expanded"
      class="w-full flex items-center justify-between px-4 py-2 text-xs font-medium text-content-secondary hover:bg-surface-alt transition-colors"
    >
      <span class="flex items-center gap-1.5">
        <Paperclip class="w-3.5 h-3.5" />
        {{ t('attachment.title') }} ({{ attachments.length }})
      </span>
      <ChevronDown class="w-3.5 h-3.5 transition-transform" :class="expanded ? '' : '-rotate-90'" />
    </button>

    <div v-if="expanded" class="px-4 pb-3 space-y-2">
      <!-- Add button -->
      <button
        @click="handleAddAttachment"
        class="flex items-center gap-1 text-xs text-indigo-600 hover:text-indigo-700 transition-colors"
        :disabled="uploading"
      >
        <Plus class="w-3 h-3" />
        {{ uploading ? '...' : t('attachment.add') }}
      </button>

      <!-- Drop zone hint (shown when dragging) -->
      <div
        v-if="isDragging"
        class="border-2 border-dashed border-indigo-300 rounded-lg p-4 text-center text-xs text-indigo-500 bg-indigo-50/50"
      >
        {{ t('attachment.add') }}
      </div>

      <!-- Empty state -->
      <div
        v-if="attachments.length === 0 && !isDragging"
        class="text-xs text-content-tertiary py-1"
      >
        {{ t('attachment.empty') }}
      </div>

      <!-- Attachment list -->
      <div
        v-for="item in attachments"
        :key="item.id"
        class="flex items-center justify-between group py-1"
      >
        <button
          @click="handleOpen(item)"
          class="flex items-center gap-1.5 text-xs text-content hover:text-indigo-600 truncate flex-1 text-left transition-colors"
          :title="item.fileName"
        >
          <component
            :is="getFileIcon(item.mimeType)"
            class="w-3.5 h-3.5 shrink-0 text-content-tertiary"
          />
          <span class="truncate">{{ item.fileName }}</span>
          <span class="text-content-tertiary shrink-0">({{ formatFileSize(item.fileSize) }})</span>
        </button>
        <button
          @click="handleDelete(item)"
          class="opacity-0 group-hover:opacity-100 p-0.5 text-content-tertiary hover:text-red-500 transition-all shrink-0 ml-1"
          :title="t('attachment.delete')"
        >
          <X class="w-3 h-3" />
        </button>
      </div>
    </div>

    <!-- Delete confirm -->
    <ConfirmDialog
      v-model="deleteConfirmVisible"
      :title="t('attachment.delete')"
      :message="t('attachment.deleteConfirm')"
      type="danger"
      @confirm="confirmDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount } from 'vue'
import { useI18n } from 'vue-i18n'
import {
  Paperclip,
  ChevronDown,
  Plus,
  X,
  FileText,
  FileImage,
  FileVideo,
  FileAudio,
  FileArchive,
  FileSpreadsheet,
  File,
} from 'lucide-vue-next'
import { open } from '@tauri-apps/plugin-dialog'
import { readFile } from '@tauri-apps/plugin-fs'
import { attachmentApi } from '../api/note'
import { showNotification } from './ui/notification'
import { ConfirmDialog } from './ui'
import type { NoteAttachment } from '../types'

const { t } = useI18n()

const props = defineProps<{
  noteId: number
}>()

const expanded = ref(true)
const attachments = ref<NoteAttachment[]>([])
const uploading = ref(false)
const isDragging = ref(false)
const deleteConfirmVisible = ref(false)
const pendingDeleteId = ref<number | null>(null)

let dragCounter = 0

// Load attachments for the note
const loadAttachments = async () => {
  if (!props.noteId || props.noteId <= 0) return
  try {
    attachments.value = await attachmentApi.findAttachments(props.noteId)
  } catch {
    // ignore
  }
}

// Format file size to human-readable string
const formatFileSize = (bytes: number): string => {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

// Get appropriate icon for file type
const getFileIcon = (mimeType: string) => {
  if (mimeType.startsWith('image/')) return FileImage
  if (mimeType.startsWith('video/')) return FileVideo
  if (mimeType.startsWith('audio/')) return FileAudio
  if (
    mimeType.includes('zip') ||
    mimeType.includes('rar') ||
    mimeType.includes('tar') ||
    mimeType.includes('gz') ||
    mimeType.includes('7z')
  )
    return FileArchive
  if (mimeType.includes('spreadsheet') || mimeType.includes('excel') || mimeType.includes('csv'))
    return FileSpreadsheet
  if (
    mimeType.includes('pdf') ||
    mimeType.includes('document') ||
    mimeType.includes('text') ||
    mimeType.includes('word')
  )
    return FileText
  return File
}

// Guess MIME type from extension
const guessMimeType = (fileName: string): string => {
  const ext = fileName.split('.').pop()?.toLowerCase() || ''
  const mimeMap: Record<string, string> = {
    pdf: 'application/pdf',
    doc: 'application/msword',
    docx: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    xls: 'application/vnd.ms-excel',
    xlsx: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    ppt: 'application/vnd.ms-powerpoint',
    pptx: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
    zip: 'application/zip',
    rar: 'application/x-rar-compressed',
    '7z': 'application/x-7z-compressed',
    tar: 'application/x-tar',
    gz: 'application/gzip',
    png: 'image/png',
    jpg: 'image/jpeg',
    jpeg: 'image/jpeg',
    gif: 'image/gif',
    webp: 'image/webp',
    svg: 'image/svg+xml',
    mp3: 'audio/mpeg',
    wav: 'audio/wav',
    mp4: 'video/mp4',
    avi: 'video/x-msvideo',
    txt: 'text/plain',
    csv: 'text/csv',
    json: 'application/json',
    xml: 'application/xml',
    html: 'text/html',
    md: 'text/markdown',
  }
  return mimeMap[ext] || 'application/octet-stream'
}

// Upload a file from path
const uploadFile = async (filePath: string) => {
  const fileName = filePath.split('/').pop() || filePath.split('\\').pop() || 'file'
  const mimeType = guessMimeType(fileName)

  try {
    uploading.value = true
    const fileData = await readFile(filePath)

    // Check file size (50 MB limit)
    if (fileData.byteLength > 50 * 1024 * 1024) {
      showNotification({ type: 'error', message: t('attachment.fileTooLarge') })
      return
    }

    await attachmentApi.saveAttachment(
      props.noteId,
      fileName,
      Array.from(new Uint8Array(fileData)),
      mimeType,
    )
    await loadAttachments()
  } catch {
    showNotification({ type: 'error', message: t('attachment.uploadFailed') })
  } finally {
    uploading.value = false
  }
}

// Handle "Add Attachment" button
const handleAddAttachment = async () => {
  const selected = await open({
    multiple: true,
    title: t('attachment.add'),
  })

  if (!selected) return

  const paths = Array.isArray(selected) ? selected : [selected]
  for (const filePath of paths) {
    await uploadFile(filePath)
  }
}

// Open attachment with system default program
const handleOpen = async (item: NoteAttachment) => {
  try {
    await attachmentApi.openAttachment(item.filePath)
  } catch {
    // ignore
  }
}

// Delete attachment
const handleDelete = (item: NoteAttachment) => {
  pendingDeleteId.value = item.id
  deleteConfirmVisible.value = true
}

const confirmDelete = async () => {
  if (pendingDeleteId.value === null) return
  try {
    await attachmentApi.deleteAttachment(pendingDeleteId.value)
    await loadAttachments()
  } catch {
    // ignore
  }
  pendingDeleteId.value = null
}

// Drag and drop support
const handleDragEnter = (e: DragEvent) => {
  e.preventDefault()
  dragCounter++
  if (e.dataTransfer?.types.includes('Files')) {
    isDragging.value = true
    if (!expanded.value) expanded.value = true
  }
}

const handleDragLeave = (e: DragEvent) => {
  e.preventDefault()
  dragCounter--
  if (dragCounter === 0) {
    isDragging.value = false
  }
}

const handleDragOver = (e: DragEvent) => {
  e.preventDefault()
}

const handleDrop = (e: Event) => {
  e.preventDefault()
  isDragging.value = false
  dragCounter = 0
}

// Wrapper functions typed as EventListener for addEventListener
const onDragEnter: EventListener = (e) => handleDragEnter(e as DragEvent)
const onDragLeave: EventListener = (e) => handleDragLeave(e as DragEvent)
const onDragOver: EventListener = (e) => handleDragOver(e as DragEvent)
const onDrop: EventListener = (e) => handleDrop(e)

// Watch note changes
watch(
  () => props.noteId,
  () => {
    loadAttachments()
  },
  { immediate: true },
)

onMounted(() => {
  const el = document.querySelector('.editor-main')
  if (el) {
    el.addEventListener('dragenter', onDragEnter)
    el.addEventListener('dragleave', onDragLeave)
    el.addEventListener('dragover', onDragOver)
    el.addEventListener('drop', onDrop)
  }
})

onBeforeUnmount(() => {
  const el = document.querySelector('.editor-main')
  if (el) {
    el.removeEventListener('dragenter', onDragEnter)
    el.removeEventListener('dragleave', onDragLeave)
    el.removeEventListener('dragover', onDragOver)
    el.removeEventListener('drop', onDrop)
  }
})
</script>
