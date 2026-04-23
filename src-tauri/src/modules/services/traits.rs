use crate::modules::contexts::settings::domain::entities::Settings;
use crate::modules::shared::kernel::errors::ConfigError;
use crate::modules::shared::kernel::values::Path;
pub trait TConfigService {
    fn read_settings(&self)->Result<Settings, ConfigError>;
    fn save_settings(&self, settings: &Settings)->Result<(), ConfigError>;
    
    fn get_data_dir(&self)->Result<Path, ConfigError>;
    
    fn make_data_dir(&self)->Result<(), ConfigError>;
}

pub trait TConfigRecoveryService {
    fn check_data_dir(&self) -> Result<(), ConfigError>;
    fn repair_data_dir(&self) -> Result<(), ConfigError>;
}
