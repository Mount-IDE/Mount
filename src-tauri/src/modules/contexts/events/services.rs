use crate::modules::app::{APP, PARSING_SERVICE};
use crate::modules::contexts::events::domain::{Event, EventType};
use crate::modules::contexts::events::traits::TEventService;
use crate::modules::services::traits::TParsingService;
use tauri::Emitter;

pub struct EventService();

impl TEventService for EventService {
    fn write(&self, window: String, chan: &str, data: Option<String>, typ: EventType) -> bool {
        println!("zWRITE");
        let app = APP.get();
        if let Some(app) = app {
            let dat = Event {
                typ: Some(typ),
                data,
            };
            let json = PARSING_SERVICE.to_string(dat);
            if let Err(_) = json {
                return false;
            }
            let res = app.emit_to(window, chan, json.unwrap());
            return res.is_ok();
        }
        println!("ERR");
        false
    }

    fn send(&self, window: String, chan: &str, data: String) -> bool {
        println!("SEND {data}");
        let app = APP.get();
        if let Some(app) = app {
            let dat = Event {
                typ: None,
                data: Some(data),
            };
            let json = PARSING_SERVICE.to_string(dat);
            if let Err(_) = json {
                println!("not");
                return false;
            }
            let res = app.emit_to(window, chan, json.unwrap());
            return res.is_ok();
        }
        println!("not2");
        false
    }
}
