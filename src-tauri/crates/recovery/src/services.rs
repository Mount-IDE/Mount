use crate::domain::{FSDir, FsEntity_, TFsEntity};
use crate::traits::TConfigRecoveryService;
use config::CONFIG_SERVICE;
use config::domain::config::FsConfigIcons;
use config::domain::settings::Settings;
use config::errors::ConfigError;
use config::traits::TConfigService;
use filesystem::domain::{ConfigFsTemplate, FileSystemError, PDirectory, PFile};
use filesystem::traits::{TFSReadService, TFSWriteService};
use filesystem::values::FileWriteAccess;
use filesystem::{FS_READ_SERVICE, FS_WRITE_SERVICE};
use macroses::on_debug;
use mount_core::domain::Path;
use mount_core::{APP, make_path};
use parsing::traits::TParsingService;
use project::project::domain::{Package, ProjectTemplate};
use services::PARSING_SERVICE;
use tauri::Manager;

pub struct ConfigRecoveryService;

fn get_files_new() -> Vec<FsEntity_> {
    let mut templates = vec![ConfigFsTemplate::file(), ConfigFsTemplate::dir()];

    on_debug!(templates.push(ConfigFsTemplate::cargo()));
    vec![
        FsEntity_::file_s_content(
            "settings.json",
            PARSING_SERVICE.to_string(Settings::new()).unwrap().as_str(),
        ),
        FsEntity_::file_s_content("recent-projects.json", "[]"),
        FsEntity_::file_s_content(
            "templates.json",
            PARSING_SERVICE
                .to_string(&vec![
                    ProjectTemplate::default().clone(),
                    ProjectTemplate::rust(),
                ])
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
            PARSING_SERVICE.to_string(&templates).unwrap().as_str(),
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
                FsEntity_::dir_entities(
                    "vs.dark",
                    vec![FsEntity_::file_s_content(
                        "theme.json",
                        include_str!("../../../assets/vs-dark.json"),
                    )],
                ),
            ],
        ),
        FsEntity_::dir_entities(
            "packages",
            vec![
                FsEntity_::dir_entities(
                    "opie.python",
                    vec![FsEntity_::file_s_content(
                        "config.json",
                        PARSING_SERVICE
                            .to_string(Package::python())
                            .unwrap()
                            .as_str(),
                    )],
                ),
                FsEntity_::dir_entities(
                    "opie.rust",
                    vec![FsEntity_::file_s_content(
                        "config.json",
                        PARSING_SERVICE.to_string(Package::rust()).unwrap().as_str(),
                    )],
                ),
            ],
        ),
        FsEntity_::dir_entities(
            "licenses",
            vec![
                FsEntity_::file_s_content(
                    "gpl-3.0.txt",
                    include_str!("../../../assets/licenses/gpl-3.0"),
                ),
                FsEntity_::file_s_content(
                    "lgpl-3.0.txt",
                    include_str!("../../../assets/licenses/lgpl-3.0"),
                ),
                FsEntity_::file_s_content("mit.txt", include_str!("../../../assets/licenses/mit")),
                FsEntity_::file_s_content(
                    "bsd-3.txt",
                    include_str!("../../../assets/licenses/bsd-3"),
                ),
                FsEntity_::file_s_content(
                    "apache-2.0.txt",
                    include_str!("../../../assets/licenses/apache-2.0"),
                ),
            ],
        ),
        FsEntity_::dir("toolchains"),
    ]
}

fn create_dir(dir: FSDir, path: Path) -> Result<(), FileSystemError> {
    let path = make_path![path, dir.get_path()];
    FS_WRITE_SERVICE.create_dir(&path)?;
    if let Some(entities) = dir.entities {
        for i in entities.iter() {
            match i {
                FsEntity_::FILE(file) => {
                    let path_ = file.path.clone();
                    let path_ = make_path![path, path_, file.name];
                    let file_ = FS_WRITE_SERVICE.create_file(&path_)?;
                    if let Some(content) = file.content.clone() {
                        FS_WRITE_SERVICE.write_file(&file_, content, FileWriteAccess::WRITE)?;
                    }
                }
                FsEntity_::DIR(dir) => {
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
            // println!("no dir");
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
            // println!("ENTRY {file}");
            match file {
                FsEntity_::DIR(dir_) => {
                    let path_ = make_path![dir.get(), dir_.get_path().get()];
                    if !FS_READ_SERVICE.exists(path_.clone()) {
                        self.repair_data_dir()?;
                        return Ok(());
                    }
                }
                FsEntity_::FILE(file_) => {
                    let path = make_path![dir.get(), file_.path.get(), file_.name];
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
        //  println!("dir {dir}");

        let projects = CONFIG_SERVICE.get_projects_dir();
        if projects.is_err() {
            CONFIG_SERVICE.make_projects_dir()?;
        }
        let files = get_files_new();

        for i in files {
            // println!("ENTRY2 {i}");
            match i {
                FsEntity_::DIR(dir_) => create_dir(dir_, dir__.clone())?,
                FsEntity_::FILE(file) => {
                    let path_ = make_path![dir, file.path.get()];
                    if !FS_READ_SERVICE.exists(path_.clone()) {
                        FS_WRITE_SERVICE.create_dir(&path_.clone())?;
                    }
                    let path_ = make_path![path_.get(), file.name];
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
        let path = make_path![dir.get().as_str(), "settings.json"];
        let file = PFile::from_path_reg(path.clone());
        let json = PARSING_SERVICE.to_string(&settings)?;
        FS_WRITE_SERVICE.create_file(&path.clone())?;
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
        Ok(())
    }

    fn add_recents_by_default(&self) -> Result<(), ConfigError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path = make_path![dir.get().as_str(), "recent-projects.json"];
        let file = PFile::from_path_reg(path.clone());
        let json = String::from("[ ]");

        FS_WRITE_SERVICE.create_file(&path.clone())?;
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
        Ok(())
    }
}
