use crate::project::services::create_project_service::CreateProjectService;
use crate::project::services::project_service::ProjectService;

mod errors;
pub mod launch;
pub mod project;
pub mod terminal;

pub const CREATE_PROJECT_SERVICE: CreateProjectService = CreateProjectService;
pub const PROJECT_SERVICE: ProjectService = ProjectService;
