use crate::domain::settings::Settings;
use crate::errors::{ConfigError, SettingsError};
use filesystem::domain::{ConfigFsEntity, ConfigFsTemplate, FileSystemError};
use mount_core::domain::Path;

pub trait TSettingsService {
    fn read_themes(&self) -> Result<Vec<String>, SettingsError>;
}

pub trait TConfigService {
    fn read_settings(&self) -> Result<Settings, ConfigError>;

    #[allow(unused)]
    fn save_settings(&self, settings: &Settings) -> Result<(), ConfigError>;
    fn get_data_dir(&self) -> Result<Path, ConfigError>;

    #[allow(unused)]
    fn make_data_dir(&self) -> Result<(), ConfigError>;

    fn get_home_dir(&self) -> Result<Path, ConfigError>;
    fn make_projects_dir(&self) -> Result<(), ConfigError>;
    fn get_projects_dir(&self) -> Result<Path, ConfigError>;

    fn get_settings(&self) -> Result<Settings, ConfigError>;

    fn get_file_templates(&self) -> Result<Vec<ConfigFsTemplate>, ConfigError>;

    #[allow(unused)]
    fn get_licenses(&self) -> Result<Vec<String>, ConfigError>; // only names

    #[allow(unused)]
    fn get_license(&self, name: String) -> Result<String, ConfigError>;

    #[allow(dead_code, unused)]
    fn get_shell(&self) -> String;
}

#[allow(dead_code, unused)]
pub trait TCreateFsService {
    fn create(&self, path: &Path, config: ConfigFsTemplate) -> Result<(), FileSystemError>;

    fn create_entity(&self, path: &Path, entity: ConfigFsEntity) -> Result<(), FileSystemError>;
}
