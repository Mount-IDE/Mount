use crate::launch::domain::{LaunchError, LaunchFlatTask, LaunchObject, LaunchTemplateReference};
use crate::launch::managers::{LaunchSession, SharedLaunchManager};
use crate::launch::traits::TLaunchRunService;
use crate::project::domain::Project;
use macroses::log;
use mount_core::domain::Shell;
use mount_core::{APP, make_path};
use process_wrap::tokio::{CommandWrap, CreationFlags, JobObject, KillOnDrop};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use uuid::Uuid;
use windows::Win32::System::Threading::CREATE_NO_WINDOW;

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
        log!("START TASK{:?}", task.command);
        let cmd = &task.command;

        let path = make_path![project.path, project.name];
        let mut path = PathBuf::from(path.get());
        if let Some(val) = &task.cwd {
            path.push(val.clone());
        }

        let shell = Shell::default();
        let key = shell.get_key().ok_or(LaunchError::Spawn)?;

        let mut command = CommandWrap::with_new(shell.0.as_str(), |command| {
            command
                .arg(key)
                //.arg(cmd)
                .args(cmd)
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

    async fn launch_object(
        &self,
        object: &LaunchObject,
        objects: &Vec<LaunchObject>,
        current: &LaunchTemplateReference,
        project: &Project,
        window: &String,
        app: &AppHandle,
        state: &State<'_, SharedLaunchManager>,
    ) -> Result<(), LaunchError> {
        if let Some(b) = &object.before {
            for i in b.iter() {
                let Some(found) = objects.iter().find(|e| e.id == *i) else {
                    return Err(LaunchError::InvalidObject(object.clone()));
                };
                Box::pin(
                    self.launch_object(&found, &objects, &current, &project, &window, &app, &state),
                )
                .await?;
            }
        }
        let tasks = object.tasks.clone();
        let tasks = tasks
            .into_iter()
            .map(|e| {
                let cwd = e.cwd;
                return e.commands.into_iter().map(move |e2| {
                    return LaunchFlatTask {
                        command: e2.command,
                        env: e2.env,
                        cwd: Some(cwd.clone()),
                    };
                });
            })
            .flatten()
            .collect::<Vec<LaunchFlatTask>>();

        log!("TASKS {tasks:?}");
        for i in tasks.iter() {
            let _ = self
                .launch_task(i, window.clone().as_str(), &project, &app, &state)
                .await;
        }
        Ok(())
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
