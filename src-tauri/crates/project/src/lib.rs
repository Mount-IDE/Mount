use crate::launch::services::launch_compile_service::LaunchCompileService;
use crate::launch::services::launch_run_service::LaunchRunService;
use crate::project::services::create_project_service::CreateProjectService;
use crate::project::services::package_service::PackageService;
use crate::project::services::project_service::ProjectService;
use crate::terminal::services::TerminalService;

pub mod errors;
pub mod launch;
pub mod project;
pub mod terminal;

pub const CREATE_PROJECT_SERVICE: CreateProjectService = CreateProjectService;
pub const PROJECT_SERVICE: ProjectService = ProjectService;
pub const PACKAGE_SERVICE: PackageService = PackageService;

pub const LAUNCH_RUN_SERVICE: LaunchRunService = LaunchRunService;
pub const LAUNCH_COMPILE_SERVICE: LaunchCompileService = LaunchCompileService;
pub const TERMINAL_SERVICE: TerminalService = TerminalService;
