pub mod cmd;
pub mod commands;
pub mod utils;

use super::contexts::filesystem::app::services::{
    FileSystemReadService, FileSystemWatchService, FileSystemWriteService,
};
use crate::modules::contexts::launch::app::services::LaunchCompileService;
use crate::modules::contexts::project::app::services::{ActionProjectService, ProjectService};
use crate::modules::contexts::settings::domain::entities::Settings;
use crate::modules::contexts::terminal::app::services::TerminalService;
use crate::modules::services::config::{ConfigRecoveryService, ConfigService, ParsingService};
use crate::modules::services::traits::TParsingService;
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

pub static TERMINAL_SERVICE: TerminalService = TerminalService;
pub static ACTION_PROJECT_SERVICE: ActionProjectService = ActionProjectService();

pub static PARSING_SERVICE: ParsingService = ParsingService();
pub static LAUNCH_COMPILE_SERVICE: LaunchCompileService = LaunchCompileService();
