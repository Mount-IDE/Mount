use crate::modules::app::{
    CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, EVENT_SERVICE, FORMAT_SERVICE, FS_READ_SERVICE,
    FS_WRITE_SERVICE, PARSING_SERVICE, PROJECT_SERVICE, SETTINGS,
};
use crate::modules::contexts::events::traits::TEventService;
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::path_from;
use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::FileWriteAccess;
use crate::modules::contexts::launch::domain::entities::LaunchTemplate;
use crate::modules::contexts::project::app::traits::{
    TCreateProjectService, TPackageService, TProjectService,
};
use crate::modules::contexts::project::domain::entities::{
    ActionTask, Button, ButtonComponentType, Package, PackageAction, PackageActionCommand, Project,
    ProjectTag, ProjectTemplate, SharedPackages, Var,
};
use crate::modules::contexts::project::domain::values::{
    ActionOnError, ButtonPos, CreateProjectPackageResults, CreateProjectResult, ProjectMeta,
};
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::services::traits::{
    TConfigRecoveryService, TConfigService, TFormatService, TParsingService,
};
use crate::modules::shared::kernel::errors::ProjectError;
use crate::modules::shared::kernel::values::{
    Dependency, DependencyLevel, IfStatementOperation, IfStatementPart, Path, Platform,
    PlatformType, Val,
};
use macroses::log;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;

#[allow(unused)]
pub fn get_key(sh: String) -> String {
    match sh.as_str() {
        "bash" => "-c".to_string(),
        "sh" => "-c".to_string(),
        "dash" => "-c".to_string(),
        "ksh" => "-c".to_string(),
        "zsh" => "-c".to_string(),
        "fish" => "-c".to_string(),
        "cmd" => "/C".to_string(),
        "powershell" => "-Command".to_string(),
        _ => "".to_string(),
    }
}

pub struct ProjectService;

pub struct CreateProjectService;

impl TCreateProjectService for CreateProjectService {
    fn compile_vars(&self, vars: &mut Vec<Var>, results: &HashMap<String, Val>) {
        for i in 0..vars.len() {
            let Ok(name) = FORMAT_SERVICE.format_string(vars[i].name.clone(), &vars, &results)
            else {
                continue;
            };

            let val = if let Val::STRING(val) = vars[i].value.clone() {
                let Ok(res) = FORMAT_SERVICE.format_string(val, &vars, &results) else {
                    continue;
                };
                Val::STRING(res)
            } else {
                vars[i].value.clone()
            };

            vars[i].name = name;
            vars[i].value = val;
        }
    }

