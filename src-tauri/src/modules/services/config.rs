use crate::modules::app::{APP, CONFIG_SERVICE, FS_READ_SERVICE, FS_WRITE_SERVICE};
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::make_path;
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::FileType;
use crate::modules::contexts::settings::domain::entities::Settings;
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService};
use crate::modules::shared::kernel::entities::ConfigError;
use crate::modules::shared::kernel::values::Path;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

pub struct ConfigService();
impl TConfigService for ConfigService {
    fn read_settings(&self) -> Result<Settings, ConfigError> {
        let dir = self.get_data_dir();
        if dir.is_err() {
            return Err(ConfigError::empty());
        }
        let dir = dir?;
        let path_to_settings = make_path(vec![dir.get().as_str(), "settings.json"]);
        let ext = fs::exists(path_to_settings.clone());
        if ext.is_err() {
            return Err(ConfigError::empty());
        }
        if !ext.unwrap() {
            return Err(ConfigError::empty()); // need launch repair
        }
        let file = PFile {
            name: "settings.json".to_string(),
            path: Path(path_to_settings),
            typ: FileType::REGULAR,
        };
        let file = FS_READ_SERVICE.read_file(&file);
        if file.is_err() {
            return Err(ConfigError::empty());
        }
        let file = file.unwrap();
        let settings = serde_json::from_str::<Settings>(file.as_str());
        if settings.is_err() {
            return Err(ConfigError::empty());
        }

        Ok(settings.unwrap())
    }

    fn save_settings(&self, settings: &Settings) -> Result<(), ConfigError> {
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
                Err(ConfigError::empty())
            }
        }
    }

    fn make_data_dir(&self) -> Result<(), ConfigError> {
        let dir = APP.get().unwrap().path().app_data_dir();
        if dir.is_err() {
            return Err(ConfigError::empty());
        }
        let dir = dir.unwrap().to_str().unwrap().to_owned();
        let ext = fs::exists(dir.clone());
        if ext.is_err() {
            return Err(ConfigError::empty());
        }
        if !ext.unwrap() {
            let res = FS_WRITE_SERVICE.create_dir(&Path(dir));
            if res.is_err() {
                return Err(ConfigError::empty());
            }
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
            let res = self.repair_data_dir();
            if res.is_err() {
                return Err(ConfigError::empty());
            }
            return Ok(());
        }
        let dir = dir.unwrap();
        let settings = make_path(vec![dir.get().as_str(), "settings.json"]);
        let file = PFile::regular("settings.json".to_string(), Path(settings));
        let ext = FS_READ_SERVICE.exist_file(&file);
        if !ext {
            println!("no settings");
            let res = self.repair_data_dir();
            if res.is_err() {
                return Err(ConfigError::empty());
            }
            return Ok(());
        }
        let recent = make_path(vec![dir.get().as_str(), "recent-projects.json"]);
        let file = PFile::regular("recent-projects.json".to_string(), Path(recent));
        let ext = FS_READ_SERVICE.exist_file(&file);
        if !ext {
            println!("no recent");
            let res = self.repair_data_dir();
            if res.is_err() {
                return Err(ConfigError::empty());
            }
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
        let path_to_settings = make_path(vec![dir.clone().as_str(), "settings.json"]);
        let path_to_projects = make_path(vec![dir.clone().as_str(), "recent-projects.json"]);
        let file1 = PFile::from_path_reg(Path(path_to_settings));
        let file2 = PFile::from_path_reg(Path(path_to_projects));
        let ext1 = FS_READ_SERVICE.exist_file(&file1);
        let ext2 = FS_READ_SERVICE.exist_file(&file2);
        if !ext1 {
            println!("settings created");
            let _ = FS_WRITE_SERVICE.create_file(&file1.path);
        }
        if !ext2 {
            println!("recent created");
            let _ = FS_WRITE_SERVICE.create_file(&file2.path);
        }
        Ok(())
    }
}
