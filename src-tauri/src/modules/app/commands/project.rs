use crate::modules::app::utils::project::make_buttons;
use crate::modules::app::{
    ACTION_PROJECT_SERVICE, CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, EVENT_SERVICE,
    FS_READ_SERVICE, FS_WRITE_SERVICE, PARSING_SERVICE, PROJECT_SERVICE,
};
use crate::modules::contexts::events::traits::TEventService;
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::contexts::filesystem::app::utils::{make_path, make_path_string, path_from};
use crate::modules::contexts::filesystem::domain::entities::PFile;
use crate::modules::contexts::filesystem::domain::values::{FileType, FileWriteAccess};
use crate::modules::contexts::launch::domain::entities::LaunchTemplate;
use crate::modules::contexts::project::app::traits::{TActionProjectService, TProjectService};
use crate::modules::contexts::project::domain::entities::{
    Package, PackageAction, PackageActionCommand, Project, ProjectTag, ProjectTemplate,
    SharedPackages, Var,
};
use crate::modules::contexts::project::domain::values::{
    ActionOnError, CreateProjectPackageResults, CreateProjectResult, ProjectMeta,
};
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService, TParsingService};
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::errors::ProjectError;
use crate::modules::shared::kernel::values::{
    Dependency, DependencyLevel, IfStatementOperation, IfStatementPart, Path, Platform,
    PlatformType, Val,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;

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
        // println!("get second");
    }
    let recent = PROJECT_SERVICE.get_recent_projects()?;
    // println!("GET_RECENT_OK");
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
    // println!("READ_RECENT_OK");
    Ok(res)
}

