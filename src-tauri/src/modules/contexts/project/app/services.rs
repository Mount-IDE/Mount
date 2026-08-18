use crate::modules::app::{
    APP, CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, FS_READ_SERVICE, FS_WRITE_SERVICE,
    PARSING_SERVICE,
};
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::path_from;
use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::{FileType, FileWriteAccess};
use crate::modules::contexts::project::app::traits::{
    TActionProjectService, TPackageService, TProjectService,
};
use crate::modules::contexts::project::domain::entities::{
    Action, Package, PackageAction, Project, ProjectTemplate, TaskCommand, Var, _Task,
};
use crate::modules::contexts::project::domain::values::{
    ActionOnError, CreateProjectPackageResults, CreateProjectResult, CreateProjectTemplate,
    ResultsRecord,
};
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService, TParsingService};
use crate::modules::shared::kernel::errors::{ParsingError, ProjectError};
use crate::modules::shared::kernel::utils::get_os;
use crate::modules::shared::kernel::values::{Dependency, DependencyLevel, Path, Val};
use std::collections::HashMap;
use std::ops::Deref;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Emitter;
use which::which;

#[allow(unused)]
fn get_key(sh: String) -> String {
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

pub struct ProjectService();

impl TProjectService for ProjectService {
    ///
    ///
    ///
    fn create_project(&self, project: &Project) -> Result<(), ProjectError> {
        let path_to_project = path_from![project.path];

        FS_WRITE_SERVICE.create_dir(&path_to_project)?;

        let path_to_mount = path_from![path_to_project, project.name, ".mount",];
        FS_WRITE_SERVICE.create_dir(&path_to_mount)?;
        let str = serde_json::to_string(&project).map_err(|e| ProjectError::ParsingError {
            err: ParsingError::Serialize {
                path: path_to_mount.clone(),
                err: e,
            },
        })?;

        let path_to_conf = path_from![path_to_project, project.name, "project.json",];
        let file = PFile::from_path_reg(path_to_conf);
        FS_WRITE_SERVICE.write_file(&file, str, FileWriteAccess::WRITE)?;

        Ok(())
    }

    ///
    ///
    ///
    fn open_project(&self, project_path: &Path) -> Result<Project, ProjectError> {
        // println!("path open {}", project_path.clone());
        let path = path_from![project_path, ".mount", "project.json",];
        let config = PFile {
            name: "project.json".to_string(),
            path: path.clone(),
            typ: FileType::REGULAR,
        };
        // println!("path {path}");
        let json = FS_READ_SERVICE.read_file(&config)?;
        let proj =
            serde_json::from_str::<Project>(&json).map_err(|e| ProjectError::ParsingError {
                err: ParsingError::Deserialize {
                    json: json.clone(),
                    path: config.path.clone(),
                    err: e,
                },
            })?;
        Ok(proj)
    }

    ///
    ///
    ///
    fn delete_project(&self, project_path: &Path) -> Result<Project, ProjectError> {
        let path_ = path_from![project_path, ".mount", "project.json",];
        let file = PFile::from_path_reg(path_.clone());
        let text = FS_READ_SERVICE.read_file(&file)?;
        let json = serde_json::from_str(&text).map_err(|e| ParsingError::Deserialize {
            json: text.clone(),
            err: e,
            path: path_.clone(),
        })?;

        let proj_dir = PDirectory::from_path(&project_path.clone());
        FS_WRITE_SERVICE.remove_dir(&proj_dir)?;
        Ok(json)
    }

    ///
    ///
    ///
    fn get_projects(&self, dir: &Path) -> Result<Vec<Project>, ProjectError> {
        let dir = PDirectory {
            name: "".to_string(),
            path: dir.clone(),
            files: vec![],
            directories: vec![],
        };
        let directory = FS_READ_SERVICE.read_dir(&dir)?;
        let mut projects: Vec<Project> = vec![];
        for dir in directory.directories {
            let mount = dir.directories.iter().find(|e| e.name == ".mount");
            if mount.is_none() {
                continue;
            }
            let mount = mount.unwrap().clone();
            let path_to = path_from![mount.path, "project.json"];
            let file = PFile {
                name: "project.json".to_string(),
                path: path_to.clone(),
                typ: FileType::REGULAR,
            };
            let file = FS_READ_SERVICE.read_file(&file);
            if file.is_err() {
                continue;
            }
            let file = file.unwrap();
            let proj =
                serde_json::from_str::<Project>(&file).map_err(|e| ProjectError::ParsingError {
                    err: ParsingError::Deserialize {
                        json: file,
                        path: path_to.clone(),
                        err: e,
                    },
                })?;
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
        let projects = PARSING_SERVICE._from_string::<Vec<RecentProject>>(text)?;
        Ok(projects)
    }
    ///
    ///
    ///
    fn save_project(&self, _project: &Project) -> Result<(), ProjectError> {
        // println!("PROJECT SAVING...");
        let path = _project.path.clone();
        let json =
            serde_json::to_string(&_project.clone()).map_err(|e| ParsingError::Serialize {
                path: path.clone(),
                err: e,
            })?;
        // println!("PROJECT JSON CONSTRUCTED");
        let path_ = path_from![path, _project.name, ".mount", "project.json",];
        let file = PFile::from_path_reg(path_);
        // println!("FILE {file:?}");
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
        // println!("PROJECT SAVED");
        Ok(())
    }

    fn remove_from_recents(&self, _proj: &Project) -> Result<(), ProjectError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path_ = path_from![dir, "recent-projects.json"];
        let file = PFile::from_path_reg(path_.clone());
        let text = FS_READ_SERVICE.read_file(&file)?;
        let json =
            serde_json::from_str::<Vec<RecentProject>>(text.clone().as_str()).map_err(|e| {
                ParsingError::Deserialize {
                    path: path_.clone(),
                    json: text,
                    err: e,
                }
            })?;
        let new_json = json
            .iter()
            .filter(|el| {
                let path1 = path_from![_proj.path, _proj.name,];
                let path2 = path_from![el.path, el.name,];
                // println!("PATH {path1} {path2}");
                return path1.get() != path2.get();
            })
            .map(|e| e.clone())
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
        let mut data = PARSING_SERVICE._from_string::<Vec<RecentProject>>(text)?;
        data.push(recent.clone());
        data.sort_by(|a, b| a.last_opened.cmp(&b.last_opened));

        let text = PARSING_SERVICE.to_string(&data)?;
        FS_WRITE_SERVICE.write_file(&file, text, FileWriteAccess::WRITE)?;
        Ok(())
    }

    fn check_dependencies(&self, dependencies: Vec<Dependency>) -> Vec<Dependency> {
        let os = get_os();
        let mut error: Vec<Dependency> = vec![];
        for i in dependencies {
            let prog = i.program.clone();
            let platform = i.platform.clone();
            if let Some(plat) = platform {
                if os != plat {
                    continue;
                }
            }
            let res = which(prog);
            if let Err(_) = res {
                if i.level != DependencyLevel::CONFLICTS {
                    error.push(i.clone());
                    continue;
                }
            }
            if let DependencyLevel::CONFLICTS = i.level {
                error.push(i.clone())
            }
        }
        error
    }
}

#[allow(unused)]
pub struct ActionProjectService();

#[allow(unused)]
const VAR_PREFIX: &str = "@";

#[allow(unused)]
const PARAM_PREFIX: &str = "#";

impl TActionProjectService for ActionProjectService {
    fn compile(
        &self,
        template: &ProjectTemplate,
        values: &CreateProjectResult,
        vars: &Vec<Var>,
        packages: &Vec<Package>,
        pack_results: &CreateProjectPackageResults,
    ) -> Option<(Vec<Var>, Vec<_Task>)> {
        let meta = values.get("__meta__").unwrap();
        let mut sections = vec![meta.to_owned()];
        let current_values = values.get(template.id.clone().as_str());
        if let Some(cur) = current_values {
            sections.push(cur.clone());
        }
        let vars = self.compile_vars(vars, &sections);
        if let None = vars.clone() {
            return None;
        }
        let mut vars = vars.unwrap();

        let actions = template.startup.actions.clone();
        let mut tasks = Vec::<_Task>::new();

        for action in actions.iter() {
            if let Some(_) = action.if_ {
                let cond = self.precompile_condition(
                    &sections,
                    &vars,
                    &action,
                    false,
                    &(HashMap::new() as ResultsRecord),
                );
                if !cond {
                    continue;
                }
            }
            let task__ = self.make_task(
                &action,
                &actions,
                &vars,
                &sections,
                false,
                &(HashMap::new() as ResultsRecord),
            );
            if let Some(task_) = task__ {
                tasks.push(task_);
            }
        }

        for package in packages {
            if let None = package.startup.actions.clone() {
                continue;
            }
            let actions = package.startup.actions.clone().unwrap();
            let vars_ = package.var.clone();
            if let Some(var) = vars_ {
                for i in var {
                    vars.push(i);
                }
            }
            let Some(results) = pack_results.get(&package.id) else {
                continue;
            };
            for action in actions.iter() {
                let cond = self.precompile_condition(&sections, &vars, &action, true, &results);
                if !cond {
                    continue;
                }
                let task__ = self.make_task(&action, &actions, &vars, &sections, false, &results);
                if let Some(task) = task__ {
                    tasks.push(task);
                }
            }
        }

        Some((vars, tasks))
    }

    ///
    ///
    ///
    fn precompile_condition(
        &self,
        sections: &Vec<CreateProjectTemplate>,
        vars: &Vec<Var>,
        action: &PackageAction,
        is_pack: bool,
        pack_params: &ResultsRecord,
    ) -> bool {
        let condition = action.if_.clone().unwrap();
        let mut pass_conditions = true; // check conditions
        'cond: for cond in condition {
            let mut passed = 0;
            'part: for part in cond {
                let mut from = Val::NONE;
                if let Some(from_) = part.from {
                    if from_.starts_with(VAR_PREFIX) {
                        let res = self.get_from_vars(&vars, from_);
                        if let Some(val) = res {
                            from = val;
                        } else {
                            continue 'part;
                        }
                    } else if from_.starts_with(PARAM_PREFIX) {
                        if is_pack {
                            let res = self.get_from_pack_params(pack_params, from_.clone());
                            if let Some(v) = res {
                                from = v;
                            } else {
                                let Some(val) = self.get_from_params(&sections, from_.clone())
                                else {
                                    continue 'part;
                                };
                                from = val;
                            }
                        } else {
                            let Some(val) = self.get_from_params(&sections, from_) else {
                                continue 'part;
                            };
                            from = val;
                        }
                    } else {
                        from = Val::STRING(from_);
                    }
                }
                let op = part.oper.get_fn();

                let mut val = Val::NONE;
                if let Some(value) = part.value {
                    if let Val::STRING(val_) = value.clone() {
                        if val_.starts_with(PARAM_PREFIX) {
                            if is_pack {
                                let res = self.get_from_pack_params(pack_params, val_.clone());
                                if let Some(res) = res {
                                    val = res;
                                } else {
                                    let Some(res) = self.get_from_params(&sections, val_.clone())
                                    else {
                                        continue 'part;
                                    };
                                }
                            } else {
                                let Some(res) = self.get_from_params(&sections, val_.clone())
                                else {
                                    continue 'part;
                                };
                                val = res
                            }
                        }
                        if val_.starts_with(VAR_PREFIX) {
                            val = self.get_from_vars(&vars, val_.clone()).unwrap();
                        }
                    } else {
                        val = value;
                    }
                }
                let res = op(from, val);
                if res {
                    passed += 1;
                    break 'part;
                }
            }
            if passed == 0 {
                pass_conditions = false;
                break 'cond;
            }
        }
        pass_conditions // if conditions not passed, then action truncates
    }

    fn format(
        &self,
        string: String,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
        is_pack: bool,
        pack_param: &ResultsRecord,
    ) -> Option<String> {
        enum Token {
            Str(String),
            Var(String),
            Param(String),
        }

        let mut tokens = Vec::<Token>::new();

        let mut is_str = true;
        let mut is_slash = false;
        let mut is_slash2 = false;
        let mut buff = String::new();
        for i in string.chars().peekable() {
            if is_str {
                if i.to_string() == PARAM_PREFIX || i.to_string() == VAR_PREFIX {
                    is_str = false;
                    tokens.push(Token::Str(buff.clone()));
                    buff.clear();
                    buff.push(i)
                } else {
                    buff.push(i)
                }
            } else {
                if !is_slash2 && is_slash && i.to_string() == "}" {
                    if buff.starts_with(PARAM_PREFIX) {
                        tokens.push(Token::Param(buff.clone()));
                    } else {
                        tokens.push(Token::Var(buff.clone()))
                    }
                    buff.clear();
                    is_slash2 = false;
                    is_slash = false;
                    continue;
                }
                if !is_slash && i.to_string() == " " {
                    if buff.starts_with(PARAM_PREFIX) {
                        tokens.push(Token::Param(buff.clone()));
                    } else {
                        tokens.push(Token::Var(buff.clone()))
                    }
                    buff.clear();
                    is_slash = false;
                    is_slash2 = false;
                    buff.push(i);
                    continue;
                }
                if is_slash2 {
                    is_slash2 == false;
                }
                if !is_slash && i.to_string() == "{" {
                    is_slash = true;
                    continue;
                }
                if i.to_string() == "\\" && !is_slash2 {
                    is_slash2 = true
                }

                buff.push(i)
            }
        }

        let mut string = String::new();

        for i in tokens {
            if let Token::Str(v) = i {
                string += &v;
                continue;
            }
            if let Token::Var(v) = i {
                let Some(val) = self.get_from_vars(vars, v) else {
                    return None;
                };

                string += val.to_str().as_str();
            } else if let Token::Param(v) = i {
                if is_pack {
                    let val = self.get_from_pack_params(pack_param, v.clone());
                    if let Some(v) = val {
                        string += v.to_str().as_str();
                    } else {
                        let Some(v) = self.get_from_params(params, v) else {
                            return None;
                        };
                        string += v.to_str().as_str();
                    }
                } else {
                    let Some(val) = self.get_from_params(params, v) else {
                        return None;
                    };
                    string += val.to_str().as_str();
                }
            }
        }
        Some(string)
    }

    ///
    ///
    ///
    fn get_from_vars(&self, vars: &Vec<Var>, addr: String) -> Option<Val> {
        let mut address = addr.clone();
        if addr.starts_with(VAR_PREFIX) {
            address = addr
                .chars()
                .skip(VAR_PREFIX.chars().count())
                .collect::<String>();
        }
        for var in vars {
            if var.name == address {
                return Some(var.value.clone());
            }
        }
        None
    }

    ///
    ///
    ///
    fn get_from_params(&self, params: &Vec<CreateProjectTemplate>, addr: String) -> Option<Val> {
        let mut address = addr.clone();
        if addr.starts_with(PARAM_PREFIX) {
            address = addr
                .chars()
                .skip(PARAM_PREFIX.chars().count())
                .collect::<String>();
        }
        // println!("PARAM FROM {address}");
        let address = Action::getaddr(address);
        // println!("PARAM GET {address:?}");
        if let Ok((section, parameter)) = address {
            let parameter = parameter.chars().skip(1).collect::<String>();
            // println!("SEC PAR {section} {parameter}");
            for sect in params {
                let section_res = sect.get(&section);
                // println!("SECTION RES {section_res:?}");
                if let Some(section) = section_res {
                    let param = section.get(&parameter);
                    // println!("PARAM RES {param:?}");
                    if let Some(par) = param {
                        return Some(par.clone());
                    }
                }
            }
        }
        None
    }

    fn get_from_pack_params(&self, params: &ResultsRecord, addr: String) -> Option<Val> {
        let addr = if addr.starts_with(PARAM_PREFIX) {
            addr.chars().skip(PARAM_PREFIX.chars().count()).collect()
        } else {
            addr
        };

        let got = params.get(&addr)?;
        Some(got.clone())
    }

    ///
    ///
    ///
    fn compile_vars(
        &self,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
    ) -> Option<Vec<Var>> {
        let mut res = Vec::<Var>::new();
        for i in vars {
            let mut param = false;
            if let Val::STRING(val) = i.value.clone() {
                if val.starts_with(PARAM_PREFIX) {
                    let res_ = self.get_from_params(&params, val.clone());
                    if let None = res_ {
                        return None;
                    }
                    param = true;
                    res.push(Var {
                        name: i.name.clone(),
                        value: res_.unwrap(),
                    });
                }
            }
            if !param {
                res.push(i.clone());
            }
        }
        Some(res)
    }

    ///
    ///
    ///
    fn make_task(
        &self,
        action: &PackageAction,
        actions: &Vec<PackageAction>,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
        is_pack: bool,
        pack_params: &ResultsRecord,
    ) -> Option<_Task> {
        let def_ = || {
            if cfg!(target_os = "linux") {
                std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string())
            } else if cfg!(target_os = "windows") {
                "powershell".to_string()
            } else {
                "zsh".to_string()
            }
        };

        let mut commands = Vec::<TaskCommand>::new();
        let commands_ = action.command.clone()?;

        if let Some(plat) = action.platform.clone() {
            if !plat.is_correct() {
                return None;
            }
        }

        for command in commands_ {
            if let Some(plat) = command.platform {
                if !plat.is_correct() {
                    continue;
                }
            }

            let command_ = self.format(command.command, vars, params, is_pack, pack_params);
            if let None = command_ {
                continue;
            }
            let mut shell = if let Some(v) = command.shell {
                v
            } else {
                if cfg!(windows) {
                    "cmd".to_string()
                } else {
                    "sh".to_string()
                }
            };

            let task_command = TaskCommand {
                shell,
                command: command_.unwrap(),
                env: command.env.clone(),
            };
            commands.push(task_command);
        }

        if let Some(next) = action.next.clone() {
            let act = actions.iter().find(|el| {
                if let Some(val) = el.next.clone() {
                    return next == val;
                }
                false
            });
            if let Some(val) = act {
                let inner = self.make_task(&val, &actions, &vars, &params, is_pack, &pack_params);
                if let Some(inner) = inner {
                    let res = _Task::GRAPH {
                        commands,
                        on_error: val.on_error.clone(),
                        next: Box::new(inner),
                    };
                    return Some(res);
                }
            }
        }
        let task = _Task::SINGLE {
            commands,
            on_error: action.on_error.clone(),
        };
        Some(task)
    }

    fn run_tasks(&self, project: &Project, tasks: &Vec<_Task>, window: String) {
        let app = { APP.get().unwrap() };
        let _ = app.emit_to(
            window.clone(),
            "task-run",
            format!("Compiled tasks: {}", tasks.len()),
        );
        let path_to = path_from![project.path, project.name,];
        //let proj = project.clone();
        let mut n = 1;
        for task in tasks {
            let _ = app.emit_to(window.clone(), "task-start", format!("{n}"));
            let err = self.run_task(&task, &path_to);
            let _ = app.emit_to(window.clone(), "task-end", format!("{n}"));
            //println!("ERR? {err}");
            if err >= 0 {
                //println!("ERRR!");
                let _ = app.emit_to(window.clone(), "task-error", format!("{n}"));
            }
            if err == 2 {
                break;
            }
            n += 1;
        }
    }
    fn run_task(&self, task: &_Task, path: &Path) -> i8 {
        // println!("TYPE TASK {:?}", task);
        match task.clone() {
            _Task::GRAPH {
                next,
                commands,
                on_error,
            } => {
                let mut res_ = -1i8;
                for command in commands {
                    let key = get_key(command.shell.clone());
                    let mut process = Command::new(&command.shell);
                    process.arg(&key).arg(&command.command).stdin(Stdio::null());

                    if let Some(env) = command.env.clone() {
                        let mut map = HashMap::<String, String>::new();
                        for (first, sec) in env {
                            map.insert(first, sec);
                        }
                        process.envs(map.iter());
                    }
                    process.current_dir(path.get().clone());
                    let code = process.status();
                    if let Err(_) = code {
                        res_ = match on_error {
                            ActionOnError::CONTINUE => 0,
                            ActionOnError::StopAll => 2,
                            ActionOnError::StopGraph => 1,
                        }
                    } else {
                        let code = code.unwrap();
                        if !code.success() {
                            res_ = match on_error {
                                ActionOnError::CONTINUE => 0,
                                ActionOnError::StopAll => 2,
                                ActionOnError::StopGraph => 1,
                            }
                        }
                    }
                    if res_ > 0 {
                        return res_;
                    }
                }
                let new_task = next.deref();
                res_ = self.run_task(new_task, path);
                res_
            }
            _Task::SINGLE { commands, on_error } => {
                let mut res_ = -1i8;
                for command in commands {
                    let key = get_key(command.shell.clone());
                    let mut process = Command::new(&command.shell);
                    process.arg(&key).arg(&command.command);

                    if let Some(env) = command.env.clone() {
                        let mut map = HashMap::<String, String>::new();
                        for (first, sec) in env {
                            map.insert(first, sec);
                        }
                        process.envs(map.iter());
                    }
                    process.current_dir(path.get().clone());
                    let code = process.status();
                    if let Err(_) = code {
                        res_ = match on_error {
                            ActionOnError::CONTINUE => 0,
                            ActionOnError::StopAll => 2,
                            ActionOnError::StopGraph => 1,
                        }
                    } else {
                        let code = code.unwrap();
                        if !code.success() {
                            res_ = match on_error {
                                ActionOnError::CONTINUE => 0,
                                ActionOnError::StopAll => 2,
                                ActionOnError::StopGraph => 1,
                            }
                        }
                    }
                }
                res_
            }
        }
    }
}

