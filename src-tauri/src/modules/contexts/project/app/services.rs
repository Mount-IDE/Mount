use crate::modules::app::{
    CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, FS_READ_SERVICE, FS_WRITE_SERVICE,
};
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::{make_path, make_path_string};
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::{FileType, FileWriteAccess};
use crate::modules::contexts::project::app::traits::{TActionProjectService, TProjectService};
use crate::modules::contexts::project::domain::entities::{
    Action, Project, ProjectTemplate, TaskCommand, Var, _Task,
};
use crate::modules::contexts::project::domain::values::{
    ActionCommandIn, ActionOnError, CreateProjectResult, CreateProjectTemplate,
};
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService};
use crate::modules::shared::kernel::errors::{ParsingError, ProjectError};
use crate::modules::shared::kernel::values::{Path, Val};
use regex::Regex;
use std::collections::HashMap;
use std::ops::Deref;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

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
        let path_to_project = make_path_string(vec![project.path.get().as_str()]);

        FS_WRITE_SERVICE.create_dir(&Path(path_to_project.clone()))?;

        let path_to_mount = make_path_string(vec![
            path_to_project.clone().as_str(),
            project.name.as_str(),
            ".mount",
        ]);
        FS_WRITE_SERVICE.create_dir(&Path(path_to_mount.clone()))?;
        let str = serde_json::to_string(&project).map_err(|e| ProjectError::ParsingError {
            err: ParsingError::Serialize {
                path: Path(path_to_mount.clone()),
                err: e,
            },
        })?;

        let path_to_conf = make_path_string(vec![
            path_to_project.as_str(),
            project.name.as_str(),
            "project.json",
        ]);
        let file = PFile::regular("project.json".to_string(), Path(path_to_conf));
        FS_WRITE_SERVICE.write_file(&file, str, FileWriteAccess::WRITE)?;

        Ok(())
    }

    ///
    ///
    ///
    fn open_project(&self, project_path: &Path) -> Result<Project, ProjectError> {
        // println!("path open {}", project_path.clone());
        let path = make_path(vec![
            project_path.clone().get().as_str(),
            ".mount",
            "project.json",
        ]);
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
    fn delete_project(&self, _project_path: &Path) -> Result<(), ProjectError> {
        todo!()
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
            let path_to = make_path_string(vec![mount.path.get().as_str(), "project.json"]);
            let file = PFile {
                name: "project.json".to_string(),
                path: Path(path_to.clone()),
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
                        path: Path(path_to.clone()),
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
        let path_to = make_path_string(vec![dir.path.get().as_str(), "recent-projects.json"]);
        let file = PFile::from_path_reg(Path(path_to.clone()));
        let ext = FS_READ_SERVICE.exist_file(&file);
        if !ext {
            FS_WRITE_SERVICE.create_file(&Path(path_to))?;
            return Ok(vec![]);
        }
        let file = PFile::regular("recent-projects.json".to_string(), Path(path_to));

        let text = FS_READ_SERVICE.read_file(&file)?;
        let projects = serde_json::from_str::<Vec<RecentProject>>(&text).map_err(|e| {
            ProjectError::ParsingError {
                err: ParsingError::Deserialize {
                    json: text,
                    path: file.path,
                    err: e,
                },
            }
        })?;
        Ok(projects)
    }
    ///
    ///
    ///
    fn save_project(&self, _project: &Project) -> Result<(), ProjectError> {
        todo!()
    }

    fn remove_from_recents(&self, _proj: &Project) -> Result<(), ProjectError> {
        todo!()
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
        let path_ = make_path(vec![dir.get().clone().as_str(), "recent-projects.json"]);
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
        let mut data = serde_json::from_str::<Vec<RecentProject>>(text.as_str()).map_err(|e| {
            ProjectError::ParsingError {
                err: ParsingError::Deserialize {
                    json: text.clone(),
                    path: path_.clone(),
                    err: e,
                },
            }
        })?;
        data.push(recent.clone());
        data.sort_by(|a, b| a.last_opened.cmp(&b.last_opened));

        let text = serde_json::to_string(&data).map_err(|e| ProjectError::ParsingError {
            err: ParsingError::Serialize {
                path: path_.clone(),
                err: e,
            },
        })?;
        FS_WRITE_SERVICE.write_file(&file, text, FileWriteAccess::WRITE)?;
        Ok(())
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
    ) -> Option<(Vec<Var>, Vec<_Task>)> {
        let meta = values.get("__meta__").unwrap();
        let mut sections = vec![meta.to_owned()];
        let current_values = values.get(template.id.clone().as_str());
        if let Some(cur) = current_values {
            sections.push(cur.clone());
        }
        let vars = self.compile_vars(vars, &sections);
        println!("VAR {vars:?}");
        if let None = vars.clone() {
            return None;
        }
        let vars = vars.unwrap();

        let os = if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else {
            "linux"
        };
        let os = String::from(os);
        let actions = template.startup.actions.clone();
        let mut tasks = Vec::<_Task>::new();

        for action in actions.iter() {
            let cond = self.precompile_condition(&sections, &vars, &action);
            println!("IF RES {cond}");
            if !cond {
                continue;
            }
            let task__ = self.make_task(&action, &actions, &vars, &sections, &os);
            println!("TASK RES {task__:?}");
            if let Some(task_) = task__ {
                tasks.push(task_);
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
        action: &Action,
    ) -> bool {
        let condition = action.if_.clone();
        let mut pass_conditions = true; // check conditions
        'cond: for cond in condition {
            //and
            println!("-----------------------------------------");
            let mut passed = 0;
            'part: for part in cond {
                println!("....................");
                let mut from = Val::STRING(part.from.clone());
                if part.from.starts_with(VAR_PREFIX) {
                    let res = self.get_from_vars(&vars, part.from.clone());
                    if let Some(val) = res {
                        from = val;
                    } else {
                        continue 'part;
                    }
                } else if part.from.starts_with(PARAM_PREFIX) {
                    let res = self.get_from_params(&sections, part.from.clone());
                    println!("IF PARAM {res:?} {}", part.from.clone());
                    if let Some(val) = res {
                        from = val;
                    } else {
                        continue 'part;
                    }
                }

                // == != < > <= >= in reg !reg !in
                let op = match part.oper.clone().as_str() {
                    "==" => |a: Val, b: Val| -> bool { a == b },
                    ">" => |a: Val, b: Val| -> bool { a > b },
                    "<" => |a: Val, b: Val| -> bool { a < b },
                    ">=" => |a: Val, b: Val| -> bool { a >= b },
                    "<=" => |a: Val, b: Val| -> bool { a <= b },
                    "!=" => |a: Val, b: Val| -> bool { a != b },
                    "in" => |a: Val, b: Val| -> bool {
                        if let Val::ARRAY(val) = b {
                            if let Val::STRING(elem) = a {
                                return val.contains(&elem);
                            }
                        }
                        false
                    },
                    "reg" => |a: Val, b: Val| -> bool {
                        if let Val::STRING(reg_) = b {
                            if let Val::STRING(a) = a {
                                let reg = Regex::new(reg_.as_str());
                                if let Err(_) = reg {
                                    return false;
                                }
                                return reg.unwrap().is_match(a.as_str());
                            }
                        }
                        false
                    },
                    "!reg" => |a: Val, b: Val| -> bool {
                        if let Val::STRING(reg_) = b {
                            if let Val::STRING(a) = a {
                                let reg = Regex::new(reg_.as_str());
                                if let Err(_) = reg {
                                    return false;
                                }
                                return !reg.unwrap().is_match(a.as_str());
                            }
                        }
                        false
                    },
                    "!in" => |a: Val, b: Val| -> bool {
                        if let Val::ARRAY(val) = b {
                            if let Val::STRING(elem) = a {
                                return !val.contains(&elem);
                            }
                        }
                        false
                    },
                    _ => |_: Val, _: Val| -> bool { false },
                };
                let mut val = part.value.clone();
                if let Val::STRING(val_) = val.clone() {
                    if val_.starts_with(PARAM_PREFIX) {
                        val = self.get_from_params(&sections, val_.clone()).unwrap();
                    }
                    if val_.starts_with(VAR_PREFIX) {
                        val = self.get_from_vars(&vars, val_.clone()).unwrap();
                    }
                }
                println!("FROM TO {from:?} {val:?} {}", part.oper.clone());
                let res = op(from, val);
                if res {
                    passed += 1;
                    break 'part;
                }
            }
            println!("IF PASS {passed}");
            if passed == 0 {
                pass_conditions = false;
                break 'cond;
            }
        }
        pass_conditions // if conditions not passed, then action truncates
    }

    ///
    ///
    ///
    fn get_from_vars(&self, vars: &Vec<Var>, addr: String) -> Option<Val> {
        let mut address = addr.clone();
        if addr.starts_with(VAR_PREFIX) {
            address = addr[VAR_PREFIX.len()..].to_string();
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
            address = addr[PARAM_PREFIX.len()..].to_string();
        }
        println!("PARAM FROM {address}");
        let address = Action::getaddr(address);
        println!("PARAM GET {address:?}");
        if let Ok((section, parameter)) = address {
            let parameter = parameter[1..].to_string();
            println!("SEC PAR {section} {parameter}");
            for sect in params {
                let section_res = sect.get(&section);
                println!("SECTION RES {section_res:?}");
                if let Some(section) = section_res {
                    let param = section.get(&parameter);
                    println!("PARAM RES {param:?}");
                    if let Some(par) = param {
                        return Some(par.clone());
                    }
                }
            }
        }
        None
    }

    ///
    ///
    ///
    fn format_string(
        &self,
        string: String,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
    ) -> Option<String> {
        let mut res = String::new();
        let mut start = 0;
        let mut prefix = String::new();
        let mut write = true;
        let mut is_shielded = false;
        for (i, ch) in string.char_indices().enumerate() {
            let ch = ch.1.to_string();
            if ch != PARAM_PREFIX && ch != VAR_PREFIX && write {
                res += ch.as_str();
                continue;
            }
            if !write {
                if ch.as_str() == "{" && !is_shielded && i - start == 1 {
                    is_shielded = true;
                    start = i;
                }
                if (ch.as_str() == " " || i == string.len() - 1) && !is_shielded {
                    let slice = &string[start..i];
                    let result: String;
                    match prefix.as_str() {
                        VAR_PREFIX => {
                            let res = self.get_from_vars(&vars, slice.to_string());
                            if let Some(res) = res {
                                result = res.to_str();
                            } else {
                                return None;
                            }
                        }
                        _ => {
                            let res = self.get_from_params(&params, slice.to_string());
                            if let Some(res) = res {
                                result = res.to_str()
                            } else {
                                return None;
                            }
                        }
                    };
                    res += result.as_str();
                    write = true;
                } else if ch.as_str() == "}" && is_shielded {
                    let slice = &string[start..i];
                    let result: String;
                    match prefix.as_str() {
                        VAR_PREFIX => {
                            let res = self.get_from_vars(&vars, slice.to_string());
                            if let Some(res) = res {
                                result = res.to_str();
                            } else {
                                return None;
                            }
                        }
                        _ => {
                            let res = self.get_from_params(&params, slice.to_string());
                            if let Some(res) = res {
                                result = res.to_str()
                            } else {
                                return None;
                            }
                        }
                    };
                    res += result.as_str();
                    write = true;
                }
            } else {
                write = false;
                prefix = ch.clone();
                start = i;
            }
        }
        Some(res)
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
        action: &Action,
        actions: &Vec<Action>,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
        os: &String,
    ) -> Option<_Task> {
        let on_error = match action.on_error.as_str() {
            "stop_all" => ActionOnError::StopAll,
            "stop_graph" => ActionOnError::StopGraph,
            "continue" => ActionOnError::CONTINUE,
            _ => ActionOnError::CONTINUE,
        };
        let mut commands = Vec::<TaskCommand>::new();
        let commands_ = action.command.clone();
        for command in commands_ {
            if *os != command.platform {
                continue;
            }
            let command_ = match command.command.clone() {
                ActionCommandIn::Single(cmd) => Some(cmd),
                ActionCommandIn::WithArgs(cmd, args) => {
                    let mut command_ = cmd.clone();
                    for arg in args {
                        let res = self.format_string(arg, &vars, &params);
                        if let None = res {
                            return None;
                        }
                        command_ = format!("{command_} {}", res.unwrap());
                    }
                    Some(command_)
                }
            };
            if let None = command_ {
                continue;
            }
            let task_command = TaskCommand {
                shell: command.shell,
                command: command_.unwrap(),
                env: command.env.clone(),
            };
            commands.push(task_command);
        }

        if let Some(next) = action.next {
            let act = actions.iter().find(|el| {
                if let Some(val) = el.next {
                    return next == val;
                }
                false
            });
            if let Some(val) = act {
                let inner = self.make_task(&val, &actions, &vars, &params, &os);
                if let Some(inner) = inner {
                    let res = _Task::GRAPH {
                        commands,
                        on_error,
                        next: Box::new(inner),
                    };
                    return Some(res);
                }
            }
        }
        let task = _Task::SINGLE { commands, on_error };
        Some(task)
    }

    fn run_tasks(&self, project: &Project, tasks: &Vec<_Task>) {
        let path_to = make_path(vec![
            project.path.clone().get().as_str(),
            project.name.clone().as_str(),
        ]);
        for task in tasks {
            let err = self.run_task(&task, &path_to);
            if err == 2 {
                break;
            }
        }
    }
    fn run_task(&self, task: &_Task, path: &Path) -> i8 {
        match task.clone() {
            _Task::GRAPH {
                next,
                commands,
                on_error,
            } => {
                let mut res_ = 0i8;
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
                    let code = process.status().unwrap();
                    if !code.success() {
                        match on_error {
                            ActionOnError::CONTINUE => (),
                            ActionOnError::StopAll => return 2,
                            ActionOnError::StopGraph => return 1,
                        }
                    }
                    let new_task = next.deref();
                    res_ = self.run_task(new_task, path);
                }
                res_
            }
            _Task::SINGLE { commands, on_error } => {
                let mut res_ = 0i8;
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
                    let code = process.status().unwrap();
                    if !code.success() {
                        res_ = match on_error {
                            ActionOnError::CONTINUE => 0,
                            ActionOnError::StopAll => 2,
                            ActionOnError::StopGraph => 1,
                        }
                    }
                }
                res_
            }
        }
    }
}
