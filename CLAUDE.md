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

## Database

Supports SQLite (default), MySQL, and PostgreSQL via SeaORM. Configuration in `doc/application.yml`.

## Testing

Vitest is configured (`vitest.config.ts`) but no tests exist yet. Setup file referenced at `src/test/setup.ts` needs creation.
