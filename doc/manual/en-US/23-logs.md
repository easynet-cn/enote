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
