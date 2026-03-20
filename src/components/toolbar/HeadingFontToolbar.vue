<template>
  <div class="toolbar-section">
    <Tooltip :content="t('editor.toolbarTooltip.headingLevel')" placement="bottom">
      <select
        :value="headingLevel"
        @change="$emit('update:headingLevel', ($event.target as HTMLSelectElement).value)"
        :disabled="!editMode"
        class="h-8 px-2 text-sm border border-edge rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent disabled:opacity-50 disabled:cursor-not-allowed"
      >
        <option value="0">{{ t('editor.headingOptions.normal') }}</option>
        <option value="1">{{ t('editor.headingOptions.h1') }}</option>
        <option value="2">{{ t('editor.headingOptions.h2') }}</option>
        <option value="3">{{ t('editor.headingOptions.h3') }}</option>
        <option value="4">{{ t('editor.headingOptions.h4') }}</option>
        <option value="5">{{ t('editor.headingOptions.h5') }}</option>
        <option value="6">{{ t('editor.headingOptions.h6') }}</option>
      </select>
    </Tooltip>

    <Tooltip :content="t('editor.toolbarTooltip.fontFamily')" placement="bottom">
      <select
        :value="fontFamily"
        @change="$emit('update:fontFamily', ($event.target as HTMLSelectElement).value)"
        :disabled="!editMode"
        class="h-8 px-2 text-sm border border-edge rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent ml-1 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        <option value="">{{ t('editor.fontOptions.default') }}</option>
        <optgroup :label="t('editor.fontOptions.sansSerif')">
          <option value="Arial, sans-serif">Arial</option>
          <option value="Helvetica, sans-serif">Helvetica</option>
          <option value="Verdana, sans-serif">Verdana</option>
          <option value="Tahoma, sans-serif">Tahoma</option>
          <option value="Trebuchet MS, sans-serif">Trebuchet MS</option>
          <option value="Microsoft YaHei, sans-serif">{{ t('fontName.microsoftYaHei') }}</option>
          <option value="PingFang SC, sans-serif">{{ t('fontName.pingFang') }}</option>
        </optgroup>
        <optgroup :label="t('editor.fontOptions.serif')">
          <option value="Times New Roman, serif">Times New Roman</option>
          <option value="Georgia, serif">Georgia</option>
          <option value="Palatino, serif">Palatino</option>
          <option value="SimSun, serif">{{ t('fontName.simSun') }}</option>
          <option value="KaiTi, serif">{{ t('fontName.kaiTi') }}</option>
          <option value="FangSong, serif">{{ t('fontName.fangSong') }}</option>
        </optgroup>
        <optgroup :label="t('editor.fontOptions.monospace')">
          <option value="Courier New, monospace">Courier New</option>
          <option value="Consolas, monospace">Consolas</option>
          <option value="Monaco, monospace">Monaco</option>
          <option value="Source Code Pro, monospace">Source Code Pro</option>
        </optgroup>
        <optgroup :label="t('editor.fontOptions.artistic')">
          <option value="Comic Sans MS, cursive">Comic Sans MS</option>
          <option value="Impact, fantasy">Impact</option>
          <option value="Brush Script MT, cursive">Brush Script</option>
        </optgroup>
      </select>
    </Tooltip>

    <Tooltip :content="t('editor.toolbarTooltip.fontSize')" placement="bottom">
      <select
        :value="fontSize"
        @change="$emit('update:fontSize', ($event.target as HTMLSelectElement).value)"
        :disabled="!editMode"
        class="h-8 px-2 text-sm border border-edge rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent ml-1 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        <option value="">{{ t('editor.fontOptions.defaultSize') }}</option>
        <option value="12px">12px</option>
        <option value="14px">14px</option>
        <option value="16px">16px</option>
        <option value="18px">18px</option>
        <option value="20px">20px</option>
        <option value="24px">24px</option>
        <option value="28px">28px</option>
        <option value="32px">32px</option>
        <option value="36px">36px</option>
        <option value="48px">48px</option>
        <option value="72px">72px</option>
      </select>
    </Tooltip>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { Tooltip } from '../ui'

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
</script>
