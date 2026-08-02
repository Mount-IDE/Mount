use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventType {
    ProjectCreationStart,
    ProjectCreationActionStart,
    DependencyError,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub(crate) typ: Option<EventType>,
    pub(crate) data: Option<String>,
}
