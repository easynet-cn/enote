<template>
  <Dialog
    :model-value="visible ?? false"
    @update:model-value="visible = $event"
    title="历史记录"
    :fullscreen="true"
    @open="$emit('open')"
  >
    <div class="h-[88vh] overflow-hidden flex flex-col">
      <div class="flex-1 overflow-auto">
        <table class="w-full border-collapse">
          <thead class="bg-gray-50 sticky top-0">
            <tr>
              <th class="px-4 py-3 text-left text-sm font-medium text-gray-700 border-b">ID</th>
              <th class="px-4 py-3 text-left text-sm font-medium text-gray-700 border-b">
                笔记本名称
              </th>
              <th class="px-4 py-3 text-left text-sm font-medium text-gray-700 border-b">标题</th>
              <th class="px-4 py-3 text-left text-sm font-medium text-gray-700 border-b">标签</th>
              <th class="px-4 py-3 text-left text-sm font-medium text-gray-700 border-b">
                操作类型
              </th>
              <th class="px-4 py-3 text-left text-sm font-medium text-gray-700 border-b">
                操作时间
              </th>
              <th class="px-4 py-3 text-left text-sm font-medium text-gray-700 border-b">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="!showData?.length">
              <td colspan="7" class="px-4 py-8 text-center text-gray-500">没有数据</td>
            </tr>
            <tr v-for="row in showData" :key="row.id" class="hover:bg-gray-50 transition-colors">
              <td class="px-4 py-3 text-sm text-gray-900 border-b">{{ row.id }}</td>
              <td class="px-4 py-3 text-sm text-gray-900 border-b">{{ row.notebookName }}</td>
              <td class="px-4 py-3 text-sm text-gray-900 border-b">{{ row.title }}</td>
              <td class="px-4 py-3 text-sm text-gray-900 border-b">{{ row.tags }}</td>
              <td class="px-4 py-3 text-sm text-gray-900 border-b">{{ row.operateType }}</td>
              <td class="px-4 py-3 text-sm text-gray-900 border-b">{{ row.operateTime }}</td>
              <td class="px-4 py-3 text-sm border-b">
                <button
                  @click="handleView(row)"
                  class="px-3 py-1 text-sm text-white bg-green-500 rounded hover:bg-green-600 transition-colors"
                >
                  查看
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end">
        <Pagination
          :current-page="currentPage!"
          :page-size="pageSize!"
          :total="total!"
          :page-sizes="[20, 50, 100, 200]"
          :show-total="true"
          :show-sizes="true"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
        />
      </div>
    </template>
  </Dialog>

  <!-- 内容查看对话框 -->
  <Dialog v-model="viewVisible" title="内容查看" :width="1200">
    <div class="h-[70vh] overflow-hidden flex">
      <!-- 旧内容区域 -->
      <div class="flex-1 border-r border-gray-200 pr-4">
        <div class="text-lg font-semibold mb-4">旧内容</div>
        <div class="h-full overflow-auto bg-gray-50 p-4 rounded border">
          <TipTapEditor
            :model-value="viewOldContent"
            :editable="false"
            :show-toolbar="false"
            class="h-full"
          />
        </div>
      </div>

      <!-- 新内容区域 -->
      <div class="flex-1 pl-4">
        <div class="text-lg font-semibold mb-4">新内容</div>
        <div class="h-full overflow-auto bg-green-50 p-4 rounded border">
          <TipTapEditor
            :model-value="viewNewContent"
            :editable="false"
            :show-toolbar="false"
            class="h-full"
          />
        </div>
      </div>
    </div>
  </Dialog>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { NoteHistory } from '../types'
import TipTapEditor from './TipTapEditor.vue'
import { Dialog, Pagination } from './ui'

const visible = defineModel<boolean>('visible')
const data = defineModel<NoteHistory[]>('data')
const currentPage = defineModel<number>('currentPage')
const pageSize = defineModel<number>('pageSize')
const total = defineModel<number>('total')

// 查看对话框相关状态
const viewVisible = ref(false)
const viewOldContent = ref('')
const viewNewContent = ref('')

interface ShowRow {
  id: string
  notebookId: number
  notebookName: string
  title: string
  tags: string
  oldContent: string
  newContent: string
  operateType: string
  operateTime: string
}

const showData = computed(() => {
  return data.value?.map((item): ShowRow => {
    return {
      id: item.id,
      notebookId: item.extra.notebookId,
      notebookName: item.extra.notebookName,
      title: item.extra.title,
      tags: item.extra.tags.map((t) => t.name).join(' '),
      oldContent: item.oldContent,
      newContent: item.newContent,
      operateType:
        item.operateType === 1
          ? '添加'
          : item.operateType === 2
            ? '修改'
            : item.operateType === 3
              ? '删除'
              : '',
      operateTime: item.operateTime,
    }
  })
})

const emit = defineEmits<{
  sizeChange: [pageSize: number]
  currentChange: [currentPage: number]
  open: []
}>()

const handleSizeChange = (val: number) => {
  emit('sizeChange', val)
}

const handleCurrentChange = (val: number) => {
  emit('currentChange', val)
}

const handleView = (row: ShowRow) => {
  const oldContent = row.oldContent || ''
  const newContent = row.newContent || ''

  // 直接使用原始HTML内容，TipTap编辑器会处理显示
  viewOldContent.value = oldContent
  viewNewContent.value = newContent
  viewVisible.value = true
}
</script>
