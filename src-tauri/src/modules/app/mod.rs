pub mod cmd;
pub mod commands;
pub mod macroses;
pub mod stores;
pub mod utils;
use super::contexts::filesystem::app::services::{
    FileSystemReadService, FileSystemWatchService, FileSystemWriteService,
};
use crate::modules::contexts::events::services::EventService;
use crate::modules::contexts::launch::app::services::{LaunchCompileService, LaunchRunService};
use crate::modules::contexts::package::services::LspService;
use crate::modules::contexts::project::app::services::{
    ActionProjectService, /*PackageCompileService,*/ PackageService, ProjectService,
};
use crate::modules::contexts::settings::app::services::SettingsService;
use crate::modules::contexts::settings::domain::entities::Settings;
use crate::modules::contexts::terminal::app::services::TerminalService;
use crate::modules::contexts::toolchains::services::{
    ToolchainComponentService, ToolchainMakeService, ToolchainPackageService,
};
use crate::modules::services::config::{ConfigRecoveryService, ConfigService, ParsingService};
use std::sync::{Mutex, OnceLock};
use tauri::AppHandle;

pub const FS_READ_SERVICE: FileSystemReadService = FileSystemReadService {};
pub const FS_WRITE_SERVICE: FileSystemWriteService = FileSystemWriteService {};
pub const FS_WATCH_SERVICE: FileSystemWatchService = FileSystemWatchService {};
pub const PROJECT_SERVICE: ProjectService = ProjectService {};
pub const CONFIG_SERVICE: ConfigService = ConfigService();
pub const CONFIG_RECOVERY_SERVICE: ConfigRecoveryService = ConfigRecoveryService();

pub static APP: OnceLock<AppHandle> = OnceLock::new();

pub static SETTINGS: OnceLock<Mutex<Settings>> = OnceLock::new();

pub const TERMINAL_SERVICE: TerminalService = TerminalService;
pub const ACTION_PROJECT_SERVICE: ActionProjectService = ActionProjectService();

pub const PARSING_SERVICE: ParsingService = ParsingService();
pub const LAUNCH_COMPILE_SERVICE: LaunchCompileService = LaunchCompileService();

pub const LAUNCH_RUN_SERVICE: LaunchRunService = LaunchRunService();

pub const SETTINGS_SERVICE: SettingsService = SettingsService();

pub const EVENT_SERVICE: EventService = EventService();

pub const PACKAGE_SERVICE: PackageService = PackageService();
//pub static PACKAGE_COMPILE_SERVICE: PackageCompileService = PackageCompileService();

pub const LSP_SERVICE: LspService = LspService();

pub const TOOLCHAIN_MAKE_SERVICE: ToolchainMakeService = ToolchainMakeService();
pub const TOOLCHAIN_COMPONENT_SERVICE: ToolchainComponentService = ToolchainComponentService();
pub const TOOLCHAIN_PACKAGE_SERVICE: ToolchainPackageService = ToolchainPackageService();
