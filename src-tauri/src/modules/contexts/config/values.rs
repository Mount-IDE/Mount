use crate::modules::contexts::config::entities::{ConfigFsTemplate, FsConfigIcons};
use crate::modules::contexts::project::domain::entities::{ProjectPackage, ProjectTemplate};
use crate::modules::contexts::settings::domain::entities::{Settings, Theme};
use crate::modules::shared::kernel::values::Path;
use serde::{Deserialize, Serialize};

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FsIcon {
    //#[serde(rename="type")]
    #[serde(default)]
    pub typ: String,
    #[serde(default)]
    pub ext: Option<Vec<String>>,
    #[serde(default)]
    pub name: Option<Vec<String>>,
    #[serde(default)]
    pub icon: String,
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FsType {
    #[allow(non_camel_case_types)]
    file,
    #[allow(non_camel_case_types)]
    dir,
}

///
///
///
impl Default for FsType {
    fn default() -> Self {
        FsType::file
    }
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cache {
    pub settings: Settings,
    pub templates: Vec<ProjectTemplate>,
    pub packages: Vec<ProjectPackage>,
    pub groups: Vec<String>,
    pub projects_dir: Path,
    pub data_dir_path: Path,
    pub os: String,
    pub file_templates: Vec<ConfigFsTemplate>,
    pub file_icons: Vec<FsConfigIcons>,
    pub shells: Vec<String>,
    pub themes: Vec<Theme>,
}
