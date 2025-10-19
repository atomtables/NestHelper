use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

pub(crate) fn get_sshpass_plink(app: AppHandle) -> Result<String, Box<dyn std::error::Error>> {

    #[cfg(target_os = "linux")]
    let sshpass_path = app.path().resolve("assets/sshpass.linux", BaseDirectory::Resource)
         .expect("Failed to resolve resource path").to_string_lossy().to_string();
    #[cfg(target_os = "macos")]
    let sshpass_path = app.path().resolve("assets/sshpass.darwin", BaseDirectory::Resource)
         .expect("Failed to resolve resource path").to_string_lossy().to_string();
    #[cfg(target_os = "windows")]
    let sshpass_path = app.path().resolve("assets/plink.exe", BaseDirectory::Resource)
         .expect("Failed to resolve resource path").into_os_string().into_string().expect("Failed to convert OsString to String");

    Ok(sshpass_path)
}
