pub mod commands;

use crate::modules::contexts::project::app::services::ProjectService;
use super::contexts::filesystem::app::services::{FileSystemReadService, FileSystemWriteService};

pub const FS_READ_SERVICE: FileSystemReadService= FileSystemReadService {};
pub const FS_WRITE_SERVICE: FileSystemWriteService= FileSystemWriteService {};
pub const PROJECT_SERVICE: ProjectService = ProjectService {};