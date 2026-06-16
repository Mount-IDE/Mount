use crate::modules::app::cmd::{get_projects, show_win};
use crate::modules::app::commands::cache::*;
use crate::modules::app::commands::config::*;
use crate::modules::app::commands::fs::*;
use crate::modules::app::commands::project::*;
use crate::modules::app::commands::terminal::*;
use crate::modules::app::commands::utils::*;
use crate::modules::app::{APP, CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, SETTINGS};
use crate::modules::contexts::filesystem::app::managers::FileSystemWatchManager;
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService};

use crate::modules::contexts::terminal::app::managers::TerminalManager;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::{generate_handler, Manager, WindowEvent};

mod modules;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let out_dir = Path::new("../src/types");
    fs::create_dir_all(out_dir).unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            APP.set(app.handle().clone()).expect("no app btw :(");
            CONFIG_RECOVERY_SERVICE
                .check_data_dir()
                .expect("TODO: panic message");
            let settings = CONFIG_SERVICE.read_settings()?;
            SETTINGS.set(settings).expect("Unable to sert settings");
            Ok(())
        })
        .on_window_event(|window, event| {
            if matches!(
                event,
                WindowEvent::CloseRequested { .. } | WindowEvent::Destroyed
            ) {
                let state = window.state::<Arc<Mutex<TerminalManager>>>();
                let sessions = {
                    let mut manager = state.lock().unwrap();
                    manager.remove_window_terminals(window.label())
                };

                for session in sessions {
                    session.join();
                }
            }
        })
        .invoke_handler(generate_handler![
            show_win,
            get_projects,
            get_recent_projects,
            read_recent_projects,
            read_templates,
            read_packages,
            get_home_dir,
            get_projects_dir,
            get_groups,
            create_project,
            make_path_command,
            read_project,
            get_data_dir,
            make_path_from_icon,
            make_base64,
            read_dir_rec,
            get_fs_ext_icons,
            read_file,
            watch_project,
            unwatch_project,
            get_file_templates,
            get_os,
            create_file,
            create_dir,
            write_file,
            remove_file,
            remove_dir,
            rename_file,
            get_cache,
            open_terminal,
            write_terminal,
            resize_terminal,
            close_terminal,
            close_window_terminals,
            remove_project,
            save_project
        ])
        .manage(Arc::new(Mutex::new(FileSystemWatchManager::new())))
        .manage(Arc::new(Mutex::new(TerminalManager::new())))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
