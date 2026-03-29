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
- **AI Access Permission:** Set the access level for AI tools accessing this note via the MCP protocol. Available values include "Inherit" (default, inherited from tags and notebooks), "Read-Write", "Read-Only", and "Deny". Note-level settings have the highest priority and can override notebook and tag settings. See [20.4 Access Control](20-mcp.md#204-access-control) for details.

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
