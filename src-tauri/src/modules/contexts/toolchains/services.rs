use crate::modules::app::{CONFIG_SERVICE, FS_READ_SERVICE, FS_WRITE_SERVICE};
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::contexts::filesystem::domain::entities::PDirectory;
use crate::modules::contexts::toolchains::domain::{Toolchain, ToolchainTyp};
use crate::modules::contexts::toolchains::traits::TToolchainMakeService;
use crate::modules::services::traits::TConfigService;
use crate::modules::shared::kernel::errors::ToolchainError;
use crate::modules::shared::kernel::values::Path;
use crate::modules::shared::kernel::values::Platform;
use crate::path_from;

pub struct ToolchainMakeService();
