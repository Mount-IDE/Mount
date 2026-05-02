use std::fs;
use std::path::Path;
use tauri::{generate_handler};
use crate::modules::app::cmd::{get_projects, show_win};
use crate::modules::app::{APP, CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, SETTINGS};
use crate::modules::app::commands::project::*;
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService};
use crate::modules::app::commands::cache::*;
use crate::modules::app::commands::config::*;
use crate::modules::app::commands::utils::*;
mod modules;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let out_dir = Path::new("../src/types");
    fs::create_dir_all(out_dir).unwrap();
    


    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app|{
            APP.set(app.handle().clone()).expect("no app btw :(");
            CONFIG_RECOVERY_SERVICE.check_data_dir().expect("TODO: panic message");
            let settings = CONFIG_SERVICE.read_settings()?;
            SETTINGS.set(settings).expect("Unable to sert settings");
            Ok(())
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
            make_base64
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
