use config::errors::ConfigError;

pub trait TConfigRecoveryService {
    fn check_data_dir(&self) -> Result<(), ConfigError>;
    fn repair_data_dir(&self) -> Result<(), ConfigError>;
    #[allow(unused)]
    fn add_settings_by_default(&self) -> Result<(), ConfigError>;

    #[allow(unused)]
    fn add_recents_by_default(&self) -> Result<(), ConfigError>;
}
