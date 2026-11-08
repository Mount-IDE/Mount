use crate::CONFIG_SERVICE;
use crate::errors::SettingsError;
use crate::traits::{TConfigService, TSettingsService};
use filesystem::FS_READ_SERVICE;
use filesystem::domain::{PDirectory, PFile};
use filesystem::traits::TFSReadService;
use mount_core::make_path;

pub struct SettingsService;

impl TSettingsService for SettingsService {
    fn read_themes(&self) -> Result<Vec<String>, SettingsError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        /*if let Err(e) = dir {
            //CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
            return Err(SettingsError::Config())
        }*/
        // let dir = dir.unwrap();
        let themes = make_path![dir, "themes"];
        let dir_ = PDirectory::from_path(&themes);
        let content = FS_READ_SERVICE.read_dir(&dir_)?;

        let mut res: Vec<String> = vec![];

        for i in content.directories {
            let config = make_path![i.path, "theme.json"];
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
