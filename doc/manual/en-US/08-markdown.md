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
