use format::FORMAT_SERVICE;
use format::services::FormatService;
use format::traits::TFormatService;
use macroses::log;
use mount_core::domain::platform::{Platform, PlatformType};
use mount_core::domain::{IfStatementPart, Path, Shell, Val, Var};
use mount_core::traits::PathPart;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Command, Stdio};
#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub enum ActionOnError {
    #[serde(rename = "continue")]
    CONTINUE,
    #[serde(rename = "stop_all")]
    StopAll,
    #[serde(rename = "stop_graph")]
    StopGraph,
}

impl Default for ActionOnError {
    fn default() -> Self {
        Self::StopAll
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageAction {
    pub id: i8,
    pub next: Option<Vec<i8>>,
    pub if_: Option<Vec<Vec<IfStatementPart>>>,
    pub on_error: ActionOnError,
    pub platform: Option<Platform>,
    pub command: Option<Vec<PackageActionCommand>>,
}

impl PackageAction {
    pub fn check_condition(&self, results: &HashMap<String, Val>, vars: &Vec<Var>) -> bool {
        log!("\t\t{:?}", self.if_);
        if let Some(if_) = &self.if_ {
            let mut counter = 0;
            'or: for or in if_.iter() {
                for and in or.iter() {
                    let first = if let Some(v) = and.from.clone() {
                        let Ok(res) = FORMAT_SERVICE.format_string(v, vars, results) else {
                            continue 'or;
                        };
                        Val::STRING(res)
                    } else {
                        Val::STRING("".__get())
                    };
                    let second = if let Some(v) = and.value.clone() {
                        if let Val::STRING(v) = v {
                            let Ok(v) = FORMAT_SERVICE.format_string(v, vars, results) else {
                                continue 'or;
                            };
                            Val::STRING(v)
                        } else {
                            v
                        }
                    } else {
                        Val::STRING("".__get())
                    };
                    let op = and.oper.get_fn();
                    let res = op(first, second);
                    if !res {
                        continue 'or;
                    }
                }
                counter += 1;
            }
            return counter > 0;
        }
        true
    }

    pub fn check_platform(&self) -> bool {
        let cur = Platform::current();
        match (cur, self.platform.clone()) {
            (Platform::SINGLE(v), Some(Platform::ARRAY(v2))) => {
                for i in v2.iter() {
                    if *i == PlatformType::ALL {
                        return true;
                    }
                    if *i == v {
                        return true;
                    }
                }
                false
            }
            (Platform::SINGLE(v), Some(Platform::SINGLE(v2))) => v == v2 || v2 == PlatformType::ALL,
            (_, None) => true,
            _ => false,
        }
    }

    pub fn compile(
        &self,
        actions: &Vec<PackageAction>,
        vars: &Vec<Var>,
        filtered_results: &HashMap<String, Val>,
        used_ids: &mut Vec<i8>,
        is_graph: bool,
    ) -> Result<Vec<ActionTask>, ()> {
        if used_ids.contains(&self.id) {
            log!("ID IN LOG");
            return Err(());
        }
        let mut tasks = Vec::<TaskCommand>::new();
        if let Some(cmd) = &self.command {
            for i in cmd.iter() {
                log!("\t iteration {i:?}");
                // check platform
                if let Some(p) = &i.platform {
                    if !p.is_correct() {
                        log!("INCORRECT PLATFORM {p:?}");
                        continue;
                    }
                }
                let fmt = FormatService::format_string;
                let command = match i.command.clone() {
                    PackageActionCommandType::Single(c) => {
                        let Ok(cmd) = fmt(&FORMAT_SERVICE, c, &vars, &filtered_results) else {
                            log!("INCORRECT FORMATTING1");
                            continue;
                        };
                        PackageActionCommandType::Single(cmd)
                    }
                    PackageActionCommandType::WithArgs { command, args } => {
                        let Ok(cmd) = fmt(&FORMAT_SERVICE, command, &vars, &filtered_results)
                        else {
                            log!("INCORRECT FORMATTING1");
                            continue;
                        };
                        let args = args
                            .iter()
                            .filter_map(|e| {
                                fmt(&FORMAT_SERVICE, e.clone(), &vars, &filtered_results).ok()
                            })
                            .collect::<Vec<_>>();
                        PackageActionCommandType::WithArgs { command: cmd, args }
                    }
                };
                /*let Ok(command) = fmt(&FORMAT_SERVICE, i.command.clone(), &vars, &filtered_results)
                else {
                    log!("INCORRECT FORMATTING1");
                    continue;
                };*/

                let command = match command {
                    PackageActionCommandType::Single(c) => {
                        vec![c]
                    }
                    PackageActionCommandType::WithArgs { command, args } => {
                        let mut vec = vec![command];
                        args.iter().for_each(|e| vec.push(e.clone()));
                        vec
                    }
                };
                let env = if let Some(env) = &i.env {
                    let mut map = HashMap::new();
                    for j in env.iter() {
                        let f = if let Ok(r) =
                            fmt(&FORMAT_SERVICE, j.0.clone(), &vars, &filtered_results)
                        {
                            r
                        } else {
                            j.0.clone()
                        };
                        let s = if let Ok(r) =
                            fmt(&FORMAT_SERVICE, j.1.clone(), &vars, &filtered_results)
                        {
                            r
                        } else {
                            j.1.clone()
                        };
                        map.insert(f, s);
                    }
                    map
                } else {
                    HashMap::new()
                };
                tasks.push(TaskCommand {
                    command,
                    shell: Shell::default(),
                    env: Some(env),
                })
            }
        }

        let mut act_tasks = vec![ActionTask {
            commands: tasks,
            on_error: self.on_error.clone(),
            from_graph: is_graph,
            cwd: "".to_string(),
        }];

        used_ids.push(self.id.clone());
        if let Some(next) = &self.next {
            for i in next.iter() {
                if used_ids.contains(&i) {
                    continue;
                }
                let Some(act) = actions.iter().find(|e| e.id.eq(i)) else {
                    continue;
                };
                if used_ids.contains(&act.id) {
                    continue;
                }
                let Ok(tasks) = act.compile(&actions, &vars, &filtered_results, used_ids, true)
                else {
                    continue;
                };
                for j in tasks.into_iter() {
                    act_tasks.push(j)
                }
            }
        }

        Ok(act_tasks)
    }
}

impl Default for PackageAction {
    fn default() -> Self {
        Self {
            id: 0,
            next: None,
            if_: None,
            on_error: Default::default(),
            platform: None,
            command: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageActionCommand {
    pub platform: Option<Platform>,
    pub cwd: Option<String>,
    pub shell: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub needed_exit_code: Option<Vec<i64>>,
    pub command: PackageActionCommandType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PackageActionCommandType {
    Single(String),
    WithArgs { command: String, args: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub id: String,
    pub title: String,
    pub typ: PackageOptionTyp,
    pub def: Option<Val>,
    pub while_: Option<String>,
}

#[allow(unused)]
pub type ParameterId = String;

impl Parameter {
    #[allow(unused)]
    pub fn filter_by_while(
        results: &HashMap<ParameterId, Val>,
        parameters: Vec<Self>,
    ) -> Vec<Self> {
        let mut res = Vec::<Self>::new();
        for i in parameters.iter() {
            if let Some(w) = i.while_.clone() {
                let Some(found) = parameters.iter().find(|e| e.id == w) else {
                    res.push(i.clone());
                    continue;
                };
                let Some(found2) = results.get(&i.id) else {
                    res.push(i.clone());
                    continue;
                };
                if let Some(def) = &found.def {
                    if *def != *found2 {
                        //
                        res.push(i.clone())
                    }
                } else {
                    res.push(i.clone())
                }
            } else {
                res.push(i.clone())
            }
        }
        res
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageOptionTyp {
    pub typ: PackageOptionTypEnum,
    pub fs_type: Option<PackageOptionFsType>,
    pub list_type: Option<Vec<String>>,
    pub placeholder: Option<String>,
    pub required: Option<bool>,
    pub readonly: Option<bool>,
    pub validate: Option<String>,
    pub fs_filter: Option<Vec<String>>,
    pub gen_min: Option<u8>,
    pub gen_max: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageOptionFsType {
    FILE,
    DIR,
    FS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageOptionTypEnum {
    INPUT,
    AREA,
    LIST,
    GEN,
    FILE,
    CHECK,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionTask {
    pub commands: Vec<TaskCommand>,
    pub on_error: ActionOnError,
    pub from_graph: bool,
    pub cwd: String,
}

impl ActionTask {
    pub fn execute(&self, path: Path) -> ActionOnError {
        for i in &self.commands {
            let key = i.shell.get_key().unwrap_or("".__get());
            let mut process = Command::new(&i.shell.0);
            process
                .current_dir(path.get().clone())
                .arg(&key)
                //.arg(&i.command)
                .stdin(Stdio::null());
            for j in i.command.iter() {
                process.arg(j.clone().as_str());
            }
            log!("PROCESS {process:?}");
            if let Some(env) = &i.env {
                process.envs(env);
            }
            let code = process.status();
            if let Err(_) = code {
                if self.on_error != ActionOnError::CONTINUE {
                    return self.on_error.clone();
                }
            }
            if let Ok(code) = code {
                if !code.success() {
                    return self.on_error.clone();
                }
            }
        }
        ActionOnError::CONTINUE
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCommand {
    pub command: Vec<String>,
    pub shell: Shell,
    pub env: Option<HashMap<String, String>>,
}
