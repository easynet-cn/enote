## Appendix A: Changelog

| Version | Date | Changes |
|---------|------|---------|
| V1.0.0 | March 2026 | Auto Update and PostgreSQL Compatibility Fix |
| | | - Auto Update: Automatic new version check on startup with download progress and one-click install/restart |
| | | - Manual Update Check: Available from Help menu and Command Palette |
| | | - Update Signature Verification: Based on Tauri Updater plugin with digital signature verification |
| | | - GitHub Releases Distribution: Three-platform builds via GitHub Actions |
| | | - PostgreSQL Compatibility Fix: All table primary keys unified to BIGINT, fixing int4/int8 type mismatch |
| | | - Log Error Handling Enhancement: Diagnostic info logged via tracing when log writes fail |
| | | - Clippy Warnings Cleared: All 36 Clippy warnings fixed |
| V0.12.0 | March 2026 | Help System and Application Logs |
| | | - Help System: Built-in searchable user manual with table of contents navigation and multilingual support |
| | | - Application Logs: Database operation logs and file system logs with filtering, search, and cleanup |
| | | - Sensitive Data Protection: Automatic log sanitization of passwords, keys, and other sensitive information |
| | | - Frontend Logging Integration: Configurable frontend log level with async non-blocking recording |
| | | - Performance Index Optimization: New composite search indexes for improved query performance |
| V0.11.0 | March 2026 | Internationalization and Help Infrastructure |
| | | - Complete internationalization: Backend error messages and prompts fully support Chinese and English |
| | | - Help infrastructure: Resource file loading and manual rendering framework |
| V0.10.0 | March 2026 | Responsive Layout and Shortcut Customization |
| | | - Responsive Layout: Auto-adapts to Desktop (three-column), Tablet (two-column + sidebar overlay), Mobile (single-view fullscreen switching) |
| | | - Layout Mode Settings: Manual layout mode selection (Auto/Desktop/Tablet/Mobile), persisted to settings |
| | | - Shortcut Customization: 6 application-level shortcuts are customizable with recording, conflict detection, and reset |
| | | - Shortcut Persistence: Custom shortcuts saved to database, effective after restart |
| | | - Command Palette Enhancement: New "Switch Layout Mode" command, shortcut text updates in real time |
| | | - View Transition Animations: Slide transitions for note list and editor in mobile mode |
| | | - Mobile Dialog Adaptation: Dialogs become bottom sheets on small screens |
| | | - Responsive Toolbar: Editor toolbar and action buttons auto-compact on small screens |
| | | - Profile Hot-Switch: Switching database profiles no longer restarts the process, works in all environments |
| | | - Feature Split: Rust compile features split into desktop (desktop UI) and db-full (DB drivers + Keychain) |
| | | - Safe Area Adaptation: iOS safe area support (notch/home indicator) |
| | | - Responsive Status Bar: Line/column info hidden on small screens, character count only |
| V0.9.0 | March 2026 | Cross-Profile Sync and System Optimization |
| | | - Cross-Profile Sync: Sync data to other profiles with Append/Overwrite modes |
| | | - Cross-Database Sync: SQLite <-> MySQL <-> PostgreSQL in any direction, automatic encryption conversion |
| | | - Sync History Management: Automatic per-record logging for each sync, with view/export/delete support |
| | | - Pre-Sync Auto Backup: SQL/Excel/CSV formats, both source and target backed up |
| | | - Streaming Backup Export: Large dataset export uses batched streaming to prevent OOM |
| | | - Database Index Optimization: 5 new query indexes (tag filtering, bidirectional links, history, template sorting) |
| | | - Startup Error Handling: Error dialog with retry/close on startup failure |
| | | - Encryption Service Tests: 10 unit tests covering encrypt/decrypt core logic |
| | | - Toolbar Refactoring: TiptapToolbar split into 8 independent sub-components |
| | | - Note List Loading State: Skeleton feedback during search and pagination |
| | | - Auto Backup Failure Notification: User notified on failure instead of silent ignore |
| | | - CSS Variable Normalization: Overlays and shadows unified via CSS variables for dark mode consistency |
| | | - Store Simplification: Removed redundant ID arrays, data derived directly from Map |
| | | - Cargo Workspace Restructuring: Dependency versions unified at workspace level |
| V0.8.0 | March 2026 | Multi-Profile Management and Content Security Enhancement |
| | | - Setup Wizard: First-launch guided database connection setup for SQLite/MySQL/PostgreSQL |
| | | - Multi-Profile Management: Support multiple database configurations with startup selection or auto-connect |
| | | - OS Keychain Integration: Database passwords and encryption keys securely stored in OS keychain |
| | | - Transparent Content Encryption: AES-256-GCM automatic note content encryption, keys never on disk |
| | | - SSL/TLS Certificate Authentication: MySQL/PostgreSQL support certificate-based login |
| | | - Wizard Language Switching: Setup wizard and profile selector support instant Chinese/English toggle |
| | | - Profile Management Entry: New "Switch Profile" function in the Settings dialog |
| V0.7.0 | March 2026 | MCP Integration and Template Enhancement |
| | | - MCP Server: Built-in MCP protocol support; AI tools can operate on notes via stdio (10 tools) |
| | | - MCP Access Control: Fine-grained control of each MCP tool's enable/disable in the settings panel |
| | | - MCP Dynamic Tool List: `list_tools` only returns enabled tools |
| | | - MCP Three-Layer Access Control: Notebook/Tag/Note-level AI access permissions (Inherit/Read-Write/Read-Only/Deny) |
| | | - MCP Permission Resolution: Encrypted notes auto-denied, note-level priority, multi-tag takes strictest, notebook fallback |
| | | - Operation Source Tracking: New "Operation Source" field in history records (User Operation / AI Tool) |
| | | - Template Editor: Template management upgraded to two-level interface, supporting Rich Text and Markdown dual-mode template content editing |
| | | - Configuration Enhancement: SQLite URL supports `~` path expansion, new `mcp.enabled` configuration option |
| V0.6.0 | March 2026 | Third Batch Feature Release |
| | | - Note Templates: Create/manage templates, quickly create notes from templates, save notes as templates |
| | | - Template Access Optimization: Multiple entry points from sidebar bottom toolbar, command palette, and editor toolbar |
| | | - System Tray: Minimize to tray, click tray icon to toggle window |
| | | - Bidirectional Links: Establish links between notes, linked notes panel for display and management |
| | | - Note Encryption: AES-256-GCM encryption for protecting note content |
| | | - Multi-Window: Support opening notes in new windows |
| | | - Lock Screen Security: Password protection (Argon2id hash), timeout auto-lock, minimize lock |
| | | - Command-Line Configuration: Support --config parameter for specifying custom configuration file at startup |
| V0.5.1 | March 2026 | Second Batch Feature Release |
| | | - Smart Paste: Clean external HTML formatting, support screenshot paste |
| | | - Command Palette: Ctrl+P for quick operations |
| | | - FTS5 Full-Text Search: Trigram tokenization with Chinese substring matching support |
| | | - Local Image Storage: Save images as local files, replacing Base64 inline |
| | | - Automatic Backup: Scheduled SQL backup with automatic old backup cleanup |
| V0.5.0 | March 2026 | First Batch Feature Release |
| | | - Application Settings: Theme switching (Light/Dark/Follow System), language setting persistence |
| | | - Dark Mode: Full dark theme support, semantic CSS variable system |
| | | - Note Pinning: Pin/unpin notes, pinned notes placed at the top of the list |
| | | - Trash: Soft delete notes, support restore/permanently delete/empty |
| | | - Drag-and-Drop Sorting: Notebooks and tags support drag-to-reorder |
| | | - Data Backup and Restore: Support SQL/Excel/CSV formats |
| V0.4.0 | March 2026 | Initial Version Release |
| | | - Notebook Management: Create, edit, delete, filter by notebook |
| | | - Tag Management: Create, edit, delete, filter by tag, multi-tag association |
| | | - Note Management: Create, edit, save, delete, search, note settings |
| | | - Rich Text Editor: Text styles, heading levels, font family and size, alignment, lists, indentation, blockquotes, code blocks, links, images, tables, colors, find and replace, table of contents navigation, content block drag-and-drop, math formulas, image lazy loading |
| | | - Markdown Editor: Source code editing, live preview, split-screen mode (horizontal/vertical), math formula rendering |
| | | - History: Version tracking, content comparison view |
| | | - Export: Word document, Markdown, Evernote ENEX, JSON, XML |
| | | - Import: Evernote, Youdao Notes, Notion |
| | | - Multilingual Support: Simplified Chinese, English |
| | | - Database Support: SQLite (default), MySQL, PostgreSQL |

---

*This manual is based on ENote Intelligent Note Management System V1.0.0. Please refer to the actual software for any feature updates.*
