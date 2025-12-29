<template>
  <!-- 简易模式 -->
  <div v-if="simple" class="flex items-center justify-center gap-3">
    <button
      :disabled="currentPage <= 1"
      @click="handlePrev"
      class="w-8 h-8 flex items-center justify-center rounded-lg border border-slate-300 hover:bg-slate-50 disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
    >
      <ChevronLeft class="w-4 h-4" />
    </button>

    <span class="text-sm text-slate-600">{{ currentPage }} / {{ totalPages }}</span>

    <button
      :disabled="currentPage >= totalPages"
      @click="handleNext"
      class="w-8 h-8 flex items-center justify-center rounded-lg border border-slate-300 hover:bg-slate-50 disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
    >
      <ChevronRight class="w-4 h-4" />
    </button>
  </div>

  <!-- 完整模式 -->
  <div v-else class="flex items-center gap-2">
    <!-- Total -->
    <span v-if="showTotal" class="text-sm text-slate-600">共 {{ total }} 条</span>

    <!-- Page size selector -->
    <select
      v-if="showSizes"
      :value="pageSize"
      @change="handleSizeChange"
      class="h-8 px-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
    >
      <option v-for="size in pageSizes" :key="size" :value="size">{{ size }} 条/页</option>
    </select>

    <!-- Prev button -->
    <button
      :disabled="currentPage <= 1"
      @click="handlePrev"
      class="h-8 px-3 text-sm border border-slate-300 rounded-lg hover:bg-slate-50 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
    >
      上一页
    </button>

    <!-- Page numbers -->
    <div class="flex items-center gap-1">
      <button
        v-for="page in displayedPages"
        :key="page"
        @click="page !== '...' && handlePageClick(page as number)"
        :class="[
          'h-8 min-w-[32px] px-2 text-sm rounded-lg transition-colors',
          page === currentPage
            ? 'bg-indigo-600 text-white border border-indigo-600'
            : page === '...'
              ? 'cursor-default'
              : 'border border-slate-300 hover:bg-slate-50',
        ]"
        :disabled="page === '...'"
      >
        {{ page }}
      </button>
    </div>

    <!-- Next button -->
    <button
      :disabled="currentPage >= totalPages"
      @click="handleNext"
      class="h-8 px-3 text-sm border border-slate-300 rounded-lg hover:bg-slate-50 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
    >
      下一页
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { ChevronLeft, ChevronRight } from 'lucide-vue-next'

interface Props {
  currentPage: number
  pageSize: number
  total: number
  pageSizes?: number[]
  showTotal?: boolean
  showSizes?: boolean
  simple?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  pageSizes: () => [10, 20, 50, 100],
  showTotal: false,
  showSizes: false,
  simple: false,
})

const emit = defineEmits<{
  'update:currentPage': [value: number]
  'update:pageSize': [value: number]
  sizeChange: [size: number]
  currentChange: [page: number]
}>()

const totalPages = computed(() => Math.max(1, Math.ceil(props.total / props.pageSize)))

const displayedPages = computed(() => {
  const pages: (number | string)[] = []
  const total = totalPages.value
  const current = props.currentPage

  if (total <= 7) {
    for (let i = 1; i <= total; i++) {
      pages.push(i)
    }
  } else {
    if (current <= 4) {
      for (let i = 1; i <= 5; i++) {
        pages.push(i)
      }
      pages.push('...')
      pages.push(total)
    } else if (current >= total - 3) {
      pages.push(1)
      pages.push('...')
      for (let i = total - 4; i <= total; i++) {
        pages.push(i)
      }
    } else {
      pages.push(1)
      pages.push('...')
      for (let i = current - 1; i <= current + 1; i++) {
        pages.push(i)
      }
      pages.push('...')
      pages.push(total)
    }
  }

  return pages
})

const handlePrev = () => {
  if (props.currentPage > 1) {
    const newPage = props.currentPage - 1
    emit('update:currentPage', newPage)
    emit('currentChange', newPage)
  }
}

const handleNext = () => {
  if (props.currentPage < totalPages.value) {
    const newPage = props.currentPage + 1
    emit('update:currentPage', newPage)
    emit('currentChange', newPage)
  }
}

const handlePageClick = (page: number) => {
  emit('update:currentPage', page)
  emit('currentChange', page)
}

const handleSizeChange = (event: Event) => {
  const size = Number((event.target as HTMLSelectElement).value)
  emit('update:pageSize', size)
  emit('sizeChange', size)
}
</script>
