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

**Configuration:** Configure in the "Automatic Backup" area of the "Settings" dialog. See [14.5 Automatic Backup Settings](14-settings.md#145-automatic-backup-settings) for details.

### 13.4 Cloud Backup

ENote supports uploading backup files to cloud storage for off-site disaster recovery. The following cloud storage providers are supported:

| Provider | Description |
|----------|-------------|
| **Alibaba Cloud OSS** | Alibaba Cloud Object Storage Service, fast access within China |
| **AWS S3** | Amazon Web Services object storage, global coverage |
| **Tencent Cloud COS** | Tencent Cloud Object Storage, commonly used in China |
| **MinIO** | S3-compatible self-hosted object storage, ideal for private deployments |
| **WebDAV** | Supports services like Nutstore, NextCloud, and other WebDAV-compatible services |

**How It Works:**

- Cloud backup is based on the local automatic backup SQL files -- a local backup is generated first, then uploaded to the cloud.
- Cloud backup files are stored under the configured path prefix (e.g., `enote-backups/`), with the same file names as local backups.
- The system automatically cleans up old cloud backups based on the configured cloud retention count.
- When cloud backup is enabled, each automatic backup is automatically uploaded to the cloud upon completion; cloud backup failure does not affect the local backup.

**Restoring from Cloud:**

1. In the cloud backup settings area, click "Cloud Backups" to view all cloud backup files.
2. Click the "Download" button next to the target backup file to download it to the local `backups/` directory.
3. Use the "Import & Restore" function in the "Data Backup" dialog to select the downloaded SQL file for restoration.

**Security Notes:**

- Cloud storage Access Key Secrets (or WebDAV passwords) are stored using the operating system's native secure storage (macOS Keychain / Windows Credential Store / Linux Secret Service), not saved as plaintext in the database.
- All cloud transfers are encrypted via HTTPS.
- If note content encryption is enabled, the content in backup files is already in encrypted form.

**Configuration:** Configure in the "Cloud Backup" area of the "Settings" dialog. See [14.6 Cloud Backup Settings](14-settings.md#146-cloud-backup-settings) for details.
