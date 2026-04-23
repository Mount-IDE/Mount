use crate::modules::app::{CONFIG_SERVICE, FS_READ_SERVICE, FS_WRITE_SERVICE};
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::make_path;
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::{FileType, FileWriteAccess};
use crate::modules::contexts::project::app::traits::TProjectService;
use crate::modules::contexts::project::domain::entities::Project;
use crate::modules::shared::kernel::errors::{ParsingError, ProjectError};
use crate::modules::shared::kernel::values::Path;

use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::services::traits::TConfigService;

pub struct ProjectService();

impl TProjectService for ProjectService {
    ///
    ///
    ///
    fn create_project(&self, project: &Project) -> Result<(), ProjectError> {
        let path_to_project = make_path(vec![project.path.get().as_str()]);

        FS_WRITE_SERVICE.create_dir(&Path(path_to_project.clone()))?;

        let path_to_mount = make_path(vec![
            path_to_project.clone().as_str(),
            project.name.as_str(),
            ".mount",
        ]);
        FS_WRITE_SERVICE.create_dir(&Path(path_to_mount.clone()))?;
        let str = serde_json::to_string(&project).map_err(|e| {
            ProjectError::ParsingError(ParsingError::Serialize {
                path: Path(path_to_mount.clone()),
                err: e,
            })
        })?;

        let path_to_conf = make_path(vec![
            path_to_project.as_str(),
            project.name.as_str(),
            "project.json",
        ]);
        let file = PFile::regular("project.json".to_string(), Path(path_to_conf));
        FS_WRITE_SERVICE.write_file(&file, str, FileWriteAccess::WRITE)?;
        Ok(())
    }

    ///
    ///
    ///
    fn open_project(&self, project_path: &Path) -> Result<Project, ProjectError> {
        let path = make_path(vec![project_path.get().as_str(), ".mount", "project.json"]);
        let config = PFile {
            name: "config.json".to_string(),
            path: Path(path),
            typ: FileType::REGULAR,
        };
        let json = FS_READ_SERVICE.read_file(&config)?;
        let proj = serde_json::from_str::<Project>(&json).map_err(|e| {
            ProjectError::ParsingError(ParsingError::Deserialize {
                json: json.clone(),
                path: config.path.clone(),
                err: e,
            })
        })?;
        Ok(proj)
    }

    ///
    ///
    ///
    fn delete_project(&self, _project_path: &Path) -> Result<(), ProjectError> {
        todo!()
    }

    ///
    ///
    ///
    fn get_projects(&self, dir: &Path) -> Result<Vec<Project>, ProjectError> {
        let dir = PDirectory {
            name: "".to_string(),
            path: dir.clone(),
            files: vec![],
            directories: vec![],
        };
        let directory = FS_READ_SERVICE.read_dir(&dir)?;
        let mut projects: Vec<Project> = vec![];
        for dir in directory.directories {
            let mount = dir.directories.iter().find(|e| e.name == ".mount");
            if mount.is_none() {
                continue;
            }
            let mount = mount.unwrap().clone();
            let path_to = make_path(vec![mount.path.get().as_str(), "project.json"]);
            let file = PFile {
                name: "project.json".to_string(),
                path: Path(path_to.clone()),
                typ: FileType::REGULAR,
            };
            let file = FS_READ_SERVICE.read_file(&file);
            if file.is_err() {
                continue;
            }
            let file = file.unwrap();
            let proj = serde_json::from_str::<Project>(&file).map_err(|e| {
                ProjectError::ParsingError(ParsingError::Deserialize {
                    json: file,
                    path: Path(path_to.clone()),
                    err: e,
                })
            })?;
            projects.push(proj);
        }
        Ok(projects)
    }

    ///
    ///
    ///
    fn get_recent_projects(&self) -> Result<Vec<RecentProject>, ProjectError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let dir = PDirectory::from_path(&dir);
        let ext = FS_READ_SERVICE.exist_dir(&dir);
        if !ext {
            FS_WRITE_SERVICE.create_dir(&Path(dir.path.get()))?;
            return Ok(vec![]);
        }
        let path_to = make_path(vec![dir.path.get().as_str(), "recent-projects.json"]);
        let file = PFile::from_path_reg(Path(path_to.clone()));
        let ext = FS_READ_SERVICE.exist_file(&file);
        if !ext {
            FS_WRITE_SERVICE.create_file(&Path(path_to))?;
            return Ok(vec![]);
        }
        let file = PFile::regular("recent-projects.json".to_string(), Path(path_to));

        let text = FS_READ_SERVICE.read_file(&file)?;
        let projects = serde_json::from_str::<Vec<RecentProject>>(&text).map_err(|e| {
            ProjectError::ParsingError(ParsingError::Deserialize {
                json: text,
                path: file.path,
                err: e,
            })
        })?;
        Ok(projects)
    }
    ///
    ///
    ///
    fn save_project(&self, _project: &Project) -> Result<(), ProjectError> {
        todo!()
    }

    fn remove_from_recents(&self, _proj: &Project) -> Result<(), ProjectError> {
        todo!()
    }

    fn add_to_recents(&self, _project: &Project) -> Result<(), ProjectError> {
        todo!()
    }
}