    fn create_project(
        &self,
        mut template: ProjectTemplate,
        results: CreateProjectResult,
        packages: Vec<String>,
        tags: Vec<ProjectTag>,
        pack_results: CreateProjectPackageResults,
        window: String,
        pack_state: State<'_, SharedPackages>,
    ) -> Result<Project, ProjectError> {
        let meta = results.get("__meta__").ok_or(ProjectError::MetaNotFound)?;
        // get project name
        let name = meta
            .get(&-4i8)
            .ok_or(ProjectError::MainMetaNotFound)?
            .get("project-name")
            .ok_or(ProjectError::NameNotFound)?;
        // get project path
        let path = meta
            .get(&-4i8)
            .ok_or(ProjectError::MainMetaNotFound)?
            .get("project-path")
            .ok_or(ProjectError::NameNotFound)?;

        // if name not string
        let name = match name {
            Val::STRING(val) => val.clone(),
            _ => return Err(ProjectError::NameNotFound.into()),
        };
        // if path not string
        let path = match path {
            Val::STRING(val) => val.clone(),
            _ => return Err(ProjectError::PathNotFound.into()),
        };
        let path_ = path_from![path, name];
        let ext = FS_READ_SERVICE.exists(path_.clone());
        if ext {
            // if project already exists
            return Err(ProjectError::AlreadyExists.into());
        }
        // modifying templates and add new actions for git
        self.modify_template(&mut template);

        // check dependencies
        self.check_dependencies(window.clone(), &template)?;

        let additions = self.make_meta(meta.get(&-3i8), &tags);
        let mut vars = template.startup.var.clone();
        self.add_vars(&mut vars, &additions, name.clone(), path.clone(), &template);

        let mut project = Project::new();
        project.name = name;
        project.path = Path(path.clone());
        project.meta = additions.clone();
        project.vars = vars.clone();
        project.template = template.clone();
        project.workspace.launch_templates = template.launches.clone();

        if let None = project
            .workspace
            .launch_templates
            .iter()
            .find(|e| e.id == -1)
        {
            project
                .workspace
                .launch_templates
                .insert(0, LaunchTemplate::default());
        }

        let buttons = self.make_buttons();
        project.workspace.buttons = buttons;
        let packages_ = {
            let res = pack_state.lock().unwrap();
            res.iter()
                .filter(|e| packages.contains(&e.id))
                .cloned()
                .collect::<Vec<Package>>()
        };
        project.packages = packages;

        let mut flat_results = HashMap::<String, Val>::new();
        for i in meta.iter() {
            for j in i.1.iter() {
                flat_results.insert(format!("{}.{}", i.0, j.0), j.1.clone());
            }
        }
        let needed = results.get(&template.id).cloned().unwrap_or(HashMap::new());
        for i in needed.iter() {
            for j in i.1.iter() {
                flat_results.insert(format!("{}.{}", i.0, j.0), j.1.clone());
            }
        }
        self.compile_vars(&mut vars, &flat_results);

        let path_to_mount = path_from![path_, ".mount"];
        let path_to_config = path_from![path_to_mount, "project.json"];
        let path_to_packages = path_from![path_to_mount, "packages.json"];

        log!("START PARSING");
        let config_str = PARSING_SERVICE.to_string(&project.clone())?;
        log!("PARSING1 {config_str}");
        let packages_str = PARSING_SERVICE.to_string(&packages_.clone())?;
        log!("PARSING2 {packages_str}");

        FS_WRITE_SERVICE.create_dir(&path_.clone())?;
        FS_WRITE_SERVICE.create_dir(&path_to_mount)?;
        let config = FS_WRITE_SERVICE.create_file(&path_to_config)?;
        let packages_file = FS_WRITE_SERVICE.create_file(&path_to_packages)?;

        FS_WRITE_SERVICE.write_file(&config, config_str, FileWriteAccess::WRITE)?;
        FS_WRITE_SERVICE.write_file(&packages_file, packages_str, FileWriteAccess::WRITE)?;
        PROJECT_SERVICE.add_to_recents(&project)?;
        log!("RECENTS ADD");
        self.exec_tasks(&template, &vars, &flat_results, path_);
        log!("ENDED TASKS");

        let license = additions.license.clone();
        if let Some(license) = license {
            self.add_license(&project, license, &results)?;
        }
        log!("END");

        Ok(project)
    }

