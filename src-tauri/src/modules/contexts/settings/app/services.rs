use crate::modules::app::{CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, FS_READ_SERVICE};
use crate::modules::contexts::filesystem::app::traits::TFSReadService;
use crate::modules::contexts::filesystem::app::utils::path_from;
use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::settings::app::traits::TSettingsService;
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService};
use crate::modules::shared::kernel::errors::SettingsError;
use crate::modules::shared::kernel::values::Path;

pub struct SettingsService();

impl TSettingsService for SettingsService {
    fn read_themes(&self) -> Result<Vec<String>, SettingsError> {
        let dir = CONFIG_SERVICE.get_data_dir();
        if let Err(_) = dir {
            CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
        }
        let dir = dir.unwrap();
        let themes = path_from![dir.get(), "themes"];
        let dir_ = PDirectory::from_path(&themes);
        let content = FS_READ_SERVICE.read_dir(&dir_)?;

        let mut res: Vec<String> = vec![];

        for i in content.directories {
            let config = path_from![i.path, "theme.json"];
            if !FS_READ_SERVICE.exists(config.clone()) {
                continue;
            }
            let file = PFile::from_path_reg(config);
            let text = FS_READ_SERVICE.read_file(&file)?;
            res.push(text);
        }
        Ok(res)
    }
}
