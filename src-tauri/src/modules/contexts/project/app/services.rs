use crate::modules::contexts::project::app::traits::TProjectService;
use crate::modules::contexts::project::domain::entities::Project;
use crate::modules::shared::kernel::entities::Error;
use crate::modules::shared::kernel::values::Path;

pub struct ProjectService();


impl TProjectService for ProjectService {
    fn create_project(&self) -> Result<Project, Error> {
        todo!()
    }

    fn open_project(&self, project_path: Path) -> Result<Project, Error> {
        todo!()
    }

    fn delete_project(&self, project_path: Path) -> Option<Error> {
        todo!()
    }

    fn get_projects(&self, dir: Path) -> Result<Vec<Project>, Error> {
        todo!()
    }
}