    fn add_license(
        &self,
        proj: &Project,
        license: String,
        results: &CreateProjectResult,
    ) -> Result<(), ProjectError> {
        let path_ = path_from![proj.path, proj.name];
        let settings = { SETTINGS.get().unwrap().lock().unwrap().clone() };
        let licenses = settings.licenses;
        let needed = licenses.get(&license);
        if let Some(needed) = needed {
            //    println!("has needed license");
            // if settings has needed license
            if let Ok(dir) = CONFIG_SERVICE.get_data_dir() {
                //println!("has dir");
                // if app dir exists
                let lic = path_from![dir, "licenses", needed.filename];
                let file = PFile::from_path_reg(lic);
                let read = FS_READ_SERVICE.read_file(&file);
                if let Ok(mut read) = read {
                    //  println!("read correct");
                    // if file was read correctly
                    let entries_var = needed.entries_var.clone();
                    let entries_input = needed.entries_input.clone();
                    if let Some(entry) = entries_input {
                        //println!("input");
                        for i in entry.iter() {
                            let key = format!("{license}-{}", i.0);
                            let Some(result) = results.get("__meta__") else {
                                continue;
                            };
                            let Some(result) = result.get(&-3) else {
                                continue;
                            };
                            let Some(result) = result.get(&key) else {
                                continue;
                            };
                            let result = result.to_str();
                            for j in i.1.iter() {
                                read = read.replace(j, &result);
                            }
                        }
                    } // replacing from ui inputs
                    if let Some(entry) = entries_var {
                        // println!("var");
                        for i in entry.iter() {
                            let Some(var) = proj.vars.iter().find(|e| e.name.eq(i.0)) else {
                                continue;
                            };
                            for j in i.1.iter() {
                                read = read.replace(j, &var.value.to_str());
                            }
                        }
                    } // replacing from vars

                    let lic_path = path_from![path_.clone(), "LICENSE"];
                    let file = PFile::from_path_reg(lic_path);
                    let _ = FS_WRITE_SERVICE.write_file(&file, read, FileWriteAccess::WRITE);
                }
            }
        }
        Ok(())
    }

    fn check_dependencies(
        &self,
        window: String,
        template: &ProjectTemplate,
    ) -> Result<(), ProjectError> {
        EVENT_SERVICE.send(
            window.clone(),
            "task-start",
            "Check Dependencies".to_string(),
        );
        let dependencies = &template.dependencies;
        let mut error: Vec<Dependency> = vec![];
        for i in dependencies.iter() {
            let res = self.check_dependency(i);
            if !res {
                error.push(i.clone())
            }
        }
        if error.len() > 0 {
            let crit: Vec<Dependency> = error
                .iter()
                .cloned()
                .filter(|e| e.level == DependencyLevel::CRITICAL)
                .collect();
            let conflicts: Vec<Dependency> = error
                .iter()
                .cloned()
                .filter(|e| e.level == DependencyLevel::CONFLICTS)
                .collect();
            let json = PARSING_SERVICE.to_string(error.clone())?;
            EVENT_SERVICE.send(window.clone(), "project", json);
            EVENT_SERVICE.send(window.clone(), "ERROR", "Invalid dependencies".__get());
            if conflicts.len() > 0 || crit.len() > 0 {
                return Err(ProjectError::NotAllDependenciesSuplied(error));
            }
        }
        EVENT_SERVICE.send(
            window.clone(),
            "task-end",
            "All dependencies checked".to_string(),
        );
        Ok(())
    }

    fn check_dependency(&self, dependency: &Dependency) -> bool {
        let platform = Platform::current();
        let prog = dependency.program.clone();
        if let Some(plat) = dependency.platform.clone() {
            if platform != plat {
                return false;
            }
        }
        let res = which::which(prog);
        if let Err(_) = res {
            return match &dependency.level {
                DependencyLevel::CONFLICTS => true,
                _ => false,
            };
        }
        match &dependency.level {
            DependencyLevel::CONFLICTS => false,
            _ => true,
        }
    }

    fn add_vars(
        &self,
        vars: &mut Vec<Var>,
        additions: &ProjectMeta,
        name: String,
        path: String,
        template: &ProjectTemplate,
    ) {
        vars.push(Var::readonly(
            "project-name".to_string(),
            Val::STRING(name.clone()),
        ));

        vars.push(Var::readonly(
            "project-path".to_string(),
            Val::STRING(path.clone()),
        ));
        vars.push(Var::new(
            "project-description".__get(),
            Val::STRING(additions.description.clone()),
        ));
        vars.push(Var::new(
            "project-license".__get(),
            Val::STRING(additions.license.clone().unwrap_or("NonLicense".__get())),
        ));
        vars.push(Var::new(
            "project-authors".__get(),
            Val::ARRAY(additions.authors.clone()),
        ));
        vars.push(Var::new(
            "project-tags".__get(),
            Val::ARRAY(additions.tags.clone()),
        ));

        vars.push(Var::new(
            "project-group".__get(),
            Val::STRING(additions.group.clone()),
        ));

        vars.push(Var::readonly(
            "project-template".__get(),
            Val::STRING(template.id.clone()),
        ));
    }

