<p align="center">
  <img src="assets/sync.svg" alt="DeskSync Logo" width="120" height="120" />
</p>

<h1 align="center">DeskSync</h1>

<p align="center">
  <strong>A lightweight, zero-dependency Windows Desktop auto-refresher daemon written in Rust.</strong>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#how-it-works">How It Works</a> •
  <a href="#configuration">CI/CD & Just</a>
</p>

---

## The Problem

On Windows (including Windows 10 and 11), saving files, downloading documents, or dragging items onto the Desktop directory often doesn't trigger a visual refresh immediately. You are forced to manually click the Desktop and press `F5` to see your files.

**DeskSync** solves this by running silently in the background, monitoring your Desktop directory for any file system events (additions, deletions, renames), and programmatically notifying the Windows Shell to refresh the view instantly.

---

## Features

- **⚡ Lightweight & Fast**: Built in Rust with optimized release settings and UPX compression. The final executable is only **~112 KB**.
- **🔇 Zero-Window Daemon**: Compiles with `#![windows_subsystem = "windows"]` in release mode, meaning it runs completely silently in the background without any console windows.
- **🧠 Intelligent Debouncing**: Monitors changes using the `notify` crate, but waits for a brief `500ms` silence window before refreshing to avoid repetitive refreshes during large file writes or downloads.
- **🔌 Native Win32 Integration**: Uses pure, zero-overhead FFI to hook into `SHChangeNotify(SHCNE_UPDATEDIR, ...)` and `SHGetSpecialFolderPathW` to locate your Desktop even if it has been relocated (e.g. to OneDrive).

---

## Quick Start

### Building
You can easily build the project using the [just](https://github.com/casey/just) command runner:

```bash
# Build release-optimized binary with UPX compression
just release
```
This produces `target/release/DeskSync.exe` embedded with the custom application icon.

### Running
Double-click `target/release/DeskSync.exe` or run it from the console. It will immediately start monitoring in the background.

### Autostart on Windows Login
To ensure DeskSync runs every time you log in:
1. Press `Win + R`, type `shell:startup` and hit **Enter** (this opens your Startup folder).
2. Right-click in the directory, select **New -> Shortcut**.
3. Browse to the built `DeskSync.exe` file and click **Finish**.

---

## How It Works

Instead of simulating keyboard input or sending violent global updates that reset all Explorer windows, DeskSync calls:
```rust
SHChangeNotify(SHCNE_UPDATEDIR, SHCNF_PATHW, desktop_path, null)
```
This targeting tells Windows Explorer that *only* the Desktop folder needs updating, making the refresh smooth and completely imperceptible.

<div align="center">
  <img src="./assets/heartbeat.svg" alt="heartbeat" width="600px">
  <p>Made with 💜. Published under <a href="LICENSE">MIT License</a>.</p>
</div>
