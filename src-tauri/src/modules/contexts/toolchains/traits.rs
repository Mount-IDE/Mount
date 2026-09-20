use crate::modules::contexts::project::domain::entities::{Package, Repair};
use crate::modules::contexts::toolchains::domain::{Toolchain, ToolchainComponent};
use crate::modules::shared::kernel::errors::ToolchainError;
use crate::modules::shared::kernel::values::Platform;

pub trait TToolchainMakeService {
    fn create_toolchain(&self, tool: Toolchain) -> Result<(), ToolchainError>;

    fn remove_toolchain(&self, id: String, version: String) -> Result<(), ToolchainError>;

    fn get_toolchain(&self, id: String, version: String) -> Result<Toolchain, ToolchainError>;

    fn check_platform(&self, toolchain: Toolchain, plat: Platform) -> bool;
}

pub trait TToolchainComponentsService {
    fn add_compoonent(
        &self,
        toolchain: Toolchain,
        cmp: ToolchainComponent,
    ) -> Result<(), ToolchainError>;

    fn get_component(&self, toolchain: Toolchain, cmp: String) -> Option<&ToolchainComponent>;

    fn repair_component(&self, toolchain: Toolchain, cmp: String) -> Result<(), ToolchainError>;

    fn check_platform(
        &self,
        toolchain: Toolchain,
        cmp: String,
        platform: Platform,
    ) -> Result<bool, ToolchainError>;
}

pub trait ToolchainPackageComponent {
    fn start_repairing(&self, package: Package) -> Result<Toolchain, ToolchainError>;

    fn repair_component(
        &self,
        tool: Toolchain,
        repair: Repair,
    ) -> Result<Toolchain, ToolchainError>;
}
