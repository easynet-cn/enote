<template>
  <Dialog v-model="visible" :title="t('editor.toolbar.image')" :width="400">
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-1">{{
          t('editor.linkDialog.imageUrlLabel')
        }}</label>
        <input
          v-model="imageUrl"
          type="text"
          :placeholder="t('editor.linkDialog.imageUrl')"
          class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
        />
      </div>
      <div class="text-center text-slate-500 text-sm">{{ t('editor.linkDialog.or') }}</div>
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-1">{{
          t('editor.linkDialog.uploadImage')
        }}</label>
        <input
          type="file"
          accept="image/*"
          @change="handleImageUpload"
          class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
        />
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end gap-3">
        <Button type="secondary" @click="visible = false">{{ t('common.cancel') }}</Button>
        <Button type="primary" @click="insertImage">{{ t('common.confirm') }}</Button>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'
import { Dialog, Button } from '../ui'

const { t } = useI18n()

const props = defineProps<{
  editor: Editor | null
}>()

const visible = defineModel<boolean>({ default: false })
const imageUrl = ref('')

const handleImageUpload = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (file) {
    const reader = new FileReader()
    reader.onload = (e) => {
      imageUrl.value = e.target?.result as string
    }
    reader.readAsDataURL(file)
  }
}

const insertImage = () => {
  if (!props.editor) return
  if (imageUrl.value) {
    props.editor.chain().focus().setImage({ src: imageUrl.value }).run()
  }
  visible.value = false
  imageUrl.value = ''
}
</script>
