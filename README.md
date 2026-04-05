# ENote

English | [中文](README.zh-CN.md)

A cross-platform desktop note-taking application built with Tauri, supporting rich text editing and Markdown mode.

## Features

### Editor
- **Rich Text Editing** - WYSIWYG editor powered by TipTap with various formatting options
- **Markdown Mode** - Switch to Markdown editing with split-screen preview
- **Find & Replace** - Global find and replace within the editor
- **Math Formulas** - KaTeX math formula rendering
- **Code Highlighting** - Syntax highlighting for code blocks via lowlight
- **Code Block Language Selector** - Quick language selection dropdown for syntax highlighting (25+ languages)
- **Table of Contents** - Auto-generated document outline
- **Drag & Drop** - Drag handles for reordering content blocks
- **Smart Paste** - Context-aware paste formatting
- **Lazy Image Loading** - Deferred image loading for performance
- **Print Support** - Print notes directly via system print dialog
- **Editor Font Size** - Configurable editor font size (12-20px) in settings

### Note Management
- **Notebook Management** - Organize notes by notebooks with drag-and-drop sorting
- **Tag System** - Flexible tag management with multi-tag filtering
- **Note Pinning** - Pin important notes to top
- **Version History** - Automatic note history with source tracking (user/sync/import)
- **Full-text Search** - FTS5 full-text search with Chinese substring matching
- **Note Templates** - Template management for quick note creation
- **Bidirectional Links** - Cross-reference notes with linked notes panel
- **Recycle Bin** - Soft delete with recovery support
- **Command Palette** - Ctrl+P for quick operations
- **Custom Shortcuts** - Configurable keyboard shortcuts
- **Note Sorting** - Sort notes by title, creation time, or update time (ascending/descending)
- **Batch Operations** - Multi-select notes for batch move, delete operations
- **Note Starring** - Star/favorite notes for quick identification
- **File Attachments** - Attach files (PDF, documents, etc.) to notes with local storage
- **Notebook Hierarchy** - Nested notebook tree with expand/collapse
- **Profile Editing** - Edit existing database profile configurations

### Security & Encryption
- **Content Encryption** - AES-256-GCM transparent note content encryption, keys stored in OS keychain
- **Note Encryption** - Per-note password protection
- **Lock Screen** - Password protection (Argon2id), timeout lock, minimize lock
- **Keychain Integration** - Passwords and keys stored in OS keychain (macOS Keychain / Windows Credential Store / Linux Secret Service)

### Data Management
- **Multi-Profile Management** - Multiple database profiles with startup selection or auto-connect
- **Setup Wizard** - First-launch guided configuration for SQLite/MySQL/PostgreSQL/ENote Server
- **ENote Server Backend** - Connect to remote ENote-compatible API servers with multiple auth methods (Bearer/Basic/JWT/Custom Header/OAuth 2.0)
- **SSL/TLS Authentication** - MySQL/PostgreSQL support certificate-based login
- **Data Backup** - SQL/Excel/CSV export/import with scheduled auto-backup
- **Import/Export** - Import from Evernote, Youdao Notes, Notion; Export to PDF/HTML/Word/Markdown/JSON/XML
- **PDF Export** - Export notes as PDF via print dialog
- **HTML Export** - Export as standalone HTML files with embedded styles and dark mode support
- **Data Sync** - Synchronization infrastructure with sync logs and detailed tracking
- **Local Image Storage** - Images saved locally, served via Tauri asset protocol

### AI Integration
- **MCP Integration** - AI tools operate notes via MCP protocol with three-layer access control (note/tag/notebook level)

### Interface & Experience
- **Dark Mode** - Light/Dark/System theme switching
- **Multilingual** - Simplified Chinese and English
- **System Tray** - Minimize to tray
- **Multi-Window** - Open notes in separate windows
- **Auto Update** - Automatic update check on startup with one-click install via GitHub Releases
- **Help System** - Built-in searchable help documentation
- **Application Logs** - Audit logging with sensitive data sanitization
- **Cross-platform** - Supports Windows, macOS, and Linux

## Tech Stack

### Frontend
- **Vue 3** - Using Composition API with `<script setup>` syntax
- **TypeScript** - Type-safe development experience
- **Vite** - Fast development build tool
- **Tailwind CSS v4** - Utility-first CSS framework
- **TipTap** - Extensible rich text editor (8 custom extensions)
- **Pinia** - State management
- **Vue I18n** - Internationalization
- **Lucide Icons** - Icon library

### Backend
- **Rust** (Edition 2024) - High-performance systems programming language
- **Tauri 2.x** - Lightweight cross-platform desktop application framework
- **SeaORM** - Async ORM framework
- **SQLite/MySQL/PostgreSQL** - Multiple database support
- **reqwest** - HTTP client for ENote Server backend
- **aes-gcm** - AES-256-GCM encryption
- **argon2** - Password hashing
- **keyring** - OS keychain integration
- **rmcp** - MCP protocol server

