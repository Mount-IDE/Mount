use crate::modules::app::stores::TOOLCHAINS;
use crate::modules::app::{CONFIG_SERVICE, FS_READ_SERVICE, FS_WRITE_SERVICE, PARSING_SERVICE};
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::FileWriteAccess;
use crate::modules::contexts::project::domain::entities::{Package, Repair};
use crate::modules::contexts::toolchains::domain::{
    ComponentRepairRaw, Toolchain, ToolchainComponent,
};
use crate::modules::contexts::toolchains::traits::{
    TToolchainComponentsService, TToolchainMakeService, TToolchainPackageComponent,
};
use crate::modules::services::traits::{TConfigService, TParsingService};
use crate::modules::shared::kernel::errors::ToolchainError;
use crate::modules::shared::kernel::utils::{get_os, get_platform, shell_default};
use crate::modules::shared::kernel::values::Path;
use crate::modules::shared::kernel::values::Platform;
use crate::path_from;
use which::which;

pub struct ToolchainMakeService();

impl TToolchainMakeService for ToolchainMakeService {
    fn create_toolchain(&self, tool: &Toolchain) -> Result<(), ToolchainError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path = path_from![dir, "toolchains", tool.id, tool.version];
        if FS_READ_SERVICE.exists(path.clone()) {
            return Err(ToolchainError::AlreaddyExists {
                id: tool.id.clone(),
                version: tool.version.clone(),
            });
        }
        let parsed = PARSING_SERVICE.to_string(tool.clone())?;
        FS_WRITE_SERVICE.create_dir(&path)?;
        let path_to_config = path_from![path, "toolchain.json"];
        let file = FS_WRITE_SERVICE.create_file(&path_to_config)?;
        FS_WRITE_SERVICE.write_file(&file, parsed, FileWriteAccess::WRITE)?;
        {
            TOOLCHAINS.get().unwrap().lock().unwrap().push(tool.clone())
        }
        Ok(())
    }

    fn remove_toolchain(&self, id: String, version: String) -> Result<(), ToolchainError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path = path_from![dir, "toolchains", id, version];
        let dir = PDirectory::from_path(&path);
        FS_WRITE_SERVICE.remove_dir(&dir)?;
        Ok(())
    }

    fn get_toolchain(&self, id: String, version: String) -> Result<Toolchain, ToolchainError> {
        let toolchain = {
            let toolchains = TOOLCHAINS.get().unwrap();
            let guard = toolchains.lock().unwrap();
            let val = guard
                .iter()
                .find(|e| e.version == version && e.id == id)
                .cloned();
            val
        };

        if let Some(t) = toolchain {
            return Ok(t.clone());
        }
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path = path_from![dir, "toolchains", id, version, "toolchain.json"];
        if !FS_READ_SERVICE.exists(path.clone()) {
            return Err(ToolchainError::NotFound { id, version });
        }
        let file = PFile::from_path_reg(path);
        let text = FS_READ_SERVICE.read_file(&file)?;
        let parsed: Toolchain = PARSING_SERVICE._from_string(&text)?;
        {
            TOOLCHAINS
                .get()
                .unwrap()
                .lock()
                .unwrap()
                .push(parsed.clone())
        }

        Ok(parsed)
    }

    fn check_platform(&self, toolchain: &Toolchain, plat: Platform) -> bool {
        if let Some(p) = toolchain.platform.clone() {
            return p == plat;
        }
        false
    }
}

pub struct ToolchainComponentService();

impl TToolchainComponentsService for ToolchainComponentService {
    fn add_compoonent(
        &self,
        toolchain: &mut Toolchain,
        id: String,
        cmp: ToolchainComponent,
    ) -> Result<(), ToolchainError> {
        toolchain.components.insert(id, cmp);
        {
            let mut toolchains = TOOLCHAINS.get().unwrap().lock().unwrap();
            let i = toolchains.iter().position(|e| e.id == toolchain.id);
            if let Some(i) = i {
                toolchains[i] = toolchain.clone();
            } else {
                toolchains.push(toolchain.clone())
            }
        }
        Ok(())
    }

    fn get_component<'a>(
        &self,
        toolchain: &'a Toolchain,
        cmp: String,
    ) -> Option<&'a ToolchainComponent> {
        toolchain.components.get(&cmp)
    }

    async fn repair_component(
        &self,
        toolchain: &mut Toolchain,
        cmp: String,
    ) -> Result<(), ToolchainError> {
        unimplemented!();
        let comp = toolchain
            .components
            .get(&cmp)
            .ok_or(ToolchainError::ComponentNotFound {
                id_c: cmp.clone(),
                id: toolchain.id.clone(),
                version: toolchain.version.clone(),
            })?;
        let os = get_os();
        let Some(needed) = comp.platforms.get(&os) else {
            return Err(ToolchainError::WrongComponentPlatform {
                id_c: cmp.clone(),
                id: toolchain.id.clone(),
                version: toolchain.version.clone(),
            });
        };
        let Some(repair) = needed.repair.clone() else {
            return Err(ToolchainError::RepairOfComponentNotFound {
                id_c: cmp.clone(),
                id: toolchain.id.clone(),
                version: toolchain.version.clone(),
            });
        };

        let discovery = repair.discovery.clone();
        let plat = get_platform();
        if let Some(discovery) = discovery {
            for i in discovery {
                if let Some(platform) = i.platform {
                    if let Platform::SINGLE(p) = platform {
                        if p != plat {
                            continue;
                        }
                    } else if let Platform::ARRAY(v) = platform {
                        if !v.contains(&plat) {
                            continue;
                        }
                    }
                }
            }
        }
        let install = repair.install.clone();

        if let Some(install) = install {
            for i in install {
                match i {
                    ComponentRepairRaw::CURL {
                        platform,
                        version_check_command,
                        url,
                        shell,
                    } => {
                        let Ok(_) = which("curl") else { continue };
                        let shell = if shell.is_none() {
                            shell_default()
                        } else {
                            shell.unwrap()
                        };
                        let _ = tokio::process::Command::new(&shell)
                            .arg("curl")
                            .arg(&url)
                            .arg(" | ")
                            .arg(&shell)
                            .spawn();
                    }
                    ComponentRepairRaw::PACK {
                        platform,
                        version_check_command,
                        pm,
                        packages,
                    } => {}
                    ComponentRepairRaw::INT { url } => {}
                }
            }
        }
        todo!()
    }

    fn check_platform(
        &self,
        toolchain: &Toolchain,
        cmp: String,
        platform: Platform,
    ) -> Result<bool, ToolchainError> {
        let cmp = toolchain
            .components
            .get(&cmp)
            .ok_or(ToolchainError::ComponentNotFound {
                id_c: cmp,
                id: toolchain.id.clone(),
                version: toolchain.version.clone(),
            })?;
        for i in cmp.platforms.keys() {
            if Platform::from(i.clone()) == platform {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

pub struct ToolchainPackageService();

impl TToolchainPackageComponent for ToolchainPackageService {
    fn start_repairing(&self, package: Package) -> Result<Toolchain, ToolchainError> {
        unimplemented!()
    }

    fn repair_component(
        &self,
        tool: Toolchain,
        repair: Repair,
    ) -> Result<Toolchain, ToolchainError> {
        unimplemented!()
    }
}
