use crate::modules::app::{APP, CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, FS_READ_SERVICE, FS_WRITE_SERVICE};
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::{make_path_string, make_path};
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::{FileType, FileWriteAccess};
use crate::modules::contexts::settings::domain::entities::{RecentProject, Settings};
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService};
use crate::modules::shared::kernel::errors::{ConfigError, ParsingError};
use crate::modules::shared::kernel::values::Path;
use tauri::Manager;

pub struct ConfigService();


#[allow(unused_variables)]
impl TConfigService for ConfigService {
    fn read_settings(&self) -> Result<Settings, ConfigError> {
        let dir = self.get_data_dir()?;
        let path_to_settings = make_path_string(vec![dir.get().as_str(), "settings.json"]);
        let file = PFile::from_path_reg(Path::new(&path_to_settings));
        let ext = FS_READ_SERVICE.exist_file(&file);
        if !ext {
            CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
        }
        let file_ = PFile {
            name: "settings.json".to_string(),
            path: Path(path_to_settings),
            typ: FileType::REGULAR,
        };
        let file = FS_READ_SERVICE
            .read_file(&file_)
            .map_err(|e| ConfigError::SettingsNotFound { err: e })?;
        let settings = serde_json::from_str::<Settings>(file.as_str()).map_err(|e|
            ParsingError::Deserialize {path: file_.path, json: file, err:e}
        )?;
        Ok(settings)
    }

    fn save_settings(&self, _settings: &Settings) -> Result<(), ConfigError> {
        todo!()
    }

    fn get_data_dir(&self) -> Result<Path, ConfigError> {
        let dir = APP.get().unwrap().path().app_data_dir();

        match dir {
            Ok(dir) => {
                println!("{:?}", dir);
                Ok(Path::new(dir.to_str().unwrap()))
            }
            Err(e) => {
                println!("{:?}", e);
                Err(ConfigError::GetDataDir{ err:e })
            }
        }
    }

    fn make_data_dir(&self) -> Result<(), ConfigError> {
        let dir = APP.get().unwrap().path().app_data_dir();
        if dir.is_err() {
            return Err(ConfigError::GetDataDir{ err:dir.unwrap_err() });
        }
        let dir = dir.unwrap().to_str().unwrap().to_owned();
        let dir_ = PDirectory::from_path(&Path(dir.clone()));
        let ext =FS_READ_SERVICE.exist_dir(&dir_);
        if ext {
            FS_WRITE_SERVICE.create_dir(&Path(dir)).map_err(
                |e|
                  ConfigError::MakeDataDir {err:e}
            )?;
        }
        Ok(())
    }
}

pub struct ConfigRecoveryService();

impl TConfigRecoveryService for ConfigRecoveryService {
    fn check_data_dir(&self) -> Result<(), ConfigError> {
        let dir = CONFIG_SERVICE.get_data_dir();
        if dir.is_err() {
            println!("no dir");
            self.repair_data_dir()?;
            return Ok(());
        }
        let dir = dir.unwrap();
        let settings = make_path_string(vec![dir.get().as_str(), "settings.json"]);
        let file = PFile::regular("settings.json".to_string(), Path(settings));
        let ext = FS_READ_SERVICE.exist_file(&file);
        if !ext {
            println!("no settings");
            self.repair_data_dir()?;
            return Ok(());
        }
        let recent = make_path_string(vec![dir.get().as_str(), "recent-projects.json"]);
        let file = PFile::regular("recent-projects.json".to_string(), Path(recent));
        let ext = FS_READ_SERVICE.exist_file(&file);
        if !ext {
            println!("no recent");
            self.repair_data_dir()?;
            return Ok(());
        }
        Ok(())
    }

    fn repair_data_dir(&self) -> Result<(), ConfigError> {
        let dir = CONFIG_SERVICE.get_data_dir().unwrap();
        let path_ = FS_READ_SERVICE.exist_dir(&PDirectory::from_path(&dir));
        if !path_ {
            let data = APP.get().unwrap().path().app_data_dir().unwrap();
            let _ = FS_WRITE_SERVICE.create_dir(&Path(data.to_str().unwrap().to_string()));
        }
        let dir = dir.get();
        println!("dir {dir}");
        let path_to_settings = make_path_string(vec![dir.clone().as_str(), "settings.json"]);
        let path_to_projects = make_path_string(vec![dir.clone().as_str(), "recent-projects.json"]);
        let file1 = PFile::from_path_reg(Path(path_to_settings));
        let file2 = PFile::from_path_reg(Path(path_to_projects));
        let ext1 = FS_READ_SERVICE.exist_file(&file1);
        let ext2 = FS_READ_SERVICE.exist_file(&file2);
        if !ext1 {
            println!("settings created");
            self.add_settings_by_default()?;

        }
        if !ext2 {
            println!("recent created");
            self.add_recents_by_default()?;
        }
        Ok(())
    }

    fn add_settings_by_default(&self) -> Result<(), ConfigError> {
        let settings = Settings::new();
        let dir= CONFIG_SERVICE.get_data_dir()?;
        let path = make_path(vec![dir.get().as_str(), "settings.json"]);
        let file = PFile::from_path_reg(path.clone());
        FS_WRITE_SERVICE.create_file(&path.clone())?;
        let json = serde_json::to_string(&settings).map_err(|e|
            ParsingError::Serialize {path, err:e}
        )?;
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
        Ok(())
    }

    fn add_recents_by_default(&self) -> Result<(), ConfigError> {
        let dir= CONFIG_SERVICE.get_data_dir()?;
        let path = make_path(vec![dir.get().as_str(), "recent-projects.json"]);
        let file = PFile::from_path_reg(path.clone());
        let json = String::from("[ ]");

        FS_WRITE_SERVICE.create_file(&path.clone())?;
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
        Ok(())
    }
}
