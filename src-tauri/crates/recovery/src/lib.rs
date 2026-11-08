use crate::services::ConfigRecoveryService;

pub mod domain;
pub mod services;
pub mod traits;

pub const CONFIG_RECOVERY_SERVICE: ConfigRecoveryService = ConfigRecoveryService;
