<template>
  <Dialog v-model="visible" :title="t('editor.linkDialog.title')" :width="400">
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-1">{{
          t('editor.linkDialog.urlLabel')
        }}</label>
        <input
          v-model="linkUrl"
          type="text"
          :placeholder="t('editor.linkDialog.url')"
          class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
        />
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end gap-3">
        <Button type="secondary" @click="visible = false">{{ t('common.cancel') }}</Button>
        <Button type="primary" @click="setLink">{{ t('common.confirm') }}</Button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'
import { Dialog, Button } from '../ui'

const { t } = useI18n()

const props = defineProps<{
  editor: Editor | null
}>()

const visible = defineModel<boolean>({ default: false })
const linkUrl = ref('')

// 打开时读取当前链接
watch(visible, (val) => {
  if (val && props.editor) {
    const previousUrl = props.editor.getAttributes('link').href
    linkUrl.value = previousUrl || ''
  }
})

const setLink = () => {
  if (!props.editor) return
  if (linkUrl.value) {
    props.editor.chain().focus().extendMarkRange('link').setLink({ href: linkUrl.value }).run()
  }
  visible.value = false
  linkUrl.value = ''
}
</script>
