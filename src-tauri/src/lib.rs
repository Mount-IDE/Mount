use tauri::Manager;
use crate::modules::app::commands::{show_win};
mod modules;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![show_win])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
