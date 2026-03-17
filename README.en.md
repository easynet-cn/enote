# eNote

A cross-platform desktop note-taking application built with Tauri, supporting rich text editing and Markdown mode.

## Features

- **Rich Text Editing** - WYSIWYG editor powered by TipTap with various formatting options
- **Markdown Mode** - Switch to Markdown editing with split-screen preview
- **Notebook Management** - Organize notes by notebooks with drag-and-drop sorting
- **Tag System** - Flexible tag management with multi-tag filtering
- **Version History** - Automatic note history saving with rollback capability
- **Full-text Search** - FTS5 full-text search with Chinese substring matching
- **Setup Wizard** - First-launch guided database configuration for SQLite/MySQL/PostgreSQL
- **Multi-Profile Management** - Multiple database profiles with startup selection or auto-connect
- **Content Encryption** - AES-256-GCM transparent note content encryption, keys stored in OS keychain
- **SSL/TLS Authentication** - MySQL/PostgreSQL support certificate-based login
- **Note Encryption** - Per-note password protection
- **Lock Screen** - Password protection (Argon2id), timeout lock, minimize lock
- **Note Templates** - Template management for quick note creation
- **Bidirectional Links** - Cross-reference notes with linked notes panel
- **Command Palette** - Ctrl+P for quick operations
- **System Tray** - Minimize to tray
- **Multi-Window** - Open notes in separate windows
- **MCP Integration** - AI tools operate notes via MCP protocol with three-layer access control
- **Data Backup** - SQL/Excel/CSV export/import with scheduled auto-backup
- **Import/Export** - Import from Evernote, Youdao Notes, Notion; Export to Word/Markdown/JSON/XML
- **Dark Mode** - Light/Dark/System theme switching
- **Multilingual** - Simplified Chinese and English
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

Supports SQLite (default), MySQL, and PostgreSQL.

On **first launch**, the Setup Wizard guides you through database connection configuration:

- **SQLite** - Local file database, ready to use out of the box
- **MySQL** - Supports password login and SSL certificate authentication
- **PostgreSQL** - Supports password login and SSL certificate authentication

Configurations are managed as Profiles, supporting multiple database setups that can be switched at any time. Database passwords and encryption keys are securely stored in the OS keychain (macOS Keychain / Windows Credential Store / Linux Secret Service), never in configuration files.

You can also use a traditional configuration file via command-line:

```bash
enote --config /path/to/application.yml
```

See `doc/application.yml`, `doc/application-mysql.yml`, `doc/application-posgres.yml` for examples.

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
