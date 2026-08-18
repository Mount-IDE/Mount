use crate::modules::app::{
    APP, CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, FS_READ_SERVICE, FS_WRITE_SERVICE,
    PARSING_SERVICE, SETTINGS,
};
use crate::modules::contexts::config::entities::{ConfigFsTemplate, FsConfigIcons};
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::contexts::filesystem::app::utils::{make_path, path_from};
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::{FileType, FileWriteAccess};
use crate::modules::contexts::project::domain::entities::{Package, ProjectTemplate};
use crate::modules::contexts::settings::domain::entities::{Settings};
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService, TParsingService};
use crate::modules::shared::kernel::errors::{ConfigError, FileSystemError, ParsingError};
use crate::modules::shared::kernel::values::Path;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::string::ToString;
use std::sync::Mutex;
use tauri::{Manager};

pub struct ConfigService();

#[derive(Clone, Debug)]
struct FSFile {
    pub name: String,
    pub path: Path,
    pub content: Option<String>,
}

#[derive(Clone, Debug)]
struct FSDir {
    pub name: String,
    pub path: Path,
    pub entities: Option<Vec<FsEntity_>>,
}

impl FSDir {
    pub fn get_path(&self) -> Path {
        if self.path.get().len() == 0 {
            return Path(self.name.clone());
        }
        self.path.clone()
    }
}
#[derive(Clone, Debug)]
enum FsEntity_ {
    FILE(FSFile),
    DIR(FSDir),
}

trait TFsEntity<T> {
    type F;
    #[allow(unused)]
    fn file(name: T, path: Path) -> Self;

    #[allow(unused)]
    fn file_content(name: T, path: Path, content: T) -> Self;

    fn dir(name: T) -> Self;
    fn dir_in(name: T, path: Path) -> Self;
    fn dir_entities(name: T, entities: Vec<Self::F>) -> Self;

    fn file_s(name: T) -> Self;
    fn file_s_content(name: T, content: T) -> Self;
}

impl TFsEntity<&str> for FsEntity_ {
    type F = FsEntity_;

    fn file(name: &str, path: Path) -> Self {
        Self::FILE(FSFile {
            name: String::from(name),
            path,
            content: None,
        })
    }

    fn file_content(name: &str, path: Path, content: &str) -> Self {
        Self::FILE(FSFile {
            name: String::from(name),
            path,
            content: Some(content.to_string()),
        })
    }

    fn dir(name: &str) -> Self {
        Self::DIR(FSDir {
            name: name.to_string(),
            path: Path::empty(),
            entities: None,
        })
    }

    fn dir_in(name: &str, path: Path) -> Self {
        Self::DIR(FSDir {
            name: name.to_string(),
            path,
            entities: None,
        })
    }

    fn dir_entities(name: &str, entities: Vec<Self::F>) -> Self {
        Self::DIR(FSDir {
            name: name.to_string(),
            entities: Some(entities),
            path: Path::empty(),
        })
    }

    fn file_s(name: &str) -> Self {
        Self::FILE(FSFile {
            name: name.to_string(),
            path: Path::empty(),
            content: None,
        })
    }

    fn file_s_content(name: &str, content: &str) -> Self {
        Self::FILE(FSFile {
            name: name.to_string(),
            path: Path::empty(),
            content: Some(content.to_string()),
        })
    }
}

impl FsEntity_ {
    #[allow(unused)]
    pub fn get_path(&self) -> Path {
        match self {
            Self::DIR(dir) => {
                if dir.path.get().len() == 0 {
                    Path(dir.name.clone())
                } else {
                    dir.path.clone()
                }
            }
            Self::FILE(file) => file.path.clone(),
        }
    }
}

impl Display for FsEntity_ {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FILE(file) => {
                write!(f, "FILE {} :: {}", file.name, file.path)
            }
            Self::DIR(dir) => {
                write!(
                    f,
                    "DIR {} :: {} :: {}",
                    dir.name,
                    dir.path,
                    dir.entities.clone().unwrap_or(vec![]).len()
                )
            }
        }
    }
}

