use portable_pty::{Child, MasterPty};
use std::io::Write;
use std::sync::{Arc, Mutex};

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
