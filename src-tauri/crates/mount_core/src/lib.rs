use std::sync::OnceLock;
use tauri::AppHandle;

pub mod domain;
pub mod traits;

pub static APP: OnceLock<AppHandle> = OnceLock::new();
