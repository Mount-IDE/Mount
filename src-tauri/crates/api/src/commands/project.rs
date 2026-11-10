use crate::domain::ErrorDto;
use crate::get_data_dir;
use filesystem::domain::PFile;
use filesystem::traits::{TFSReadService, TFSWriteService};
use filesystem::values::FileWriteAccess;
use filesystem::{FS_READ_SERVICE, FS_WRITE_SERVICE};
use mount_core::domain::Path;
use mount_core::make_path;
use parsing::PARSING_SERVICE;
use parsing::traits::TParsingService;
use project::launch::managers::SharedLaunchManager;
use project::project::domain::{
    Package, Project, ProjectTag, ProjectTemplate, RecentProject, SharedPackages,
};
use project::project::traits::{TCreateProjectService, TProjectService};
use project::project::values::{CreateProjectPackageResults, CreateProjectResult};
use project::terminal::managers::SharedTerminalManager;
use project::{CREATE_PROJECT_SERVICE, PROJECT_SERVICE};
use recovery::CONFIG_RECOVERY_SERVICE;
use recovery::traits::TConfigRecoveryService;
use tauri::{State, Window};
use macroses::log;

pub fn calculate_untitled_max(recent: &Vec<RecentProject>) -> i32 {
    let names = recent.iter().filter_map(|e| {
        if e.name.trim().to_lowercase().starts_with("untitled") {
            return Some(e.name.chars().filter(|e| e.is_ascii_digit() || *e == '.').collect::<String>())
        } else {
            return None
        }
    }).collect::<Vec<String>>();
    log!("NAMES {names:?}");
    let mut names = names.iter().filter_map(|e| {
        if e.contains(".") {
            let p = e.chars().position(|e| e == '.');
            if let None = p {
                return None;
            }
            let slice = e[0..p.unwrap()].to_string();
            return slice.parse::<i32>().ok()
        } else {
            return e.parse::<i32>().ok()
        }
    }).collect::<Vec<i32>>();
    names.sort();
    log!("NAMES2 {names:?}");
    names.last().unwrap_or(&0).clone() + 1
}

#[tauri::command]
pub fn get_recent_projects() -> Result<(Vec<RecentProject>, i32), ErrorDto> {
    let dir = get_data_dir!();
    let file = PFile::from_path_reg(dir);

    let ext = FS_READ_SERVICE.exist_file(&file);
    if !ext {
        let _ = CONFIG_RECOVERY_SERVICE.check_data_dir()?;
    }
    let recent = PROJECT_SERVICE.get_recent_projects()?;

    let max = calculate_untitled_max(&recent);
    Ok((recent, max))
}
#[tauri::command]
pub fn read_recent_projects(recent: Vec<RecentProject>) -> Result<Vec<Project>, ErrorDto> {
    let vec_path = recent.iter().map(|e| e.path.clone()).collect::<Vec<Path>>();
    let mut res: Vec<Project> = vec![];

    for path in vec_path {
        let get: String = path.clone().get();

        let path_ = make_path![get, ".mount", "project.json"];
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
pub fn update_recents(projects: Vec<RecentProject>) -> Result<i32, ErrorDto> {
    let dir = get_data_dir!();
    let path = make_path![dir, "recent-projects.json"];
    let file = PFile::from_path_reg(path);

    let max = calculate_untitled_max(&projects);

    let parsed = PARSING_SERVICE.to_string(projects)?;
    FS_WRITE_SERVICE.write_file(&file, parsed, FileWriteAccess::WRITE)?;
    Ok(max)
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
