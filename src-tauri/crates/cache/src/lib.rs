use config::domain::config::FsConfigIcons;
use config::domain::settings::Settings;
use filesystem::domain::ConfigFsTemplate;
use mount_core::domain::Path;
use project::project::domain::{Package, ProjectTemplate, RecentProject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInner {
    pub main: Package,
    pub config: String,
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
