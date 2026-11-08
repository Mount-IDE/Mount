use crate::services::config::ConfigService;
use crate::services::settings::SettingsService;

pub mod domain;
pub mod errors;
pub mod services;
pub mod traits;

pub const CONFIG_SERVICE: ConfigService = ConfigService;
pub const SETTINGS_SERVICE: SettingsService = SettingsService;
