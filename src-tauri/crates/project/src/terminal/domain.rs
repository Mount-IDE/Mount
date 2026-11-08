use config::errors::ConfigError;
use portable_pty::{Child, MasterPty};
use std::io::Write;
use std::sync::{Arc, Mutex};
use thiserror::Error;

pub struct TerminalSession {
    pub window_id: String,
    pub master: Arc<Mutex<Box<dyn MasterPty + Send>>>,
    pub writer: Arc<Mutex<Box<dyn Write + Send>>>,
    pub child: Arc<Mutex<Box<dyn Child + Send + Sync>>>,
}

impl TerminalSession {
    pub fn join(&self) {
        if let Ok(mut child) = self.child.lock() {
            if let Err(e) = child.kill() {
                println!("kill error: {:?}", e);
            }
            let _ = child.wait();
        }

        if let Ok(mut writer) = self.writer.lock() {
            let _ = writer.flush();
        }
    }
}

#[derive(Debug, Error)]
#[allow(unused)]
pub enum TerminalError {
    #[error("error of config context tha can`t described by terminal err")]
    Config { err: ConfigError },
    #[error("cannot spawn terminal process {shell}")]
    Spawn { shell: String },
    #[error("Terminal width {id} not found")]
    NotFound { id: String },
    #[error("Cannot write to terminal")]
    Write { err: std::io::Error, id: String },
    #[error("cannot resize the terminal")]
    Resize { id: String },
    #[error("cannot close terminal")]
    Close { err: std::io::Error, id: String },
}
