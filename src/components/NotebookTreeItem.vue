<script setup lang="ts">
import { ChevronRight, ChevronDown, Shield } from 'lucide-vue-next'
import { iconComponents } from './ui/icons'
import { useI18n } from 'vue-i18n'
import type { NotebookTreeNode } from '../types'

const { t } = useI18n()

defineProps<{
  node: NotebookTreeNode
  activeNotebook: string
  depth: number
}>()

const emit = defineEmits<{
  select: [id: string]
  toggle: [id: string]
}>()
</script>

<template>
  <div>
    <li
      :data-id="node.id"
      role="option"
      :aria-selected="node.id === activeNotebook"
      class="sidebar-item"
      :class="{ active: node.id === activeNotebook }"
      :style="{ paddingLeft: `${8 + depth * 16}px` }"
      @click="emit('select', node.id)"
      @keydown.enter="emit('select', node.id)"
      tabindex="0"
    >
      <div class="flex items-center">
        <!-- Expand/collapse toggle -->
        <button
          v-if="node.children.length > 0"
          class="flex items-center justify-center w-4 h-4 mr-1 rounded hover:bg-surface-dim transition-colors flex-shrink-0"
          @click.stop="emit('toggle', node.id)"
          :aria-label="node.expanded ? t('sidebar.collapse') : t('sidebar.expand')"
        >
          <ChevronDown
            v-if="node.expanded"
            class="w-3 h-3 text-content-tertiary"
            aria-hidden="true"
          />
          <ChevronRight v-else class="w-3 h-3 text-content-tertiary" aria-hidden="true" />
        </button>
        <span v-else class="w-4 mr-1 flex-shrink-0" />

        <!-- Icon -->
        <component
          v-if="node.icon && iconComponents[node.icon]"
          :is="iconComponents[node.icon]"
          class="w-4 h-4 mr-3 text-content-secondary flex-shrink-0"
          aria-hidden="true"
        />
        <span v-else-if="node.cls" :class="['mr-3 flex-shrink-0', node.cls]" aria-hidden="true"
          >●</span
        >

        <!-- Name -->
        <span class="flex-1 truncate">{{ node.name }}</span>

        <!-- MCP badge -->
        <Shield
          v-if="node.mcpAccess && node.mcpAccess > 0"
          class="w-3 h-3 mr-1 text-content-tertiary flex-shrink-0"
        />

        <!-- Count -->
        <span class="text-xs text-content-tertiary">{{ node.count }}</span>
      </div>
    </li>

    <!-- Children (recursive) -->
    <template v-if="node.expanded && node.children.length > 0">
      <NotebookTreeItem
        v-for="child in node.children"
        :key="child.id"
        :node="child"
        :active-notebook="activeNotebook"
        :depth="depth + 1"
        @select="emit('select', $event)"
        @toggle="emit('toggle', $event)"
      />
    </template>
  </div>
</template>
