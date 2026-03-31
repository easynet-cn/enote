<template>
  <!-- 简易模式 -->
  <div v-if="simple" class="flex items-center justify-center gap-3">
    <Tooltip :content="t('pagination.prevPage')">
      <button
        :disabled="currentPage <= 1"
        @click="handlePrev"
        class="w-8 h-8 flex items-center justify-center rounded-lg border border-edge hover:bg-surface-alt disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
      >
        <ChevronLeft class="w-4 h-4" />
      </button>
    </Tooltip>

    <span class="text-sm text-content-secondary">{{ currentPage }} / {{ totalPages }}</span>

    <Tooltip :content="t('pagination.nextPage')">
      <button
        :disabled="currentPage >= totalPages"
        @click="handleNext"
        class="w-8 h-8 flex items-center justify-center rounded-lg border border-edge hover:bg-surface-alt disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
      >
        <ChevronRight class="w-4 h-4" />
      </button>
    </Tooltip>
  </div>

  <!-- 完整模式 -->
  <div v-else class="flex items-center gap-2">
    <!-- Total -->
    <span v-if="showTotal" class="text-sm text-content-secondary">{{
      t('pagination.total', { total })
    }}</span>

    <!-- Page size selector -->
    <AppSelect
      v-if="showSizes"
      :model-value="pageSize"
      :options="pageSizeOptions"
      size="sm"
      @change="handleSizeChange"
    />

    <!-- Prev button -->
    <button
      :disabled="currentPage <= 1"
      @click="handlePrev"
      class="h-8 px-3 text-sm border border-edge rounded-lg hover:bg-surface-alt disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
    >
      {{ t('pagination.prevPage') }}
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
              : 'border border-edge hover:bg-surface-alt',
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
      class="h-8 px-3 text-sm border border-edge rounded-lg hover:bg-surface-alt disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
    >
      {{ t('pagination.nextPage') }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { ChevronLeft, ChevronRight } from 'lucide-vue-next'
import Tooltip from './BaseTooltip.vue'
import AppSelect from './AppSelect.vue'
import type { AppSelectOption } from './AppSelect.vue'

const { t } = useI18n()

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

const pageSizeOptions = computed<AppSelectOption[]>(() =>
  props.pageSizes.map((size) => ({
    label: t('pagination.itemsPerPage', { size }),
    value: size,
  })),
)

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

const handleSizeChange = (value: string | number) => {
  const size = Number(value)
  emit('update:pageSize', size)
  emit('sizeChange', size)
}
</script>
