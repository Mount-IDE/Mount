pub mod cmd;
pub mod commands;
pub mod utils;

use super::contexts::filesystem::app::services::{
    FileSystemReadService, FileSystemWatchService, FileSystemWriteService,
};
use crate::modules::contexts::project::app::services::ProjectService;
use crate::modules::contexts::settings::domain::entities::Settings;
use crate::modules::services::config::{ConfigRecoveryService, ConfigService};
use std::sync::OnceLock;
use tauri::AppHandle;

pub const FS_READ_SERVICE: FileSystemReadService = FileSystemReadService {};
pub const FS_WRITE_SERVICE: FileSystemWriteService = FileSystemWriteService {};
pub const FS_WATCH_SERVICE: FileSystemWatchService = FileSystemWatchService {};
pub const PROJECT_SERVICE: ProjectService = ProjectService {};
pub const CONFIG_SERVICE: ConfigService = ConfigService();
pub const CONFIG_RECOVERY_SERVICE: ConfigRecoveryService = ConfigRecoveryService();

pub static APP: OnceLock<AppHandle> = OnceLock::new();

pub static SETTINGS: OnceLock<Settings> = OnceLock::new();
