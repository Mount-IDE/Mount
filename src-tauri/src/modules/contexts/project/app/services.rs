use crate::modules::app::{FS_READ_SERVICE, FS_WRITE_SERVICE};
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::make_path;
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::{FileType, FileWriteAccess};
use crate::modules::contexts::project::app::traits::TProjectService;
use crate::modules::contexts::project::domain::entities::Project;
use crate::modules::shared::kernel::entities::{ProjectError};
use crate::modules::shared::kernel::values::Path;

pub struct ProjectService();

impl TProjectService for ProjectService {
    ///
    /// 
    /// 
    fn create_project(&self, project: &Project) -> Result<(), ProjectError> {
        let path_to_project = make_path(vec![project.path.get().as_str()]);
        let res = FS_WRITE_SERVICE.create_dir(&Path(path_to_project.clone()));
        if res.is_err() {
            return Err(ProjectError::empty());
        }

        let path_to_mount = make_path(vec![
            path_to_project.clone().as_str(),
            project.name.as_str(),
            ".mount",
        ]);

        let res = FS_WRITE_SERVICE.create_dir(&Path(path_to_mount));
        if res.is_err() {
            return Err(ProjectError::empty());
        }
        let str = serde_json::to_string(&project);
        if str.is_err() {
            return Err(ProjectError::empty());
        }
        let str = str.unwrap();
        let path_to_conf = make_path(vec![
            path_to_project.as_str(),
            project.name.as_str(),
            "project.json",
        ]);
        let file = PFile {
            name: "project.json".to_string(),
            path: Path(path_to_conf),
            typ: FileType::REGULAR,
        };
        let res = FS_WRITE_SERVICE.write_file(&file, str, FileWriteAccess::WRITE);
        if res.is_err() {
            return Err(ProjectError::empty());
        }
        Ok(())
    }

    ///
    /// 
    /// 
    fn open_project(&self, project_path: Path) -> Result<Project, ProjectError> {
        let path = make_path(vec![project_path.get().as_str(), ".mount", "project.json"]);

        let config = PFile {
            name: "config.json".to_string(),
            path: Path(path),
            typ: FileType::REGULAR,
        };

        let json = FS_READ_SERVICE.read_file(&config);
        if json.is_err() {
            return Err(ProjectError::empty());
        }
        let json = json.unwrap();
        let res = serde_json::from_str::<Project>(&json);
        if res.is_err() {
            return Err(ProjectError::empty());
        }
        let proj: Project = res.unwrap();
        Ok(proj)
    }

    ///
    /// 
    /// 
    fn delete_project(&self, project_path: Path) -> Result<(), ProjectError> {
        todo!()
    }

    
    ///
    /// 
    /// 
    fn get_projects(&self, dir: Path) -> Result<Vec<Project>, ProjectError> {
        let dir = PDirectory {
            name: "".to_string(),
            path: dir,
            files: vec![],
            directories: vec![],
        };
        let directory = FS_READ_SERVICE.read_dir(&dir);
        if directory.is_err() {
            return Err(ProjectError::empty());
        }
        let directory = directory.unwrap();
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
                path: Path(path_to),
                typ: FileType::REGULAR,
            };
            let file = FS_READ_SERVICE.read_file(&file);
            if file.is_err() { 
                continue;
            }
            let file = file.unwrap();
            let proj = serde_json::from_str::<Project>(&file);
            if proj.is_err() {
                return Err(ProjectError::empty());
            }
            let proj = proj.unwrap();
            projects.push(proj);
        }
        Ok(projects)
    }
}
