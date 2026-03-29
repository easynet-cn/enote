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
