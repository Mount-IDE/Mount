use crate::modules::app::{
    CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, CREATE_PROJECT_SERVICE, FS_READ_SERVICE,
    FS_WRITE_SERVICE, PARSING_SERVICE, PROJECT_SERVICE,
};
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::path_from;
use crate::modules::contexts::filesystem::domain::entities::PFile;
use crate::modules::contexts::filesystem::domain::values::FileWriteAccess;
use crate::modules::contexts::launch::app::managers::SharedLaunchManager;
use crate::modules::contexts::project::app::traits::{TCreateProjectService, TProjectService};
use crate::modules::contexts::project::domain::entities::{
    Package, Project, ProjectTag, ProjectTemplate, SharedPackages,
};
use crate::modules::contexts::project::domain::values::{
    CreateProjectPackageResults, CreateProjectResult,
};
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::contexts::terminal::app::managers::SharedTerminalManager;
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService, TParsingService};
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::values::Path;
use tauri::{State, Window};

#[tauri::command]
pub fn get_recent_projects() -> Result<Vec<RecentProject>, ErrorDto> {
    let dir = CONFIG_SERVICE.get_data_dir()?;
    let file = PFile::from_path_reg(dir);

    let ext = FS_READ_SERVICE.exist_file(&file);
    if !ext {
        let _ = CONFIG_RECOVERY_SERVICE.check_data_dir()?;
    }
    let recent = PROJECT_SERVICE.get_recent_projects()?;
    Ok(recent)
}
#[tauri::command]
pub fn read_recent_projects(recent: Vec<RecentProject>) -> Result<Vec<Project>, ErrorDto> {
    let vec_path = recent.iter().map(|e| e.path.clone()).collect::<Vec<Path>>();
    let mut res: Vec<Project> = vec![];

    for path in vec_path {
        let get: String = path.clone().get();

        let path_ = path_from![get, ".mount", "project.json"];
        let file = PFile::from_path_reg(path_);

        if FS_READ_SERVICE.exist_file(&file) {
            let Ok(config) = FS_READ_SERVICE.read_file(&file) else {
                continue;
            };
            let Ok(json) = PARSING_SERVICE._from_string::<Project>(&config) else {
                continue;
            };
            res.push(json);
        }
    }
    Ok(res)
}

#[tauri::command]
pub fn create_project(
    template: ProjectTemplate,
    results: CreateProjectResult,
    packages: Vec<String>,
    tags: Vec<ProjectTag>,
    pack_results: CreateProjectPackageResults,
    window: Window,
    pack_state: State<'_, SharedPackages>,
) -> Result<Project, ErrorDto> {
    CREATE_PROJECT_SERVICE
        .create_project(
            template,
            results,
            packages,
            tags,
            pack_results,
            window.label().to_string(),
            pack_state,
        )
        .map_err(|e| e.into())
}

#[tauri::command]
pub fn remove_project(path: Path) -> Result<(), ErrorDto> {
    let project = PROJECT_SERVICE.delete_project(&path)?;
    let _ = PROJECT_SERVICE.remove_from_recents(&project)?;
    Ok(())
}

#[tauri::command]
pub fn read_project(path: Path) -> Result<Project, ErrorDto> {
    PROJECT_SERVICE.open_project(&path).map_err(|e| e.into())
}

#[tauri::command]
pub fn save_project(project: Project) -> Result<(), ErrorDto> {
    PROJECT_SERVICE.save_project(&project)?;
    Ok(())
}

#[tauri::command]
pub fn save_packages(packs: Vec<Package>, path: Path) -> Result<(), ErrorDto> {
    PROJECT_SERVICE.save_packages(packs, path)?;
    Ok(())
}

#[tauri::command]
pub fn update_recents(projects: Vec<RecentProject>) -> Result<(), ErrorDto> {
    let dir = CONFIG_SERVICE.get_data_dir()?;
    let path = path_from![dir, "recent-projects.json"];
    let file = PFile::from_path_reg(path);

    let parsed = PARSING_SERVICE.to_string(projects)?;
    FS_WRITE_SERVICE.write_file(&file, parsed, FileWriteAccess::WRITE)?;

    Ok(())
}

#[tauri::command]
pub fn has_processes(
    window: Window,
    state1: State<'_, SharedTerminalManager>,
    state2: State<'_, SharedLaunchManager>,
) -> Result<(bool, bool), ()> {
    let terminals = {
        state1
            .lock()
            .unwrap()
            .terminals
            .iter()
            .filter(|(_, obj)| obj.window_id == window.label())
            .count()
    };

    let launches = {
        state2
            .lock()
            .unwrap()
            .launches
            .iter()
            .filter(|(_, obj)| obj.window_id == window.label())
            .count()
    };

    Ok((terminals != 0, launches != 0))
}