#[allow(unused_variables)]
impl TConfigService for ConfigService {
    fn read_settings(&self) -> Result<Settings, ConfigError> {
        let dir = self.get_data_dir()?;
        println!("dir was gotten");
        let path_to_settings = path_from![dir, "settings.json"];
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
        // println!("read settings.json was gotten");

        let settings = PARSING_SERVICE._from_string::<Settings>(file);
        println!("SETTINGS {settings:?}");
        if settings.is_err() {
            let dir = self.get_data_dir()?;
            let settings_ = Settings::new();
            let json = PARSING_SERVICE.to_string(&settings_)?;
            let json = serde_json::to_string(&settings_).map_err(|e| ParsingError::Serialize {
                path: dir.clone(),
                err: e,
            })?;
            let dir = path_from![dir.get(), "settings.json"];
            let file = PFile::from_path_reg(dir.clone());
            FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
            // println!("parsing settings.json was gotten");
            return Ok(settings_);
        }
        let settings = settings.unwrap();
        // println!("parsing settings.json was gotten");
        Ok(settings)
    }

    fn save_settings(&self, _settings: &Settings) -> Result<(), ConfigError> {
        let dir = self.get_data_dir()?;
        let json = PARSING_SERVICE.to_string(_settings)?;
        let path = path_from![dir, "settings.json"];
        let file = PFile::from_path_reg(path.clone());
        FS_WRITE_SERVICE
            .write_file(&file, json, FileWriteAccess::WRITE)
            .map_err(|_| ConfigError::SavingSettings)?;
        Ok(())
    }