#[tauri::command]
pub async fn create_project(
    mut template: ProjectTemplate,
    results: CreateProjectResult,
    packages: Vec<String>,
    tags: Vec<ProjectTag>,
    pack_results: CreateProjectPackageResults,
    window: tauri::Window,
    pack_state: State<'_, SharedPackages>,
) -> Result<Project, ErrorDto> {
    EVENT_SERVICE.send(
        window.label().to_string(),
        "task-start",
        "Check Dependencies".to_string(),
    );

    let all_packages = { pack_state.lock().unwrap().clone() };

    let dependencies = template.dependencies.clone();

    let error_dependency = PROJECT_SERVICE.check_dependencies(dependencies);
    if error_dependency.len() > 0 {
        let critical: Vec<Dependency> = error_dependency
            .iter()
            .cloned()
            .filter(|e| e.level == DependencyLevel::CRITICAL)
            .collect();
        let conflicts: Vec<Dependency> = error_dependency
            .iter()
            .cloned()
            .filter(|e| e.level == DependencyLevel::CONFLICTS)
            .collect();

        let json = PARSING_SERVICE.to_string(error_dependency.clone());
        if let Err(_) = json {
            println!("not3");
            return Err(ProjectError::NotAllDependenciesSuplied(error_dependency.clone()).into());
        }

        EVENT_SERVICE.send(window.label().to_string(), "project", json.unwrap());
        EVENT_SERVICE.send(
            window.label().to_string(),
            "ERROR",
            "Invalid dependencies".to_string(),
        );

        if conflicts.len() > 0 || critical.len() > 0 {
            return Err(ProjectError::NotAllDependenciesSuplied(error_dependency.clone()).into());
        }
    }
    EVENT_SERVICE.send(
        window.label().to_string(),
        "task-end",
        "All dependencies correct".to_string(),
    );

    // getting meta info about project
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

    let path_ = make_path(vec![path.as_str(), name.as_str()]);
    let ext = FS_READ_SERVICE.exists(path_.clone());
    if ext {
        // if project already exists
        return Err(ProjectError::AlreadyExists.into());
    }

    let additions = make_meta(meta.get(&-3i8), &tags);

    let mut vars = template.clone().startup.var;

    // adding required variables
    vars.push(Var::new(
        "project-name".to_string(),
        Val::STRING(name.clone()),
    ));

    vars.push(Var::new(
        "project-path".to_string(),
        Val::STRING(path.clone()),
    ));

    // creating project object
    let mut project = Project::new();
    project.name = name;
    project.path = Path(path.clone());
    project.meta = additions;
    project.vars = vars.clone();

    // adding git actions
    template.startup.actions.insert(
        0,
        PackageAction {
            id: -1,
            if_: Some(vec![vec![IfStatementPart {
                from: Some("#-2.project-git".to_string()),
                oper: IfStatementOperation::EQ,
                value: Some(Val::BOOL(true)),
            }]]),
            on_error: ActionOnError::CONTINUE,
            next: None,
            command: Some(vec![PackageActionCommand {
                platform: None,
                shell: Some("@".to_string()),
                env: None,
                cwd: None,
                needed_exit_code: None,
                command: "".to_string(),
            }]),
            platform: None,
        },
    );
    template.startup.actions.insert(
        1,
        PackageAction {
            id: -2,
            if_: Some(vec![vec![IfStatementPart {
                from: Some("#-2.project-git-gitignore".to_string()),
                oper: IfStatementOperation::EQ,
                value: Some(Val::BOOL(true)),
            }]]),
            on_error: ActionOnError::CONTINUE,
            next: None,
            command: Some(vec![
                PackageActionCommand {
                    platform: Some(Platform::windows()),
                    cwd: None,
                    shell: None,
                    env: None,
                    needed_exit_code: None,
                    command: "echo > .gitignore".to_string(),
                },
                PackageActionCommand {
                    platform: Some(Platform::arr(vec![
                        PlatformType::LINUX,
                        PlatformType::MACOS,
                    ])),
                    cwd: None,
                    shell: None,
                    env: None,
                    needed_exit_code: None,
                    command: "touch .gitignore".to_string(),
                },
            ]),
            platform: None,
        },
    );
    template.startup.actions.insert(
        2,
        PackageAction {
            id: -2,
            if_: Some(vec![vec![IfStatementPart {
                from: Some("#-2.project-git-remote".to_string()),
                oper: IfStatementOperation::NonEmpty,
                value: None,
            }]]),
            on_error: ActionOnError::CONTINUE,
            next: None,
            command: Some(vec![PackageActionCommand {
                platform: None,
                cwd: None,
                shell: None,
                env: None,
                needed_exit_code: None,
                command: "git remote add origin #{-2.project-git-remote}".to_string()
                /*ActionCommandIn::WithArgs(ActionCommandArgs(
                    "git remote add origin".to_string(),
                    vec!["#-2.project-git-remote".to_string()],*/
            }]),
            platform: None,
        },
    );

    let buttons = make_buttons();

    project.workspace.buttons = buttons;
    let packages = {
        let res = pack_state.lock().unwrap();
        res.iter()
            .filter(|e| packages.contains(&e.id))
            .map(|e| e.clone())
            .collect::<Vec<Package>>()
    };
    // making tasks
    let tasks =
        ACTION_PROJECT_SERVICE.compile(&template, &results, &vars, &packages, &pack_results);

    // if tasks running completely
    if let Some(val) = tasks {
        let dir = FS_WRITE_SERVICE.create_dir(&path_)?;
        let path_to_mount = path_from![path_, ".mount"];

        let mount = FS_WRITE_SERVICE.create_dir(&path_to_mount)?;

        let path_to_settings = path_from![path_to_mount, "project.json"];

        let settings = FS_WRITE_SERVICE.create_file(&path_to_settings)?;

        let path_to_packages = path_from![path_to_mount, "packages.json"];
        let packages_file = FS_WRITE_SERVICE.create_file(&path_to_packages)?;

        project.vars = val.0.clone();
        project.template = template.clone();
        project.workspace.launch_templates = template.launches.clone();
        let contains = project
            .workspace
            .launch_templates
            .iter()
            .find(|e| e.id == -1);
        if contains.is_none() {
            project
                .workspace
                .launch_templates
                .insert(0, LaunchTemplate::default());
        }
        let json = PARSING_SERVICE.to_string(&project.clone())?;

        let packages_str = PARSING_SERVICE.to_string(&packages.clone())?;

        FS_WRITE_SERVICE.write_file(&packages_file, packages_str, FileWriteAccess::WRITE)?;
        FS_WRITE_SERVICE.write_file(&settings, json, FileWriteAccess::WRITE)?;
        PROJECT_SERVICE.add_to_recents(&project)?;

        ACTION_PROJECT_SERVICE.run_tasks(&project, &val.1, window.label().to_string())
    }
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

    let image = add
        .get("image")
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

    if let Val::STRING(val) = image {
        #[derive(Serialize, Deserialize, Clone, Debug)]
        struct Image {
            typ: String,
            image: Option<String>,
            color: String,
        }

        let json = PARSING_SERVICE._from_string::<Image>(val);
        if let Ok(json) = json {
            meta_.icon = match json.typ.as_str() {
                "color" => Some(json.color),
                _ => json.image,
            }
        } else {
            meta_.icon = None;
        }
    } else {
        meta_.icon = None
    }

    let _tags_ = tags
        .iter()
        .map(|el| el.name.clone())
        .collect::<Vec<String>>();
    meta_.tags = _tags_.clone();
    println!("{meta_:?}");
    meta_
}

#[tauri::command]
pub fn remove_project(path: Path) -> Result<(), ErrorDto> {
    let project = PROJECT_SERVICE.delete_project(&path)?;
    let _ = PROJECT_SERVICE.remove_from_recents(&project)?;
    Ok(())
}

#[tauri::command]
pub fn read_project(path: Path) -> Result<Project, ErrorDto> {
    // println!("path command {}", path.clone());
    PROJECT_SERVICE.open_project(&path).map_err(|e| e.into())
}

#[tauri::command]
pub fn save_project(project: Project) -> Result<(), ErrorDto> {
    PROJECT_SERVICE.save_project(&project)?;
    Ok(())
}
