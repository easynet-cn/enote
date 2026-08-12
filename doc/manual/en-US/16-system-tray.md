## 16. System Tray

ENote supports minimizing to the system tray, allowing the application to continue running in the background after the window is closed. The system tray also integrates eye care screen saver quick controls.

### 16.1 Basic Behavior

- **Close Window:** When clicking the window close button, the window is hidden to the system tray rather than exiting the application. If there are unsaved changes, a confirmation dialog will appear first.
- **Tray Icon:** While the application is running, an ENote icon is displayed in the system notification area (taskbar tray area).
- **Click Tray Icon:** Single-click the tray icon to quickly toggle the window between shown and hidden states.

### 16.2 Tray Menu

Right-click the tray icon to show the following menu:

| Menu Item | Description |
|-----------|-------------|
| Pause Eye Care | Pause the idle countdown (shows "Resume Eye Care" when paused, click to resume) |
| Reset Countdown | Reset the idle countdown to the initial value and restart timing |
| Stop Eye Care | Completely stop the eye care screen saver (must re-enable in settings to resume) |
| Show Window | Show and focus the main application window |
| Quit | Completely exit the application |

> **Note:** The application is only truly closed through the "Quit" option in the tray menu or by using system methods to force-quit. Simply closing the window only hides it to the tray; background functions such as automatic backup and eye care screen saver continue to run normally.

### 16.3 Countdown Display

When the eye care screen saver is enabled, the system tray displays real-time countdown information:

**macOS menu bar:**

- Remaining time is displayed directly next to the tray icon (using the system title feature)
- Under 1 hour: `MM:SS` format (e.g., `59:30`)
- 1 hour or above: `H:MM:SS` format (e.g., `1:00:00`)

**All platforms (mouse hover tooltip):**

- Idle countdown: `ENote - MM:SS` (e.g., `ENote - 59:30`)
- Paused: `ENote - Paused`
- Screen saver active (with duration): `ENote - Resting MM:SS`
- Screen saver active (unlimited): `ENote - Resting`
- Disabled: `ENote`

**Time format examples:**

| Remaining Time | Menu Bar | Tooltip |
|---------------|----------|---------|
| 90 minutes (5400s) | `1:30:00` | `ENote - 1:30:00` |
| 60 minutes (3600s) | `59:59` | `ENote - 59:59` |
| 5 minutes (300s) | `05:00` | `ENote - 05:00` |
| Paused | `Paused` | `ENote - Paused` |
| Resting (3 min remaining) | `03:00` | `ENote - Resting 03:00` |

> **Tip:** The countdown updates every second. To adjust the idle trigger time or screen saver duration, configure in the "Screen Saver" area of the settings dialog, see [14.13 Screen Saver Settings](14-settings.md#1413-screen-saver-settings).
