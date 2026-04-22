use crate::modules::contexts::project::domain::entities::Project;
use crate::modules::shared::kernel::entities::{ProjectError};
use crate::modules::shared::kernel::values::Path;

pub trait TProjectService{
    fn create_project(&self, proj: &Project) -> Result<(), ProjectError>;
    fn open_project(&self, project_path: Path) -> Result<Project, ProjectError>;
    fn delete_project(&self, project_path: Path) -> Result<(),ProjectError>;
    fn get_projects(&self, dir: Path) -> Result<Vec<Project>, ProjectError>;

}


