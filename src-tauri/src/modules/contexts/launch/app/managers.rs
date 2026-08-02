use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;

pub struct LaunchSession {
    pub window_id: String,
    pub child: Arc<tokio::sync::Mutex<tokio::process::Child>>,
    pub writer: Arc<tokio::sync::Mutex<tokio::process::ChildStdin>>,
}

impl LaunchSession {
    pub async fn kill(&self) {
        let mut child = self.child.lock().await;
        if child.try_wait().ok().flatten().is_none() {
            let _ = child.kill().await;
            let _ = child.wait().await;
        }
        let mut writter = self.writer.lock().await;
        let _ = writter.flush().await;
    }
}

pub struct LaunchManager {
    pub launches: HashMap<String, LaunchSession>,
}

impl LaunchManager {
    pub fn new() -> Self {
        Self {
            launches: HashMap::new(),
        }
    }

    pub fn remove_window_sessions(&mut self, window_id: &str) -> Vec<LaunchSession> {
        let ids = self
            .launches
            .iter()
            .filter_map(|(id, session)| {
                if session.window_id == window_id {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut sessions = Vec::<LaunchSession>::new();
        for id in ids {
            if let Some(val) = self.launches.remove(&id) {
                sessions.push(val);
            }
        }
        sessions
    }
}

pub type SharedLaunchManager = Arc<Mutex<LaunchManager>>;
