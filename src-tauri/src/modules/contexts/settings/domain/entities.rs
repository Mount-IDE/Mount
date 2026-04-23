use serde::{Deserialize, Serialize};
use crate::modules::shared::kernel::values::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    doctype: String,
    version: String,
    general: GeneralSettings,

}


impl Settings {
    pub fn new()->Settings {
        Self {
            doctype: String::from("opie/mount"),
            version: String::from("1.0.0"),
            general: GeneralSettings::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GeneralSettings {

}

impl GeneralSettings {
    pub fn new()->GeneralSettings{
        Self{}
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecentProject {
    pub name: String,
    pub path: Path
}
