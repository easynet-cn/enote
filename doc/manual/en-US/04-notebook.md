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
   - **AI Access Permission:** Set the access level for AI tools accessing notes in this notebook via the MCP protocol (optional). Available values include "Inherit" (default), "Read-Write", "Read-Only", and "Deny". See [20.4 Access Control](20-mcp.md#204-access-control) for details.
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
