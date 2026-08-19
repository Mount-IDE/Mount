use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::process::Child;
use tokio::sync::{mpsc, Mutex};

pub struct LspEntity {
    pub window_id: String,
    pub child: Child,
    pub writter: mpsc::UnboundedSender<Value>,
}

pub struct LspManager {
    pub servers: HashMap<String, LspEntity>, //id (uuid) -> entity
}

impl LspManager {
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
        }
    }
}

pub type SharedLspManager = Arc<Mutex<LspManager>>;
