use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventType {
    ProjectCreationStart,
    ProjectCreationActionStart,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    typ: EventType,
    data: Option<String>,
}
