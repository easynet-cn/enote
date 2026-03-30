# ENote Intelligent Note Management System User Manual

**Software Name:** ENote Intelligent Note Management System

**Version:** V1.1.0

**Date:** March 2026

---

## Table of Contents

- 1. Introduction
  - [1.1 Purpose](#11-purpose)
  - [1.2 Software Overview](#12-software-overview)
  - [1.3 System Requirements](#13-system-requirements)
- 2. Installation and Uninstallation
  - [2.1 Installation Guide](#21-installation-guide)
  - [2.2 Uninstallation Guide](#22-uninstallation-guide)
- 3. Feature Overview
  - [3.1 Feature Structure](#31-feature-structure)
  - [3.2 Interface Layout](#32-interface-layout)
- 4. Notebook Management
  - [4.1 View Notebook List](#41-view-notebook-list)
  - [4.2 Create a Notebook](#42-create-a-notebook)
  - [4.3 Edit a Notebook](#43-edit-a-notebook)
  - [4.4 Delete a Notebook](#44-delete-a-notebook)
  - [4.5 Filter by Notebook](#45-filter-by-notebook)
  - [4.6 Drag-and-Drop Notebook Sorting](#46-drag-and-drop-notebook-sorting)
  - [4.7 Notebook Hierarchy](#47-notebook-hierarchy)
- 5. Tag Management
  - [5.1 View Tag List](#51-view-tag-list)
  - [5.2 Create a Tag](#52-create-a-tag)
  - [5.3 Edit a Tag](#53-edit-a-tag)
  - [5.4 Delete a Tag](#54-delete-a-tag)
  - [5.5 Filter by Tag](#55-filter-by-tag)
  - [5.6 Drag-and-Drop Tag Sorting](#56-drag-and-drop-tag-sorting)
- 6. Note Management
  - [6.1 Note List](#61-note-list)
  - [6.2 Create a Note](#62-create-a-note)
  - [6.3 Edit a Note](#63-edit-a-note)
  - [6.4 Save a Note](#64-save-a-note)
  - [6.5 Delete Notes and Trash](#65-delete-notes-and-trash)
  - [6.6 Search Notes](#66-search-notes)
  - [6.7 Note Settings](#67-note-settings)
  - [6.8 Pin a Note](#68-pin-a-note)
  - [6.9 Note Encryption](#69-note-encryption)
  - [6.10 Linked Notes (Bidirectional Links)](#610-linked-notes-bidirectional-links)
  - [6.11 Open Note in New Window](#611-open-note-in-new-window)
  - [6.12 Note Sorting](#612-note-sorting)
  - [6.13 Batch Operations](#613-batch-operations)
  - [6.14 Note Starring](#614-note-starring)
  - [6.15 File Attachments](#615-file-attachments)
- 7. Rich Text Editor
  - [7.1 Text Styles](#71-text-styles)
  - [7.2 Headings and Fonts](#72-headings-and-fonts)
  - [7.3 Text Alignment](#73-text-alignment)
  - [7.4 Lists](#74-lists)
  - [7.5 Indentation Control](#75-indentation-control)
  - [7.6 Blockquotes and Code](#76-blockquotes-and-code)
  - [7.7 Links and Images](#77-links-and-images)
  - [7.8 Tables](#78-tables)
  - [7.9 Color Settings](#79-color-settings)
  - [7.10 Find and Replace](#710-find-and-replace)
  - [7.11 Table of Contents Navigation](#711-table-of-contents-navigation)
  - [7.12 Undo and Redo](#712-undo-and-redo)
  - [7.13 Clear Formatting](#713-clear-formatting)
  - [7.14 Content Block Drag-and-Drop Sorting](#714-content-block-drag-and-drop-sorting)
  - [7.15 Math Formulas](#715-math-formulas)
  - [7.16 Image Lazy Loading](#716-image-lazy-loading)
  - [7.17 Smart Paste](#717-smart-paste)
  - [7.18 Local Image Storage](#718-local-image-storage)
- 8. Markdown Editor
  - [8.1 Source Code Editing Mode](#81-source-code-editing-mode)
  - [8.2 Preview Mode](#82-preview-mode)
  - [8.3 Split-Screen Mode](#83-split-screen-mode)
- 9. Note History
  - [9.1 View History List](#91-view-history-list)
  - [9.2 View History Content Comparison](#92-view-history-content-comparison)
- 10. Note Templates
  - [10.1 Manage Templates](#101-manage-templates)
  - [10.2 Create a Note from Template](#102-create-a-note-from-template)
  - [10.3 Save a Note as Template](#103-save-a-note-as-template)
- 11. Export
  - [11.1 Export as Word Document](#111-export-as-word-document)
  - [11.2 Export as Markdown](#112-export-as-markdown)
  - [11.3 Export as Evernote Format](#113-export-as-evernote-format)
  - [11.4 Export as JSON](#114-export-as-json)
  - [11.5 Export as XML](#115-export-as-xml)
  - [11.6 Export as PDF](#116-export-as-pdf)
  - [11.7 Export as HTML](#117-export-as-html)
- 12. Import
  - [12.1 Import from Evernote](#121-import-from-evernote)
  - [12.2 Import from Youdao Notes](#122-import-from-youdao-notes)
  - [12.3 Import from Notion](#123-import-from-notion)
- 13. Data Backup and Restore
  - [13.1 Export Backup](#131-export-backup)
  - [13.2 Import and Restore](#132-import-and-restore)
  - [13.3 Automatic Backup](#133-automatic-backup)
- 14. Application Settings
  - [14.1 Theme Settings](#141-theme-settings)
  - [14.2 Language Settings](#142-language-settings)
  - [14.3 Layout Mode Settings](#143-layout-mode-settings)
  - [14.4 Shortcut Settings](#144-shortcut-settings)
  - [14.5 Automatic Backup Settings](#145-automatic-backup-settings)
  - [14.6 Security Settings (Lock Screen)](#146-security-settings-lock-screen)
  - [14.7 MCP Settings](#147-mcp-settings)
  - [14.8 Profile Management](#148-profile-management)
  - [14.9 System Maintenance (Cross-Profile Sync)](#149-system-maintenance-cross-profile-sync)
  - [14.10 Editor Font Size](#1410-editor-font-size)
  - [14.11 Profile Editing](#1411-profile-editing)
- 15. Command Palette
- 16. System Tray
- 17. Keyboard Shortcuts
- 18. Status Bar Information
- 19. Application Startup and Initialization
  - [19.1 Command-Line Parameters](#191-command-line-parameters)
  - [19.2 First Launch (Setup Wizard)](#192-first-launch-setup-wizard)
  - [19.3 Multi-Profile Selection](#193-multi-profile-selection)
  - [19.4 Startup Error Handling](#194-startup-error-handling)
  - [19.5 Subsequent Launches](#195-subsequent-launches)
- 20. MCP (AI Tool Integration)
  - [20.1 Feature Overview](#201-feature-overview)
  - [20.2 Configuration and Activation](#202-configuration-and-activation)
  - [20.3 Available Tools](#203-available-tools)
  - [20.4 Access Control](#204-access-control)
    - [20.4.1 Three-Layer Access Control Architecture](#2041-three-layer-access-control-architecture)
    - [20.4.2 Data-Level Access Control (mcp_access)](#2042-data-level-access-control-mcp_access)
    - [20.4.3 Permission Resolution Rules](#2043-permission-resolution-rules)
    - [20.4.4 Setting AI Access Permissions](#2044-setting-ai-access-permissions)
  - [20.5 Operation Source Tracking](#205-operation-source-tracking)
- 21. Data Storage and Configuration
  - [21.1 Local Data Storage](#211-local-data-storage)
  - [21.2 Configuration File](#212-configuration-file)
  - [21.3 Configuration Options](#213-configuration-options)
  - [21.4 Database Configuration Examples](#214-database-configuration-examples)
  - [21.5 Profile Configuration Management](#215-profile-configuration-management)
  - [21.6 Content Encryption and Key Management](#216-content-encryption-and-key-management)
  - [21.7 SSL/TLS Certificate Authentication](#217-ssltls-certificate-authentication)
  - [21.8 History Version Protection](#218-history-version-protection)
- 22. Help System
- 23. Application Logs
  - [23.1 Opening Log Management](#231-opening-log-management)
  - [23.2 Database Logs](#232-database-logs)
  - [23.3 File Logs](#233-file-logs)
  - [23.4 Frontend Log Level](#234-frontend-log-level)
  - [23.5 Sensitive Data Protection](#235-sensitive-data-protection)
- 24. Auto Update
  - [24.1 Automatic Check on Startup](#241-automatic-check-on-startup)
  - [24.2 Manual Update Check](#242-manual-update-check)
  - [24.3 Download and Install](#243-download-and-install)
- Appendix A: Changelog

---

## 1. Introduction

### 1.1 Purpose

This manual provides a detailed introduction to the features and operations of the ENote Intelligent Note Management System, helping users quickly understand and master the software, fully leverage its note management capabilities, and improve the efficiency of daily information recording and knowledge management.

### 1.2 Software Overview

ENote Intelligent Note Management System is a cross-platform desktop note-taking application that provides users with efficient and convenient note creation, editing, management, and retrieval capabilities. The software supports both rich text and Markdown editing modes, and features notebook categorization, tag system, full-text search, history version tracking, multi-format import/export, note encryption, bidirectional links, template system, automatic backup, and other core functions to meet all personal knowledge management and information recording needs.

The software adopts a front-end/back-end separation architecture. The front end is built with the Vue 3 framework and TypeScript, while the back end uses Rust and the Tauri 2.x framework. The data storage layer uses the SeaORM object-relational mapping framework, with default support for SQLite and compatibility with MySQL and PostgreSQL databases. This architecture ensures high performance, low resource usage, and cross-platform compatibility.

### 1.3 System Requirements

**Operating Systems:**
- Windows 10 or later (64-bit)
- macOS 10.15 (Catalina) or later
- Linux (mainstream distributions such as Ubuntu 20.04+, Fedora 34+)

**Hardware Requirements:**
- Processor: Dual-core 1.6 GHz or higher
- Memory: 4 GB or more
- Disk Space: 200 MB or more available
- Display: 1280 x 720 resolution or higher

**Software Dependencies:**
- System WebView2 runtime (automatically installed on Windows)
- No additional runtime environment required

---

## 2. Installation and Uninstallation

### 2.1 Installation Guide

**Windows:**

1. Obtain the installer file (.msi or .exe format).
2. Double-click the installer to launch the setup wizard.
3. Read and accept the software license agreement.
4. Choose the installation directory (default: C:\Program Files\enote).
5. Click the "Install" button and wait for the installation to complete.
6. After installation, find the ENote icon on the desktop or in the Start menu, and double-click to launch.

**macOS:**

1. Obtain the installer file (.dmg format).
2. Double-click the .dmg file to mount the disk image.
3. Drag the ENote application icon to the "Applications" folder.
4. Find ENote in "Applications" and double-click to launch.
5. On first launch, if the system prompts a security confirmation, go to "System Preferences > Security & Privacy" to allow it to run.

**Linux:**

1. Obtain the installer file (.deb or .AppImage format).
2. Debian/Ubuntu: Install using `sudo dpkg -i enote_x.x.x_amd64.deb`.
3. Other distributions: Grant execute permission to the AppImage with `chmod +x enote_x.x.x.AppImage`, then double-click to run.

### 2.2 Uninstallation Guide

**Windows:** Find ENote in "Control Panel > Programs and Features" and click "Uninstall".

**macOS:** Drag ENote from "Applications" to the Trash.

**Linux:** Debian/Ubuntu: Uninstall using `sudo apt remove enote`.

---

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
│   ├── Minimize to Tray
│   ├── Tray Menu (Show Window / Exit)
│   └── Click to Toggle Window Visibility
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

> **Tip:** The layout mode can be manually switched in Settings (Auto/Desktop/Tablet/Mobile), or cycled via the Ctrl+Shift+M shortcut. See [14.3 Layout Mode Settings](#143-layout-mode-settings).

---

## 4. Notebook Management

Notebooks are the basic organizational unit for notes in ENote. Each note belongs to one notebook, and users can categorize and manage notes through notebooks.

### 4.1 View Notebook List

In the upper section of the sidebar, the notebook list is displayed under the "NOTEBOOKS" heading. The first item is always "All", which shows notes from all notebooks. Each notebook item displays the following information:

- **Icon:** A Lucide icon selected by the user; if not set, a colored dot marker is shown.
- **Name:** The notebook's name text.
- **Note Count:** The number of notes in the notebook, displayed on the right side.

### 4.2 Create a Notebook

**Steps:**

1. On the right side of the "NOTEBOOKS" heading in the sidebar, click the menu icon (three horizontal lines icon).
2. In the dropdown menu, select the "New" option.
3. A "New Notebook" dialog appears. Fill in the following information:
   - **Name (required):** Enter the notebook name.
   - **Description:** Enter a description for the notebook (optional).
   - **Icon:** Click the icon selector to search or browse icons by category and select an appropriate icon (optional). The icon selector supports searching by name, with icons grouped by category.
   - **Sort Order:** Enter a sort value; smaller values appear earlier in the list.
   - **AI Access Permission:** Set the access level for AI tools accessing notes in this notebook via the MCP protocol (optional). Available values include "Inherit" (default), "Read-Write", "Read-Only", and "Deny". See [20.4 Access Control](#204-access-control) for details.
4. Click "OK" to complete creation.

> **Tip:** Notebooks with an AI access permission other than "Inherit" will display a shield icon in the sidebar list for quick identification.

### 4.3 Edit a Notebook

**Steps:**

1. In the sidebar notebook list, click to select the notebook you want to edit.
2. Click the menu icon to the right of the "NOTEBOOKS" heading.
3. In the dropdown menu, select the "Edit" option (only visible when a notebook other than "All" is selected).
4. Modify the relevant information in the "Edit Notebook" dialog that appears.
5. Click "OK" to save the changes.

### 4.4 Delete a Notebook

**Steps:**

1. Select the notebook you want to delete.
2. Click the menu icon and select the "Delete" option from the dropdown menu.
3. A delete confirmation dialog appears, warning that this operation will delete the notebook and all its notes.
4. Click "Confirm" to execute the deletion, or click "Cancel" to abort.

> **Note:** Deleting a notebook will also delete all notes within it. Please proceed with caution. The virtual "All" notebook cannot be deleted.

### 4.5 Filter by Notebook

In the sidebar notebook list, click any notebook name, and the note list area will automatically filter and display all notes in that notebook. The selected notebook item will be highlighted. Click "All" to view notes from all notebooks.

### 4.6 Drag-and-Drop Notebook Sorting

The sidebar notebook list supports drag-and-drop sorting. Press and hold a notebook item and drag it up or down to adjust its position. The new order is automatically saved to the database when the mouse is released. The virtual "All" notebook is always fixed at the top and does not participate in sorting.

---

## 5. Tag Management

Tags are a flexible means of labeling and categorizing notes. A single note can be associated with multiple tags, making it easy to retrieve and manage notes from multiple dimensions.

### 5.1 View Tag List

Below the notebook list in the sidebar, the tag list is displayed under the "TAGS" heading. The first item is always "All", meaning no tag filter is applied. Each tag item displays an icon or color marker along with the tag name.

### 5.2 Create a Tag

**Steps:**

1. On the right side of the "TAGS" heading in the sidebar, click the menu icon.
2. In the dropdown menu, select the "New" option.
3. A "New Tag" dialog appears. Fill in the following information:
   - **Name (required):** Enter the tag name.
   - **Icon:** Select an icon using the icon selector (optional).
   - **Style:** Select a color marker for the tag using the style selector (optional), with multiple preset colors available.
   - **Sort Order:** Enter a sort value.
   - **AI Access Permission:** Set the access level for AI tools accessing notes with this tag (optional). Available values include "Inherit" (default), "Read-Write", "Read-Only", and "Deny". See [20.4 Access Control](#204-access-control) for details.
4. Click "OK" to complete creation.

> **Tip:** Tags with an AI access permission other than "Inherit" will display a shield icon in the sidebar list for quick identification.

### 5.3 Edit a Tag

**Steps:**

1. Select the tag you want to edit in the tag list.
2. Click the menu icon to the right of the "TAGS" heading.
3. Select the "Edit" option.
4. Modify the tag information in the dialog that appears.
5. Click "OK" to save.

### 5.4 Delete a Tag

**Steps:**

1. Select the tag you want to delete.
2. Click the menu icon and select the "Delete" option.
3. Click "Confirm" in the confirmation dialog to execute the deletion.

> **Note:** Deleting a tag does not delete the associated notes; it only removes the association between the tag and the notes.

### 5.5 Filter by Tag

Click any tag in the sidebar to filter and display all notes associated with that tag in the note list.

### 5.6 Drag-and-Drop Tag Sorting

Similar to notebooks, the tag list also supports drag-and-drop sorting. Press and hold a tag item and drag it up or down to adjust its position. The new order is automatically saved when released.

---

## 6. Note Management

### 6.1 Note List

The note list is located in the center of the interface, displaying all notes matching the current filter criteria. Each note card shows the following information:

- **Pin Indicator:** Pinned notes display a pin icon on the card and are always placed at the top of the list.
- **Title:** Displayed in bold, truncated to a single line. If no title is set, "Untitled" is shown.
- **Content Preview:** Displays up to two lines of plain text preview, with HTML and Markdown markup automatically stripped.
- **Notebook:** The notebook the note belongs to is displayed in the lower-left corner.
- **Updated Time:** The last modification time is displayed in the lower-right corner.

The currently selected note card is highlighted. Keyboard arrow keys can be used to move up and down, and Enter or Space keys to confirm selection.

**Pagination:** Pagination controls at the bottom of the list support browsing through large numbers of notes.

**Empty State:** When there are no notes, a guidance message "No notes yet, click to create a new note" is displayed; when a search returns no results, "No matching notes found" is shown.

**List Width Adjustment:** Drag the right edge of the note list area to adjust the list width, supporting a range of 200 to 600 pixels.

**List Panel Collapse/Expand:** A circular collapse button is located at the center of the right edge of the note list. Clicking it completely collapses the note list panel, providing more space for the editor. Clicking again expands the list, restoring it to the default width of 320 pixels.

### 6.2 Create a Note

**Method 1:** Click the "Add Note" button at the top of the sidebar (displayed as a full button when the sidebar is expanded, or as a circular plus icon when collapsed).

**Method 2:** Use the keyboard shortcut Ctrl+N (Cmd+N on macOS).

**Method 3:** Use the command palette (Ctrl+P), search for and select the "New Note" command.

When creating a new note:
1. The note is automatically assigned to the currently selected notebook.
2. The editor enters editing mode, and the cursor is automatically placed in the title input field.
3. The user needs to select the content type from the left side of the toolbar:
   - **Rich Text:** Use the WYSIWYG rich text editor.
   - **Markdown:** Use the Markdown syntax editor.

> **Note:** The content type of a note cannot be changed after the first save.

### 6.3 Edit a Note

**Steps:**

1. Click a note in the note list to select it. The editor area displays the note content (read-only mode).
2. Click the "Edit" button (pencil icon) on the right side of the toolbar, or use the shortcut Ctrl+E (Cmd+E on macOS) to enter editing mode.
3. Edit the note title and content.
4. When finished editing, click the "Save" button or use the shortcut Ctrl+S (Cmd+S on macOS) to save.
5. To discard changes, click the "Cancel" button or press the Esc key.

> **Note:** For newly created but unsaved notes, clicking "Cancel" will automatically remove the note from the list without leaving a blank note. For already saved notes, canceling the edit restores the content to the last saved state.

### 6.4 Save a Note

In editing mode, click the "Save" button (checkmark icon) on the right side of the toolbar or use the shortcut Ctrl+S (Cmd+S on macOS) to save the current note. After a successful save, the system displays a success notification, and the note automatically switches to read-only mode.

The system automatically records a history version for each save, enabling subsequent version tracking and content recovery.

### 6.5 Delete Notes and Trash

**Delete a Note (Move to Trash):**

1. Select the note you want to delete.
2. Click the "Delete" button (red trash can icon) on the right side of the toolbar.
3. Click "Confirm" in the confirmation dialog to execute the deletion.

Deleted notes are not immediately removed from the database; they are moved to the trash (soft delete).

**Trash Operations:**

Click the "Trash" icon button at the bottom of the sidebar to open the trash dialog, where you can perform the following operations:

- **View Deleted Notes:** The trash list displays all deleted notes, including their titles and deletion times.
- **Restore a Note:** Click the "Restore" button on the right side of a note to restore it to its original notebook.
- **Permanently Delete:** Click the "Permanently Delete" button to permanently delete a single note. This operation cannot be undone.
- **Empty Trash:** Click the "Empty Trash" button to permanently delete all notes in the trash. A confirmation dialog will appear; after confirmation, the empty operation is executed. This operation cannot be undone.

### 6.6 Search Notes

A search bar is provided at the top of the note list area:

1. Enter keywords in the search box.
2. Press Enter to execute the search.
3. The note list will filter and display notes whose titles or content contain the keywords.
4. A clear button (X icon) appears on the right side of the search box; click it to clear the search criteria and restore the full list.

The search feature is based on the SQLite FTS5 full-text search engine, using trigram tokenization technology with support for Chinese substring matching. Search results are sorted by relevance (BM25 algorithm), with title matches weighted higher than content matches to ensure the most relevant notes appear first.

### 6.7 Note Settings

In editing mode, click the "Settings" button (gear icon) in the toolbar to open the note settings dialog for managing the note's assignment, tags, and AI access permissions:

- **Notebook:** Use a searchable dropdown selector to move the note to a specified notebook.
- **Tags:** All available tags are displayed in a grid format with checkboxes to select or deselect associated tags. A note can be associated with multiple tags.
- **AI Access Permission:** Set the access level for AI tools accessing this note via the MCP protocol. Available values include "Inherit" (default, inherited from tags and notebooks), "Read-Write", "Read-Only", and "Deny". Note-level settings have the highest priority and can override notebook and tag settings. See [20.4 Access Control](#204-access-control) for details.

Click the "Save" button to apply the changes.

### 6.8 Pin a Note

Users can pin important notes so they always appear at the very top of the note list.

**How to Pin:**

- In the note list, right-click a note card or click the pin icon button on the card to toggle the note's pinned status.
- Pinned notes display a pin indicator on the card.
- Click again to unpin.

Pinned notes are placed before all unpinned notes in the list. Multiple pinned notes are sorted by update time among themselves.

### 6.9 Note Encryption

ENote supports AES-256-GCM encryption for note content to protect sensitive information.

**Encrypt a Note:**

1. Select the note you want to encrypt and make sure it is in editing mode.
2. Click the "Encrypt" button in the toolbar.
3. In the encryption dialog that appears, enter a password and confirm it.
4. Click the "Encrypt" button. The note content will be encrypted and saved.

**Decrypt a Note:**

1. Open an encrypted note (the editor will display a "This note's content is encrypted" message).
2. Click the "Decrypt" button.
3. In the dialog that appears, enter the password that was set during encryption.
4. If the password is correct, the note content is restored to plaintext display.

> **Note:**
> - Encryption uses the AES-256-GCM algorithm with the key derived from the user's password via SHA-256, providing strong security.
> - Please make sure to remember the encryption password. If the password is lost, the note content cannot be recovered.
> - Encrypted note content is stored in ciphertext form in the database and cannot be found through search by its original content.

### 6.10 Linked Notes (Bidirectional Links)

ENote supports establishing bidirectional link relationships between notes, making it easy to build a knowledge network.

**View Linked Notes:**

In the "Linked Notes" panel at the bottom of the editor, click the panel header to expand/collapse the panel. The panel displays all notes linked to the current note, including note titles. Click any linked note's title to quickly jump to and view that note.

**Add a Link:**

1. In the linked notes panel, click the "Add Link" button.
2. In the search box that appears, enter keywords from the title of the note you want to link.
3. Click to select the target note from the search results list. The link is established immediately.

**Remove a Link:**

Hover over a linked note and click the "x" button that appears on the right side to remove the link.

> **Note:** Bidirectional links are symmetric -- if Note A links to Note B, then Note A will also appear in Note B's linked notes panel. Removing the link from either end removes the association from both sides.

### 6.11 Open Note in New Window

ENote supports opening notes in independent windows, making it convenient to view or compare multiple notes simultaneously.

**How to Use:** Through the command palette or the "Open in New Window" option in the toolbar, open the current note in an independent application window. The new window can be independently resized and repositioned and has full editing capabilities.

---

## 7. Rich Text Editor

ENote integrates a feature-rich WYSIWYG (What You See Is What You Get) rich text editor. The toolbar is located at the top of the editor, and when there are too many toolbar buttons, horizontal scrolling is supported with scroll indicator arrows on both sides.

### 7.1 Text Styles

The toolbar provides the following text formatting buttons:

| Button | Function | Description |
|--------|----------|-------------|
| **B** | Bold | Set selected text to bold |
| *I* | Italic | Set selected text to italic |
| U | Underline | Add underline to selected text |
| ~~S~~ | Strikethrough | Add strikethrough to selected text |
| X² | Superscript | Set selected text to superscript |
| X₂ | Subscript | Set selected text to subscript |

Click a button to toggle the corresponding format on/off. Select text and click a button to apply the format. Active format buttons are highlighted.

### 7.2 Headings and Fonts

**Heading Levels:** Use the dropdown selector to set paragraph heading levels, supporting:
- Normal text (default paragraph)
- Heading 1 (H1) through Heading 6 (H6), with decreasing font sizes.

**Font Family:** The dropdown selector provides multiple font choices, grouped by category:
- **Sans-serif:** Arial, Helvetica, Verdana, Tahoma, Trebuchet MS, Microsoft YaHei, PingFang
- **Serif:** Times New Roman, Georgia, Palatino, SimSun, KaiTi, FangSong
- **Monospace:** Courier New, Consolas, Monaco, Source Code Pro
- **Decorative:** Comic Sans MS, Impact, Brush Script

**Font Size:** The dropdown selector supports multiple preset sizes including 12px, 14px, 16px, 18px, 20px, 24px, 28px, 32px, 36px, 48px, and 72px.

### 7.3 Text Alignment

Four paragraph alignment buttons are provided:

- **Left Align:** Text aligned to the left (default).
- **Center Align:** Text centered.
- **Right Align:** Text aligned to the right.
- **Justify:** Text justified on both sides.

### 7.4 Lists

Three list types are supported:

- **Unordered List:** A list marked with bullet points.
- **Ordered List:** A list marked with numbered items.
- **Task List:** A checklist with checkboxes. Click a checkbox to toggle the completion status. Completed task items display a strikethrough and gray text effect. Nested task lists are supported.

### 7.5 Indentation Control

- **Increase Indent:** Click the increase indent button or press Tab to indent the paragraph one level to the right (2em per level).
- **Decrease Indent:** Click the decrease indent button or press Shift+Tab to decrease the paragraph indent by one level.
- Maximum indent level is 10; minimum is 0.

### 7.6 Blockquotes and Code

- **Blockquote:** Click the blockquote button to convert a paragraph to a blockquote style, displaying a themed vertical line on the left side with gray italic text.
- **Code Block:** Click the code block button to insert a syntax-highlighted code block. Supports syntax coloring for multiple programming languages (based on the highlight.js engine).
- **Inline Code:** Click the inline code button to set the selected text to a monospace inline code style.

### 7.7 Links and Images

**Insert a Link:**
1. Select the text you want to add a link to.
2. Click the link button in the toolbar.
3. In the dialog that appears, enter the link URL.
4. Click "OK" to insert the link. The linked text is displayed in blue underline style.

**Remove a Link:** Select text with an existing link and click the "Unlink" button to remove the hyperlink.

**Insert an Image:**
1. Click the image button in the toolbar.
2. In the dialog that appears, choose the insertion method:
   - **Enter URL:** Enter the image's web address in the text field.
   - **Upload File:** Click the upload button to select a local image file. Common image formats are supported.
3. Click "OK" to insert the image.

Images are displayed with rounded corners in the editor, support lazy loading technology, and show an animated placeholder during loading.

### 7.8 Tables

- **Insert Table:** Click the insert table button to automatically insert a 3-row by 3-column table.
- **Add Column:** Insert a new column after the current column.
- **Add Row:** Insert a new row below the current row.
- **Delete Table:** Delete the entire table.
- Table column widths can be adjusted by dragging column borders. Selected cells are highlighted.

### 7.9 Color Settings

**Text Color:** Click the text color button to open the color picker, which provides a grid of 10 preset colors, and also supports custom colors through a color picker or hexadecimal color value input.

**Background Color:** Click the background color button to set a background highlight color for selected text. Color selection works the same way as text color.

**Highlighter:** Click the highlight button to quickly add a yellow highlight background to selected text.

### 7.10 Find and Replace

Click the "Find and Replace" button in the toolbar to open the find and replace panel above the editor:

- **Find Input:** Enter the text to search for. Press Enter to navigate to the next match.
- **Replace Input:** Enter the replacement text.
- **Case Sensitive:** A checkbox to control whether matching is case-sensitive.
- **Result Count:** Displays the current match position and total number of matches (e.g., "3 / 12").
- **Previous/Next:** Arrow buttons to navigate between matches.
- **Replace:** Replace the current match.
- **Replace All:** Replace all matches at once.

All matches are highlighted in yellow in the editor, and the currently focused match is distinguished with an orange highlight.

### 7.11 Table of Contents Navigation

Click the "Table of Contents" button in the toolbar to open a floating table of contents navigation panel (240 pixels wide) on the right side of the editor:

- Automatically extracts all headings (H1-H6) from the document and displays them with hierarchical indentation.
- Click any entry in the table of contents to automatically scroll the editor to the corresponding heading position.
- The heading currently visible in the viewport is highlighted in the table of contents (with a themed vertical line on the left).
- Click the close button (X) in the upper-right corner of the panel to close the table of contents.

### 7.12 Undo and Redo

- **Undo:** Click the undo button to revert to the previous editing operation.
- **Redo:** Click the redo button to restore an undone operation.

### 7.13 Clear Formatting

Click the "Clear Formatting" button in the toolbar to remove all formatting styles from the selected text, restoring it to the default text format.

### 7.14 Content Block Drag-and-Drop Sorting

The rich text editor supports rearranging content blocks through drag-and-drop. When hovering over block-level elements such as paragraphs, headings, blockquotes, or code blocks, a drag handle appears on the left side of the element. Press and hold the drag handle and drag to move the content block to another position in the document. During dragging, the editor displays visual indicators of the current drag state, and the content block is inserted at the target position when the mouse is released.

This feature is useful for quickly adjusting document structure without the need for cut-and-paste operations.

### 7.15 Math Formulas

The rich text editor supports math formula rendering based on the KaTeX math typesetting engine. Users can insert math formulas into notes, and the system automatically renders the formula syntax into standard mathematical notation. Both inline formulas and standalone formula blocks are supported, meeting the mathematical expression needs of academic notes and technical documents.

### 7.16 Image Lazy Loading

Images in the editor use smart lazy loading technology. When a note contains many images, only images within the current visible area are loaded. Images not yet in the visible area display a skeleton placeholder with animation effects. When the user scrolls the page and an image enters the visible range, the image automatically begins loading and appears with a fade-in animation. This mechanism effectively reduces initial loading time and improves browsing fluidity for long documents.

### 7.17 Smart Paste

The editor features a smart paste function that automatically cleans HTML content copied and pasted from external sources (such as Microsoft Office, Google Docs, web pages, etc.):

- **Clean Office Markup:** Automatically removes MS Office-specific XML namespaces, `mso-` style prefixes, and other redundant markup.
- **Clean Web Formatting:** Removes excessive CSS class names and `data-*` attributes, retaining only basic text formatting (bold, italic, color, etc.).
- **Tag Conversion:** Automatically converts block-level elements like `<div>` to standard `<p>` paragraphs.
- **Remove Empty Elements:** Automatically removes empty paragraphs and empty `<span>` tags with no content.
- **Screenshot Paste:** Supports pasting screenshot images directly from the clipboard. Images are automatically saved as local files.

### 7.18 Local Image Storage

Images inserted into the editor through paste or drag-and-drop are automatically saved as local files rather than stored inline in note content as Base64 encoding. Image files are saved in the `images/` subdirectory under the application data directory. Each image uses a UUID as the filename and supports common formats including PNG, JPG, GIF, WebP, SVG, BMP, and ICO.

Advantages of local file storage:
- **Reduced Database Size:** Note content no longer contains large amounts of Base64-encoded image data.
- **Improved Loading Performance:** Images are loaded directly from the local file system via Tauri asset protocol, resulting in faster loading.
- **Easy Management:** Image files are stored independently and can be viewed and managed directly in the file system.

---

## 8. Markdown Editor

When the note content type is set to Markdown, the editor provides a professional Markdown editing experience.

### 8.1 Source Code Editing Mode

In source code mode, the editing area displays as a dark-themed monospace text editing area where users write Markdown syntax directly. Standard Markdown syntax is supported, including headings, lists, links, images, code blocks, tables, blockquotes, and more.

### 8.2 Preview Mode

Click the "Source/Preview Toggle" button in the toolbar to switch between Markdown source code editing and rendered preview. In preview mode, the formatted content rendered from Markdown syntax is displayed.

Markdown preview supports the following enhanced rendering:
- **Math Formulas:** Renders math formulas using the KaTeX engine. Use `$...$` for inline formulas and `$$...$$` for standalone formula blocks.
- **Syntax Highlighting:** Code blocks automatically detect the programming language and display with color-coded syntax.
- **Tables, Task Lists:** Standard Markdown extension syntax is rendered correctly.

### 8.3 Split-Screen Mode

The Markdown editor supports two split-screen layouts for simultaneously viewing source code and rendered preview:

**Vertical Split:** Click the "Vertical Layout" button in the toolbar. The Markdown source code is displayed at the top and the rendered preview at the bottom.

**Horizontal Split:** Click the "Horizontal Layout" button in the toolbar. The Markdown source code is displayed on the left and the rendered preview on the right.

Split-screen mode features:
- The center divider supports drag-to-adjust the ratio between the two panels (range 20%-80%).
- The source and preview areas support synchronized scrolling, with the preview automatically following during editing.
- Large documents use Web Worker technology for asynchronous preprocessing to ensure smooth editing.

---

## 9. Note History

ENote automatically records the history of every modification and deletion operation on notes, supporting version tracking and content comparison.

### 9.1 View History List

**Steps:**

1. Select a note.
2. Click the "History" button (clock icon) on the right side of the toolbar.
3. The system displays a full-screen history dialog with the history records in a table format.

The history table contains the following columns:

| Column | Description |
|--------|-------------|
| ID | Unique identifier for the history record |
| Notebook | The notebook the note belongs to |
| Title | The note title |
| Content Type | Rich Text or Markdown |
| Tags | Associated tags |
| Operation Type | Create, Modify, or Delete |
| Operation Source | User operation or AI Tool (MCP) |
| Operation Time | The time the operation occurred |
| Actions | View details button |

The list supports paginated browsing, with options to display 20, 50, 100, or 200 records per page. A skeleton screen placeholder animation is displayed during loading.

### 9.2 View History Content Comparison

In the history list, click the "View" button of a record to open the content comparison window:

- **Left Panel (gray header):** Displays the old content before modification.
- **Right Panel (themed header):** Displays the new content after modification.

Both sides are rendered in rich text format for intuitive comparison of the differences.

---

## 10. Note Templates

ENote provides a note template feature that allows users to create and manage commonly used note templates for quickly generating new notes with preset content structures.

### 10.1 Manage Templates

The "Template Management" dialog can be opened in the following ways:

- **Sidebar:** Click the "Template Management" icon button (template icon) in the bottom toolbar of the sidebar.
- **Command Palette:** Press Ctrl+P to open the command palette, search for "Template Management" and execute.

The template management dialog uses a two-level interface design:

**Level 1 -- Template List:**

- **View Template List:** The dialog lists all created templates, showing the template name and update time.
- **New Template:** Click the "+ New Template" button at the top to enter the template editing interface.
- **Edit Template:** Click the "Edit" button on the right side of a template to enter the template editing interface and modify its name and content.
- **Delete Template:** Click the "Delete" button on the right side of a template to delete templates that are no longer needed.

**Level 2 -- Template Editing:**

- **Back Button:** Click the back arrow in the upper-left corner to return to the template list.
- **Template Name and Content Type:** The top row contains a name input field and a content type toggle button (Rich Text / Markdown), consistent with the dual-mode note editor. New templates default to rich text mode, and editing existing templates automatically restores their original content type. Switching content type preserves the current editing content.
- **Editing Toolbar (Rich Text Mode):** In rich text mode, a simplified formatting toolbar is displayed, including heading levels (Normal / H1 / H2 / H3), text styles (bold, italic, underline, strikethrough), lists (unordered, ordered, task list), block elements (blockquote, code block, horizontal rule), and undo/redo buttons.
- **Markdown Mode:** When Markdown is selected, the editing area switches to Markdown source code editing mode. No formatting toolbar is displayed, and users write Markdown syntax directly.
- **Save:** Click the "Save" button at the bottom to save the template. The template's content type is saved simultaneously.

> **Note:** In the template list, each template name displays a content type badge: **RT** (Rich Text) or **MD** (Markdown) for quick identification.

### 10.2 Create a Note from Template

In the template management dialog, click the "Use Template" button on the right side of a template. The system will create a new note based on that template's content. The new note automatically inherits the template's content type (Rich Text or Markdown) and enters editing mode, allowing the user to continue editing based on the template content.

You can also search for "New Note from Template" in the command palette to quickly create a note from a template.

### 10.3 Save a Note as Template

While editing a note, click the "Save as Template" button (template icon) in the action button area on the right side of the editor toolbar to save the current note's content and content type as a template. This operation can also be performed by searching for "Save as Template" in the command palette. The original note is not affected after saving as a template.

---

## 11. Export

ENote supports exporting notes to multiple formats to meet different use cases and data migration needs.

**Steps:**

1. Select the note you want to export.
2. Click the "Export" button (download icon) on the right side of the toolbar.
3. In the export dialog that appears, select the target format.
4. Click the "Export" button.
5. A file save dialog appears. Choose the save location and file name.
6. A success notification is displayed when the export is complete.

### 11.1 Export as Word Document

Exports the note as a .doc format Word document, preserving text formatting and styles. The document includes the note title as the main heading at the top and can be opened and edited in Microsoft Word, WPS, and other office software.

### 11.2 Export as Markdown

Exports the note as a .md format Markdown file. Rich text notes are automatically converted to Markdown syntax; Markdown notes are exported with their original content.

### 11.3 Export as Evernote Format

Exports the note as .enex format, compatible with Evernote's import function, making it easy to migrate notes to the Evernote platform.

### 11.4 Export as JSON

Exports the note as a .json structured data file containing the following fields:
- Title (title)
- Content (content)
- Content Type (contentType)
- Tags (tags)
- Created Time (createTime)
- Updated Time (updateTime)

### 11.5 Export as XML

Exports the note as .xml format. The data structure is identical to JSON export and is suitable for scenarios requiring XML format data exchange.

---

## 12. Import

ENote supports importing data from multiple mainstream note platforms, helping users conveniently migrate existing notes.

Click the "Import Notes" icon button (import icon, tooltip shown on hover) at the bottom of the sidebar to launch the import wizard, which consists of four steps.

### 12.1 Import from Evernote

**Steps:**

1. **Select Source:** In the first step of the import wizard, select "Evernote".
2. **Select File:** Drag the .enex file exported from Evernote to the upload area, or click the area to select the file.
3. **Select Target Notebook:** In the dropdown selector, choose the target notebook to import into. Optionally check the "Auto-create non-existent tags" option (checked by default).
4. **Execute Import:** Click the "Start Import" button. The system displays import progress, including the current note title being processed and the progress percentage.
5. **Import Complete:** An import result summary is displayed, including the number of successful imports and failure details.

### 12.2 Import from Youdao Notes

**Steps:**

1. **Select Source:** Select "Youdao Notes".
2. **Select File:** Select the .zip archive exported from Youdao Notes.
3. **Select target notebook and configure options.**
4. **Execute import and view results.**

### 12.3 Import from Notion

**Steps:**

1. **Select Source:** Select "Notion".
2. **Select File:** Select the .zip archive exported from Notion (containing Markdown files).
3. **Select target notebook and configure options.**
4. **Execute import and view results.**

> **Note:** During the import process, tag information from the source files is automatically parsed. If "Auto-create non-existent tags" is checked, the system will automatically create tags that exist in the source data but not in the system.

---

## 13. Data Backup and Restore

ENote provides comprehensive data backup and restore capabilities, supporting both manual export and automatic scheduled backup to ensure data safety.

Click the "Data Backup" icon button (database icon) at the bottom of the sidebar to open the data backup dialog. The dialog contains two tabs: "Export Backup" and "Import & Restore".

### 13.1 Export Backup

**Steps:**

1. Open the data backup dialog. The "Export Backup" tab is selected by default.
2. Select the backup format:
   - **SQL:** Export as a standard SQL statement file (.sql), suitable for cross-database migration.
   - **Excel:** Export as an Excel spreadsheet (.xlsx), with each data table as a separate sheet.
   - **CSV:** Export as CSV files packaged in a ZIP archive (.zip).
3. Click the "Export" button.
4. In the system file save dialog, choose the save location and file name.
5. A success notification is displayed when the export is complete.

### 13.2 Import and Restore

**Steps:**

1. Open the data backup dialog and switch to the "Import & Restore" tab.
2. Select the format matching the backup file (SQL / Excel / CSV).
3. Click the "Import" button.
4. In the system file selection dialog, select the backup file.
5. A confirmation dialog appears, warning that the import operation will clear all current data and replace it with the backup data.
6. Click "Confirm Import" to execute the restore, or click "Cancel" to abort.
7. A success notification is displayed when the restore is complete.

> **Note:** The import operation will overwrite all current data. This operation cannot be undone. Please make sure to back up your current data before importing. After import, it is recommended to refresh the page to load the latest data.

### 13.3 Automatic Backup

ENote supports automatic scheduled backup, which creates database backups automatically at set intervals in the background without manual intervention.

**How It Works:**

- Automatic backups are saved in SQL format to the `backups/` subdirectory under the application data directory.
- Backup files are named in the format `enote_backup_YYYYMMDD_HHMMSS.sql`, including a complete timestamp.
- The system automatically cleans up old backups based on the configured retention count to avoid excessive disk space usage.
- When the application starts, it automatically checks whether a backup is needed (whether the time since the last backup exceeds the configured interval).

**Configuration:** Configure in the "Automatic Backup" area of the "Settings" dialog. See [14.5 Automatic Backup Settings](#145-automatic-backup-settings) for details.

---

## 14. Application Settings

Click the "Settings" icon button at the bottom of the sidebar to open the settings dialog, where you can configure the following options.

### 14.1 Theme Settings

ENote supports three theme modes:

- **Light Mode:** A classic white background interface, suitable for daytime use.
- **Dark Mode:** A dark background interface that reduces eye strain, suitable for nighttime or low-light environments.
- **Follow System:** Automatically follows the operating system's light/dark setting. The application interface changes in real time when the system theme switches.

In the "Appearance" area of the settings dialog, click the corresponding theme button to switch. Theme changes take effect immediately, and settings are saved automatically.

### 14.2 Language Settings

ENote supports two interface languages: Simplified Chinese (zh-CN) and English (en-US).

In the "Appearance" area of the settings dialog, click the corresponding language button to switch. After switching, all interface text is updated immediately without requiring an application restart. The language preference is saved automatically and applied on the next launch.

### 14.3 Layout Mode Settings

ENote supports automatic layout adaptation based on window width, or manual layout mode selection:

| Mode | Description |
|------|-------------|
| **Auto (Default)** | Automatically selects based on window width: > 1024px Desktop, 640-1024px Tablet, < 640px Mobile |
| **Desktop** | Force three-column side-by-side layout |
| **Tablet** | Force two-column layout (sidebar as overlay) |
| **Mobile** | Force single-view fullscreen switching layout |

In the "Appearance" area of the settings dialog, click the corresponding layout button to switch. You can also use the Ctrl+Shift+M shortcut (Cmd+Shift+M on macOS) to cycle through modes, or search "Switch Layout" in the command palette.

### 14.4 Shortcut Settings

In the "Shortcuts" area of the settings dialog, you can customize application-level keyboard shortcuts.

**System-level shortcuts** (such as Ctrl+S for save, Esc for cancel editing) are fixed bindings that cannot be modified, indicated by a "System" label.

**To customize a shortcut:**

1. Click the shortcut display area to enter recording mode. The interface shows "Press shortcut keys...".
2. Press the new key combination (e.g., Ctrl+K). The recording completes and saves automatically.
3. If the new shortcut conflicts with an existing one, a red conflict warning is displayed and the change is not saved.
4. Press Esc to cancel recording.

Each customizable shortcut has a "Reset" button on the right to restore its default value. A "Reset All to Default" button at the top restores all shortcuts at once.

Shortcut settings are automatically persisted and remain effective after restarting the application. Shortcut text displayed in the command palette is updated accordingly.

### 14.5 Automatic Backup Settings

In the "Automatic Backup" area of the settings dialog, the following options can be configured:

| Option | Description |
|--------|-------------|
| **Enable Automatic Backup** | Toggle switch to enable or disable automatic backup |
| **Backup Interval** | Dropdown to select backup frequency: 1 hour, 4 hours, 8 hours, 24 hours |
| **Retention Count** | Dropdown to select the maximum number of backups to retain: last 5, 10, 20, 50 |
| **Backup Now** | Manually trigger a one-time backup operation |
| **Last Backup** | Displays the file name and time of the most recent automatic backup |

After enabling automatic backup, the system automatically performs backups at the set interval in the background and automatically cleans up old backup files based on the retention count.

### 14.6 Security Settings (Lock Screen)

ENote provides a lock screen security feature that can automatically lock the interface when the application starts or after a period of inactivity, requiring a password to continue using the application and preventing unauthorized access.

**Default State:** The lock screen feature is disabled by default. Users can enable it manually as needed.

In the "Security" area of the settings dialog, the following options can be configured:

| Option | Description |
|--------|-------------|
| **Lock Screen Mode** | Toggle lock screen protection on/off. Options: "Off" or "Password" |
| **Set Password** | Set the lock screen password when enabling for the first time (minimum 4 characters) |
| **Change Password** | When a password is already set, enter a new password to replace the current one |
| **Remove Password** | Clear the set lock screen password and disable the lock screen feature |
| **Timeout Lock** | Set the idle time before automatic lock: Startup only, 1 minute, 5 minutes, 15 minutes, 30 minutes |
| **Lock on Minimize** | Toggle switch. When enabled, the application automatically locks when the window is minimized or switched to the background |

**Lock Screen Interface:**

When the application is locked, a full-screen semi-transparent overlay is displayed with a centered lock screen card. Users must enter the correct lock screen password and click the "Unlock" button to access the application. When an incorrect password is entered, an error message is displayed with a shake animation.

**Manual Lock:** Press Ctrl+L (Cmd+L on macOS) to manually lock the application at any time. You can also search for "Lock Application" in the command palette.

**Password Security:** The lock screen password is stored after being hashed with the Argon2id algorithm, not saved in plaintext, ensuring password security.

### 14.7 MCP Settings

MCP (Model Context Protocol) is an AI tool integration feature that allows AI clients such as Claude Desktop and Claude Code to operate on note data through a standard protocol. MCP functionality is disabled by default and must be manually enabled in the settings.

In the "MCP (AI Tool Integration)" area of the settings dialog, the following options can be configured:

| Option | Description |
|--------|-------------|
| **Enable MCP** | Master switch controlling whether AI tools can operate on notes via the MCP protocol |
| **Search Notes** | Allow AI tools to search note content |
| **Get Note Details** | Allow AI tools to read full note content |
| **Create Note** | Allow AI tools to create new notes |
| **Update Note** | Allow AI tools to modify existing notes |
| **Delete Note** | Allow AI tools to delete notes (move to trash) |
| **List Notebooks** | Allow AI tools to view the notebook list |
| **Create Notebook** | Allow AI tools to create new notebooks |
| **List Tags** | Allow AI tools to view the tag list |
| **Create Tag** | Allow AI tools to create new tags |
| **Note Statistics** | Allow AI tools to get note count statistics |

After enabling the master switch, expand the individual tool switch list to precisely control the availability of each MCP tool. Settings take effect in real time without requiring an MCP Server restart.

> **Note:** MCP settings control global and tool-level access permissions for AI tools. After disabling a tool, AI clients will not be able to see or call that tool. Notes created or modified through AI tools are marked with the "AI Tool" source in the history records, making it easy to distinguish between human operations and AI operations.
>
> In addition to global and tool-level control, ENote also supports more fine-grained **data-level access control** -- you can set AI access permissions (Inherit / Read-Write / Read-Only / Deny) separately on notebooks, tags, and individual notes. See [20.4 Access Control](#204-access-control) for details.

### 14.8 Profile Management

In the Settings dialog, the **Profile Management** section provides the following:

- **Current Profile Info:** Displays the name and database type of the currently active profile.
- **Switch Profile:** Click the "Switch Profile" button to return to the profile selection page. From there you can:
  - Select an existing database configuration and connect
  - Click "New Profile" to enter the Setup Wizard and create a new database configuration
  - Edit or delete existing configurations
  - Click the close button in the upper right to close the selection page and return to the main interface

### 14.9 System Maintenance (Cross-Profile Sync)

At the bottom of the Settings dialog, the "System Maintenance" section provides cross-profile data synchronization and sync history management.

#### 14.9.1 Cross-Profile Sync

Synchronize data from the current profile to another profile, supporting cross-database synchronization (SQLite <-> MySQL <-> PostgreSQL).

**Steps:**

1. In the "System Maintenance" section, click "Start Sync" to open the sync dialog.
2. Select the target profile from the dropdown (lists all available profiles except the current one).
3. If the target profile uses password authentication (MySQL/PostgreSQL), enter the target database password.
4. Select the sync mode:

| Mode | Description |
|------|-------------|
| **Append (Default)** | Appends data to the target database without affecting existing data. IDs are auto-assigned by the target |
| **Overwrite** | Clears the target database before writing all data. Requires confirmation |

5. Select the sync scope (multiple selections allowed):

| Data Type | Description |
|-----------|-------------|
| **Notebooks** | Notebooks and hierarchy structure |
| **Tags** | All tags |
| **Notes** | Note content, tag associations, pin status, trash status |
| **Note History** | Original history records from the source |
| **Templates** | Note templates |
| **Settings** | Application settings (key-value pairs) |

6. Backup settings: It is recommended to enable "Auto backup before sync". You can choose the backup format (SQL / Excel / CSV).
7. After selecting the target profile, a data count preview is displayed at the bottom.
8. Click "Start Sync" to execute. The interface shows real-time progress (per-table progress + overall progress bar).

**Key Behaviors:**

- **Encryption Handling:** If the source and target have different encryption configurations, the system automatically handles the conversion -- decrypts from source, re-encrypts to target, ensuring data consistency.
- **History Records:** Notes are synced through the service layer. The target automatically generates history records with the operation source marked as "Sync".
- **Foreign Key Mapping:** Notebook and tag IDs are reassigned in the target database. Note associations with notebooks and tags are automatically corrected.

#### 14.9.2 Sync History Management

Each sync operation automatically generates detailed records, including the status of each data record.

Click "View Details" in the "System Maintenance" section to open the sync history management dialog.

**Sync History List:** Displays all historical sync operations with the following information:

| Field | Description |
|-------|-------------|
| Source -> Target | Source and target profile names |
| Status | Completed / Failed |
| Success/Failed Count | Number of successful and failed records |
| Sync Mode | Append / Overwrite |
| Database Types | e.g., sqlite -> mysql |
| Time | Time when the sync was executed |

**Actions:**

- **View Details:** Expand the complete result of a single sync, including summary table and per-record details. Details support filtering by table name and status.
- **Export Log:** Export the sync record as a JSON file for troubleshooting.
- **Delete Record:** Delete a single sync record and its details.
- **Clear All Records:** Remove all historical sync records.

---

## 15. Command Palette

ENote provides a command palette feature similar to VS Code for quickly executing various operations via the keyboard.

**How to Open:** Press the Ctrl+P (Cmd+P on macOS) shortcut to open the command palette.

**How to Use:**

1. After opening the command palette, a search input field appears at the top.
2. Enter keywords, and the command palette filters and displays matching commands in real time.
3. Use arrow keys to move up and down to select a command, and press Enter to execute.
4. Press Esc to close the command palette.

**Available Commands:**

| Command | Description | Category |
|---------|-------------|----------|
| New Note | Create a new note | Note |
| Save Note | Save the currently edited note | Note |
| Edit Note | Enter editing mode | Note |
| Delete Note | Delete the current note | Note |
| Expand/Collapse Sidebar | Toggle sidebar visibility | View |
| Toggle Dark Mode | Switch between light/dark theme | App |
| Open Settings | Open the settings dialog | App |
| Open Trash | Open the trash dialog | App |
| Data Backup | Open the data backup dialog | App |
| Template Management | Open the template management dialog | Template |
| New Note from Template | Open the template list and create a note from a template | Template |
| Save as Template | Save the current note as a template | Template |
| Lock Application | Lock the application immediately (requires lock screen to be enabled) | App |
| Switch Layout Mode | Cycle through layout modes (Auto/Desktop/Tablet/Mobile) | View |

Commands are displayed grouped by category for quick reference. Shortcut text displayed in the command palette is updated in real time based on user customizations.

---

## 16. System Tray

ENote supports minimizing to the system tray, allowing the application to continue running in the background after the window is closed.

**Basic Behavior:**

- **Close Window:** When clicking the window close button, the window is hidden to the system tray rather than exiting the application. If there are unsaved changes, a confirmation dialog will appear first.
- **Tray Icon:** While the application is running, an ENote icon is displayed in the system notification area (taskbar tray area).
- **Click Tray Icon:** Single-click the tray icon to quickly toggle the window between shown and hidden states.

**Tray Right-Click Menu:**

| Menu Item | Description |
|-----------|-------------|
| Show Window | Show and focus the main application window |
| Exit | Completely exit the application |

> **Note:** The application is only truly closed through the "Exit" option in the tray menu or by using system methods to force-quit. Simply closing the window only hides it to the tray; background functions such as automatic backup continue to run normally.

---

## 17. Keyboard Shortcuts

ENote provides the following keyboard shortcuts to improve operational efficiency:

| Shortcut | Function | Customizable |
|----------|----------|-------------|
| Ctrl/Cmd + S | Save the current note | No (System) |
| Esc | Cancel editing / Close command palette | No (System) |
| Ctrl/Cmd + N | Create a new note | Yes |
| Ctrl/Cmd + E | Enter editing mode | Yes |
| Ctrl/Cmd + B | Expand/Collapse sidebar | Yes |
| Ctrl/Cmd + P | Open command palette | Yes |
| Ctrl/Cmd + L | Lock the application (requires lock screen to be enabled) | Yes |
| Ctrl/Cmd + Shift + M | Cycle through layout modes | Yes |
| Tab | Increase indent (in editor) | -- |
| Shift + Tab | Decrease indent (in editor) | -- |
| Arrow Keys Up/Down | Move focus up/down in note list / Select command in command palette | -- |
| Enter/Space | Select the focused item in note list / Execute the command in command palette | -- |

> **Note:** On macOS, the Ctrl key corresponds to the Cmd key. Shortcuts marked as "Customizable" can be remapped in Settings. See [14.4 Shortcut Settings](#144-shortcut-settings).

---

## 18. Status Bar Information

The status bar at the bottom of the editor displays the following real-time information when a note is open:

- **Cursor Position (left):** Displays the current cursor's line number and column number, in the format "Line X, Column Y".
- **Selected Character Count (left):** When text is selected, additionally displays the number of selected characters.
- **Total Character Count (right):** Displays the total character count of the current note's content.
- **Total Word Count (right):** Displays the total word count of the current note's content.

---

## 19. Application Startup and Initialization

### 19.1 Command-Line Parameters

ENote supports specifying the configuration file path through command-line parameters, useful for development debugging or multi-environment deployment scenarios:

```bash
# Launch with a custom configuration file (legacy mode)
enote --config /path/to/application.yml
enote -c /path/to/application.yml
```

- Both absolute and relative paths are supported (relative paths are resolved based on the current working directory).
- When using the `--config` parameter, the Profile mode is skipped and the specified configuration file is used directly.
- When the `--config` parameter is not specified, the application enters Profile mode (see below).

### 19.2 First Launch (Setup Wizard)

On the first launch of ENote, the system detects that no Profile configuration exists and automatically displays the **Setup Wizard** to guide the user through database connection configuration.

The Setup Wizard includes the following steps:

**Step 1: Select Database Type**

- **SQLite (Recommended):** Local file database, no additional service installation required, ready to use out of the box. Suitable for personal use.
- **MySQL:** Network database, suitable for team collaboration or remote access. Requires pre-installation of MySQL 5.7+.
- **PostgreSQL:** Feature-rich enterprise database. Requires pre-installation of PostgreSQL 12+.

**Step 2: Configure Connection**

Fill in the corresponding connection parameters based on the selected database type:

- **Profile Name:** Give this configuration an identifiable name (e.g., "Local Notes", "Work Database").
- **SQLite:** Select the database file storage path. Supports "New" (choose a save location to create a new database) and "Open Existing" (select an existing .db file). Leave empty to use the default location (application data directory).
- **MySQL / PostgreSQL:** Enter host address, port, and database name, then select the authentication method:
  - **Password Login:** Enter username and password. The password is securely stored in the operating system keychain, not saved in the configuration file.
  - **Certificate Login:** Enter username, select the SSL mode (Preferred/Required/Verify CA/Verify Identity), and specify the CA certificate, client certificate, and client key file paths.

You can click the "Test Connection" button to verify the configuration.

**Step 3: Security Settings**

- **Content Encryption:** Toggle to enable note content encryption. When enabled:
  - The system automatically generates an AES-256 encryption key, securely stored in the operating system keychain (macOS Keychain / Windows Credential Store / Linux Secret Service).
  - All note content is automatically encrypted before writing to the database and automatically decrypted when reading, completely transparent to the user.
  - Even if the database file is accessed directly, note content cannot be read.
  - **Note:** With encryption enabled, note titles remain searchable, but content cannot be retrieved through full-text search.

**Step 4: Save and Connect**

After confirming the configuration, the system automatically saves the configuration file and keys, connects to the database, performs initialization, then restarts the application to enter the main interface.

> **Language Switching:** The Setup Wizard provides a language switch button in the upper right corner. Click to toggle between Chinese and English, convenient for non-Chinese users on first use.

> **Window Close:** On first launch, the Setup Wizard is a mandatory step and does not provide a skip or cancel button. To exit, simply close the window (click the title bar close button), and the application will exit immediately.

### 19.3 Multi-Profile Selection

When multiple Profile configurations exist, a **Profile Selection Page** is displayed after startup:

- All configurations are displayed as cards showing name, database type, connection info, encryption status, etc.
- The last used configuration is marked with a "Last Used" label.
- Double-click or select and click "Connect" to start with that configuration.
- Click "New Profile" to enter the Setup Wizard and create a new database configuration.
- Supports editing and deleting existing configurations.
- Check "Auto-connect to last used profile next time" to skip the selection page and directly use the last configuration on next startup.

When only one configuration exists, or when auto-connect is enabled, the selection page is skipped and the application starts directly.

> **Close Behavior:** The profile selection page on first launch does not have a close button (a configuration must be selected). The profile selection page accessed from the main interface via Settings provides a close button to return to the main interface and continue using the current configuration.

### 19.4 Startup Error Handling

If an error occurs during the startup process, the system displays a native operating system error dialog with specific error information:

- **Configuration File Loading Failed:** Indicates a configuration file reading error. Please check whether the configuration file format is correct.
- **Database Connection Failed:** Indicates a database connection error. Please check the database configuration or whether the database service is available. On connection failure, the system automatically falls back to the Setup Wizard, allowing the user to reconfigure.
- **Database Migration Failed:** Indicates a table structure update error.

Error messages support internationalization in Chinese and English, automatically displaying in the corresponding language based on the system locale.

### 19.5 Subsequent Launches

On non-first launches, the system automatically detects and performs database structure upgrades (if there are table structure changes in a new version) to ensure data compatibility. The entire process is transparent to the user and requires no manual intervention. During startup, the system also automatically checks automatic backup settings. If enabled and the time since the last backup exceeds the configured interval, a backup is automatically performed.

---

## 20. MCP (AI Tool Integration)

### 20.1 Feature Overview

MCP (Model Context Protocol) is an open protocol proposed by Anthropic that allows AI assistants to interact with external tools and data sources. ENote includes a built-in MCP Server that enables AI clients such as Claude Desktop, Claude Code, and Cursor to directly read and write note data.

Through MCP integration, AI assistants can:

- Search and read note content to obtain knowledge context
- Create new notes based on conversations to automatically organize information
- Update existing notes to supplement or modify content
- Manage notebooks and tags to organize knowledge structures

All write operations performed through MCP are recorded in the note history and marked as "AI Tool" source, ensuring auditability and rollback capability.

### 20.2 Configuration and Activation

Enabling MCP functionality requires two steps:

**Step 1: Enable in Configuration File**

Enable MCP in the `application.yml` configuration file:

```yaml
# MCP Server Configuration
mcp:
  enabled: true
```

**Step 2: Build MCP Server**

```bash
cd src-tauri
cargo build --release --bin enote-mcp
```

After building, the `enote-mcp` executable is generated in the `target/release/` directory.

**Step 3: Configure AI Client**

Using Claude Desktop as an example, add the following to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "enote": {
      "command": "/path/to/enote-mcp",
      "args": ["--config", "/path/to/application.yml"]
    }
  }
}
```

The configuration for Claude Code is similar; configure the MCP Server in `.claude/settings.json`.

**Step 4: Fine-Grained Control in Settings Panel**

In ENote's settings panel (Section 14.5), enable the MCP master switch and turn on/off individual tools as needed.

### 20.3 Available Tools

The MCP Server provides the following 10 tools:

| Tool Name | Description | Type | Access Control |
|-----------|-------------|------|----------------|
| `search_notes` | Search notes with keyword, notebook, and tag filtering; returns paginated results | Read | Automatically filters "Deny" notes |
| `get_note` | Get the full content of a specified note (including HTML, tags, notebook info) | Read | Checks read permission |
| `create_note` | Create a new note; can specify title, content, notebook, and tags | Write | Checks write permission on target notebook |
| `update_note` | Update an existing note's title, content, notebook, or tags | Write | Checks write permission on the note |
| `delete_note` | Delete a note (moves to trash; can be restored in the app) | Write | Checks write permission on the note |
| `list_notebooks` | List all notebooks (includes AI access permission field) | Read | Metadata always visible |
| `create_notebook` | Create a new notebook | Write | -- |
| `list_tags` | List all tags (includes AI access permission field) | Read | Metadata always visible |
| `create_tag` | Create a new tag | Write | -- |
| `note_stats` | Get note statistics (total count and per-notebook counts) | Read | -- |

> **Security Note:** Dangerous operations such as permanent deletion, emptying the trash, and encryption/decryption are not exposed through MCP, preventing AI tools from accidentally causing data loss.

### 20.4 Access Control

ENote's MCP access control uses a three-layer architecture, providing coarse-to-fine access control from global to data level.

#### 20.4.1 Three-Layer Access Control Architecture

```
AI Request -> (1) Global MCP Switch -> (2) Tool-Level Switch -> (3) Data-Level Access Control
              (Settings Panel)         (Settings Panel)         (mcp_access field)
```

| Layer | Scope | Configuration Location |
|-------|-------|----------------------|
| **Layer 1: Global Switch** | Controls whether MCP is enabled; all AI tools are unavailable when off | Settings -> MCP -> Enable MCP |
| **Layer 2: Tool-Level Switch** | Controls whether each MCP tool (e.g., search, create, update) is available | Settings -> MCP -> Individual tool switches |
| **Layer 3: Data-Level Access Control** | Controls AI read/write permissions for specific notebooks/tags/notes | Notebook/Tag edit dialog, Note settings dialog |

The three layers complement each other without conflict. Global and tool-level controls determine "whether you can use this tool", while data-level control determines "whether you can access this data".

Regarding tool-level control:

1. **`list_tools` Dynamic Filtering:** When an AI client calls `list_tools`, the MCP Server reads the list of enabled tools from the settings database and returns only enabled tools. Disabled tools are invisible to the AI.

2. **Secondary Verification on Call:** Before each tool invocation, the MCP Server checks again whether the tool is enabled to prevent bypassing.

Changes to MCP switches in the settings panel take effect immediately -- after a user disables a tool, the AI client will no longer see that tool the next time it calls `list_tools`.

#### 20.4.2 Data-Level Access Control (mcp_access)

Notebooks, tags, and notes each have an "AI Access Permission" (`mcp_access`) field supporting four levels:

| Value | Name | Meaning |
|-------|------|---------|
| Inherit | Inherit | Inherit permission from the parent level (default value, backward compatible) |
| Read-Write | ReadWrite | AI can read and modify |
| Read-Only | ReadOnly | AI can only read, cannot modify or delete |
| Deny | Deny | AI cannot access at all; invisible in search results |

#### 20.4.3 Permission Resolution Rules

For a note, the effective permission is resolved by the following priority from highest to lowest:

1. **Encrypted Note -> Forced Deny:** Encrypted notes are always denied AI access and cannot be overridden.
2. **Note's Own Setting:** If the note's AI access permission is not "Inherit", the note's own setting is used. Note-level settings can **relax** or **tighten** upper-level restrictions.
3. **Tag Settings:** If the note is associated with multiple tags, the **most restrictive** permission among all non-"Inherit" tags is used.
4. **Notebook Settings:** If all of the above are "Inherit", the notebook's setting is used. If both tags and notebook have non-"Inherit" settings, the more restrictive of the two is used.
5. **Default Read-Write:** If all levels are "Inherit", read-write access is allowed by default (the global MCP switch already controls the overall entry point).

**Examples:**

| Notebook Setting | Tag Setting | Note Setting | Effective Permission | Explanation |
|-----------------|-------------|--------------|---------------------|-------------|
| Read-Write | Read-Only | Inherit | **Read-Only** | Inherits tag restriction |
| Deny | Read-Write | Inherit | **Deny** | Notebook denied, takes the more restrictive value |
| Deny | -- | Read-Write | **Read-Write** | Note explicitly overrides, relaxing the upper-level restriction |
| Read-Only | -- | Deny | **Deny** | Note explicitly overrides, tightening the upper-level restriction |
| Read-Write | -- | Read-Write (encrypted note) | **Deny** | Encrypted notes are always denied, cannot be overridden |

**Metadata Visibility Note:** `list_notebooks` and `list_tags` always return all entries (including the `mcpAccess` field). The AI can see notebook and tag names and permission markers but cannot access the content of denied notes.

**Access Control Behavior for Each MCP Tool:**

| Tool | Access Control Behavior |
|------|------------------------|
| `search_notes` | Search results automatically filter out "Deny" notes; "Read-Only" notes are returned normally |
| `get_note` | Checks read permission; "Deny" notes return a rejection message |
| `create_note` | Checks write permission on the target notebook; "Read-Only" or "Deny" notebooks do not allow creation |
| `update_note` | Checks write permission; "Read-Only" or "Deny" notes are rejected for updates |
| `delete_note` | Checks write permission; "Read-Only" or "Deny" notes are rejected for deletion |
| `list_notebooks` | Returns all notebooks (including `mcpAccess` field), no filtering |
| `list_tags` | Returns all tags (including `mcpAccess` field), no filtering |
| `note_stats` | Normal statistics (counts may include denied notes) |

#### 20.4.4 Setting AI Access Permissions

**Notebook Level:**

1. Select the notebook in the sidebar and click Menu -> Edit.
2. Find the "AI Access Permission" dropdown at the bottom of the edit dialog.
3. Select the desired permission level and click Save.
4. All notes in this notebook that do not have individual permission settings will inherit this setting.

**Tag Level:**

1. Select the tag in the sidebar and click Menu -> Edit.
2. Find the "AI Access Permission" dropdown at the bottom of the edit dialog.
3. Select the desired permission level and click Save.
4. All notes with this tag that do not have individual permission settings will be affected by this setting.

**Note Level:**

1. In editing mode, click the "Settings" button (gear icon) in the toolbar.
2. Find the "AI Access Permission" dropdown at the bottom of the note settings dialog.
3. Select the desired permission level and click Save.
4. Note-level explicit settings override notebook and tag settings.

> **Tip:** In the sidebar, notebooks and tags with an AI access permission other than "Inherit" display a shield icon, making it easy to quickly identify which items have AI access control enabled.

### 20.5 Operation Source Tracking

The note history records include an "Operation Source" field that distinguishes the originator of the operation:

| Source | Description |
|--------|-------------|
| **User Operation** | Operations performed through the ENote interface |
| **AI Tool** | Operations initiated by an AI client through the MCP protocol |

In the history table, the operation source is displayed alongside the operation type (Create / Modify / Delete), making it easy for users to identify which modifications were made by AI, facilitating auditing and rollback.

---

## 21. Data Storage and Configuration

### 21.1 Local Data Storage

All note data is stored by default in a local SQLite database. Data is kept entirely on the user's local device without any network transmission, ensuring data privacy and security.

The application configuration file is stored in the operating system's application data directory. When using the default SQLite database, the database file is also located in this directory; when using MySQL or PostgreSQL, data is stored on the corresponding database server, and only the configuration file is kept in this directory. The application data directory paths for different operating systems are as follows:

| Operating System | Application Data Directory | Example |
|-----------------|---------------------------|---------|
| **Windows** | `{FOLDERID_RoamingAppData}\net.easycloudcn.enote\` | `C:\Users\Alice\AppData\Roaming\net.easycloudcn.enote\` |
| **macOS** | `$HOME/Library/Application Support/net.easycloudcn.enote/` | `/Users/Alice/Library/Application Support/net.easycloudcn.enote/` |
| **Linux** | `$XDG_DATA_HOME/net.easycloudcn.enote/` or `$HOME/.local/share/net.easycloudcn.enote/` | `/home/alice/.local/share/net.easycloudcn.enote/` |

> **Quick Access:**
> - **Windows:** Enter `%APPDATA%\net.easycloudcn.enote` in the File Explorer address bar to open it quickly.
> - **macOS:** In Finder, press `Cmd+Shift+G` and enter `~/Library/Application Support/net.easycloudcn.enote` to open it.
> - **Linux:** Run `xdg-open "${XDG_DATA_HOME:-$HOME/.local/share}/net.easycloudcn.enote"` in the terminal to open it.

This directory contains the following files and subdirectories:

| File/Directory | Description |
|----------------|-------------|
| `profiles.yml` | Profile index file, records the active configuration and auto-connect settings |
| `profiles/` | Profile configuration directory, each database configuration is a YAML file |
| `application.yml` | Legacy configuration file (compatibility mode, used with the `--config` parameter) |
| `enote.db` | SQLite database file, present only when using SQLite configuration |
| `enote.db-wal` | SQLite WAL journal file (automatically generated), present only with SQLite |
| `enote.db-shm` | SQLite shared memory file (automatically generated), present only with SQLite |
| `images/` | Local image storage directory, stores image files inserted in the editor |
| `backups/` | Automatic backup directory, stores scheduled backup SQL files |

### 21.2 Configuration File

The application configuration file is `application.yml`, using YAML format, located by default in the application data directory described above. On first launch, the system automatically generates a file with the default SQLite configuration; users do not need to create it manually. You can also specify a custom configuration file path via the `--config` command-line parameter (see [19.1 Command-Line Parameters](#191-command-line-parameters)).

**Default generated configuration file content:**

```yaml
# ENote Application Configuration File
# This file is automatically generated by the application

datasource:
  # SQLite Database Connection URL
  # mode=rwc means read-write mode; creates the file if it doesn't exist
  url: "sqlite:/path/to/net.easycloudcn.enote/enote.db?mode=rwc"

  # Connection Pool Configuration
  # Maximum connections: recommended 5 or fewer for SQLite
  max-connections: 5
  # Minimum connections: maintain resident connections to reduce reconnection overhead
  min-connections: 1
  # Connection timeout (seconds): timeout for establishing a new connection
  connect_timeout: 10
  # Acquire timeout (seconds): timeout for obtaining a connection from the pool
  acquire-timeout: 5
  # Idle timeout (seconds): how long idle connections are kept
  idle-time: 300
  # Maximum lifetime (seconds): maximum lifetime of a connection
  max-lifetime: 1800

# MCP Server Configuration
# When enabled, AI tools (such as Claude Desktop) can operate on notes
mcp:
  enabled: false
```

> **Note:**
> - The actual path in the `url` field of the configuration file will be automatically populated with the full absolute path based on the operating system. The SQLite URL supports using `~` to represent the user's home directory (e.g., `sqlite://~/data/enote.db?mode=rwc`), and the system will automatically expand it to the full path.
> - After modifying the configuration file, you must **restart the application** for the changes to take effect.

### 21.3 Configuration Options

The following are detailed descriptions of all configurable options under the `datasource` node:

| Option | Type | Default | Valid Range | Description |
|--------|------|---------|-------------|-------------|
| `url` | String | (auto-generated) | -- | **Required.** Database connection URL; format depends on the database type (see Section 21.4) |
| `max-connections` | Integer | SQLite: 5, Others: 20 | 1 - 1000 | Maximum number of connections in the pool. Recommended not to exceed 5 for SQLite |
| `min-connections` | Integer | 1 | 0 - max-connections | Minimum number of connections maintained in the pool. Set to 0 to not maintain any resident connections |
| `connect_timeout` | Integer | 10 | 1 - 300 | Timeout for establishing a new database connection (seconds) |
| `acquire-timeout` | Integer | 5 | 1 - 300 | Timeout for acquiring an available connection from the pool (seconds) |
| `idle-time` | Integer | 300 | 1 - 86400 | Idle connection retention time (seconds). Connections unused beyond this time will be recycled |
| `max-lifetime` | Integer | 1800 | 1 - 86400 | Maximum connection lifetime (seconds). Connections exceeding this time will be forcibly closed and rebuilt |

The following are configuration options under the `mcp` node:

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enabled` | Boolean | false | MCP Server master switch. The MCP Server only accepts connections when set to true |

### 21.4 Database Configuration Examples

ENote supports three databases, switchable by modifying `datasource.url`. In addition to the default SQLite, MySQL and PostgreSQL are also supported to meet advanced users' data management needs.

**SQLite (Default):**

```yaml
datasource:
  url: "sqlite:/path/to/enote.db?mode=rwc"
  max-connections: 5
  min-connections: 1
```

- `mode=rwc`: Read-write mode; the database file is automatically created if it doesn't exist.
- SQLite is a single-file database that requires no additional service installation and is suitable for personal use.

**MySQL:**

```yaml
datasource:
  url: "mysql://username:password@server-address:3306/database-name"
  max-connections: 20
  min-connections: 2
  connect_timeout: 10
  acquire-timeout: 5
  idle-time: 300
  max-lifetime: 1800
```

- Requires pre-installation of MySQL 5.7+ or MariaDB 10.3+ and creation of the target database.
- Example: `url: "mysql://root:password@127.0.0.1:3306/enote"`

**PostgreSQL:**

```yaml
datasource:
  url: "postgres://username:password@server-address:5432/database-name"
  max-connections: 20
  min-connections: 2
  connect_timeout: 10
  acquire-timeout: 5
  idle-time: 300
  max-lifetime: 1800
```

- Requires pre-installation of PostgreSQL 12+ and creation of the target database.
- Example: `url: "postgres://enote:password@127.0.0.1:5432/enote"`

> **Note:** After switching database types, data from the original database will not be automatically migrated. If you need to preserve data, please use the export function to back up your notes before switching, then import them afterwards. When connecting to a new database for the first time, the system will automatically create the required table structures.

### 21.5 Profile Configuration Management

ENote uses a Profile system to manage multiple database configurations. Profile files are stored in the `profiles/` subdirectory of the application data directory.

**Index file `profiles.yml`:**

```yaml
active: local-sqlite    # Currently active Profile ID
auto-connect: false      # Whether to auto-connect to the last used profile
```

**Single Profile configuration example (`profiles/local-sqlite.yml`):**

```yaml
name: Local Notes
icon: ''
datasource:
  type: sqlite
  path: /path/to/enote.db
security:
  content-encryption: false
```

**MySQL Profile example (`profiles/work-mysql.yml`):**

```yaml
name: Work Database
icon: ''
datasource:
  type: mysql
  host: 192.168.1.100
  port: 3306
  database: enote
  username: enote_user
  auth-method: password
  ssl:
    mode: ''
security:
  content-encryption: true
```

> **Note:** Database passwords and encryption keys are not stored in configuration files; they are securely stored in the operating system keychain.

### 21.6 Content Encryption and Key Management

When content encryption is enabled, the system uses the AES-256-GCM algorithm to encrypt note content.

**Key Storage:**

Encryption keys are stored in the operating system's native secure storage:

| Operating System | Storage Location | Management |
|-----------------|-----------------|------------|
| **macOS** | Keychain | Viewable via "Keychain Access" app |
| **Windows** | Credential Store | Viewable via "Control Panel > Credential Manager" |
| **Linux** | Secret Service (e.g., GNOME Keyring) | Viewable via `seahorse` or similar tools |

Key entries use the application identifier `net.easycloudcn.enote` as the service name, with the format `{profile_id}.encryption_key`.

**Encryption Behavior:**

- Note content is automatically encrypted before saving to the database and automatically decrypted when reading.
- Encryption is completely transparent to the user and requires no manual operation.
- Note titles are **not encrypted** and remain searchable.
- Note content is stored as ciphertext in the database; even direct access to the database file cannot reveal the original content.
- Content in note history records is also protected by encryption.

### 21.7 SSL/TLS Certificate Authentication

When using MySQL or PostgreSQL, SSL/TLS certificates can be used for secure authentication to protect data transmission.

**MySQL SSL Modes:**

| Mode | Description |
|------|-------------|
| `disabled` | Do not use SSL |
| `preferred` | Prefer SSL, fall back if unavailable |
| `required` | Must use SSL |
| `verify-ca` | Verify server CA certificate |
| `verify-identity` | Verify server identity (strictest) |

**PostgreSQL SSL Modes:**

| Mode | Description |
|------|-------------|
| `disabled` | Do not use SSL |
| `preferred` | Prefer SSL |
| `required` | Must use SSL |
| `verify-ca` | Verify server CA certificate |
| `verify-full` | Full verification (strictest) |

**Required Certificate Files (PEM format):**

- **CA Certificate:** Server CA root certificate (`.pem` or `.crt`)
- **Client Certificate:** Client identity certificate (`.pem` or `.crt`)
- **Client Key:** Client private key file (`.pem` or `.key`)

Certificate file paths are specified via file selectors in the Setup Wizard and saved in the Profile configuration file.

### 21.8 History Version Protection

The system automatically records the history version of every modification and deletion operation on notes, including the complete content before and after the operation, supporting tracking and comparison at any time and effectively preventing accidental data loss. History records contain the following information:

- Operation type (Create, Modify, Delete)
- Operation source (User operation, AI Tool)
- Old content before the operation
- New content after the operation
- Time of the operation
- Associated notebook and tag information

---

## 22. Help System

ENote includes a built-in help system that provides searchable user documentation.

### 22.1 Opening the Help Manual

**How to Access:**

- **Settings Dialog:** Click "Settings" → In the "Help Manual" section, click the "View Manual" button.
- **New Window:** Click the "Open in New Window" button to view the manual in a separate window.

### 22.2 Manual Features

**Search:**
- The search bar at the top supports real-time search. Matching text is highlighted in yellow.
- Non-matching content is automatically hidden during search and restored when the search is cleared.

**Table of Contents Navigation:**
- The left panel displays a three-level table of contents (H1/H2/H3) with hierarchical indentation.
- Click any entry to smoothly scroll to the corresponding section.
- The currently visible section is highlighted in the table of contents.

**Content Display:**
- The right panel displays rendered Markdown content, supporting tables, code blocks, images, and other formatting.
- A "Back to Top" button appears after scrolling more than 300 pixels.

**Multilingual Support:**
- Manual content automatically loads the version matching the current language setting (Chinese/English).
- Manual content refreshes automatically when the language is switched.

---

## 23. Application Logs

ENote provides comprehensive log management with two types: database operation logs and file system logs, facilitating troubleshooting and operation auditing.

### 23.1 Opening Log Management

In the "Log Management" section of the Settings dialog, click "Open Log Manager" to open the log management dialog. The dialog contains two tabs: "Database Logs" and "File Logs".

### 23.2 Database Logs

Database logs record various operations within the application, including note CRUD, tag and notebook operations, backup/sync, encryption operations, and more.

**Log Fields:**

| Field | Description |
|-------|-------------|
| Time | When the operation occurred |
| Level | DEBUG, INFO, WARN, ERROR |
| Module | Module of the operation (notebook, note, tag, backup, sync, settings, encrypt, system, frontend) |
| Action | Specific action performed |
| Message | Operation description |

**Filtering and Search:**
- **Level Filter:** Filter by log level (All/INFO/WARN/ERROR).
- **Module Filter:** Filter by module.
- **Keyword Search:** Fuzzy search within message content.

**Management Actions:**
- **Delete Single:** Click the delete button on the right side of a log entry to remove it.
- **Clear All:** Remove all database logs.
- **Cleanup Old Logs:** Delete logs older than 30 days.

Log levels are color-coded: ERROR (red), WARN (yellow), INFO (blue), DEBUG (gray). The list supports paginated browsing with 50 items per page.

### 23.3 File Logs

File logs record system-level application runtime logs, stored in the `logs/` subdirectory of the application data directory.

**File List:**
- The left panel lists all log files (`enote*.log`), showing file size and modification time.
- Error log files are highlighted in red.

**Log Viewing:**
- After selecting a file, the right panel displays the file content.
- Supports searching within file content, with matching text highlighted in yellow.
- ERROR-level lines are automatically highlighted in red.

**Management Actions:**
- **Delete File:** Delete the selected log file.
- **Cleanup Old Files:** Delete log files older than 30 days.
- **Refresh:** Reload the file list.

### 23.4 Frontend Log Level

In the "Log Management" section of the Settings dialog, you can set the frontend log level:

| Level | Description |
|-------|-------------|
| DEBUG | Record all debug information |
| INFO | Record general information (default) |
| WARN | Record warnings and errors only |
| ERROR | Record errors only |
| NONE | Do not record frontend logs |

### 23.5 Sensitive Data Protection

The logging system automatically sanitizes sensitive information:

- **Password-related fields:** Content containing keywords like "password", "key", "secret", "token" is automatically masked (first 2 characters + **** + last 2 characters).
- **Encrypted note content:** Content of encrypted notes is not recorded in logs.

---

## 24. Auto Update

ENote supports in-app automatic updates, distributing new versions via GitHub Releases. Update packages are verified with digital signatures to ensure security.

### 24.1 Automatic Check on Startup

The application automatically checks for new versions in the background after startup:

- Silent check 3 seconds after launch, without affecting normal usage
- No notification if already on the latest version
- An update dialog appears if a new version is found
- Check results are recorded in the activity log (module: `updater`)

### 24.2 Manual Update Check

In addition to automatic checking, you can manually trigger an update check:

- **Menu Bar**: Click Help → Check for Updates
- **Command Palette**: Press `Ctrl+P` (macOS: `Cmd+P`), search for "Check for Updates"

When checking manually:
- If already up to date, a "You are up to date" notification appears
- If the check fails (e.g., network issues), an error notification appears

### 24.3 Download and Install

When a new version is found, the update dialog displays:

- Current version number and new version number
- Release Notes
- **Update Now** and **Later** options

After clicking "Update Now":

1. A download progress bar is displayed
2. The update is installed automatically after download
3. A restart prompt appears after installation
4. The application restarts automatically and loads the new version

> **Note**: Do not close the application during the download. If the download fails, you can try again later.

---

## Appendix A: Changelog

| Version | Date | Changes |
|---------|------|---------|
| V1.1.0 | March 2026 | New Features, Performance Optimization, and UX Enhancement |
| | | **New Features:** |
| | | - Note Sorting: Sort by title, creation time, or update time with ascending/descending toggle |
| | | - Batch Operations: Multi-select notes for batch move to notebook and batch delete |
| | | - Note Starring: Star/favorite notes for quick identification |
| | | - File Attachments: Attach files to notes with local storage, drag-and-drop upload, and system file opener |
| | | - Notebook Hierarchy: Nested notebook tree with expand/collapse, parent notebook selection in edit dialog |
| | | - Code Block Language Selector: Quick language dropdown for syntax highlighting (25+ languages) |
| | | - PDF Export: Export notes as PDF via system print dialog |
| | | - HTML Export: Standalone HTML files with embedded styles, responsive design, and dark mode support |
| | | - Print Support: Direct print from toolbar and command palette |
| | | - Editor Font Size Setting: Configurable font size (12-20px) in appearance settings |
| | | - Profile Editing: Edit existing database profile configurations without recreating |
| | | **Performance Optimization:** |
| | | - Trash Empty: Single transaction for batch deletion with individual history preservation |
| | | - Settings Cache Write-Through: Merge updates into cache instead of invalidating |
| | | - Reorder Transactions: Notebook/tag reorder operations wrapped in single transaction |
| | | - Sync History Dedup: Skip duplicate history generation during cross-profile sync |
| | | - History Cache: LRU cache (30s TTL) for note history pagination |
| | | - Auto Backup Hot Reload: Backup timer resets when interval settings change |
| | | **UX Enhancement:** |
| | | - Editor Error Boundary: Graceful error recovery instead of white screen on editor crash |
| | | - Title/Content Separator: Visual divider between note title and content area |
| | | - Editor Focus: Auto-focus to title start (new note) or content start (existing note) on edit |
| | | - Sidebar Layout: Notebooks and tags each occupy 50% height with independent scrolling |
| | | - Floating Drawer Panel: Attachments and linked notes in right-side slide-in drawer with shadow overlay |
| | | - SettingsDialog Split: Refactored into 5 modular sub-components |
| | | **Security:** |
| | | - Decryption Failure Protection: Returns placeholder text instead of leaking ciphertext |
| | | - DB URL Sanitization: Database connection string passwords automatically masked in logs |
| | | - Sanitization Tests: 8 new unit tests for log sanitization functions |
| | | **i18n:** |
| | | - Complete i18n audit: Fixed hardcoded strings in toolbar, tree items, and help components |
| | | - Added missing tooltips on all icon-only buttons |
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

*This manual is based on ENote Intelligent Note Management System V1.1.0. Please refer to the actual software for any feature updates.*
