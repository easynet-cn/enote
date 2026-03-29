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
