<p align="center">
  <img src="assets/sync.svg" alt="DeskSync Logo" width="120" height="120" />
</p>

<h1 align="center">DeskSync</h1>

<p align="center">
  <strong>A lightweight, zero-dependency Windows Desktop auto-refresher daemon written in Rust.</strong>
</p>

<p align="center">
  <a href="#-features">Features</a> •
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-how-it-works">How It Works</a> •
  <a href="#-development">Development</a>
</p>

---

## 📌 The Problem

On Windows (including Windows 10 and 11), saving files, downloading documents, or dragging items onto the Desktop directory often doesn't trigger a visual refresh immediately. You are forced to manually click the Desktop and press `F5` to see your files.

**DeskSync** solves this by running silently in the background, monitoring your Desktop directory for any file system events (additions, deletions, renames), and programmatically notifying the Windows Shell to refresh the view instantly.

---

## ✨ Features

- **⚡ Lightweight & Fast**: Built in Rust with optimized release settings and UPX compression. The final executable is only **~156 KB**.
- **🔇 Zero-Window Daemon**: Compiles with `#![windows_subsystem = "windows"]` in release mode, meaning it runs completely silently in the background without any console windows.
- **🧠 Intelligent Debouncing**: Monitors changes using the `notify` crate, but waits for a brief `500ms` silence window before refreshing to avoid repetitive refreshes during large file writes or downloads.
- **🔌 Native Win32 Integration**: Uses pure, zero-overhead FFI to hook into `SHChangeNotify(SHCNE_UPDATEDIR, ...)` and `SHGetSpecialFolderPathW` to locate your Desktop even if it has been relocated (e.g. to OneDrive).

---

## ⚡ Quick Start

### 📥 Download
1. Go to the [Releases](https://github.com/Noktomezo/DeskSync/releases) page.
2. Download the latest compiled `DeskSync.exe` executable.

### 🏃 Running
Double-click `DeskSync.exe`. It will immediately start monitoring in the background (no window will appear). You can verify it is running in your **Task Manager** under the process name `DeskSync.exe`.

### ⚙️ Autostart on Windows Login
To ensure DeskSync runs every time you log in:
1. Press `Win + R`, type `shell:startup` and hit **Enter** (this opens your Startup folder).
2. Right-click inside the folder, select **New -> Shortcut**.
3. Browse to the downloaded `DeskSync.exe` file and click **Finish**.

---

## 🔍 How It Works

Instead of simulating keyboard input or sending violent global updates that reset all Explorer windows, DeskSync calls:
```rust
SHChangeNotify(SHCNE_UPDATEDIR, SHCNF_PATHW, desktop_path, null)
```
This targeting tells Windows Explorer that *only* the Desktop folder needs updating, making the refresh smooth and completely imperceptible.

---

## 🛠️ Development

### 🛠️ Building
You can easily build the project using the [just](https://github.com/casey/just) command runner:

```bash
# Build release-optimized binary with UPX compression
just release
```
This compiles the pure-Rust resource compiler (`build.rs`), rasterizes `assets/sync.svg` into a multi-resolution `.ico` icon, embeds it, and packs the executable to `target/release/DeskSync.exe`.

### 🧪 Local Debugging
```bash
# Run in debug mode (with active console logs)
cargo run
```

---

<div align="center">
  <img src="./assets/heartbeat.svg" alt="heartbeat" width="600px">
  <p>Made with 💜. Published under <a href="LICENSE">MIT License</a>.</p>
</div>
