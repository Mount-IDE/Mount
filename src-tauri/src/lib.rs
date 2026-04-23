use tauri::{generate_handler};
use crate::modules::app::cmd::{get_projects, show_win};
use crate::modules::app::{APP, CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, SETTINGS};
use crate::modules::app::commands::project::{get_recent_projects, read_recent_projects};
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService};

mod modules;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
            read_recent_projects
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
