use crate::modules::contexts::config::entities::{ConfigFsTemplate, FsConfigIcons};
use crate::modules::contexts::package::domain::Grammar;
use crate::modules::contexts::project::domain::entities::{Package, ProjectTemplate};
use crate::modules::contexts::settings::domain::entities::{RecentProject, Settings};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInner {
    pub main: Package,
    pub config: String,
    pub grammars: Vec<Grammar>,
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cache {
    pub(crate) recent_projects: Vec<RecentProject>,
    pub settings: Settings,
    pub templates: Vec<ProjectTemplate>,
    pub packages: Vec<Package>,
    pub groups: Vec<String>,
    pub projects_dir: Path,
    pub data_dir_path: Path,
    pub os: String,
    pub file_templates: Vec<ConfigFsTemplate>,
    pub file_icons: Vec<FsConfigIcons>,
    pub shells: Vec<String>,
    pub themes: Vec<String>,
}
