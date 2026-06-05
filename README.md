<table>
  <tr>
    <td width="130" align="center" valign="middle">
      <img src="assets/sync.svg" alt="DeskSync Logo" width="120" height="120" />
    </td>
    <td valign="middle">
      <h1>DeskSync</h1>
      <p><strong>A tiny utility that automatically refreshes your Windows Desktop when new files appear.</strong></p>
      <p>
        <a href="#-quick-start">Quick Start</a> • 
        <a href="#-how-to-stop">How to Stop</a>
      </p>
    </td>
  </tr>
</table>

## 📌 The Problem

On Windows (including Windows 10 and 11), when you save a file, download a document, or drag something onto your Desktop, the icons often do not appear immediately. You have to manually click the Desktop and press `F5` to refresh it.

**DeskSync** runs silently in the background, watches your Desktop, and automatically refreshes the screen the instant any files are added, renamed, or deleted.

## 🚀 Quick Start

1. **Download**: Get `DeskSync.exe` from [Releases](https://github.com/Noktomezo/DeskSync/releases).
2. **Run**: Double-click the file. It runs silently in the background (visible only in Task Manager).
3. **Autostart**: Press `Win + R`, type `shell:startup`, and place a shortcut to `DeskSync.exe` there to run it on startup.

## 🛑 How to Stop

If you ever need to close the utility:
1. Open the Windows **Task Manager** (press `Ctrl + Shift + Esc`).
2. Look for **DeskSync** (or `DeskSync.exe`) in the list of background processes.
3. Select it and click **End Task**.

<div align="center">
  <img src="./assets/heartbeat.svg" alt="heartbeat" width="600px">
  <p>Made with 💜. Published under <a href="LICENSE">MIT License</a>.</p>
</div>
