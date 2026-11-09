use crate::domain::SETTINGS;
use crate::domain::settings::Settings;
use crate::errors::ConfigError;
use crate::traits::TConfigService;
use filesystem::domain::{ConfigFsTemplate, PDirectory, PFile};
use filesystem::traits::{TFSReadService, TFSWriteService};
use filesystem::values::FileWriteAccess;
use filesystem::{FS_READ_SERVICE, FS_WRITE_SERVICE};
use mount_core::domain::Path;
use mount_core::{APP, make_path};
use parsing::PARSING_SERVICE;
use parsing::traits::TParsingService;
use std::path::PathBuf;
use std::string::ToString;
use std::sync::Mutex;
use tauri::Manager;

pub struct ConfigService;

#[allow(unused_variables)]
impl TConfigService for ConfigService {
    fn read_settings(&self) -> Result<Settings, ConfigError> {
        let dir = self.get_data_dir()?;
        let path_to_settings = make_path![dir, "settings.json"];
        let file = PFile::from_path_reg(path_to_settings.clone());
        let ext = FS_READ_SERVICE.exist_file(&file);
        if !ext {
            //CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
            return Err(ConfigError::SettingsIsEmpty);
        }
        let file_ = PFile::from_path_reg(path_to_settings);
        let file = FS_READ_SERVICE
            .read_file(&file_)
            .map_err(|e| ConfigError::SettingsNotFound { err: e })?;

        let settings = PARSING_SERVICE._from_string::<Settings>(&file);
        if settings.is_err() {
            let dir = self.get_data_dir()?;
            let settings_ = Settings::new();
            let json = PARSING_SERVICE.to_string(&settings_)?;
            let dir = make_path![dir, "settings.json"];
            let file = PFile::from_path_reg(dir.clone());
            FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
            return Ok(settings_);
        }
        let settings = settings.unwrap();
        Ok(settings)
    }

    fn save_settings(&self, _settings: &Settings) -> Result<(), ConfigError> {
        let dir = self.get_data_dir()?;
        let json = PARSING_SERVICE.to_string(_settings)?;
        let path = make_path![dir, "settings.json"];
        let file = PFile::from_path_reg(path.clone());
        FS_WRITE_SERVICE
            .write_file(&file, json, FileWriteAccess::WRITE)
            .map_err(|_| ConfigError::SavingSettings)?;
        Ok(())
    }

    fn get_data_dir(&self) -> Result<Path, ConfigError> {
        let app = APP.get();
        #[cfg(not(test))]
        if let None = app {
            return Err(ConfigError::App);
        }
        #[cfg(not(test))]
        let dir = APP.get().unwrap().path().app_data_dir();
        #[cfg(test)]
        let dir = Ok(PathBuf::new());
        match dir {
            Ok(dir) => {
                //println!("{:?}", dir);
                Ok(Path::new(dir.to_str().unwrap()))
            }
            Err(e) => {
                // println!("{:?}", e);
                Err(ConfigError::GetDataDir { err: e })
            }
        }
    }

    fn make_data_dir(&self) -> Result<(), ConfigError> {
        let dir = APP.get().unwrap().path().app_data_dir();
        if dir.is_err() {
            return Err(ConfigError::GetDataDir {
                err: dir.unwrap_err(),
            });
        }
        let dir = dir.unwrap().to_str().unwrap().to_owned();
        let dir_ = PDirectory::from_path(&Path(dir.clone()));
        let ext = FS_READ_SERVICE.exist_dir(&dir_);
        if !ext {
            FS_WRITE_SERVICE
                .create_dir(&Path(dir))
                .map_err(|e| ConfigError::MakeDataDir { err: e })?;
            // CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
        }
        Ok(())
    }

