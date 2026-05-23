use crate::modules::contexts::filesystem::domain::values::WatchInstance;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct FileSystemWatchManager {
    pub watchers: HashMap<String, WatchInstance>,
}

impl FileSystemWatchManager {
    pub fn new() -> Self {
        Self {
            watchers: HashMap::new(),
        }
    }
}

pub type SharedWatcherManager = Arc<Mutex<FileSystemWatchManager>>;
