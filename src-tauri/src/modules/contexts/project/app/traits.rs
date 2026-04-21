use crate::modules::contexts::project::domain::entities::Project;
use crate::modules::shared::kernel::entities::Error;
use crate::modules::shared::kernel::values::Path;

pub trait TProjectService{
    fn create_project(&self) -> Result<Project, Error>;
    fn open_project(&self, project_path: Path) -> Result<Project, Error>;
    fn delete_project(&self, project_path: Path) -> Option<Error>;
    fn get_projects(&self, dir: Path) -> Result<Vec<Project>, Error>;

}


