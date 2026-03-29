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
