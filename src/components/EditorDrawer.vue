<template>
  <Transition name="drawer-right">
    <div
      v-if="(showAttachment || showLink) && activeNote && !isNewNote"
      class="editor-drawer-overlay"
    >
      <!-- 点击遮罩区域关闭 -->
      <div class="drawer-backdrop" @click="emit('close')" />
      <!-- 抽屉面板 -->
      <aside class="editor-drawer">
        <!-- 标题栏 -->
        <div class="drawer-header">
          <div class="flex items-center gap-2">
            <button
              class="drawer-tab"
              :class="{ active: showAttachment }"
              @click="emit('switchTab', 'attachment')"
            >
              <Paperclip class="w-3.5 h-3.5" />
              {{ t('attachment.title') }}
            </button>
            <button
              class="drawer-tab"
              :class="{ active: showLink }"
              @click="emit('switchTab', 'link')"
            >
              <Link2 class="w-3.5 h-3.5" />
              {{ t('noteLink.title') }}
            </button>
          </div>
          <button
            class="p-1 rounded-md text-content-tertiary hover:text-content hover:bg-surface-dim transition-colors"
            @click="emit('close')"
            :title="t('common.close')"
          >
            <XIcon class="w-4 h-4" />
          </button>
        </div>
        <!-- 内容 -->
        <div class="drawer-body">
          <AttachmentPanel v-if="showAttachment" :note-id="Number(activeNote.id)" />
          <NoteLinkPanel
            v-if="showLink"
            :note-id="Number(activeNote.id)"
            @navigate-to-note="(id: number) => emit('navigateToNote', id)"
          />
        </div>
      </aside>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { Paperclip, Link2, X as XIcon } from 'lucide-vue-next'
import AttachmentPanel from './AttachmentPanel.vue'
import NoteLinkPanel from './NoteLinkPanel.vue'
import type { ShowNote } from '../types'

const { t } = useI18n()

interface Props {
  activeNote: ShowNote | null
  isNewNote: boolean
  showAttachment: boolean
  showLink: boolean
}

defineProps<Props>()

const emit = defineEmits<{
  switchTab: [tab: 'attachment' | 'link']
  close: []
  navigateToNote: [id: number]
}>()
</script>

<style scoped>
/* 右侧悬浮抽屉 */
.editor-drawer-overlay {
  position: absolute;
  inset: 0;
  z-index: 30;
  display: flex;
}

.drawer-backdrop {
  flex: 1;
  min-width: 0;
}

.editor-drawer {
  width: 400px;
  max-width: 80%;
  display: flex;
  flex-direction: column;
  background: var(--color-bg, #fff);
  box-shadow:
    -8px 0 24px rgba(0, 0, 0, 0.12),
    -2px 0 8px rgba(0, 0, 0, 0.06);
  overflow: hidden;
}

.drawer-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
}

.drawer-tab {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 5px 12px;
  font-size: 13px;
  border-radius: 6px;
  color: var(--color-text-secondary);
  transition: all 0.15s;
  cursor: pointer;
}

.drawer-tab:hover {
  background: var(--color-surface-dim, #f1f5f9);
  color: var(--color-text);
}

.drawer-tab.active {
  background: var(--color-primary-lighter, #eef2ff);
  color: var(--color-primary, #4f46e5);
  font-weight: 500;
}

.drawer-body {
  flex: 1;
  overflow-y: auto;
  padding: 4px 0;
}

/* drawer-right 过渡 */
.drawer-right-enter-active,
.drawer-right-leave-active {
  transition: opacity 0.2s ease;
}

.drawer-right-enter-active .editor-drawer,
.drawer-right-leave-active .editor-drawer {
  transition: transform 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.drawer-right-enter-from,
.drawer-right-leave-to {
  opacity: 0;
}

.drawer-right-enter-from .editor-drawer,
.drawer-right-leave-to .editor-drawer {
  transform: translateX(100%);
}
</style>
