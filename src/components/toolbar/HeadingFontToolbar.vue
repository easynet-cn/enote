<template>
  <div class="toolbar-section">
    <Tooltip :content="t('editor.toolbarTooltip.headingLevel')" placement="bottom">
      <AppSelect
        :modelValue="headingLevel"
        @update:modelValue="$emit('update:headingLevel', String($event))"
        :options="headingOptions"
        :disabled="!editMode"
        size="sm"
      />
    </Tooltip>

    <Tooltip :content="t('editor.toolbarTooltip.fontFamily')" placement="bottom">
      <AppSelect
        :modelValue="fontFamily"
        @update:modelValue="$emit('update:fontFamily', String($event))"
        :options="fontFamilyOptions"
        :disabled="!editMode"
        size="sm"
        class="ml-1"
      />
    </Tooltip>

    <Tooltip :content="t('editor.toolbarTooltip.fontSize')" placement="bottom">
      <AppSelect
        :modelValue="fontSize"
        @update:modelValue="$emit('update:fontSize', String($event))"
        :options="fontSizeOptions"
        :disabled="!editMode"
        size="sm"
        class="ml-1"
      />
    </Tooltip>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { Tooltip, AppSelect } from '../ui'
import type { AppSelectOption } from '../ui'

defineProps<{
  headingLevel: string
  fontFamily: string
  fontSize: string
  editMode: boolean
}>()

defineEmits<{
  'update:headingLevel': [value: string]
  'update:fontFamily': [value: string]
  'update:fontSize': [value: string]
}>()

const { t } = useI18n()

const headingOptions = computed<AppSelectOption[]>(() => [
  { label: t('editor.headingOptions.normal'), value: '0' },
  { label: t('editor.headingOptions.h1'), value: '1' },
  { label: t('editor.headingOptions.h2'), value: '2' },
  { label: t('editor.headingOptions.h3'), value: '3' },
  { label: t('editor.headingOptions.h4'), value: '4' },
  { label: t('editor.headingOptions.h5'), value: '5' },
  { label: t('editor.headingOptions.h6'), value: '6' },
])

const fontFamilyOptions = computed<AppSelectOption[]>(() => [
  { label: t('editor.fontOptions.default'), value: '' },
  {
    group: t('editor.fontOptions.sansSerif'),
    label: t('editor.fontOptions.sansSerif'),
    value: '__sans',
    children: [
      { label: 'Arial', value: 'Arial, sans-serif' },
      { label: 'Helvetica', value: 'Helvetica, sans-serif' },
      { label: 'Verdana', value: 'Verdana, sans-serif' },
      { label: 'Tahoma', value: 'Tahoma, sans-serif' },
      { label: 'Trebuchet MS', value: 'Trebuchet MS, sans-serif' },
      { label: t('fontName.microsoftYaHei'), value: 'Microsoft YaHei, sans-serif' },
      { label: t('fontName.pingFang'), value: 'PingFang SC, sans-serif' },
    ],
  },
  {
    group: t('editor.fontOptions.serif'),
    label: t('editor.fontOptions.serif'),
    value: '__serif',
    children: [
      { label: 'Times New Roman', value: 'Times New Roman, serif' },
      { label: 'Georgia', value: 'Georgia, serif' },
      { label: 'Palatino', value: 'Palatino, serif' },
      { label: t('fontName.simSun'), value: 'SimSun, serif' },
      { label: t('fontName.kaiTi'), value: 'KaiTi, serif' },
      { label: t('fontName.fangSong'), value: 'FangSong, serif' },
    ],
  },
  {
    group: t('editor.fontOptions.monospace'),
    label: t('editor.fontOptions.monospace'),
    value: '__mono',
    children: [
      { label: 'Courier New', value: 'Courier New, monospace' },
      { label: 'Consolas', value: 'Consolas, monospace' },
      { label: 'Monaco', value: 'Monaco, monospace' },
      { label: 'Source Code Pro', value: 'Source Code Pro, monospace' },
    ],
  },
  {
    group: t('editor.fontOptions.artistic'),
    label: t('editor.fontOptions.artistic'),
    value: '__artistic',
    children: [
      { label: 'Comic Sans MS', value: 'Comic Sans MS, cursive' },
      { label: 'Impact', value: 'Impact, fantasy' },
      { label: 'Brush Script', value: 'Brush Script MT, cursive' },
    ],
  },
])

const fontSizeOptions = computed<AppSelectOption[]>(() => [
  { label: t('editor.fontOptions.defaultSize'), value: '' },
  { label: '12px', value: '12px' },
  { label: '14px', value: '14px' },
  { label: '16px', value: '16px' },
  { label: '18px', value: '18px' },
  { label: '20px', value: '20px' },
  { label: '24px', value: '24px' },
  { label: '28px', value: '28px' },
  { label: '32px', value: '32px' },
  { label: '36px', value: '36px' },
  { label: '48px', value: '48px' },
  { label: '72px', value: '72px' },
])
</script>
