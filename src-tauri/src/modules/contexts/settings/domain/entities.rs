use serde::{Deserialize, Serialize};
use crate::modules::shared::kernel::values::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    doctype: String,
    version: String,
    general: GeneralSettings,

}

#[derive(Serialize, Deserialize, Clone)]
pub struct GeneralSettings {

}

#[derive(Serialize, Deserialize)]
pub struct RecentProject {
    pub name: String,
    pub path: Path
}
