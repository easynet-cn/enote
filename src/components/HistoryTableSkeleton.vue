<template>
  <tbody>
    <tr v-for="i in count" :key="i" class="animate-pulse">
      <td class="px-4 py-3 border-b">
        <Skeleton variant="text" width="40px" height="14px" />
      </td>
      <td class="px-4 py-3 border-b">
        <Skeleton variant="text" width="80px" height="14px" />
      </td>
      <td class="px-4 py-3 border-b">
        <Skeleton variant="text" width="120px" height="14px" />
      </td>
      <td class="px-4 py-3 border-b">
        <Skeleton variant="text" width="60px" height="14px" />
      </td>
      <td class="px-4 py-3 border-b">
        <Skeleton variant="text" width="100px" height="14px" />
      </td>
      <td class="px-4 py-3 border-b">
        <Skeleton variant="text" width="50px" height="14px" />
      </td>
      <td class="px-4 py-3 border-b">
        <Skeleton variant="text" width="140px" height="14px" />
      </td>
      <td class="px-4 py-3 border-b">
        <Skeleton variant="rounded" width="48px" height="28px" />
      </td>
    </tr>
  </tbody>
</template>

<script setup lang="ts">
import Skeleton from './ui/Skeleton.vue'

interface Props {
  count?: number
}

withDefaults(defineProps<Props>(), {
  count: 5,
})
</script>
