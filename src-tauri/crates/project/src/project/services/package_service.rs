use crate::errors::ProjectError;
use crate::project::domain::Package;
use crate::project::traits::TPackageService;
use config::CONFIG_SERVICE;
use config::traits::TConfigService;
use filesystem::FS_READ_SERVICE;
use filesystem::domain::{PDirectory, PFile};
use filesystem::traits::TFSReadService;
use mount_core::make_path;
use parsing::traits::TParsingService;
use services::PARSING_SERVICE;

pub struct PackageService;

impl TPackageService for PackageService {
    fn read_packages(&self) -> Result<Vec<Package>, ProjectError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path = make_path![dir, "packages"];
        let mut packages = Vec::<Package>::new();
        let dir = PDirectory::from_path(&path);
        let dirs = FS_READ_SERVICE.read_dir(&dir)?;

        for i in dirs.directories {
            let path = make_path![i.path, "config.json"];
            if FS_READ_SERVICE.exists(path.clone()) {
                let file = PFile::from_path_reg(path);
                let Ok(text) = FS_READ_SERVICE.read_file(&file) else {
                    continue;
                };
                let Ok(parsed) = PARSING_SERVICE._from_string::<Package>(&text) else {
                    continue;
                };
                packages.push(parsed);
            }
        }
        Ok(packages)
    }

    fn add_package(&self, _pack: Package) -> Result<(), ProjectError> {
        todo!()
    }

    fn rem_package(&self, _pack: Package) -> Result<(), ProjectError> {
        todo!()
    }

    fn read_config(&self, id: String) -> Result<String, ProjectError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path_to_package = make_path![dir, "packages", id, "config.js"];
        let file = PFile::from_path_reg(path_to_package);
        let text = FS_READ_SERVICE.read_file(&file)?;
        Ok(text)
    }
}