## Project Structure

```
enote/
├── src/                        # Frontend source code
│   ├── api/                    # Tauri IPC call wrappers
│   ├── components/             # Vue components (70+)
│   │   ├── toolbar/            # Editor toolbar components (17)
│   │   ├── ui/                 # Common UI component library (16)
│   │   ├── NoteEditor.vue      # Editor container
│   │   ├── TipTapEditor.vue    # TipTap rich text editor
│   │   ├── MarkdownSplitEditor.vue  # Markdown split-screen editor
│   │   ├── AppSidebar.vue      # Sidebar (notebooks/tags)
│   │   ├── NoteList.vue        # Note list
│   │   ├── LockScreen.vue      # Lock screen
│   │   ├── SetupWizard.vue     # Setup wizard
│   │   ├── ProfileSelector.vue # Profile selector
│   │   ├── CommandPalette.vue  # Command palette
│   │   ├── HelpManual.vue      # Help documentation
│   │   ├── LogDialog.vue       # Application logs
│   │   └── UpdateChecker.vue   # Auto update checker
│   ├── composables/            # Composable functions (12)
│   ├── extensions/             # Custom TipTap extensions (8)
│   ├── utils/                  # Utility functions
│   │   └── import/             # Import parsers (Evernote/Youdao/Notion)
│   ├── stores/                 # Pinia state management
│   ├── i18n/                   # Internationalization (zh-CN/en-US)
│   ├── types/                  # TypeScript type definitions
│   └── App.vue                 # Application root component
├── src-tauri/                  # Tauri backend source code
│   └── src/
│       ├── command.rs          # IPC command handlers
│       ├── service/            # Business logic layer (19 services)
│       ├── entity/             # Database entities (13)
│       ├── migration/          # Database migrations (25)
│       ├── config.rs           # Configuration management
│       ├── error.rs            # Error handling
│       ├── model.rs            # Data transfer objects
│       └── i18n.rs             # Backend internationalization
├── crates/
│   └── enote-mcp/              # MCP server (standalone binary)
└── doc/                        # Documentation and config examples
```

## UI Component Library

The project includes a custom UI component library:

- **BaseButton** - Button component with multiple types and states
- **BaseInput** - Input field component
- **BaseSelect** - Dropdown select component
- **BaseDialog** - Dialog/modal component
- **BaseDropdown** - Dropdown menu component
- **BasePagination** - Pagination component
- **BaseTooltip** - Tooltip component
- **BaseSkeleton** - Skeleton loading component
- **ColorPicker** - Color picker
- **IconPicker** - Icon picker
- **StylePicker** - Style picker
- **ConfirmDialog** - Confirmation dialog
- **ShortcutRecorder** - Keyboard shortcut recorder
- **Notification** - Toast notifications

## Development Guide

### Requirements

- Node.js 18+
- pnpm 10+
- Rust 1.85+ (Edition 2024)
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

### Code Formatting & Linting

```bash
# Format code
pnpm format

# ESLint check
pnpm lint

# ESLint auto-fix
pnpm lint:fix

# Run tests
pnpm test
```

## Database Configuration

Supports SQLite (default), MySQL, PostgreSQL, and ENote Server.

On **first launch**, the Setup Wizard guides you through backend configuration:

- **SQLite** - Local file database, ready to use out of the box
- **MySQL** - Supports password login and SSL certificate authentication
- **PostgreSQL** - Supports password login and SSL certificate authentication
- **ENote Server** - Connect to a remote ENote-compatible API server with multiple authentication methods:
  - Bearer Token, Basic Auth, JWT (auto-refresh), Custom Header, OAuth 2.0

Configurations are managed as Profiles, supporting multiple backend setups that can be switched at any time. Database passwords, encryption keys, and server authentication credentials are securely stored in the OS keychain (macOS Keychain / Windows Credential Store / Linux Secret Service), never in configuration files.

You can also use a traditional configuration file via command-line:

```bash
enote --config /path/to/application.yml
```

See `doc/application.yml`, `doc/application-mysql.yml`, `doc/application-posgres.yml` for examples.

## MCP Integration

ENote includes a built-in MCP (Model Context Protocol) server, allowing AI tools to operate on notes through a standardized protocol.

### Three-Layer Access Control

| Layer | Scope | Description |
|-------|-------|-------------|
| Note level | Per note | Highest priority |
| Tag level | By tag | Strictest wins when multiple tags |
| Notebook level | By notebook | Lowest priority |

### Access Permissions

- **Inherit** - Inherit from parent level (default)
- **ReadWrite** - AI can read and write
- **ReadOnly** - AI can only read
- **Deny** - AI access completely blocked
- Encrypted notes always force-deny AI access

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
- [SeaORM Documentation](https://www.sea-ql.org/SeaORM)
