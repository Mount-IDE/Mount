pub mod cache;
pub mod config;
pub mod dialog;
pub mod fs;
pub mod launch;
pub mod package;
pub mod project;
pub mod terminal;
pub mod utils;

#[macro_export]
macro_rules! get_data_dir {
    () => {{
        let mut res = ::config::traits::TConfigService::get_data_dir(&::config::CONFIG_SERVICE);
        if let Err(_) = res {
            ::recovery::traits::TConfigRecoveryService::repair_data_dir(
                &::recovery::CONFIG_RECOVERY_SERVICE,
            )?;
            res = ::config::traits::TConfigService::get_data_dir(&::config::CONFIG_SERVICE);
            if let Err(e) = res {
                return Err(e.into());
            }
        }
        res.unwrap()
    }};
}
