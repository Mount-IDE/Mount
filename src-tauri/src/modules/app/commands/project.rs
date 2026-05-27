use crate::modules::app::utils::project::make_buttons;
use crate::modules::app::{
    CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, FS_READ_SERVICE, FS_WRITE_SERVICE, PROJECT_SERVICE,
};
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::{make_path, make_path_string};
use crate::modules::contexts::filesystem::domain::entities::PFile;
use crate::modules::contexts::filesystem::domain::values::{FileType, FileWriteAccess};
use crate::modules::contexts::project::app::traits::TProjectService;
use crate::modules::contexts::project::domain::entities::{
    Project, ProjectPackage, ProjectTag, ProjectTemplate,
};
use crate::modules::contexts::project::domain::values::{CreateProjectResult, ProjectMeta};
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService};
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::errors::{ParsingError, ProjectError};
use crate::modules::shared::kernel::values::{Path, Val};
use std::collections::HashMap;

#[tauri::command]
pub fn get_recent_projects() -> Result<Vec<RecentProject>, ErrorDto> {
    let dir = CONFIG_SERVICE.get_data_dir()?;
    let file = PFile {
        name: "recent-projects.json".to_string(),
        path: dir,
        typ: FileType::REGULAR,
    };
    let ext = FS_READ_SERVICE.exist_file(&file);
    if !ext {
        let _ = CONFIG_RECOVERY_SERVICE.check_data_dir()?;
        println!("get second");
    }
    let recent = PROJECT_SERVICE.get_recent_projects()?;
    println!("GET_RECENT_OK");
    Ok(recent)
}
#[tauri::command]
pub fn read_recent_projects(recent: Vec<RecentProject>) -> Result<Vec<Project>, ErrorDto> {
    let vec_path = recent.iter().map(|e| e.path.clone()).collect::<Vec<Path>>();
    let mut res: Vec<Project> = vec![];

    for path in vec_path {
        let get = path.get().clone();

        let path_ = make_path_string(vec![get.as_str(), ".mount", "project.json"]);
        let file = PFile {
            name: "project.json".to_string(),
            path: Path(path_.clone()),
            typ: FileType::REGULAR,
        };
        if FS_READ_SERVICE.exist_file(&file) {
            let config = FS_READ_SERVICE.read_file(&file);
            if config.is_err() {
                continue;
            }
            let config = config.unwrap();
            let json = serde_json::from_str::<Project>(config.as_str());
            if json.is_err() {
                continue;
            }
            let json = json.unwrap();
            res.push(json);
        }
    }
    println!("READ_RECENT_OK");
    Ok(res)
}

#[tauri::command]
pub fn create_project(
    template: ProjectTemplate,
    results: CreateProjectResult,
    packages: HashMap<String, ProjectPackage>,
    tags: Vec<ProjectTag>,
) -> Result<Project, ErrorDto> {
    let meta = results.get("__meta__").ok_or(ProjectError::MetaNotFound)?;
    let name = meta
        .get(&-4i8)
        .ok_or(ProjectError::MainMetaNotFound)?
        .get("project-name")
        .ok_or(ProjectError::NameNotFound)?;

    let path = meta
        .get(&-4i8)
        .ok_or(ProjectError::MainMetaNotFound)?
        .get("project-path")
        .ok_or(ProjectError::NameNotFound)?;

    let name = match name {
        Val::STRING(val) => val.clone(),
        _ => return Err(ProjectError::NameNotFound.into()),
    };

    let path = match path {
        Val::STRING(val) => val.clone(),
        _ => return Err(ProjectError::PathNotFound.into()),
    };
    println!("{}::{}", name, path);

    let path_ = make_path(vec![path.as_str(), name.as_str()]);
    let ext = FS_READ_SERVICE.exists(path_.clone());
    if ext {
        return Err(ProjectError::AlreadyExists.into());
    }

    let additions = make_meta(meta.get(&-3i8), &tags);

    let vars = template.startup.var;
    let actions = template.startup.actions;
    let mut project = Project::new();
    project.name = name;
    project.path = Path(path.clone());
    project.meta = additions;
    project.vars = vars;

    let buttons = make_buttons();

    project.workspace.buttons = buttons;

    let json = serde_json::to_string(&project.clone()).map_err(|e| ProjectError::ParsingError {
        err: ParsingError::Serialize {
            path: Path(path.clone()),
            err: e,
        },
    })?;

    let dir = FS_WRITE_SERVICE.create_dir(&path_)?;
    let path_to_mount = make_path(vec![path_.get().as_str(), ".mount"]);
    let mount = FS_WRITE_SERVICE.create_dir(&path_to_mount)?;
    let path_to_settings = make_path(vec![path_to_mount.get().as_str(), "project.json"]);
    let settings = FS_WRITE_SERVICE.create_file(&path_to_settings)?;
    FS_WRITE_SERVICE.write_file(&settings, json, FileWriteAccess::WRITE)?;
    PROJECT_SERVICE.add_to_recents(&project)?;
    Ok(project)
}

fn make_meta(additions: Option<&HashMap<String, Val>>, tags: &Vec<ProjectTag>) -> ProjectMeta {
    if additions.is_none() {
        return ProjectMeta::default();
    }
    let mut meta_ = ProjectMeta::new();
    let add = additions.unwrap();
    let authors = add
        .get("project-authors")
        .unwrap_or(&Val::STRING("".to_string()))
        .clone();
    let desc = add
        .get("project-description")
        .unwrap_or(&Val::STRING("".to_string()))
        .clone();
    let license = add
        .get("project-license")
        .unwrap_or(&Val::STRING("".to_string()))
        .clone();
    let group = add
        .get("project-group")
        .unwrap_or(&Val::STRING("".to_string()))
        .clone();

    if let Val::STRING(val) = authors {
        let splited = val.split(":").collect::<Vec<&str>>();
        meta_.authors = splited.iter().map(|s| s.to_string()).collect();
    }

    if let Val::STRING(val) = desc {
        meta_.description = val.to_string();
    }
    if let Val::STRING(val) = license {
        meta_.license = Some(val);
    }
    if let Val::STRING(val) = group {
        meta_.group = val;
    } else {
        meta_.group = "general".to_string();
    }
    let _tags_ = tags
        .iter()
        .map(|el| el.name.clone())
        .collect::<Vec<String>>();
    meta_.tags = _tags_.clone();

    meta_
}

#[tauri::command]
pub fn read_project(path: Path) -> Result<Project, ErrorDto> {
    println!("path command {}", path.clone());
    PROJECT_SERVICE.open_project(&path).map_err(|e| e.into())
}
