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

    pub fn remove_window_terminals(&mut self, window_id: &str) -> Vec<TerminalSession> {
        let ids = self
            .terminals
            .iter()
            .filter_map(|(id, session)| {
                if session.window_id == window_id {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        // println!("remove_window_terminals {} count={}", window_id, ids.len());
        ids.into_iter()
            .filter_map(|id| self.terminals.remove(&id))
            .collect()
    }
}

pub type SharedTerminalManager = Arc<Mutex<TerminalManager>>;
