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
    ActionCommandArgs, ActionCommandIn, ActionOnError, CreateProjectResult, CreateProjectTemplate,
};
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService};
use crate::modules::shared::kernel::errors::{ParsingError, ProjectError};
use crate::modules::shared::kernel::values::{Path, Val};
use regex::Regex;
use std::collections::HashMap;
use std::ops::Deref;
use std::process::{Command, Stdio};
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
    fn delete_project(&self, project_path: &Path) -> Result<Project, ProjectError> {
        let path_ = make_path(vec![
            project_path.get().clone().as_str(),
            ".mount",
            "project.json",
        ]);
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
        let path = _project.path.clone();
        let json =
            serde_json::to_string(&_project.clone()).map_err(|e| ParsingError::Serialize {
                path: path.clone(),
                err: e,
            })?;
        let file = PFile::from_path_reg(path.clone());
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
        Ok(())
    }

    fn remove_from_recents(&self, _proj: &Project) -> Result<(), ProjectError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path_ = make_path(vec![dir.get().as_str(), "recent-projects.json"]);
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
                let path1 = make_path(vec![
                    _proj.path.get().clone().as_str(),
                    _proj.name.clone().as_str(),
                ]);
                let path2 = make_path(vec![
                    el.path.get().clone().as_str(),
                    el.name.clone().as_str(),
                ]);
                println!("PATH {path1} {path2}");
                return path1.get() != path2.get();
            })
            .map(|e| e.clone())
            .collect::<Vec<RecentProject>>();

        let json = serde_json::to_string(&new_json).map_err(|e| ParsingError::Serialize {
            path: path_.clone(),
            err: e,
        })?;
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
        // println!("VAR {vars:?}");
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
            if let Some(_) = action.if_ {
                let cond = self.precompile_condition(&sections, &vars, &action);
                // println!("IF RES {cond}");
                if !cond {
                    continue;
                }
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
        let condition = action.if_.clone().unwrap();
        let mut pass_conditions = true; // check conditions
        'cond: for cond in condition {
            //and
            // println!("-----------------------------------------");
            let mut passed = 0;
            'part: for part in cond {
                // println!("....................");
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
                    // println!("IF PARAM {res:?} {}", part.from.clone());
                    if let Some(val) = res {
                        from = val;
                    } else {
                        continue 'part;
                    }
                }

                // == != < > <= >= in reg !reg !in len
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
                    "len" => |a: Val, b: Val| -> bool {
                        let val = match a {
                            Val::NUMBER(v) => Val::NUMBER(v),
                            Val::STRING(v) => Val::NUMBER(v.chars().count() as f64),
                            Val::BOOL(v) => Val::NUMBER(if v { 1f64 } else { 0f64 }),
                            Val::ARRAY(v) => Val::NUMBER(v.len() as f64),
                        };
                        return val == b;
                    },
                    "!empty" => |a: Val, b: Val| -> bool {
                        match a {
                            Val::STRING(v) => !v.is_empty(),
                            Val::ARRAY(v) => !v.is_empty(),
                            _ => false,
                        }
                    },
                    "empty" => |a: Val, b: Val| -> bool {
                        match a {
                            Val::STRING(v) => v.is_empty(),
                            Val::ARRAY(v) => v.is_empty(),
                            _ => false,
                        }
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
                // println!("FROM TO {from:?} {val:?} {}", part.oper.clone());
                let res = op(from, val);
                if res {
                    passed += 1;
                    break 'part;
                }
            }
            // println!("IF PASS {passed}");
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

    ///
    ///
    ///
    fn format_string(
        &self,
        input: String,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
    ) -> Option<String> {
        #[derive(Debug, Clone, Copy)]
        enum State {
            Normal,
            Var,
            Param,
        }

        let mut result = String::new();
        let mut state = State::Normal;

        let mut buf = String::new();
        let mut brace_mode = false;

        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match state {
                State::Normal => {
                    if ch == VAR_PREFIX.chars().next().unwrap() {
                        state = State::Var;
                        buf.clear();
                        brace_mode = false;
                    } else if ch == PARAM_PREFIX.chars().next().unwrap() {
                        state = State::Param;
                        buf.clear();
                        brace_mode = false;
                    } else {
                        result.push(ch);
                    }
                }

                State::Var | State::Param => {
                    // вход в {token}
                    if ch == '{' && buf.is_empty() {
                        brace_mode = true;
                        continue;
                    }

                    if brace_mode {
                        if ch == '}' {
                            let value = if let State::Var = state {
                                self.get_from_vars(vars, format!("@{}", buf))?
                            } else {
                                self.get_from_params(params, format!("#{}", buf))?
                            };

                            result.push_str(&value.to_str());

                            state = State::Normal;
                            buf.clear();
                            brace_mode = false;
                            continue;
                        } else {
                            buf.push(ch);
                        }
                    } else {
                        // без {} — читаем до разделителя
                        if ch.is_whitespace() {
                            let value = if let State::Var = state {
                                self.get_from_vars(vars, format!("@{}", buf))?
                            } else {
                                self.get_from_params(params, format!("#{}", buf))?
                            };

                            result.push_str(&value.to_str());
                            result.push(' ');

                            state = State::Normal;
                            buf.clear();
                        } else {
                            buf.push(ch);
                        }
                    }
                }
            }
        }

        // flush tail
        match state {
            State::Normal => {}
            State::Var => {
                if !buf.is_empty() {
                    let value = self.get_from_vars(vars, format!("@{}", buf))?;
                    result.push_str(&value.to_str());
                }
            }
            State::Param => {
                if !buf.is_empty() {
                    let value = self.get_from_params(params, format!("#{}", buf))?;
                    result.push_str(&value.to_str());
                }
            }
        }

        Some(result)
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
        let def_ = || {
            if cfg!(target_os = "linux") {
                std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string())
            } else if cfg!(target_os = "windows") {
                "powershell".to_string()
            } else {
                "zsh".to_string()
            }
        };

        let on_error = match action.on_error.as_str() {
            "stop-all" => ActionOnError::StopAll,
            "stop-graph" => ActionOnError::StopGraph,
            "continue" => ActionOnError::CONTINUE,
            _ => ActionOnError::CONTINUE,
        };
        let mut commands = Vec::<TaskCommand>::new();
        let commands_ = action.command.clone();

        let mut used_platforms = Vec::<String>::new();

        let platforms = vec![
            "windows".to_string(),
            "macos".to_string(),
            "linux".to_string(),
        ];

        for command in commands_ {
            let mut platform: String = command.platform.clone();
            if platform == "!".to_string() {
                for plat in platforms.iter() {
                    if !used_platforms.contains(plat) {
                        platform = plat.clone();
                    }
                }
            }
            used_platforms.push(platform.clone());

            if *os != platform && platform != "all".to_string() {
                continue;
            }
            let command_ = match command.command.clone() {
                ActionCommandIn::Single(cmd) => Some(cmd),
                ActionCommandIn::WithArgs(ActionCommandArgs(cmd, args)) => {
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
            let shell = if command.shell == "@" {
                def_()
            } else {
                command.shell
            };
            let task_command = TaskCommand {
                shell,
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
        let proj = project.clone();

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
                    process.arg(&key).arg(&command.command).stdin(Stdio::null());

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
                    println!(
                        "RUN: {} {} {} {}",
                        if code.code().is_some() {
                            code.code().unwrap()
                        } else {
                            -20
                        },
                        command.shell,
                        key,
                        command.command
                    );
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
