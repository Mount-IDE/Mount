use crate::modules::app::APP;
use crate::modules::contexts::launch::app::functions::FunctionResult;
use crate::modules::contexts::launch::app::managers::{LaunchSession, SharedLaunchManager};
use crate::modules::contexts::launch::app::traits::{TLaunchCompileService, TLaunchRunService};
use crate::modules::contexts::launch::domain::entities::{
    LaunchFlatTask, LaunchFunction, LaunchObject, LaunchTemplate, LaunchTemplateReference,
    LaunchTemplateResult,
};
use crate::modules::contexts::project::domain::entities::{
    ActionTask, PackageAction, Project, ProjectTemplate, Var,
};
use crate::modules::shared::kernel::errors::LaunchError;
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::modules::contexts::project::app::services::get_key;
use crate::modules::shared::kernel::utils::shell_default;
use crate::modules::shared::kernel::values::Val;
use crate::path_from;
use macroses::log;
use process_wrap::tokio::*;
#[cfg(windows)]
use windows::Win32::System::Threading::CREATE_NO_WINDOW;

#[allow(unused)]
pub struct LaunchCompileService;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LaunchOutputType {
    READ,
    ERROR,
    START,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchTaskOutput {
    id: String,
    typ: LaunchOutputType,
    data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaunchExit {
    id: String,
    code: i32,
}

#[allow(unused)]
const VAR_PREFIX: &str = "@";
#[allow(unused)]
const PARAM_PREFIX: &str = "#";

impl TLaunchCompileService for LaunchCompileService {
    fn compile_reference(
        &self,
        l_template: &LaunchTemplate,
        template: &ProjectTemplate,
    ) -> Option<LaunchTemplateReference> {
        let obj = LaunchTemplateReference {
            id: 0,
            template: (template.id.clone(), l_template.id.clone()),
            scheme: Default::default(),
            icon: l_template.icon.clone(),
            name: l_template.title.clone(),
            results: HashMap::new(),
            before: None,
        };

        Some(obj)
    }

    fn compile_object(
        &self,
        reference: &LaunchTemplateReference,
        vars: &Vec<Var>,
        template: &LaunchTemplate,
        results: &LaunchTemplateResult,
    ) -> Option<LaunchObject> {
        let mut flat_results = HashMap::<String, Val>::new();
        for i in results.iter() {
            for j in i.1.iter() {
                flat_results.insert(format!("{}.{}", i.0, j.0.clone()), j.1.clone());
            }
        }

        log!("RESULTS {results:?} {template:?}");
        let Some(needed) = results.get(&template.id) else {
            log!("NONE1");
            return None;
        };
        let passed = self.compile_condition(&vars, &template, &flat_results);
        log!("{passed:?}");
        if passed.len() == 0 {
            log!("NONE2");
            return None;
        }
        let mut rng = rand::rng();
        let num: i32 = rng.random();

        let actions = passed
            .iter()
            .filter_map(|e| {
                e.compile(&passed, &vars, &flat_results, &mut vec![], false)
                    .ok()
            })
            .flatten()
            .collect::<Vec<ActionTask>>();
        let obj = LaunchObject {
            id: num,
            launch_reference: reference.id,
            scheme: Default::default(),
            tasks: actions,
        };
        log!("COMPILED :: {reference:?}\n :: {obj:?}");
        Some(obj)
    }

    fn compile_condition(
        &self,
        vars: &Vec<Var>,
        template: &LaunchTemplate,
        results: &HashMap<String, Val>,
    ) -> Vec<PackageAction> {
        println!("\n\n\t{results:?} {template:?}");

        let actions = &template.actions;
        actions
            .iter()
            .filter(|e| {
                let a = e.check_condition(&results, &vars) && e.check_platform();
                println!("\t {a}");
                return a;
            })
            .cloned()
            .collect()
    }

    fn run_function(
        &self,
        _function: &LaunchFunction,
        _template: &LaunchTemplate,
        _results: &HashMap<String, Val>,
    ) -> Option<FunctionResult> {
        unimplemented!()
        /*let mut first: LaunchFunctionArgument = LaunchFunctionArgument::STRING(Default::default());
        let mut seq = false;

        let result: FunctionResult;
        for act in function.actions.iter() {
            let res: Option<FunctionResult> = match act.function.as_str() {
                "read_from" => {
                    if let Some(args) = &act.args {
                        if args.len() == 1 {
                            let arg = if seq { first.clone() } else { args[0].clone() };
                            read_from(arg.clone(), &results)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                "read_fields" => {
                    if let Some(args) = &act.args {
                        if args.len() == 2 {
                            if let LaunchFunctionArgument::VEC(fields) = args[1].clone() {
                                let arg = if seq { first.clone() } else { args[0].clone() };
                                read_fields(arg.clone(), fields.clone(), &results)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                "return" => {
                    if seq {
                        match first.clone() {
                            LaunchFunctionArgument::STRING(v) => result = FunctionResult::SCALAR(v),
                            LaunchFunctionArgument::VEC(v) => result = FunctionResult::VEC(v),
                        }
                        return Some(result);
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if !seq {
                seq = true;
            }
            if let Some(val) = res {
                if let FunctionResult::SCALAR(s) = val.clone() {
                    first = LaunchFunctionArgument::STRING(s)
                }
                if let FunctionResult::VEC(v) = val {
                    first = LaunchFunctionArgument::VEC(v)
                }
            }
        }
        None*/
    }
}

#[allow(unused)]
pub struct LaunchRunService;

impl TLaunchRunService for LaunchRunService {
    async fn launch_task(
        &self,
        task: &LaunchFlatTask,
        window_id: &str,
        project: &Project,
        app: &AppHandle,
        state: &State<'_, SharedLaunchManager>,
    ) -> Result<String, LaunchError> {
        log!("START TASK{}", task.command);
        let cmd = &task.command;

        let path = path_from![project.path, project.name];
        let mut path = PathBuf::from(path.get());
        if let Some(val) = &task.cwd {
            path.push(val.clone());
        }

        let shell = shell_default();
        let key = get_key(shell.clone());

        let mut command = CommandWrap::with_new(shell.as_str(), |command| {
            command
                .arg(key)
                .arg(cmd)
                .current_dir(path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
        });

        command.wrap(KillOnDrop);

        #[cfg(unix)]
        {
            command.wrap(ProcessGroup::leader());
        }

        #[cfg(windows)]
        {
            command.wrap(CreationFlags(CREATE_NO_WINDOW));
            command.wrap(JobObject);
        }

        let mut command = command.spawn().map_err(|_| LaunchError::Spawn)?;

        let stdin = command.stdin().take().unwrap();
        let stdout = command.stdout().take().unwrap();
        let stderr = command.stderr().take().unwrap();

        let app_clone_out = app.clone();
        let app_clone_ex = app.clone();
        let app_clone_err = app.clone();

        let child = Arc::new(Mutex::new(command));
        let child_clone = child.clone();

        let id = Uuid::new_v4().to_string();

        let _ = app.emit_to(
            window_id,
            "launch-start",
            LaunchTaskOutput {
                id: id.clone(),
                typ: LaunchOutputType::START,
                data: "".to_string(),
            },
        );

        let id_out = id.clone();
        let id_err = id.clone();
        let id_exit = id.clone();

        let window_id_out = window_id.to_string();
        let window_id_err = window_id.to_string();
        let window_id_exit = window_id.to_string();

        let _ = tokio::spawn(async move {
            let mut stdout = stdout;
            let mut buffer = [0u8; 1024];
            let window = window_id_out.clone();
            loop {
                let a = stdout.read(&mut buffer).await;
                if let Ok(n) = a {
                    if n == 0 {
                        break;
                    } else {
                        let chunk = String::from_utf8_lossy(&buffer[..n]);
                        let _ = app_clone_out.emit_to(
                            window.clone(),
                            "launch-read",
                            LaunchTaskOutput {
                                id: id_out.clone(),
                                typ: LaunchOutputType::READ,
                                data: chunk.to_string(),
                            },
                        );
                    }
                } else {
                    break;
                }
            }
        });
        let _ = tokio::spawn(async move {
            let mut stderr = stderr;
            let mut buffer = [0u8; 1024];
            let window = window_id_err.clone();

            loop {
                let a = stderr.read(&mut buffer).await;
                if let Ok(n) = a {
                    if n == 0 {
                        break;
                    } else {
                        let chunk = String::from_utf8_lossy(&buffer[..n]);
                        let _ = app_clone_err.emit_to(
                            window.clone(),
                            "launch-read",
                            LaunchTaskOutput {
                                id: id_err.clone(),
                                typ: LaunchOutputType::ERROR,
                                data: chunk.to_string(),
                            },
                        );
                    }
                } else {
                    break;
                }
            }
        });
        let _ = tokio::spawn(async move {
            let status;
            loop {
                {
                    let mut val = child_clone.lock().await;
                    match val.try_wait() {
                        Ok(Some(exit_status)) => {
                            status = exit_status;
                            break;
                        }
                        Ok(None) => {}
                        Err(_) => {
                            return;
                        }
                    }
                }
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }

            let window = window_id_exit.clone();
            let _ = app_clone_ex.emit_to(
                window,
                "launch-exit",
                LaunchExit {
                    id: id_exit,
                    code: status.code().unwrap_or(-1),
                },
            );
        });

        {
            let mut ex = state.lock().unwrap();
            ex.launches.insert(
                id.clone(),
                LaunchSession {
                    window_id: window_id.to_string(),
                    child,
                    writer: Arc::new(Mutex::new(stdin.into())),
                },
            );
        }

        Ok(id)
    }

    async fn write_to_launch(
        &self,
        id: String,
        text: String,
        state: State<'_, SharedLaunchManager>,
    ) {
        let writer = {
            state
                .lock()
                .unwrap()
                .launches
                .get(&id)
                .map(|e| e.writer.clone())
        };
        if let Some(found) = writer {
            let mut writer = found.lock().await;
            let _ = writer.write_all(text.as_bytes()).await;
        }
    }

    async fn close_task(
        &self,
        id: String,
        state: State<'_, SharedLaunchManager>,
        _label: &str,
    ) -> Result<(), ()> {
        let app = APP.get();
        if let None = app {
            return Err(());
        }

        let ex = { state.lock().unwrap().launches.remove(&id) };
        if let Some(session) = ex {
            println!("close task 22");
            let mut child = session.child.lock().await;
            println!("get child");
            let _ = child.start_kill();
            println!("kill");
            let _ = child.wait().await;
            println!("close task 222");
            let mut writter = session.writer.lock().await;
            let _ = writter.flush();
            /*if let Ok(code)=code{
                let _ = app.unwrap().emit_to(label, "launch-exit", LaunchExit {
                    id,
                    code: code.code().unwrap()
                });
            }*/
        }
        println!("close task2");
        Ok(())
    }

    async fn close_window_tasks(
        &self,
        state: State<'_, SharedLaunchManager>,
        label: &str,
    ) -> Result<(), ()> {
        let app = APP.get();
        if let None = app {
            return Err(());
        }
        let matched: Vec<(String, _, _)> = {
            let launches = &state.lock().unwrap().launches;
            launches
                .iter()
                .filter(|(_, obj)| obj.window_id == label)
                .map(|(id, obj)| (id.clone(), obj.child.clone(), obj.writer.clone()))
                .collect()
        };

        for (_, child, writer) in &matched {
            let mut child = child.lock().await;
            let _ = child.start_kill();
            let _ = child.wait().await;
            let mut writer = writer.lock().await;
            let _ = writer.flush().await;
        }

        {
            let launches = &mut state.lock().unwrap().launches;
            for (id, _, _) in &matched {
                launches.remove(id);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::modules::app::LAUNCH_COMPILE_SERVICE;
    use crate::modules::contexts::filesystem::app::utils::PathPart;
    use macroses::log;

    #[test]
    pub fn compile_() {
        let tmp: LaunchTemplate = LaunchTemplate::default();
        let ref_: LaunchTemplateReference = LaunchTemplateReference {
            id: 0,
            template: ("".to_string(), -1),
            icon: None,
            scheme: Default::default(),
            name: "".to_string(),
            results: HashMap::<i8, HashMap<String, Val>>::from([
                (
                    -1i8,
                    HashMap::from([("name".__get(), Val::STRING("name".__get()))]),
                ),
                (
                    0i8,
                    HashMap::from([("command".__get(), Val::STRING("rustc main.rs".__get()))]),
                ),
            ]),
            before: None,
        };

        let vars = vec![Var::readonly(
            "project_name".__get(),
            Val::STRING("45".__get()),
        )];

        let compile = LAUNCH_COMPILE_SERVICE.compile_object(&ref_, &vars, &tmp, &ref_.results);
        assert!(compile.is_some());
        log!("{compile:?}")
    }
}
