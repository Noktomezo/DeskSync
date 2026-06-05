<p align="center">
  <img src="assets/sync.svg" alt="DeskSync Logo" width="120" height="120" />
</p>

<h1 align="center">DeskSync</h1>

<p align="center">
  <strong>A tiny utility that automatically refreshes your Windows Desktop when new files appear.</strong>
</p>

<p align="center">
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-features">Features</a> •
  <a href="#-how-to-stop">How to Stop</a>
</p>

## 📌 The Problem

On Windows (including Windows 10 and 11), when you save a file, download a document, or drag something onto your Desktop, the icons often do not appear immediately. You have to manually click the Desktop and press `F5` to refresh it.

**DeskSync** runs silently in the background, watches your Desktop, and automatically refreshes the screen the instant any files are added, renamed, or deleted.

## 🚀 Quick Start

1. **Download**: Get `DeskSync.exe` from [Releases](https://github.com/Noktomezo/DeskSync/releases).
2. **Run**: Double-click the file. It runs silently in the background (visible only in Task Manager).
3. **Autostart**: Press `Win + R`, type `shell:startup`, and place a shortcut to `DeskSync.exe` there to run it on startup.

## ✨ Features

- 👻 **Completely Invisible**: Runs silently in the background without cluttering your screen, taskbar, or system tray.
- 🪶 **Zero Resource Usage**: Built to be extremely lightweight; uses virtually 0% CPU and a negligible amount of RAM.
- 🧠 **Smart Updates**: Intelligently waits for files to finish downloading or copying before refreshing, avoiding screen flickering.
- 📁 **OneDrive Compatible**: Works perfectly even if your Desktop folder is synced to OneDrive or moved to another drive.

## 🛑 How to Stop

If you ever need to close the utility:
1. Open the Windows **Task Manager** (press `Ctrl + Shift + Esc`).
2. Look for **DeskSync** (or `DeskSync.exe`) in the list of background processes.
3. Select it and click **End Task**.

<div align="center">
  <img src="./assets/heartbeat.svg" alt="heartbeat" width="600px">
  <p>Made with 💜. Published under <a href="LICENSE">MIT License</a>.</p>
</div>
