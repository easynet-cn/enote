## 19. Application Startup and Initialization

### 19.1 Command-Line Parameters

ENote supports specifying the configuration file path through command-line parameters, useful for development debugging or multi-environment deployment scenarios:

```bash
# Launch with a custom configuration file (legacy mode)
enote --config /path/to/application.yml
enote -c /path/to/application.yml
```

- Both absolute and relative paths are supported (relative paths are resolved based on the current working directory).
- When using the `--config` parameter, the Profile mode is skipped and the specified configuration file is used directly.
- When the `--config` parameter is not specified, the application enters Profile mode (see below).

### 19.2 First Launch (Setup Wizard)

On the first launch of ENote, the system detects that no Profile configuration exists and automatically displays the **Setup Wizard** to guide the user through database connection configuration.

The Setup Wizard includes the following steps:

**Step 1: Select Backend Type**

- **SQLite (Recommended):** Local file database, no additional service installation required, ready to use out of the box. Suitable for personal use.
- **MySQL:** Network database, suitable for team collaboration or remote access. Requires pre-installation of MySQL 5.7+.
- **PostgreSQL:** Feature-rich enterprise database. Requires pre-installation of PostgreSQL 12+.
- **Git Repository:** Notes and todos are stored as plain-text Markdown+YAML files in a Git repository, with automatic commits forming a complete version history. Files can be opened in any editor and inspected with any Git tool. Ideal for users who want full ownership of their data. Requires git installed on the system.
- **ENote Server:** Connect to a remote ENote-compatible API server. Suitable for self-hosted services or enterprise deployments.

**Step 2: Configure Connection**

Fill in the corresponding connection parameters based on the selected backend type:

- **Profile Name:** Give this configuration an identifiable name (e.g., "Local Notes", "Work Database").
- **SQLite:** Select the database file storage path. Supports "New" (choose a save location to create a new database) and "Open Existing" (select an existing .db file). Leave empty to use the default location (application data directory).
- **Git Repository:** Choose a local repository directory (automatically created and initialized with git init if it does not exist). Optionally provide a remote repository URL to sync with GitHub/GitLab/Gitee or a self-hosted Git server:
  - **Leave empty:** Local-only use, no remote connection.
  - **HTTPS auth:** Enter a username and a Personal Access Token (a token is recommended over your account password). The token is securely stored in the operating system keychain, never written to the configuration file.
  - **SSH auth:** Uses your system SSH keys (e.g., `~/.ssh/id_ed25519`) and ssh-agent; the app never stores private keys.
  - **Sync behavior:** If the local directory is empty, the remote repository is cloned automatically. With an existing local repository, remote updates are pulled (rebase) and local commits pushed automatically. Every subsequent note save is pushed to the remote. When the remote is unreachable (offline, auth failure), local features are fully unaffected and pushing retries on the next commit.
  - Click "Test Connection" to verify the remote URL and credentials.
- **MySQL / PostgreSQL:** Enter host address, port, and database name, then select the authentication method:
  - **Password Login:** Enter username and password. The password is securely stored in the operating system keychain, not saved in the configuration file.
  - **Certificate Login:** Enter username, select the SSL mode (Preferred/Required/Verify CA/Verify Identity), and specify the CA certificate, client certificate, and client key file paths.
- **ENote Server:** Enter the server URL and select the authentication method:
  - **None:** No authentication required (for internal networks or testing only).
  - **Bearer Token:** Authenticate using an API Key or Token. The token is securely stored in the operating system keychain.
  - **Basic (Username/Password):** HTTP Basic authentication. The password is securely stored in the keychain.
  - **JWT (Auto-refresh):** JSON Web Token authentication with automatic token refresh. Requires username, password, and token refresh URL.
  - **Custom Header:** Authenticate using a custom HTTP header (e.g., X-API-Key), suitable for enterprise API gateways. The header value is stored in the keychain.
  - **OAuth 2.0:** Authenticate using the OAuth 2.0 protocol. Requires Token URL, Client ID, Client Secret, and scopes.

You can click the "Test Connection" button to verify the configuration.

**Step 3: Security Settings**

- **Database backends:** Toggle to enable note content encryption. When enabled:
  - The system automatically generates an AES-256 encryption key, securely stored in the operating system keychain (macOS Keychain / Windows Credential Store / Linux Secret Service).
  - All note content is automatically encrypted before writing to the database and automatically decrypted when reading, completely transparent to the user.
  - Even if the database file is accessed directly, note content cannot be read.
  - **Note:** With encryption enabled, note titles remain searchable, but content cannot be retrieved through full-text search.
- **ENote Server backend:** Content encryption is handled by the server; no client-side configuration is needed.

**Step 4: Save and Connect**

After confirming the configuration, the system automatically saves the configuration file and keys, connects to the database, performs initialization, then restarts the application to enter the main interface.

> **Language Switching:** The Setup Wizard provides a language switch button in the upper right corner. Click to toggle between Chinese and English, convenient for non-Chinese users on first use.

> **Window Close:** On first launch, the Setup Wizard is a mandatory step and does not provide a skip or cancel button. To exit, simply close the window (click the title bar close button), and the application will exit immediately.

### 19.3 Multi-Profile Selection

When multiple Profile configurations exist, a **Profile Selection Page** is displayed after startup:

- All configurations are displayed as cards showing name, database type, connection info, encryption status, etc.
- The last used configuration is marked with a "Last Used" label.
- Double-click or select and click "Connect" to start with that configuration.
- Click "New Profile" to enter the Setup Wizard and create a new database configuration.
- Supports editing and deleting existing configurations.
- Check "Auto-connect to last used profile next time" to skip the selection page and directly use the last configuration on next startup.

When only one configuration exists, or when auto-connect is enabled, the selection page is skipped and the application starts directly.

> **Close Behavior:** The profile selection page on first launch does not have a close button (a configuration must be selected). The profile selection page accessed from the main interface via Settings provides a close button to return to the main interface and continue using the current configuration.

> **Switch Anytime:** Besides startup, you can open the profile selection page at any time from the main interface via **Help menu → Switch Note Library**, the shortcut **Ctrl/Cmd + Shift + P**, or **Settings → Profile Management → Switch Profile**.

### 19.4 Startup Error Handling

If an error occurs during the startup process, the system displays a native operating system error dialog with specific error information:

- **Configuration File Loading Failed:** Indicates a configuration file reading error. Please check whether the configuration file format is correct.
- **Database Connection Failed:** Indicates a database connection error. Please check the database configuration or whether the database service is available. On connection failure, the system automatically falls back to the Setup Wizard, allowing the user to reconfigure.
- **Database Migration Failed:** Indicates a table structure update error.

Error messages support internationalization in Chinese and English, automatically displaying in the corresponding language based on the system locale.

### 19.5 Subsequent Launches

On non-first launches, the system automatically detects and performs database structure upgrades (if there are table structure changes in a new version) to ensure data compatibility. The entire process is transparent to the user and requires no manual intervention. During startup, the system also automatically checks automatic backup settings. If enabled and the time since the last backup exceeds the configured interval, a backup is automatically performed.
