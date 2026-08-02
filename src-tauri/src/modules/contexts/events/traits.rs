use crate::modules::contexts::events::domain::EventType;

pub trait TEventService {
    fn write(&self, window: String, chan: &str, data: Option<String>, typ: EventType) -> bool;

    fn send(&self, window: String, chan: &str, data: String) -> bool;
}