pub struct PackageService();
pub struct PackageCompileService();

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
                let Ok(parsed) = PARSING_SERVICE._from_string::<Package>(text) else {
                    continue;
                };
                packages.push(parsed);
            }
        }
        Ok(packages)
    }

    fn add_package(&self, pack: Package) -> Result<(), ProjectError> {
        todo!()
    }

    fn rem_package(&self, pack: Package) -> Result<(), ProjectError> {
        todo!()
    }
}

/*impl TPackageCompileService for PackageCompileService {
    fn compile_package_actions(
        &self,
        pack: Package,
        results: &CreateProjectPackageResults,
    ) -> Result<(Vec<Var>, Vec<_Task>), ProjectError> {
        let id = pack.id;
        let needed = results.get(&id);
        let actions = pack.startup.actions;
        if let None = actions {
            return Ok((pack.var.unwrap_or(vec![]).clone(), vec![]));
        }
        let actions = actions.unwrap();

        let mut actions2 = Vec::<PackageAction>::new();
        for i in actions {
            if let Some(plat) = i.platform.clone() {
                if !plat.is_correct() {
                    continue;
                }
            }
            let if_ = i.if_.clone();
            if let None = if_ {
                actions2.push(i.clone());
                continue;
            }
            let if_ = if_.unwrap();
            let mut results: Vec<bool> = vec![];
            'or: for or in if_ {
                let mut all = true;
                'and: for and in or {
                    let from = and.from;
                    let op = and.oper;
                    let value = and.value;

                    let op = op.get_fn();
                }

                results.push(all)
            }
        }

        Ok((pack.var.unwrap_or(vec![]).clone(), vec![]))
    }
}
*/
