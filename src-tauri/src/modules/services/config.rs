use crate::modules::app::{
    APP, CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, FS_READ_SERVICE, FS_WRITE_SERVICE,
};
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::make_path;
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::{FileType, FileWriteAccess};
use crate::modules::contexts::project::domain::entities::{ProjectPackage, ProjectTemplate};
use crate::modules::contexts::settings::domain::entities::Settings;
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService};
use crate::modules::shared::kernel::errors::{ConfigError, ParsingError};
use crate::modules::shared::kernel::values::Path;
use std::string::ToString;
use tauri::Manager;

pub struct ConfigService();

struct _File {
    pub name: String,
    pub path: Path,
    pub content: String,
}

impl _File {
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            path: Path(path),
            content: String::from("{}"),
        }
    }
    pub fn content(name: String, path: String, content: String) -> Self {
        Self {
            name,
            path: Path(path),
            content,
        }
    }
}

#[allow(unused_variables)]
impl TConfigService for ConfigService {
    fn read_settings(&self) -> Result<Settings, ConfigError> {
        let dir = self.get_data_dir()?;
        println!("dir was gotten");
        let path_to_settings = make_path(vec![dir.get().as_str(), "settings.json"]);
        let file = PFile::from_path_reg(path_to_settings.clone());
        let ext = FS_READ_SERVICE.exist_file(&file);
        if !ext {
            CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
            println!("repair data dir was gotten");
        }
        println!("building file");
        let file_ = PFile {
            name: "settings.json".to_string(),
            path: path_to_settings,
            typ: FileType::REGULAR,
        };
        let file = FS_READ_SERVICE
            .read_file(&file_)
            .map_err(|e| ConfigError::SettingsNotFound { err: e })?;
        println!("read settings.json was gotten");
        let settings = serde_json::from_str::<Settings>(file.as_str()).map_err(|e| {
            ParsingError::Deserialize {
                path: file_.path,
                json: file,
                err: e,
            }
        })?;
        println!("parsing settings,json was gotten");
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
            CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
        }
        Ok(())
    }

    fn read_packages(&self) -> Result<Vec<ProjectPackage>, ConfigError> {
        let dir = self.get_data_dir()?;
        let path_ = make_path(vec![dir.get().as_str(), "packages.json"]);
        let file = PFile::from_path_reg(path_.clone());

        let content = FS_READ_SERVICE.read_file(&file)?;

        let json = serde_json::from_str::<Vec<ProjectPackage>>(&content).map_err(|e| {
            ConfigError::ParsingError {
                err: ParsingError::Deserialize {
                    json: content,
                    path: path_.clone(),
                    err: e,
                },
            }
        })?;
        Ok(json)
    }

    fn read_templates(&self) -> Result<Vec<ProjectTemplate>, ConfigError> {
        let dir = self.get_data_dir()?;
        let path_ = make_path(vec![dir.get().as_str(), "templates.json"]);
        let file = PFile::from_path_reg(path_.clone());

        let content = FS_READ_SERVICE.read_file(&file)?;

        let json = serde_json::from_str::<Vec<ProjectTemplate>>(&content).map_err(|e| {
            ConfigError::ParsingError {
                err: ParsingError::Deserialize {
                    json: content,
                    path: path_.clone(),
                    err: e,
                },
            }
        })?;
        Ok(json)
    }

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
        let mut path = APP
            .get()
            .unwrap()
            .path()
            .home_dir()
            .map_err(|e| ConfigError::HomeDir { err: e })?;
        
        path.push("MountProjects");
        let path_ = path.as_path().to_str().unwrap().to_string();
        Ok(Path(path_))
    }
}

pub struct ConfigRecoveryService();

fn get_files() -> Vec<_File> {
    vec![
        _File::new("settings.json".to_string(), "".to_string()),
        _File::content(
            "recent-projects.json".to_string(),
            "".to_string(),
            "[]".to_string(),
        ),
        _File::content(
            "templates.json".to_string(),
            "".to_string(),
            "[]".to_string(),
        ),
        _File::content(
            "packages.json".to_string(),
            "".to_string(),
            "[]".to_string(),
        ),
        _File::new("".to_string(), "icons".to_string()),
    ]
}

