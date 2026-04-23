use crate::modules::contexts::project::domain::entities::Project;
use crate::modules::contexts::project::domain::values::ProjectToFS;
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::shared::kernel::entities::{ProjectError};
use crate::modules::shared::kernel::values::Path;

pub trait TProjectService{
    fn create_project(&self, proj: &Project) -> Result<(), ProjectToFS>;
    fn open_project(&self, project_path: Path) -> Result<Project, ProjectToFS>;
    fn delete_project(&self, project_path: Path) -> Result<(),ProjectToFS>;
    fn get_projects(&self, dir: Path) -> Result<Vec<Project>, ProjectToFS>;
    fn get_recent_projects(&self) -> Result<Vec<RecentProject>, ProjectToFS>;
    fn save_project(&self, project: Project) -> Result<(), ProjectToFS>;
    fn remove_from_recents(&self, proj: &Project) -> Result<(), ProjectToFS>;
    fn add_to_recents(&self, project: &Project) -> Result<(), ProjectToFS>;

}


