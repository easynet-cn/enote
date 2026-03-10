<template>
  <Teleport to="body">
    <Transition name="command-palette">
      <div v-if="visible" class="command-palette-overlay" @click.self="close">
        <div class="command-palette" @keydown="handleKeyDown">
          <!-- 搜索输入 -->
          <div class="command-palette-input-wrapper">
            <Search class="w-5 h-5 text-content-tertiary shrink-0" />
            <input
              ref="inputRef"
              v-model="query"
              type="text"
              :placeholder="t('commandPalette.placeholder')"
              class="command-palette-input"
              @input="handleSearch"
            />
            <kbd class="command-palette-kbd">ESC</kbd>
          </div>

          <!-- 命令列表 -->
          <div class="command-palette-list" ref="listRef">
            <template v-if="filteredCommands.length > 0">
              <div
                v-for="(group, groupIndex) in groupedCommands"
                :key="group.category"
                class="command-palette-group"
              >
                <div class="command-palette-category">{{ group.label }}</div>
                <div
                  v-for="(cmd, cmdIndex) in group.commands"
                  :key="cmd.id"
                  class="command-palette-item"
                  :class="{ active: isActive(groupIndex, cmdIndex) }"
                  :data-index="`${groupIndex}-${cmdIndex}`"
                  @click="executeCommand(cmd)"
                  @mouseenter="setActive(groupIndex, cmdIndex)"
                >
                  <div class="flex items-center gap-3 flex-1 min-w-0">
                    <component :is="cmd.icon" class="w-4 h-4 shrink-0 text-content-secondary" />
                    <span class="truncate">{{ cmd.name }}</span>
                  </div>
                  <kbd v-if="cmd.shortcut" class="command-palette-shortcut">{{ cmd.shortcut }}</kbd>
                </div>
              </div>
            </template>
            <div v-else class="command-palette-empty">
              {{ t('commandPalette.noResults') }}
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, type Component } from 'vue'
import { useI18n } from 'vue-i18n'
import { Search } from 'lucide-vue-next'

const { t } = useI18n()

export interface PaletteCommand {
  id: string
  name: string
  icon: Component
  category: string
  shortcut?: string
  handler: () => void
}

const visible = defineModel<boolean>({ default: false })

const props = defineProps<{
  commands: PaletteCommand[]
}>()

const query = ref('')
const activeGroup = ref(0)
const activeIndex = ref(0)
const inputRef = ref<HTMLInputElement>()
const listRef = ref<HTMLElement>()

// 过滤命令
const filteredCommands = computed(() => {
  if (!query.value.trim()) return props.commands
  const q = query.value.toLowerCase()
  return props.commands.filter(
    (cmd) => cmd.name.toLowerCase().includes(q) || cmd.id.toLowerCase().includes(q),
  )
})

// 分组命令
const groupedCommands = computed(() => {
  const groups = new Map<string, { label: string; commands: PaletteCommand[] }>()
  for (const cmd of filteredCommands.value) {
    if (!groups.has(cmd.category)) {
      groups.set(cmd.category, { label: cmd.category, commands: [] })
    }
    groups.get(cmd.category)!.commands.push(cmd)
  }
  return Array.from(groups.values())
})

const isActive = (groupIndex: number, cmdIndex: number) => {
  return activeGroup.value === groupIndex && activeIndex.value === cmdIndex
}

const setActive = (groupIndex: number, cmdIndex: number) => {
  activeGroup.value = groupIndex
  activeIndex.value = cmdIndex
}

const close = () => {
  visible.value = false
  query.value = ''
  activeGroup.value = 0
  activeIndex.value = 0
}

const executeCommand = (cmd: PaletteCommand) => {
  close()
  cmd.handler()
}

const handleSearch = () => {
  activeGroup.value = 0
  activeIndex.value = 0
}

