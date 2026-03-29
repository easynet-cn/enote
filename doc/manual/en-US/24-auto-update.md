## 24. Auto Update

ENote supports in-app automatic updates, distributing new versions via GitHub Releases. Update packages are verified with digital signatures to ensure security.

### 24.1 Automatic Check on Startup

The application automatically checks for new versions in the background after startup:

- Silent check 3 seconds after launch, without affecting normal usage
- No notification if already on the latest version
- An update dialog appears if a new version is found
- Check results are recorded in the activity log (module: `updater`)

### 24.2 Manual Update Check

In addition to automatic checking, you can manually trigger an update check:

- **Menu Bar**: Click Help → Check for Updates
- **Command Palette**: Press `Ctrl+P` (macOS: `Cmd+P`), search for "Check for Updates"

When checking manually:
- If already up to date, a "You are up to date" notification appears
- If the check fails (e.g., network issues), an error notification appears

### 24.3 Download and Install

When a new version is found, the update dialog displays:

- Current version number and new version number
- Release Notes
- **Update Now** and **Later** options

After clicking "Update Now":

1. A download progress bar is displayed
2. The update is installed automatically after download
3. A restart prompt appears after installation
4. The application restarts automatically and loads the new version

> **Note**: Do not close the application during the download. If the download fails, you can try again later.