    fn modify_template(&self, template: &mut ProjectTemplate) {
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
                    command: "git init".to_string(),
                }]),
                platform: None,
            },
        );
        template.startup.actions.insert(
            1,
            PackageAction {
                id: -1,
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
                    command: "git remote add origin #{-2.project-git-remote}".to_string(),
                }]),
                platform: None,
            },
        );
    }

    fn make_meta(
        &self,
        additions: Option<&HashMap<String, Val>>,
        tags: &Vec<ProjectTag>,
    ) -> ProjectMeta {
        if additions.is_none() {
            return ProjectMeta::default();
        }
        let mut meta_ = ProjectMeta::new();
        let add = additions.unwrap();
        let authors = add
            .get("project-authors")
            .cloned()
            .unwrap_or(Val::STRING("".to_string()));
        let desc = add
            .get("project-description")
            .cloned()
            .unwrap_or(Val::STRING("".to_string()));
        let license = add
            .get("project-license")
            .cloned()
            .unwrap_or(Val::STRING("".to_string()));
        let group = add
            .get("project-group")
            .cloned()
            .unwrap_or(Val::STRING("".to_string()));

        let image = add
            .get("image")
            .cloned()
            .unwrap_or(Val::STRING("".to_string()));

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

            let json = PARSING_SERVICE._from_string::<Image>(&val);
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
        meta_
    }

    fn make_buttons(&self) -> Vec<Button> {
        let mut res = Vec::<Button>::new();
        res.push(Button {
            alt: "Project".to_string(),
            pos: ButtonPos::LeftTop,
            widget: String::from("FsAside"),
            component_type: ButtonComponentType::Light,
            order: 0,
            keys: String::new(),
            icon: "dir.svg".to_string(),
        });
        res.push(Button {
            alt: "Commit".to_string(),
            pos: ButtonPos::LeftTop,
            widget: String::from("CommitAside"),
            component_type: ButtonComponentType::Light,
            order: 1,
            keys: String::new(),
            icon: "commit.svg".to_string(),
        });

        res.push(Button {
            alt: "Debug".to_string(),
            pos: ButtonPos::LeftBottom,
            widget: String::from("DebugAside"),
            component_type: ButtonComponentType::Light,
            order: 0,
            keys: String::new(),
            icon: "debug.svg".to_string(),
        });
        res.push(Button {
            alt: "Launch".to_string(),
            pos: ButtonPos::LeftBottom,
            widget: String::from("AsideLaunch"),
            component_type: ButtonComponentType::Heavy,
            order: 1,
            keys: String::new(),
            icon: "play.svg".to_string(),
        });
        res.push(Button {
            alt: "Terminal".to_string(),
            pos: ButtonPos::LeftBottom,
            widget: String::from("TerminalAside"),
            component_type: ButtonComponentType::Heavy,
            order: 2,
            keys: String::new(),
            icon: "terminal.svg".to_string(),
        });
        res.push(Button {
            alt: "Problems".to_string(),
            pos: ButtonPos::LeftBottom,
            widget: String::from("ProblemsAside"),
            component_type: ButtonComponentType::Heavy,
            order: 3,
            keys: String::new(),
            icon: "problems.svg".to_string(),
        });
        res.push(Button {
            alt: "Git".to_string(),
            pos: ButtonPos::LeftBottom,
            widget: String::from("GitAside"),
            component_type: ButtonComponentType::Light,
            order: 4,
            keys: String::new(),
            icon: "git.svg".to_string(),
        });
        res.push(Button {
            alt: "Notifications".to_string(),
            pos: ButtonPos::RightTop,
            widget: String::from("AsideNotifications"),
            component_type: ButtonComponentType::Light,
            order: 0,
            keys: String::new(),
            icon: "settings.svg".to_string(),
        });

        res
    }

    fn exec_tasks(
        &self,
        template: &ProjectTemplate,
        vars: &Vec<Var>,
        results: &HashMap<String, Val>,
        path: Path,
    ) {
        let actions = &template.startup.actions;
        let tasks = actions
            .iter()
            .filter_map(|e| {
                e.compile(&actions, &vars, &results, &mut vec![], false)
                    .ok()
            })
            .flatten()
            .collect::<Vec<ActionTask>>();
        log!("COMPILED {tasks:?}");

        let mut last = ActionOnError::CONTINUE;
        for i in tasks {
            if last == ActionOnError::StopAll {
                break;
            }
            if i.from_graph && last != ActionOnError::CONTINUE {
                continue;
            }
            last = i.execute(path.clone());
            log!("EXEC LAST {last:?} {:?}", i.commands);
        }
    }
}

