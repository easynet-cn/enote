## 16. System Tray

ENote supports minimizing to the system tray, allowing the application to continue running in the background after the window is closed.

**Basic Behavior:**

- **Close Window:** When clicking the window close button, the window is hidden to the system tray rather than exiting the application. If there are unsaved changes, a confirmation dialog will appear first.
- **Tray Icon:** While the application is running, an ENote icon is displayed in the system notification area (taskbar tray area).
- **Click Tray Icon:** Single-click the tray icon to quickly toggle the window between shown and hidden states.

**Tray Right-Click Menu:**

| Menu Item | Description |
|-----------|-------------|
| Show Window | Show and focus the main application window |
| Exit | Completely exit the application |

> **Note:** The application is only truly closed through the "Exit" option in the tray menu or by using system methods to force-quit. Simply closing the window only hides it to the tray; background functions such as automatic backup continue to run normally.
