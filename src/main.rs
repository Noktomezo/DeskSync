#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use notify_debouncer_mini::{DebounceEventResult, new_debouncer};
use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

// Win32 Constants
const CSIDL_DESKTOPDIRECTORY: i32 = 0x0010;
const SHCNE_UPDATEDIR: i32 = 0x0000_1000;
const SHCNF_PATHW: u32 = 0x0005;

// Win32 FFI declarations
unsafe extern "system" {
  fn SHGetSpecialFolderPathW(
    hwnd: *mut std::ffi::c_void,
    pszPath: *mut u16,
    csidl: i32,
    fCreate: i32,
  ) -> i32;

  fn SHChangeNotify(
    wEventId: i32,
    uFlags: u32,
    dwItem1: *const u16,
    dwItem2: *const u16,
  );
}

fn get_desktop_path() -> Option<PathBuf> {
  let mut buf = vec![0u16; 260];
  let result = unsafe {
    SHGetSpecialFolderPathW(
      std::ptr::null_mut(),
      buf.as_mut_ptr(),
      CSIDL_DESKTOPDIRECTORY,
      0,
    )
  };
  if result != 0 {
    let len = buf.iter().position(|&x| x == 0).unwrap_or(buf.len());
    let os_str = OsString::from_wide(&buf[..len]);
    Some(PathBuf::from(os_str))
  } else {
    None
  }
}

fn refresh_desktop(path: &Path) {
  let mut path_w: Vec<u16> = path.as_os_str().encode_wide().collect();
  path_w.push(0); // null terminator
  unsafe {
    SHChangeNotify(
      SHCNE_UPDATEDIR,
      SHCNF_PATHW,
      path_w.as_ptr(),
      std::ptr::null(),
    );
  }
}

fn main() {
  let desktop_path = match get_desktop_path() {
    Some(path) => path,
    None => {
      eprintln!("Failed to locate Desktop directory.");
      std::process::exit(1);
    }
  };

  println!("Monitoring Desktop: {}", desktop_path.display());

  // Create a channel to receive events
  let (tx, rx) = channel::<DebounceEventResult>();

  // Initialize debouncer with 500ms delay
  let mut debouncer = match new_debouncer(Duration::from_millis(500), tx) {
    Ok(d) => d,
    Err(e) => {
      eprintln!("Failed to initialize watcher: {:?}", e);
      std::process::exit(1);
    }
  };

  // Watch the desktop path (NonRecursive since we only care about elements directly on Desktop)
  if let Err(e) = debouncer
    .watcher()
    .watch(&desktop_path, notify::RecursiveMode::NonRecursive)
  {
    eprintln!("Failed to watch Desktop path: {:?}", e);
    std::process::exit(1);
  }

  // Event loop
  for result in rx {
    match result {
      Ok(events) => {
        if !events.is_empty() {
          println!("Change detected on Desktop! Refreshing Desktop view...");
          refresh_desktop(&desktop_path);
        }
      }
      Err(err) => {
        eprintln!("Watcher error: {:?}", err);
      }
    }
  }
}