    fn get_data_dir(&self) -> Result<Path, ConfigError> {
        let dir = APP.get().unwrap().path().app_data_dir();

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
            CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
        }
        Ok(())
    }

    fn read_packages(&self) -> Result<Vec<Package>, ConfigError> {
        let dir = self.get_data_dir()?;
        let path_ = make_path(vec![dir.get().as_str(), "packages.json"]);
        let file = PFile::from_path_reg(path_.clone());

        let content = FS_READ_SERVICE.read_file(&file)?;

        let json = serde_json::from_str::<Vec<Package>>(&content).map_err(|e| {
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

    fn get_settings(&self) -> Result<Settings, ConfigError> {
        let dir = self.get_data_dir()?;
        let path = path_from![dir, "settings.json"];
        let file = PFile::from_path_reg(path);
        let text = FS_READ_SERVICE.read_file(&file)?;
        let settings = PARSING_SERVICE._from_string::<Settings>(text)?;
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
        let path_ = make_path(vec![dir.get().as_str(), "file_templates.json"]);
        let file = PFile::from_path_reg(path_.clone());
        let content = FS_READ_SERVICE.read_file(&file)?;
        let json = serde_json::from_str::<Vec<ConfigFsTemplate>>(&content).map_err(|e| {
            ParsingError::Deserialize {
                err: e,
                json: content,
                path: path_.clone(),
            }
        })?;

        Ok(json)
    }
}

pub struct ConfigRecoveryService();

fn get_files_new() -> Vec<FsEntity_> {
    vec![
        FsEntity_::file_s_content(
            "settings.json",
            PARSING_SERVICE.to_string(Settings::new()).unwrap().as_str(),
        ),
        FsEntity_::file_s_content("recent-projects.json", "[]"),
        FsEntity_::file_s_content(
            "templates.json",
            PARSING_SERVICE
                .to_string(&vec![ProjectTemplate::default().clone()])
                .unwrap()
                .as_str(),
        ),
        FsEntity_::file_s_content(
            "file_ext_icons.json",
            PARSING_SERVICE
                .to_string(vec![FsConfigIcons::default()])
                .unwrap()
                .as_str(),
        ),
        FsEntity_::file_s_content(
            "file_templates.json",
            PARSING_SERVICE
                .to_string(&vec![ConfigFsTemplate::file(), ConfigFsTemplate::dir()])
                .unwrap()
                .as_str(),
        ),
        FsEntity_::dir("icons"),
        FsEntity_::dir("aside_icons"),
        FsEntity_::dir_entities(
            "themes",
            vec![
                FsEntity_::dir_entities(
                    "opie.dark",
                    vec![FsEntity_::file_s_content(
                        "theme.json",
                        include_str!("../../../assets/dark.json"),
                    )],
                ),
                FsEntity_::dir_entities(
                    "opie.light",
                    vec![FsEntity_::file_s_content(
                        "theme.json",
                        include_str!("../../../assets/light.json"),
                    )],
                ),
                FsEntity_::dir_entities(
                    "opie.islands_dark",
                    vec![FsEntity_::file_s_content(
                        "theme.json",
                        include_str!("../../../assets/islands_dark.json"),
                    )],
                ),
            ],
        ),
        FsEntity_::dir_entities("packages",
                                vec![
                                    FsEntity_::dir_entities("opie.py", vec![
                                        FsEntity_::file_s_content("config.json",
                                                                  PARSING_SERVICE.to_string(Package::python()).unwrap().as_str()
                                        )
                                    ])
                                ]
        )
    ]
}

fn create_dir(dir: FSDir, path: Path) -> Result<(), FileSystemError> {
    let path = path_from![path.get(), dir.get_path().get()];
    println!("PATH {path}");
    FS_WRITE_SERVICE.create_dir(&path)?;
    if let Some(entities) = dir.entities {
        let mut n = 0;
        for i in entities.iter() {
            println!("entity {n} :: {}", dir.name);
            n += 1;
            match i {
                FsEntity_::FILE(file) => {
                    let path_ = file.path.clone();
                    let path_ = path_from![path.get(), path_.get(), file.name.clone()];
                    println!("FILE {} '{}'", file.name, path_.get());
                    let file_ = FS_WRITE_SERVICE.create_file(&path_)?;
                    if let Some(content) = file.content.clone() {
                        FS_WRITE_SERVICE.write_file(&file_, content, FileWriteAccess::WRITE)?;
                    }
                }
                FsEntity_::DIR(dir) => {
                    println!("DIR {}", dir.name);
                    create_dir(dir.clone(), path.clone())?;
                }
            }
        }
    }

    Ok(())
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
        let files = get_files_new();
        for file in files {
            println!("ENTRY {file}");
            match file {
                FsEntity_::DIR(dir_) => {
                    let path_ = path_from![dir.get(), dir_.get_path().get()];
                    if !FS_READ_SERVICE.exists(path_.clone()) {
                        self.repair_data_dir()?;
                        return Ok(());
                    }
                }
                FsEntity_::FILE(file_) => {
                    let path = path_from![dir.get(), file_.path.get(), file_.name];
                    if !FS_READ_SERVICE.exists(path.clone()) {
                        self.repair_data_dir()?;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }

    fn repair_data_dir(&self) -> Result<(), ConfigError> {
        let dir__ = CONFIG_SERVICE.get_data_dir()?;
        let path_ = FS_READ_SERVICE.exist_dir(&PDirectory::from_path(&dir__.clone()));
        if !path_ {
            let data = APP
                .get()
                .unwrap()
                .path()
                .app_data_dir()
                .map_err(|e| ConfigError::GetDataDir { err: e })?;
            let _ = FS_WRITE_SERVICE.create_dir(&Path(data.to_str().unwrap().to_string()));
        }
        let dir = dir__.get();
        println!("dir {dir}");

        let projects = CONFIG_SERVICE.get_projects_dir();
        if projects.is_err() {
            CONFIG_SERVICE.make_projects_dir()?;
        }
        let files = get_files_new();

        for i in files {
            println!("ENTRY2 {i}");
            match i {
                FsEntity_::DIR(dir_) => create_dir(dir_, dir__.clone())?,
                FsEntity_::FILE(file) => {
                    let path_ = path_from![dir.clone(), file.path.get()];
                    if !FS_READ_SERVICE.exists(path_.clone()) {
                        FS_WRITE_SERVICE.create_dir(&path_.clone())?;
                    }
                    let path_ = path_from![path_.get(), file.name];
                    if FS_READ_SERVICE.exists(path_.clone()) {
                        let file_ = PFile::from_path_reg(path_.clone());
                        let text = FS_READ_SERVICE.read_file(&file_)?;
                        if text.len() == 0 {
                            if let Some(content) = file.content.clone() {
                                FS_WRITE_SERVICE.write_file(
                                    &file_,
                                    content,
                                    FileWriteAccess::WRITE,
                                )?;
                            }
                        }
                    } else {
                        let file_ = FS_WRITE_SERVICE.create_file(&path_)?;
                        if let Some(content) = file.content.clone() {
                            FS_WRITE_SERVICE.write_file(&file_, content, FileWriteAccess::WRITE)?;
                        }
                    }
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

pub struct ParsingService();

impl TParsingService for ParsingService {
    fn to_string<T: Serialize>(&self, obj: T) -> Result<String, ParsingError> {
        let res: String = serde_json::to_string(&obj).map_err(|e| ParsingError::Serialize {
            path: Default::default(),
            err: e,
        })?;

        Ok(res)
    }

    fn _from_string<T: DeserializeOwned>(&self, obj: String) -> Result<T, ParsingError> {
        let res: T = serde_json::from_str(&obj).map_err(|e| ParsingError::Deserialize {
            path: Path::new(""),
            json: obj.to_string(),
            err: e,
        })?;
        Ok(res)
    }
}
