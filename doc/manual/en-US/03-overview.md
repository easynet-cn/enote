## 3. Feature Overview

### 3.1 Feature Structure

```
ENote Intelligent Note Management System
├── Notebook Management
│   ├── Create Notebook
│   ├── Edit Notebook
│   ├── Delete Notebook
│   ├── Drag-and-Drop Notebook Sorting
│   └── Filter Notes by Notebook
├── Tag Management
│   ├── Create Tag
│   ├── Edit Tag
│   ├── Delete Tag
│   ├── Drag-and-Drop Tag Sorting
│   └── Filter Notes by Tag
├── Note Management
│   ├── Create Note
│   ├── Edit Note
│   ├── Delete Note (Move to Trash)
│   ├── Trash (Restore / Permanently Delete / Empty)
│   ├── Search Notes (FTS5 Full-Text Search)
│   ├── Pin Note
│   ├── Encrypt / Decrypt Note
│   ├── Linked Notes (Bidirectional Links)
│   ├── Open in New Window
│   └── Note Settings (Notebook / Tag Assignment)
├── Note Templates
│   ├── Template Management (Create / Edit / Delete, supports Rich Text and Markdown dual modes)
│   ├── Create Note from Template (auto-inherits content type)
│   └── Save Note as Template (preserves content type)
├── Rich Text Editor
│   ├── Text Formatting (Bold, Italic, Underline, Strikethrough, etc.)
│   ├── Heading Levels (H1-H6)
│   ├── Font Family and Size Settings
│   ├── Text Alignment
│   ├── Lists (Unordered, Ordered, Task List)
│   ├── Indentation Control
│   ├── Blockquotes and Code Blocks
│   ├── Link and Image Insertion
│   ├── Table Editing
│   ├── Text and Background Color
│   ├── Find and Replace
│   ├── Table of Contents Navigation
│   ├── Undo and Redo
│   ├── Content Block Drag-and-Drop Sorting
│   ├── Math Formula Rendering
│   ├── Image Lazy Loading
│   ├── Smart Paste (Clean External Formatting)
│   └── Local Image File Storage
├── Markdown Editor
│   ├── Source Code Editing
│   ├── Live Preview
│   ├── Split-Screen Mode (Horizontal / Vertical)
│   └── Math Formula Rendering
├── History
│   ├── Version List
│   └── Content Comparison View
├── Export
│   ├── Word Document
│   ├── Markdown File
│   ├── Evernote Format
│   ├── JSON Format
│   └── XML Format
├── Import
│   ├── Evernote Import
│   ├── Youdao Notes Import
│   └── Notion Import
├── Data Backup and Restore
│   ├── Export Backup (SQL / Excel / CSV)
│   ├── Import and Restore
│   └── Automatic Scheduled Backup
├── Application Settings
│   ├── Theme Switching (Light / Dark / Follow System)
│   ├── Language Switching (Chinese / English)
│   ├── Automatic Backup Configuration
│   ├── Security Settings (Lock Screen Password / Timeout Lock / Minimize Lock)
│   ├── Eye Care Screen Saver (scheduled rest / custom appearance / tray countdown)
│   └── MCP Settings (Master Switch / Individual Tool Switches)
├── MCP (AI Tool Integration)
│   ├── Note CRUD (Search / Get / Create / Update / Delete)
│   ├── Notebook Management (List / Create)
│   ├── Tag Management (List / Create)
│   ├── Note Statistics
│   ├── Access Control (Dynamic Tool List / Call Verification)
│   └── Operation Source Tracking (History Record Annotation)
├── Command Palette (Ctrl+P Quick Actions)
├── System Tray
│   ├── Minimize to tray
│   ├── Tray menu (Show window / Pause eye care / Reset countdown / Stop eye care / Quit)
│   ├── Click to toggle window visibility
│   └── Menu bar countdown display (macOS)
└── Application Startup and Initialization
    ├── Command-Line Configuration File (--config)
    ├── Auto-Create Configuration and Database
    ├── Startup Lock Screen Verification
    ├── Automatic Database Migration
    └── Startup Error Notification
```

### 3.2 Interface Layout

ENote uses a responsive three-column layout that automatically adapts to different screen sizes:

#### Desktop Mode (width > 1024px)

Three columns displayed side by side, arranged from left to right:

**Sidebar (Left):** 256 pixels wide, collapsible to a 48-pixel icon bar. Contains the new note button, notebook list, and tag list. The bottom toolbar includes icon buttons for importing notes, data backup, template management, settings, and trash (tooltips appear on hover).

**Note List Area (Center):** Default width of 320 pixels, supports drag-to-resize (200-600 pixel range), and can be fully collapsed. Contains the current notebook name as title, search bar, note card list, and pagination controls. Pinned notes are displayed at the top of the list with a pin indicator.

**Editor Area (Right):** Occupies all remaining width. The top features a feature-rich editing toolbar, followed by the note title input field and the content editing area. Below the editing area is the linked notes panel (expandable/collapsible), and the bottom status bar displays cursor position and word count.

#### Tablet Mode (width 640px ~ 1024px)

Two columns displayed: Note List (fixed 320px) + Editor. The sidebar opens as an overlay by clicking the menu icon in the top-left of the note list, and can be dismissed by clicking the backdrop or close button. Drag-to-resize and collapse buttons are hidden in this mode.

#### Mobile Mode (width < 640px)

Single-view fullscreen switching: the note list and editor are each displayed in fullscreen, with slide animations for transitions. Selecting a note in the list automatically switches to the editor view; a back button in the editor's top-left returns to the list. The sidebar also opens as an overlay.

> **Tip:** The layout mode can be manually switched in Settings (Auto/Desktop/Tablet/Mobile), or cycled via the Ctrl+Shift+M shortcut. See [14.3 Layout Mode Settings](14-settings.md#143-layout-mode-settings).
