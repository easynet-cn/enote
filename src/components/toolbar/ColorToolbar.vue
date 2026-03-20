<template>
  <div class="toolbar-section">
    <Tooltip :content="t('editor.toolbarTooltip.textColor')" placement="bottom">
      <ColorPicker
        :model-value="textColor"
        @change="$emit('update:textColor', $event)"
        :predefine="predefineColors"
        :disabled="!editMode"
      />
    </Tooltip>

    <Tooltip :content="t('editor.toolbarTooltip.backgroundColor')" placement="bottom">
      <ColorPicker
        :model-value="highlightColor"
        @change="$emit('update:highlightColor', $event)"
        :predefine="predefineColors"
        :disabled="!editMode"
      />
    </Tooltip>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { Tooltip, ColorPicker } from '../ui'

defineProps<{
  textColor: string
  highlightColor: string
  editMode: boolean
}>()

defineEmits<{
  'update:textColor': [color: string]
  'update:highlightColor': [color: string]
}>()

const { t } = useI18n()

const predefineColors = [
  '#000000',
  '#ffffff',
  '#ff4500',
  '#ff8c00',
  '#ffd700',
  '#90ee90',
  '#00ced1',
  '#1e90ff',
  '#c71585',
  '#8b4513',
]
</script>
