use crate::modules::contexts::config::entities::ConfigFsTemplate;
use crate::modules::contexts::project::domain::entities::{ProjectPackage, ProjectTemplate};
use crate::modules::contexts::settings::domain::entities::Settings;
use crate::modules::shared::kernel::errors::{ConfigError, ParsingError};
use crate::modules::shared::kernel::values::Path;
use serde::de::DeserializeOwned;
use serde::Serialize;
pub trait TConfigService {
    fn read_settings(&self) -> Result<Settings, ConfigError>;
    fn save_settings(&self, settings: &Settings) -> Result<(), ConfigError>;
    fn get_data_dir(&self) -> Result<Path, ConfigError>;
    fn make_data_dir(&self) -> Result<(), ConfigError>;

    fn read_packages(&self) -> Result<Vec<ProjectPackage>, ConfigError>;
    fn read_templates(&self) -> Result<Vec<ProjectTemplate>, ConfigError>;

    fn get_home_dir(&self) -> Result<Path, ConfigError>;
    fn make_projects_dir(&self) -> Result<(), ConfigError>;
    fn get_projects_dir(&self) -> Result<Path, ConfigError>;

    fn get_settings(&self) -> Result<Settings, ConfigError>;

    fn get_file_templates(&self) -> Result<Vec<ConfigFsTemplate>, ConfigError>;
}

pub trait TConfigRecoveryService {
    fn check_data_dir(&self) -> Result<(), ConfigError>;
    fn repair_data_dir(&self) -> Result<(), ConfigError>;

    fn add_settings_by_default(&self) -> Result<(), ConfigError>;
    fn add_recents_by_default(&self) -> Result<(), ConfigError>;
}

pub trait TParsingService {
    fn to_string<T: Serialize>(&self, obj: T) -> Result<String, ParsingError>;

    fn from_string<T: DeserializeOwned>(&self, obj: String) -> Result<T, ParsingError>;
}
