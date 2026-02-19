<template>
  <Dropdown :teleported="false" placement="bottom-end">
    <template #trigger>
      <Button type="tertiary" :icon="LanguagesIcon" class="locale-switcher">
        {{ currentLocaleLabel }}
      </Button>
    </template>
    <DropdownItem
      v-for="locale in availableLocales"
      :key="locale.value"
      :command="locale.value"
      :danger="false"
      @command="handleLocaleChange"
    >
      <span>{{ locale.label }}</span>
      <Check v-if="currentLocale === locale.value" class="w-4 h-4 ml-auto" />
    </DropdownItem>
  </Dropdown>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { Languages, Check } from 'lucide-vue-next'
import { Button, Dropdown, DropdownItem } from './ui'
import { availableLocales, setLocale, type LocaleType } from '../i18n'
import { useAppStore } from '../stores/app'

const { locale } = useI18n()
const appStore = useAppStore()
const LanguagesIcon = Languages

const currentLocale = computed(() => locale.value as LocaleType)

const currentLocaleLabel = computed(() => {
  const locale = availableLocales.find((l) => l.value === currentLocale.value)
  return locale?.label || 'Language'
})

const handleLocaleChange = (newLocale: string) => {
  setLocale(newLocale as LocaleType)
  // 更新 store 中的默认项语言
  appStore.updateDefaultItems()
}
</script>

<style scoped>
.locale-switcher {
  min-width: 100px;
}
</style>