impl TConfigRecoveryService for ConfigRecoveryService {
    fn check_data_dir(&self) -> Result<(), ConfigError> {
        let dir = CONFIG_SERVICE.get_data_dir();
        if dir.is_err() {
            println!("no dir");
            self.repair_data_dir()?;
            return Ok(());
        }
        let dir = dir.unwrap();
        let projects = CONFIG_SERVICE.get_projects_dir();
        if projects.is_err() {
            self.repair_data_dir()?;
            return Ok(());
        }
        let files = get_files();
        for file in files {
            if file.name.len() == 0 {
                let path_ = make_path(vec![
                    dir.get().clone().as_str(),
                    file.path.get().clone().as_str(),
                ]);
                let exists = FS_READ_SERVICE.exists(path_.clone());
                if !exists {
                    self.repair_data_dir()?;
                }
            } else {
                let path_ = make_path(vec![
                    dir.get().clone().as_str(),
                    file.path.get().clone().as_str(),
                    file.name.clone().as_str(),
                ]);
                let exists = FS_READ_SERVICE.exists(path_.clone());
                if !exists {
                    self.repair_data_dir()?;
                }
                let file = PFile::regular(file.name.clone(), path_.clone());
                let content = FS_READ_SERVICE.read_file(&file)?;
                if content.len() == 0 {
                    self.repair_data_dir()?;
                }
            }
        }

        Ok(())
    }

    fn repair_data_dir(&self) -> Result<(), ConfigError> {
        let dir = CONFIG_SERVICE.get_data_dir().unwrap();
        let path_ = FS_READ_SERVICE.exist_dir(&PDirectory::from_path(&dir));
        if !path_ {
            println!("repair");
            let data = APP.get().unwrap().path().app_data_dir().unwrap();
            let _ = FS_WRITE_SERVICE.create_dir(&Path(data.to_str().unwrap().to_string()));
        }
        let dir = dir.get();
        println!("dir {dir}");
        
        let projects = CONFIG_SERVICE.get_projects_dir();
        if projects.is_err() {
            CONFIG_SERVICE.make_projects_dir()?;
        }
        let files = get_files();

        for i in files {
            if i.name.len() == 0 {
                let path_ = make_path(vec![dir.clone().as_str(), i.path.get().as_str()]);
                if !FS_READ_SERVICE.exists(path_.clone()) {
                    FS_WRITE_SERVICE.create_dir(&path_)?;
                }
            } else {
                let path_ = make_path(vec![dir.clone().as_str(), i.path.get().as_str()]);
                if !FS_READ_SERVICE.exists(path_.clone()) {
                    FS_WRITE_SERVICE.create_dir(&path_)?;
                }
                let path_ = make_path(vec![path_.get().as_str(), i.name.clone().as_str()]);
                if !FS_READ_SERVICE.exists(path_.clone()) {
                    FS_WRITE_SERVICE.create_file(&path_)?;
                    let file = PFile::regular(i.name.clone(), path_);
                    FS_WRITE_SERVICE.write_file(
                        &file,
                        i.content.clone(),
                        FileWriteAccess::APPEND,
                    )?;
                }
            }
        }
        Ok(())
    }

    fn add_settings_by_default(&self) -> Result<(), ConfigError> {
        let settings = Settings::new();
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path = make_path(vec![dir.get().as_str(), "settings.json"]);
        let file = PFile::from_path_reg(path.clone());
        FS_WRITE_SERVICE.create_file(&path.clone())?;
        let json = serde_json::to_string(&settings)
            .map_err(|e| ParsingError::Serialize { path, err: e })?;
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
        Ok(())
    }

    fn add_recents_by_default(&self) -> Result<(), ConfigError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path = make_path(vec![dir.get().as_str(), "recent-projects.json"]);
        let file = PFile::from_path_reg(path.clone());
        let json = String::from("[ ]");

        FS_WRITE_SERVICE.create_file(&path.clone())?;
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
        Ok(())
    }
}
