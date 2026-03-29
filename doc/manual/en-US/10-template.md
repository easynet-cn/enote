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
