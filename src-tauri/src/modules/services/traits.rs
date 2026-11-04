use crate::modules::contexts::config::entities::{ConfigFsEntity, ConfigFsTemplate};
use crate::modules::contexts::project::domain::entities::{Package, ProjectTemplate, Var};
use crate::modules::contexts::settings::domain::entities::Settings;
use crate::modules::shared::kernel::errors::{
    ConfigError, FileSystemError, FormatError, ParsingError,
};
use crate::modules::shared::kernel::values::{Path, Val};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
pub trait TConfigService {
    fn read_settings(&self) -> Result<Settings, ConfigError>;

    #[allow(unused)]
    fn save_settings(&self, settings: &Settings) -> Result<(), ConfigError>;
    fn get_data_dir(&self) -> Result<Path, ConfigError>;

    #[allow(unused)]
    fn make_data_dir(&self) -> Result<(), ConfigError>;

    #[allow(unused)]
    fn read_packages(&self) -> Result<Vec<Package>, ConfigError>;
    #[allow(unused)]
    fn read_templates(&self) -> Result<Vec<ProjectTemplate>, ConfigError>;

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

pub trait TConfigRecoveryService {
    fn check_data_dir(&self) -> Result<(), ConfigError>;
    fn repair_data_dir(&self) -> Result<(), ConfigError>;
    #[allow(unused)]
    fn add_settings_by_default(&self) -> Result<(), ConfigError>;

    #[allow(unused)]
    fn add_recents_by_default(&self) -> Result<(), ConfigError>;
}

pub trait TParsingService {
    fn to_string<T: Serialize>(&self, obj: T) -> Result<String, ParsingError>;

    fn _from_string<T: DeserializeOwned>(&self, obj: &String) -> Result<T, ParsingError>;
}

pub trait TFormatService {
    fn format_string(
        &self,
        string: String,
        vars: &Vec<Var>,
        results: &HashMap<String, Val>,
    ) -> Result<String, FormatError>;
}

#[allow(dead_code, unused)]
pub trait TCreateFsService {
    fn create(&self, path: &Path, config: ConfigFsTemplate) -> Result<(), FileSystemError>;

    fn create_entity(&self, path: &Path, entity: ConfigFsEntity) -> Result<(), FileSystemError>;
}
