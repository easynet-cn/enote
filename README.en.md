# eNote

A cross-platform desktop note-taking application built with Tauri, supporting rich text editing and Markdown mode.

## Features

- **Rich Text Editing** - WYSIWYG editor powered by TipTap with various formatting options
- **Markdown Mode** - Switch to Markdown plain text editing mode
- **Notebook Management** - Organize notes by notebooks
- **Tag System** - Flexible tag management with multi-tag filtering
- **Version History** - Automatic note history saving with rollback capability
- **Full-text Search** - Quick search through note content
- **Cross-platform** - Supports Windows, macOS, and Linux

## Tech Stack

### Frontend
- **Vue 3** - Using Composition API with `<script setup>` syntax
- **TypeScript** - Type-safe development experience
- **Vite** - Fast development build tool
- **Tailwind CSS v4** - Utility-first CSS framework
- **TipTap** - Extensible rich text editor
- **Lucide Icons** - Beautiful icon library

### Backend
- **Rust** - High-performance systems programming language
- **Tauri 2.x** - Lightweight cross-platform desktop application framework
- **SeaORM** - Async ORM framework
- **SQLite/MySQL/PostgreSQL** - Multiple database support

## Project Structure

```
enote/
├── src/                    # Frontend source code
│   ├── api/               # Tauri IPC call wrappers
│   ├── components/        # Vue components
│   │   ├── ui/           # Common UI component library
│   │   ├── Editor.vue    # Main editor component
│   │   ├── Sidebar.vue   # Sidebar (notebooks/tags)
│   │   ├── NoteList.vue  # Note list
│   │   └── TiptapToolbar.vue  # Editor toolbar
│   ├── composables/       # Composable functions
│   ├── types/             # TypeScript type definitions
│   └── App.vue            # Application root component
├── src-tauri/             # Tauri backend source code
│   └── src/
│       ├── command.rs     # IPC command handlers
│       ├── service/       # Business logic layer
│       ├── entity/        # Database entities
│       └── config.rs      # Configuration management
└── doc/                   # Documentation and config examples
```

## UI Component Library

The project includes a custom UI component library:

- **Button** - Button component with multiple types and states
- **Input** - Input field component
- **Select** - Dropdown select component
- **Dialog** - Dialog/modal component
- **Dropdown** - Dropdown menu component
- **Pagination** - Pagination component
- **Tooltip** - Tooltip component
- **ColorPicker** - Color picker
- **IconPicker** - Icon picker
- **StylePicker** - Style picker
- **ConfirmDialog** - Confirmation dialog
- **Notification** - Toast notifications

## Development Guide

### Requirements

- Node.js 18+
- pnpm 8+
- Rust 1.70+
- Tauri CLI

### Install Dependencies

```bash
# Install frontend dependencies
pnpm install

# Install Tauri CLI (if not installed)
cargo install tauri-cli
```

### Development Mode

```bash
# Start development server (frontend + Tauri)
pnpm tauri dev

# Start frontend development server only (port 1420)
pnpm dev
```

### Build for Production

```bash
# Build production version
pnpm build

# Generate app icons
pnpm tauri:icon
```

### Code Formatting

```bash
pnpm format
```

## Database Configuration

Supports SQLite (default), MySQL, and PostgreSQL. Configuration file is located at `doc/application.yml`.

### SQLite (Default)

```yaml
database:
  type: sqlite
  path: ./data/enote.db
```

### MySQL

```yaml
database:
  type: mysql
  host: localhost
  port: 3306
  database: enote
  username: root
  password: your_password
```

### PostgreSQL

```yaml
database:
  type: postgres
  host: localhost
  port: 5432
  database: enote
  username: postgres
  password: your_password
```

## Contributing

1. Fork this repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -m 'Add some feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Create a Pull Request

## License

MIT License

## Related Links

- [Tauri Documentation](https://tauri.app)
- [Vue 3 Documentation](https://vuejs.org)
- [TipTap Documentation](https://tiptap.dev)
- [Tailwind CSS Documentation](https://tailwindcss.com)