    /* fn read_packages(&self) -> Result<Vec<Package>, ConfigError> {
            let dir = self.get_data_dir()?;
            // let path_ = path_from![dir, "packages.json"];
            // let file = PFile::from_path_reg(path_.clone());
            //
            // let content = FS_READ_SERVICE.read_file(&file)?;
            // let json = PARSING_SERVICE._from_string::<Vec<Package>>(&content)?;
            // Ok(json)

            let path_to = make_path![dir, "packages"];
            let dir = PDirectory::from_path(&path_to);
            let dir = FS_READ_SERVICE.read_dir(&dir)?;
            let mut packages = Vec::<Package>::new();
            for i in dir.directories {
                let name = i.name;
                let path = make_path![path_to, name, "config.json"];
                let file = PFile::from_path_reg(path);
                let Ok(content) = FS_READ_SERVICE.read_file(&file) else {
                    continue;
                };
                let Ok(parsed) = PARSING_SERVICE._from_string::<Package>(&content) else {
                    continue;
                };
                packages.push(parsed);
            }
            Ok(packages)
        }

        fn read_templates(&self) -> Result<Vec<ProjectTemplate>, ConfigError> {
            let dir = self.get_data_dir()?;
            let path_ = make_path![dir, "templates.json"];
            let file = PFile::from_path_reg(path_.clone());

            let content = FS_READ_SERVICE.read_file(&file)?;

            let json = PARSING_SERVICE._from_string::<Vec<ProjectTemplate>>(&content)?;
            Ok(json)
        }
    */
    fn get_home_dir(&self) -> Result<Path, ConfigError> {
        let path = APP
            .get()
            .unwrap()
            .path()
            .home_dir()
            .map_err(|e| ConfigError::HomeDir { err: e })?;
        let path_ = path.as_path().to_str().unwrap().to_string();
        Ok(Path(path_))
    }
    fn make_projects_dir(&self) -> Result<(), ConfigError> {
        let mut path = APP
            .get()
            .unwrap()
            .path()
            .home_dir()
            .map_err(|e| ConfigError::HomeDir { err: e })?;
        path.push("MountProjects");
        let path_ = path.as_path().to_str().unwrap().to_string();
        FS_WRITE_SERVICE.create_dir(&Path(path_))?;
        Ok(())
    }

    fn get_projects_dir(&self) -> Result<Path, ConfigError> {
        #[cfg(not(test))]
        let mut path = APP
            .get()
            .unwrap()
            .path()
            .home_dir()
            .map_err(|e| ConfigError::HomeDir { err: e })?;
        #[cfg(test)]
        let mut path = PathBuf::new();
        path.push("MountProjects");
        let path_ = path.as_path().to_str().unwrap().to_string();
        Ok(Path(path_))
    }

    fn get_settings(&self) -> Result<Settings, ConfigError> {
        let dir = self.get_data_dir()?;
        let path = make_path![dir, "settings.json"];
        let file = PFile::from_path_reg(path);
        let text = FS_READ_SERVICE.read_file(&file)?;
        let settings = PARSING_SERVICE._from_string::<Settings>(&text)?;
        let settings = SETTINGS.get();
        if settings.is_none() {
            let settings_ = self.read_settings()?;
            SETTINGS
                .set(Mutex::new(settings_.clone()))
                .map_err(|_| ConfigError::ReadSettingsError)?;
            return Ok(settings_);
        }
        let settings = settings.unwrap().lock();
        if settings.is_err() {
            let settings_ = self.read_settings()?;
            SETTINGS
                .set(Mutex::new(settings_.clone()))
                .map_err(|_| ConfigError::ReadSettingsError)?;
            return Ok(settings_);
        }
        let settings = settings.unwrap().clone();

        Ok(settings)
    }

    fn get_file_templates(&self) -> Result<Vec<ConfigFsTemplate>, ConfigError> {
        let dir = self.get_data_dir()?;
        let path_ = make_path![dir, "file_templates.json"];
        let file = PFile::from_path_reg(path_.clone());
        let content = FS_READ_SERVICE.read_file(&file)?;
        let json = PARSING_SERVICE._from_string::<Vec<ConfigFsTemplate>>(&content)?;

        Ok(json)
    }

    fn get_licenses(&self) -> Result<Vec<String>, ConfigError> {
        todo!()
    }

    fn get_license(&self, name: String) -> Result<String, ConfigError> {
        todo!()
    }

    fn get_shell(&self) -> String {
        if cfg!(target_os = "linux") {
            std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string())
        } else if cfg!(target_os = "windows") {
            "powershell".to_string()
        } else {
            "zsh".to_string()
        }
    }
}
