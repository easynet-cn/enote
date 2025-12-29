<template>
  <div class="p-4 space-y-4">
    <div v-for="i in count" :key="i" class="note-skeleton">
      <Skeleton variant="text" width="60%" height="18px" />
      <Skeleton variant="text" width="100%" height="14px" class="mt-2" />
      <Skeleton variant="text" width="80%" height="14px" class="mt-1" />
      <div class="flex justify-between mt-3">
        <Skeleton variant="text" width="30%" height="12px" />
        <Skeleton variant="text" width="25%" height="12px" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import Skeleton from './ui/BaseSkeleton.vue'

interface Props {
  count?: number
}

withDefaults(defineProps<Props>(), {
  count: 5,
})
</script>

<style scoped>
.note-skeleton {
  padding: 1rem;
  border-radius: 0.5rem;
  background: white;
  border: 1px solid #f1f5f9;
}
</style>
