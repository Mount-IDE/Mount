use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grammar {
    scopeName: String,
    patterns: Vec<RawRule>,
    repository: Option<HashMap<String, RawRule>>,
    injections: Option<HashMap<String, RawRule>>,
    injectionSelector: Option<String>,
    fileTypes: Option<Vec<String>>,
    name: Option<String>,
    firstLineMatch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawRule {
    id: Option<i32>,
    include: Option<String>,
    name: Option<String>,
    contentName: Option<String>,
    #[serde(rename = "match")]
    match_: Option<String>,
    begin: Option<String>,
    end: Option<String>,
    #[serde(rename = "while")]
    while_: Option<String>,

    captures: Option<HashMap<String, RawCapture>>,
    beginCaptures: Option<HashMap<String, RawCapture>>,
    endCaptures: Option<HashMap<String, RawCapture>>,
    whileCaptures: Option<HashMap<String, RawCapture>>,

    patterns: Option<Vec<RawRule>>,
    applyEndPatternLast: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawCapture {
    name: Option<String>,
    patterns: Option<Vec<RawRule>>,
}