const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    e.preventDefault()
    close()
    return
  }

  if (e.key === 'Enter') {
    e.preventDefault()
    const group = groupedCommands.value[activeGroup.value]
    if (group) {
      const cmd = group.commands[activeIndex.value]
      if (cmd) executeCommand(cmd)
    }
    return
  }

  if (e.key === 'ArrowDown') {
    e.preventDefault()
    moveSelection(1)
    return
  }

  if (e.key === 'ArrowUp') {
    e.preventDefault()
    moveSelection(-1)
    return
  }
}

const moveSelection = (direction: number) => {
  const groups = groupedCommands.value
  if (groups.length === 0) return

  let g = activeGroup.value
  let i = activeIndex.value + direction

  // 跨组导航
  while (true) {
    if (i >= 0 && i < (groups[g]?.commands.length ?? 0)) {
      break
    }
    if (direction > 0) {
      g++
      i = 0
      if (g >= groups.length) {
        g = 0
        i = 0
        break
      }
    } else {
      g--
      if (g < 0) {
        g = groups.length - 1
        i = (groups[g]?.commands.length ?? 1) - 1
        break
      }
      i = (groups[g]?.commands.length ?? 1) - 1
    }
  }

  activeGroup.value = g
  activeIndex.value = i

  // 滚动到可见
  nextTick(() => {
    const activeEl = listRef.value?.querySelector(`[data-index="${g}-${i}"]`)
    activeEl?.scrollIntoView({ block: 'nearest' })
  })
}

// 打开时聚焦输入框
watch(visible, (val) => {
  if (val) {
    nextTick(() => inputRef.value?.focus())
  }
})
</script>

<style scoped>
.command-palette-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  justify-content: center;
  padding-top: 15vh;
}

.command-palette {
  width: 560px;
  max-height: 420px;
  background: var(--color-bg-primary);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-2xl);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 1px solid var(--color-border);
}

.command-palette-input-wrapper {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.875rem 1rem;
  border-bottom: 1px solid var(--color-border);
}

.command-palette-input {
  flex: 1;
  border: none;
  outline: none;
  font-size: 1rem;
  background: transparent;
  color: var(--color-text-primary);
}

.command-palette-input::placeholder {
  color: var(--color-text-tertiary);
}

.command-palette-kbd {
  font-size: 0.625rem;
  padding: 0.125rem 0.375rem;
  border-radius: var(--radius-sm);
  border: 1px solid var(--color-border);
  color: var(--color-text-tertiary);
  font-family: var(--font-mono);
}

.command-palette-list {
  flex: 1;
  overflow-y: auto;
  padding: 0.5rem;
}

.command-palette-group + .command-palette-group {
  margin-top: 0.5rem;
}

.command-palette-category {
  font-size: 0.6875rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--color-text-tertiary);
  padding: 0.375rem 0.5rem;
}

.command-palette-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.5rem 0.75rem;
  border-radius: var(--radius-md);
  cursor: pointer;
  color: var(--color-text-primary);
  font-size: 0.875rem;
  transition: background var(--transition-fast) var(--ease-default);
}

.command-palette-item:hover,
.command-palette-item.active {
  background: var(--color-bg-tertiary);
}

.command-palette-shortcut {
  font-size: 0.6875rem;
  padding: 0.125rem 0.375rem;
  border-radius: var(--radius-sm);
  border: 1px solid var(--color-border);
  color: var(--color-text-tertiary);
  font-family: var(--font-mono);
}

.command-palette-empty {
  padding: 2rem;
  text-align: center;
  color: var(--color-text-tertiary);
  font-size: 0.875rem;
}

/* 过渡动画 */
.command-palette-enter-active,
.command-palette-leave-active {
  transition: opacity 0.15s ease;
}

.command-palette-enter-active .command-palette,
.command-palette-leave-active .command-palette {
  transition:
    transform 0.15s ease,
    opacity 0.15s ease;
}

.command-palette-enter-from,
.command-palette-leave-to {
  opacity: 0;
}

.command-palette-enter-from .command-palette {
  transform: scale(0.95);
  opacity: 0;
}

.command-palette-leave-to .command-palette {
  transform: scale(0.95);
  opacity: 0;
}
</style>
