use crate::modules::contexts::terminal::domain::values::TerminalSession;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct TerminalManager {
    pub terminals: HashMap<String, TerminalSession>,
}

impl TerminalManager {
    pub fn new() -> Self {
        Self {
            terminals: HashMap::new(),
        }
    }
}

pub type SharedTerminalManager = Arc<Mutex<TerminalManager>>;
