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

### 14.6 Cloud Backup Settings

In the "Cloud Backup" area of the settings dialog, you can configure automatic uploading of backup files to cloud storage.

**Basic Configuration:**

| Option | Description |
|--------|-------------|
| **Enable Cloud Backup** | Toggle switch to enable or disable cloud backup |
| **Storage Provider** | Dropdown: Alibaba Cloud OSS, AWS S3, Tencent Cloud COS, MinIO, WebDAV |
| **Cloud Retention Count** | Dropdown to select the maximum number of cloud backups to retain: 5, 10, 20, 50 |

**Connection Configuration (Object Storage: S3 / OSS / COS / MinIO):**

| Option | Description |
|--------|-------------|
| **Endpoint** | Cloud storage service endpoint URL, e.g., `https://oss-cn-hangzhou.aliyuncs.com` |
| **Bucket** | Bucket name |
| **Region** | Storage region, e.g., `cn-hangzhou`, `us-east-1` |
| **Access Key ID** | Access key ID |
| **Secret Access Key** | Access key secret (stored in system Keychain) |
| **Path Prefix** | Cloud storage path prefix, default `enote-backups/` |

**Connection Configuration (WebDAV):**

| Option | Description |
|--------|-------------|
| **Endpoint** | WebDAV service URL, e.g., `https://dav.jianguoyun.com/dav/` |
| **Username** | WebDAV login username |
| **Password** | WebDAV login password (stored in system Keychain) |
| **Path Prefix** | Cloud storage path prefix, default `enote-backups/` |

**Action Buttons:**

| Button | Description |
|--------|-------------|
| **Test Connection** | Verify cloud storage configuration. A notification shows success or failure |
| **Save** | Save the current cloud backup configuration |
| **Backup to Cloud** | Immediately perform a local backup and upload it to the cloud |

**Cloud Backup List:** Click the "Cloud Backups" link to expand and view all cloud backup files, including file name, size, and modification time. Click "Download" to download a file to the local machine.

> **Security Note:** Secret Access Keys and WebDAV passwords are stored using the operating system's native secure storage (Keychain) and are never saved as plaintext in the database. It is recommended to create sub-account credentials with only read/write permissions for cloud storage, following the principle of least privilege.

### 14.7 Security Settings (Lock Screen)

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

### 14.8 MCP Settings

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
> In addition to global and tool-level control, ENote also supports more fine-grained **data-level access control** -- you can set AI access permissions (Inherit / Read-Write / Read-Only / Deny) separately on notebooks, tags, and individual notes. See [20.4 Access Control](20-mcp.md#204-access-control) for details.

### 14.9 Profile Management

In the Settings dialog, the **Profile Management** section provides the following:

- **Current Profile Info:** Displays the name and database type of the currently active profile.
- **Switch Profile:** Click the "Switch Profile" button to return to the profile selection page. From there you can:
  - Select an existing database configuration and connect
  - Click "New Profile" to enter the Setup Wizard and create a new database configuration
  - Edit or delete existing configurations
  - Click the close button in the upper right to close the selection page and return to the main interface

### 14.10 System Maintenance (Cross-Profile Sync)

At the bottom of the Settings dialog, the "System Maintenance" section provides cross-profile data synchronization and sync history management.

#### 14.10.1 Cross-Profile Sync

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

#### 14.10.2 Sync History Management

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
