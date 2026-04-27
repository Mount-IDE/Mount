use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::modules::shared::kernel::values::Path;

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct Settings {
    pub doctype: String,
    pub version: String,
    pub general: GeneralSettings,
    pub appearance: Appearance
}


impl Settings {
    pub fn new()->Settings {
        Self {
            doctype: String::from("opie/mount"),
            version: String::from("1.0.0"),
            general: GeneralSettings::new(),
            appearance: Appearance::new()
        }
    }
}



#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct GeneralSettings {
    pub path_to_projects: Path,
    pub project_groups: Vec<String>
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            path_to_projects: Path("".to_string()),
            project_groups:vec![String::from("general")]
        }
    }
}

impl GeneralSettings {
    pub fn new()->GeneralSettings{
        Self{
            path_to_projects: Path(String::new()),
            project_groups: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct Appearance {
    pub theme: String,
    pub lang: String,
    pub font: String,
    pub font_size: u64
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            lang: "en".to_string(),
            font: "Jetbrains Mono".to_string(),
            font_size: 16
        }
    }
}

impl Appearance{
    pub fn new()->Appearance{
        Self {
            theme: String::new(),
            lang: String::new(),
            font: String::new(),
            font_size: 16,
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct RecentProject {
    pub name: String,
    pub path: Path
}
