use crate::modules::app::APP;
pub(crate) use crate::modules::contexts::terminal::app::managers::SharedTerminalManager;
use crate::modules::contexts::terminal::app::traits::TTerminalService;
use crate::modules::contexts::terminal::domain::values::TerminalSession;
use crate::modules::shared::kernel::errors::{ConfigError, TerminalError};
use crate::modules::shared::kernel::values::Path;
use async_trait::async_trait;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use serde::Serialize;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{Emitter, State};
use uuid::Uuid;

#[derive(Clone, Serialize)]
struct TerminalOutput {
    id: String,
    data: String,
}

#[derive(Clone, Serialize)]
struct TerminalExit {
    id: String,
}

fn normalize_shell(shell: String) -> String {
    if !shell.trim().is_empty() {
        return shell;
    }

    #[cfg(windows)]
    {
        std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
    }

    #[cfg(not(windows))]
    {
        std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string())
    }
}

fn make_size(rows: u16, cols: u16) -> PtySize {
    PtySize {
        rows: rows.max(1),
        cols: cols.max(1),
        pixel_width: 0,
        pixel_height: 0,
    }
}

pub struct TerminalService;

#[async_trait]
impl TTerminalService for TerminalService {
    async fn open_terminal(
        &self,
        window_id: String,
        shell: String,
        cwd: Path,
        rows: u16,
        cols: u16,
        is_launch: bool,
        id: Option<String>,
        state: State<'_, SharedTerminalManager>,
    ) -> Result<String, TerminalError> {
        let app = APP
            .get()
            .ok_or(TerminalError::Config {
                err: ConfigError::App,
            })?
            .clone();

        let shell = normalize_shell(shell);
        let id = if let Some(id) = id {
            id
        } else {
            if is_launch {
                format!("LAUNCH::{}", Uuid::new_v4().to_string())
            } else {
                Uuid::new_v4().to_string()
            }
        };
        let cwd_str = cwd.get().to_string();

        let shell_for_spawn = shell.clone();
        let cwd_for_spawn = cwd_str.clone();
        let window_id_for_session = window_id.clone();

        let (session, mut reader) = tauri::async_runtime::spawn_blocking(move || {
            let pty_system = native_pty_system();

            let pair =
                pty_system
                    .openpty(make_size(rows, cols))
                    .map_err(|_| TerminalError::Spawn {
                        shell: shell_for_spawn.clone(),
                    })?;

            let mut proc = CommandBuilder::new(shell_for_spawn.as_str());
            proc.cwd(cwd_for_spawn.as_str());

            let reader = pair
                .master
                .try_clone_reader()
                .map_err(|_| TerminalError::Spawn {
                    shell: shell_for_spawn.clone(),
                })?;

            let writer = pair
                .master
                .take_writer()
                .map_err(|_| TerminalError::Spawn {
                    shell: shell_for_spawn.clone(),
                })?;

            let child = pair
                .slave
                .spawn_command(proc)
                .map_err(|_| TerminalError::Spawn {
                    shell: shell_for_spawn.clone(),
                })?;

            let session = TerminalSession {
                window_id: window_id_for_session,
                master: Arc::new(Mutex::new(pair.master)),
                writer: Arc::new(Mutex::new(writer)),
                child: Arc::new(Mutex::new(child)),
            };

            Ok::<_, TerminalError>((session, reader))
        })
        .await
        .map_err(|_| TerminalError::Spawn {
            shell: shell.clone(),
        })??;

        {
            let mut manager = state.lock().unwrap();
            manager.terminals.insert(id.clone(), session);
        }

        let app_for_reader = app.clone();
        let event_id = id.clone();

        thread::spawn(move || {
            let mut buf = [0u8; 8192];

            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(size) => {
                        let data = String::from_utf8_lossy(&buf[..size]).to_string();
                        let _ = app_for_reader.emit(
                            "terminal-output",
                            TerminalOutput {
                                id: event_id.clone(),
                                data,
                            },
                        );
                    }
                    Err(_) => break,
                }
            }

            let _ = app_for_reader.emit("terminal-exit", TerminalExit { id: event_id });
        });

        Ok(id)
    }

    async fn write_terminal(
        &self,
        id: String,
        data: String,
        state: State<'_, SharedTerminalManager>,
    ) -> Result<(), TerminalError> {
        let writer = {
            let manager = state.lock().unwrap();
            let session = manager
                .terminals
                .get(&id)
                .ok_or(TerminalError::NotFound { id: id.clone() })?;

            session.writer.clone()
        };

        let mut writer = writer.lock().unwrap();

        writer
            .write_all(data.as_bytes())
            .map_err(|err| TerminalError::Write {
                err,
                id: id.clone(),
            })?;

        writer
            .flush()
            .map_err(|err| TerminalError::Write { err, id })?;

        Ok(())
    }

    async fn resize_terminal(
        &self,
        id: String,
        rows: u16,
        cols: u16,
        state: State<'_, SharedTerminalManager>,
    ) -> Result<(), TerminalError> {
        let master = {
            let manager = state.lock().unwrap();
            let session = manager
                .terminals
                .get(&id)
                .ok_or(TerminalError::NotFound { id: id.clone() })?;

            session.master.clone()
        };

        master
            .lock()
            .unwrap()
            .resize(make_size(rows, cols))
            .map_err(|_| TerminalError::Resize { id })?;

        Ok(())
    }

    async fn close_terminal(
        &self,
        id: String,
        state: State<'_, SharedTerminalManager>,
    ) -> Result<(), TerminalError> {
        let session = {
            let mut manager = state.lock().unwrap();
            manager.terminals.remove(&id)
        };

        if let Some(session) = session {
            session.join();
        }

        Ok(())
    }

    async fn close_window_terminals(
        &self,
        window_id: String,
        state: State<'_, SharedTerminalManager>,
    ) -> Result<(), TerminalError> {
        let sessions = {
            let mut manager = state.lock().unwrap();
            manager.remove_window_terminals(&window_id)
        };

        for session in sessions {
            session.join();
        }

        Ok(())
    }
}