impl TProjectService for ProjectService {
    ///
    ///
    ///
    fn open_project(&self, project_path: &Path) -> Result<Project, ProjectError> {
        let path = path_from![*project_path, ".mount", "project.json",];
        let config = PFile::from_path_reg(path);

        let json = FS_READ_SERVICE.read_file(&config)?;
        let proj = PARSING_SERVICE._from_string::<Project>(&json)?;
        Ok(proj)
    }

    ///
    ///
    ///
    fn delete_project(&self, project_path: &Path) -> Result<Project, ProjectError> {
        let path_ = path_from![*project_path, ".mount", "project.json",];
        let file = PFile::from_path_reg(path_.clone());
        let text = FS_READ_SERVICE.read_file(&file)?;
        let json = PARSING_SERVICE._from_string::<Project>(&text)?;

        let proj_dir = PDirectory::from_path(&project_path.clone());
        FS_WRITE_SERVICE.remove_dir(&proj_dir)?;
        Ok(json)
    }

    ///
    ///
    ///
    fn get_projects(&self, dir: &Path) -> Result<Vec<Project>, ProjectError> {
        let dir = PDirectory::from_path(dir);

        let directory = FS_READ_SERVICE.read_dir(&dir)?;
        let mut projects: Vec<Project> = vec![];
        for dir in directory.directories {
            let mount = dir.directories.iter().find(|e| e.name == ".mount");
            if mount.is_none() {
                continue;
            }
            let mount = mount.unwrap().clone();
            let path_to = path_from![mount.path, "project.json"];

            let file = PFile::from_path_reg(path_to);
            let Ok(file) = FS_READ_SERVICE.read_file(&file) else {
                continue;
            };
            let proj = PARSING_SERVICE._from_string::<Project>(&file)?;
            projects.push(proj);
        }
        Ok(projects)
    }

    ///
    ///
    ///
    fn get_recent_projects(&self) -> Result<Vec<RecentProject>, ProjectError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let dir = PDirectory::from_path(&dir);
        let ext = FS_READ_SERVICE.exist_dir(&dir);
        if !ext {
            FS_WRITE_SERVICE.create_dir(&Path(dir.path.get()))?;
            return Ok(vec![]);
        }
        let path_to = path_from![dir.path, "recent-projects.json"];

        let file = PFile::from_path_reg(path_to.clone());
        let ext = FS_READ_SERVICE.exist_file(&file);
        if !ext {
            FS_WRITE_SERVICE.create_file(&path_to)?;
            return Ok(vec![]);
        }
        let file = PFile::from_path_reg(path_to);

        let text = FS_READ_SERVICE.read_file(&file)?;
        let mut projects = PARSING_SERVICE._from_string::<Vec<RecentProject>>(&text)?;

