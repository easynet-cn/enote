# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

enote is a cross-platform desktop note-taking application built with:
- **Frontend**: Vue 3 + TypeScript + Vite
- **Backend**: Rust + Tauri 2.x + SeaORM
- **UI**: Tailwind CSS v4 + TipTap rich text editor + Lucide icons

## Development Commands

```bash
# Start development server (frontend + Tauri dev mode)
pnpm tauri dev

# Frontend only dev server (port 1420)
pnpm dev

# Build for production
pnpm build

# Format code
pnpm format

# Generate app icons from SVG
pnpm tauri:icon
```

## Architecture

### Frontend (src/)

The app uses a **single composable pattern** where `src/composables/useNotes.ts` centralizes all state management and API calls. No external state management library (Pinia/Vuex) is used.

**Main components:**
- `App.vue` - 3-column layout (Sidebar | NoteList | Editor)
- `Sidebar.vue` - Notebooks and tags tree navigation
- `NoteList.vue` - Paginated note list with search
- `Editor.vue` - TipTap rich text editor with history dialog

**API layer** (`src/api/note.ts`): Wraps Tauri `invoke()` calls for IPC to Rust backend.

**UI components** (`src/components/ui/`): Custom Tailwind-based components (Dialog, Dropdown, Pagination, Tooltip, ColorPicker, notification).

### Backend (src-tauri/)

```
src-tauri/src/
├── lib.rs          # Tauri app builder & plugin setup
├── command.rs      # Tauri command handlers (IPC endpoints)
├── model.rs        # Data transfer objects with serde
├── config.rs       # Database connection & YAML config loading
├── entity/         # SeaORM entities (notebook, note, tag, note_history)
└── service/        # Business logic layer
```

### IPC Commands

Frontend invokes these Tauri commands:
- Notebooks: `find_all_notebooks`, `create_notebook`, `update_notebook`, `delete_notebook_by_id`
- Tags: `find_all_tags`, `create_tag`, `update_tag`, `delete_tag_by_id`
- Notes: `create_note`, `update_note`, `delete_note_by_id`, `search_page_notes`
- History: `search_page_note_histories`

## Code Conventions

### Frontend
- Vue 3 `<script setup>` syntax with TypeScript
- Prettier: no semicolons, single quotes, 100 char width
- All API errors shown via custom notification (`src/components/ui/notification.ts`)
- IDs converted to strings for Vue reactivity

### Backend (Rust)
- Edition 2024
- `anyhow` for error handling
- `serde` with camelCase field renaming for JSON
- Async/await with tokio runtime

### Special Patterns
- First notebook/tag in list is "全部" (All) with id='0' (virtual item)
- New notes get temporary IDs (`'0-' + timestamp`) before saving
- Note content stored as HTML (TipTap output)

## Style Guide

### Design System (Material Design 风格)

**主题色 (Primary - Indigo)**
- `--color-primary`: #4f46e5 (主色)
- `--color-primary-hover`: #4338ca (悬浮)
- `--color-primary-active`: #3730a3 (激活)
- `--color-primary-light`: #e0e7ff (浅色背景)
- `--color-primary-lighter`: #eef2ff (更浅背景)

**语义色**
- Success: #10b981 (绿色)
- Warning: #f59e0b (橙色)
- Danger: #ef4444 (红色)
- Info: #3b82f6 (蓝色)

**中性色 (Slate 灰色调)**
- 文本主色: #0f172a
- 文本次色: #64748b
- 边框色: #e2e8f0
- 背景主色: #ffffff
- 背景次色: #f8fafc

### CSS 变量使用

优先使用 CSS 变量而非硬编码颜色值：
```css
/* 正确 */
color: var(--color-primary);
background: var(--color-bg-secondary);
box-shadow: var(--shadow-md);
border-radius: var(--radius-lg);
transition: all var(--transition-normal) var(--ease-default);

/* 避免 */
color: #4f46e5;
background: #f8fafc;
```

**阴影 (Elevation)**
- `--shadow-xs` ~ `--shadow-2xl`: 6 级阴影
- `--shadow-primary`: 带主题色的悬浮阴影

**圆角**
- `--radius-sm`: 4px
- `--radius-md`: 6px
- `--radius-lg`: 8px
- `--radius-xl`: 12px
- `--radius-full`: 9999px (圆形)

**过渡**
- `--transition-fast`: 0.15s
- `--transition-normal`: 0.2s
- `--transition-slow`: 0.3s

### Tailwind CSS 规范

- 使用 Tailwind CSS v4
- 优先使用 Tailwind 工具类，复杂样式使用 scoped CSS
- 颜色使用 Tailwind 内置调色板 (slate, indigo, etc.)
- 常用类名模式:
  ```html
  <!-- 按钮 -->
  <button class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors">

  <!-- 输入框 -->
  <input class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent">

  <!-- 卡片 -->
  <div class="p-4 bg-white rounded-xl shadow-sm border border-slate-200">
  ```

### Vue 组件样式规范

- 使用 `<style scoped>` 避免样式污染
- 深度选择器使用 `:deep(.class-name)`
- 组件内样式按功能分组，添加注释分隔
- 复杂状态样式使用 computed 计算类名：
  ```typescript
  const buttonClasses = computed(() => {
    const classes = ['btn', `btn-${props.type}`, `btn-${props.size}`]
    if (props.disabled) classes.push('btn-disabled')
    return classes
  })
  ```

### 响应式设计

- 断点: 768px (移动端)
- 使用 Tailwind 响应式前缀: `sm:`, `md:`, `lg:`, `xl:`
- 工具栏等组件需处理窄屏滚动

### 图标规范

- 使用 Lucide Vue Next 图标库
- 默认尺寸: `class="w-4 h-4"` 或 `class="w-5 h-5"`
- 导入方式:
  ```typescript
  import { IconName } from 'lucide-vue-next'
  ```

### TipTap 编辑器扩展

自定义扩展存放在 `src/extensions/` 目录：
- `FontSize.ts` - 字体大小
- `Indent.ts` - 缩进控制
- `SearchAndReplace.ts` - 查找替换

扩展配置在 `src/config/editorExtensions.ts` 中集中管理

## Database

Supports SQLite (default), MySQL, and PostgreSQL via SeaORM. Configuration in `doc/application.yml`.

## Testing

Vitest is configured (`vitest.config.ts`) but no tests exist yet. Setup file referenced at `src/test/setup.ts` needs creation.
