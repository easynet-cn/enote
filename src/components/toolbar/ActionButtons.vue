<template>
  <div class="toolbar-actions">
    <!-- 编辑按钮（非编辑模式显示） -->
    <Tooltip v-if="!editMode" :content="t('editor.toolbarTooltip.edit')" placement="bottom">
      <button
        class="action-btn action-btn-primary"
        @click="emit('edit')"
        :aria-label="t('editor.toolbarTooltip.edit')"
      >
        <Pencil class="w-4 h-4" />
      </button>
    </Tooltip>

    <!-- 保存按钮（编辑模式显示） -->
    <Tooltip v-if="editMode" :content="t('editor.toolbarTooltip.save')" placement="bottom">
      <button
        class="action-btn action-btn-success"
        @click="emit('save')"
        :aria-label="t('editor.toolbarTooltip.save')"
      >
        <Check class="w-4 h-4" />
      </button>
    </Tooltip>

    <!-- 取消按钮（编辑模式显示） -->
    <Tooltip v-if="editMode" :content="t('editor.toolbarTooltip.cancel')" placement="bottom">
      <button
        class="action-btn action-btn-secondary"
        @click="emit('cancel')"
        :aria-label="t('editor.toolbarTooltip.cancel')"
      >
        <X class="w-4 h-4" />
      </button>
    </Tooltip>

    <!-- 分隔线 -->
    <div class="action-divider"></div>

    <!-- 设置按钮（编辑模式显示） -->
    <Tooltip v-if="editMode" :content="t('editor.toolbarTooltip.settings')" placement="bottom">
      <button
        class="action-btn action-btn-ghost"
        @click="emit('settings')"
        :aria-label="t('editor.toolbarTooltip.settings')"
      >
        <Settings class="w-4 h-4" />
      </button>
    </Tooltip>

    <!-- 导出按钮 -->
    <Tooltip :content="t('editor.toolbarTooltip.export')" placement="bottom">
      <button
        class="action-btn action-btn-ghost"
        @click="emit('export')"
        :aria-label="t('editor.toolbarTooltip.export')"
      >
        <Download class="w-4 h-4" />
      </button>
    </Tooltip>

    <!-- 历史记录按钮 -->
    <Tooltip :content="t('editor.toolbarTooltip.history')" placement="bottom">
      <button
        class="action-btn action-btn-ghost"
        @click="emit('history')"
        :aria-label="t('editor.toolbarTooltip.history')"
      >
        <History class="w-4 h-4" />
      </button>
    </Tooltip>

    <!-- 存为模板按钮 -->
    <Tooltip :content="t('editor.toolbarTooltip.saveAsTemplate')" placement="bottom">
      <button
        class="action-btn action-btn-ghost"
        @click="emit('saveAsTemplate')"
        :aria-label="t('editor.toolbarTooltip.saveAsTemplate')"
      >
        <LayoutTemplate class="w-4 h-4" />
      </button>
    </Tooltip>

    <!-- 删除按钮 -->
    <Tooltip :content="t('editor.toolbarTooltip.delete')" placement="bottom">
      <button
        class="action-btn action-btn-danger"
        @click="emit('delete')"
        :aria-label="t('editor.toolbarTooltip.delete')"
      >
        <Trash2 class="w-4 h-4" />
      </button>
    </Tooltip>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { Tooltip } from '../ui'
import {
  Pencil,
  Check,
  X,
  Settings,
  Download,
  History,
  Trash2,
  LayoutTemplate,
} from 'lucide-vue-next'

const { t } = useI18n()

defineProps<{
  editMode: boolean
}>()

const emit = defineEmits<{
  edit: []
  save: []
  cancel: []
  delete: []
  settings: []
  history: []
  export: []
  saveAsTemplate: []
}>()
</script>

<style scoped>
.toolbar-actions {
  display: flex;
  align-items: center;
  gap: 6px;
}

.action-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  height: 32px;
  padding: 0 12px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
  outline: none;
}

.action-btn-primary {
  background: var(--color-primary);
  color: white;
  padding: 0 10px;
  box-shadow: var(--shadow-primary);
}

.action-btn-primary:hover {
  background: var(--color-primary-hover);
  box-shadow: var(--shadow-primary);
  transform: translateY(-1px);
}

.action-btn-success {
  background: var(--color-primary);
  color: white;
  padding: 0 10px;
  box-shadow: var(--shadow-primary);
}

.action-btn-success:hover {
  background: var(--color-primary-hover);
  box-shadow: var(--shadow-primary);
  transform: translateY(-1px);
}

.action-btn-secondary {
  background: var(--color-border);
  color: var(--color-text-primary);
  padding: 0 8px;
}

.action-btn-secondary:hover {
  background: var(--color-border-dark);
}

.action-btn-ghost {
  background: transparent;
  color: var(--color-text-secondary);
  padding: 0 8px;
}

.action-btn-ghost:hover {
  background: var(--color-border);
  color: var(--color-text-primary);
}

.action-btn-danger {
  background: transparent;
  color: var(--color-text-tertiary);
  padding: 0 8px;
}

.action-btn-danger:hover {
  background: var(--color-danger-light);
  color: var(--color-danger);
}

.action-divider {
  width: 1px;
  height: 20px;
  background: var(--color-border-dark);
  margin: 0 4px;
}

/* 平板适配 */
@media (min-width: 640px) and (max-width: 1024px) {
  .toolbar-actions {
    gap: 4px;
  }

  .action-btn {
    height: 28px;
    padding: 0 8px;
    font-size: 12px;
  }

  .action-divider {
    margin: 0 2px;
    height: 16px;
  }
}

/* 手机适配 */
@media (max-width: 639px) {
  .toolbar-actions {
    gap: 2px;
  }

  .action-btn {
    height: 28px;
    padding: 0 6px;
    border-radius: 6px;
    font-size: 12px;
  }

  .action-divider {
    margin: 0 1px;
    height: 14px;
  }
}
</style>