        projects.sort_by(|a, b| b.last_opened.cmp(&a.last_opened));
        Ok(projects)
    }
    ///
    ///
    ///
    fn save_project(&self, _project: &Project) -> Result<(), ProjectError> {
        let path = _project.path.clone();
        let path_ = path_from![path, _project.name, ".mount", "project.json",];
        let json = PARSING_SERVICE.to_string(&_project)?;
        let file = PFile::from_path_reg(path_);
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
        Ok(())
    }

    fn save_packages(&self, packs: Vec<Package>, path: Path) -> Result<(), ProjectError> {
        let path_ = path_from![path, ".mount", "packages.json"];
        let json = PARSING_SERVICE.to_string(&packs)?;
        let file = PFile::from_path_reg(path_);
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;

        Ok(())
    }

    fn remove_from_recents(&self, _proj: &Project) -> Result<(), ProjectError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path_ = path_from![dir, "recent-projects.json"];
        let file = PFile::from_path_reg(path_.clone());
        let text = FS_READ_SERVICE.read_file(&file)?;
        let json = PARSING_SERVICE._from_string::<Vec<RecentProject>>(&text)?;

        let path1 = path_from![_proj.path, _proj.name,];
        let new_json = json
            .iter()
            .filter_map(|el| {
                let path2 = path_from![el.path, el.name,];
                if path1.get() == path2.get() {
                    return None;
                }
                Some(el.clone())
            })
            .collect::<Vec<RecentProject>>();

        let json = PARSING_SERVICE.to_string(&new_json)?;
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
        Ok(())
    }

    fn add_to_recents(&self, _project: &Project) -> Result<(), ProjectError> {
        let data = SystemTime::now();
        let now = data.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let path = _project.path.clone();
        let name = _project.name.clone();
        let meta = _project.meta.clone();
        let packages = _project.packages.clone();
        let recent = RecentProject {
            name,
            path,
            last_opened: now,
            meta,
            packages,
        };
        let mut dir = CONFIG_SERVICE.get_data_dir();
        if dir.is_err() {
            CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
            dir = CONFIG_SERVICE.get_data_dir();
            if dir.is_err() {
                return Err(dir.err().unwrap().into());
            }
        }
        let dir = dir.unwrap();
        let path_ = path_from![dir, "recent-projects.json"];
        let file = PFile::from_path_reg(path_.clone());
        let mut text = FS_READ_SERVICE.read_file(&file);
        if text.is_err() {
            CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
            text = FS_READ_SERVICE.read_file(&file);
            if text.is_err() {
                return Err(text.err().unwrap().into());
            }
        }
        let text = text.unwrap();
        let mut data = PARSING_SERVICE._from_string::<Vec<RecentProject>>(&text)?;
        data.push(recent);
        data.sort_by(|a, b| a.last_opened.cmp(&b.last_opened));

        let text = PARSING_SERVICE.to_string(&data)?;
        FS_WRITE_SERVICE.write_file(&file, text, FileWriteAccess::WRITE)?;
        Ok(())
    }
}

pub struct PackageService;

impl TPackageService for PackageService {
    fn read_packages(&self) -> Result<Vec<Package>, ProjectError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path = path_from![dir, "packages"];
        let mut packages = Vec::<Package>::new();
        let dir = PDirectory::from_path(&path);
        let dirs = FS_READ_SERVICE.read_dir(&dir)?;

        for i in dirs.directories {
            let path = path_from![i.path, "config.json"];
            if FS_READ_SERVICE.exists(path.clone()) {
                let file = PFile::from_path_reg(path);
                let Ok(text) = FS_READ_SERVICE.read_file(&file) else {
                    continue;
                };
                let Ok(parsed) = PARSING_SERVICE._from_string::<Package>(&text) else {
                    continue;
                };
                packages.push(parsed);
            }
        }
        Ok(packages)
    }

    fn add_package(&self, _pack: Package) -> Result<(), ProjectError> {
        todo!()
    }

    fn rem_package(&self, _pack: Package) -> Result<(), ProjectError> {
        todo!()
    }

    fn read_config(&self, id: String) -> Result<String, ProjectError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path_to_package = path_from![dir, "packages", id, "config.js"];
        let file = PFile::from_path_reg(path_to_package);
        let text = FS_READ_SERVICE.read_file(&file)?;
        Ok(text)
    }
}
