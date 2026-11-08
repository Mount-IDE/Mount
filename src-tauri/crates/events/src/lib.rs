use crate::services::EventService;

pub mod domain;
mod services;
pub mod traits;

pub const EVENT_SERVICE: EventService = EventService;
