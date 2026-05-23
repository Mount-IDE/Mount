use crate::modules::contexts::project::domain::entities::Project;
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::shared::kernel::errors::ProjectError;
use crate::modules::shared::kernel::values::Path;

pub trait TProjectService {
    fn create_project(&self, proj: &Project) -> Result<(), ProjectError>;
    fn open_project(&self, project_path: &Path) -> Result<Project, ProjectError>;
    fn delete_project(&self, project_path: &Path) -> Result<(), ProjectError>;
    fn get_projects(&self, dir: &Path) -> Result<Vec<Project>, ProjectError>;
    fn get_recent_projects(&self) -> Result<Vec<RecentProject>, ProjectError>;
    fn save_project(&self, project: &Project) -> Result<(), ProjectError>;
    fn remove_from_recents(&self, proj: &Project) -> Result<(), ProjectError>;
    fn add_to_recents(&self, project: &Project) -> Result<(), ProjectError>;
}
