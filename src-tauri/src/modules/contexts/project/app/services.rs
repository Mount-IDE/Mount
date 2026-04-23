use crate::modules::app::{APP, CONFIG_SERVICE, FS_READ_SERVICE, FS_WRITE_SERVICE};
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::app::utils::make_path;
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::{FileType, FileWriteAccess};
use crate::modules::contexts::project::app::traits::TProjectService;
use crate::modules::contexts::project::domain::entities::Project;
use crate::modules::shared::kernel::entities::{ProjectError};
use crate::modules::shared::kernel::values::{Path, ProjectErrorType};

use std::fs;
use tauri::Manager;
use crate::modules::contexts::project::domain::values::ProjectToFS;
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::services::traits::TConfigService;

pub struct ProjectService();

impl TProjectService for ProjectService {
    ///
    ///
    ///
    fn create_project(&self, project: &Project) -> Result<(), ProjectToFS> {
        let path_to_project =
            make_path(vec![project.path.get().as_str()]);

        let res =
            FS_WRITE_SERVICE.create_dir(&Path(path_to_project.clone()));
        if res.is_err() {
            return Err(ProjectToFS::FS(res.err().unwrap()));
        }

        let path_to_mount = make_path(vec![
            path_to_project.clone().as_str(),
            project.name.as_str(),
            ".mount",
        ]);

        let res =
            FS_WRITE_SERVICE.create_dir(&Path(path_to_mount));
        if res.is_err() {
            return Err(ProjectToFS::FS(res.err().unwrap()));
        }
        let str = serde_json::to_string(&project);
        if str.is_err() {
            return Err(
                ProjectToFS::Project(
                    ProjectError::error(ProjectErrorType::ProjectJsonParseError)));
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
            return Err(ProjectToFS::FS(res.err().unwrap()));
        }
        Ok(())
    }

    ///
    ///
    ///
    fn open_project(&self, project_path: Path) -> Result<Project, ProjectToFS> {
        let path = make_path(
            vec![project_path.get().as_str(), ".mount", "project.json"]);

        let config = PFile {
            name: "config.json".to_string(),
            path: Path(path),
            typ: FileType::REGULAR,
        };

        let json = FS_READ_SERVICE.read_file(&config);
        if json.is_err() {
            return Err(ProjectToFS::FS(json.err().unwrap()));
        }
        let json = json.unwrap();
        let res = serde_json::from_str::<Project>(&json);
        if res.is_err() {
            return Err(
                ProjectToFS::Project(
                    ProjectError::error(ProjectErrorType::ProjectJsonParseError)));
        }
        let proj: Project = res.unwrap();
        Ok(proj)
    }

    ///
    ///
    ///
    fn delete_project(&self, project_path: Path) -> Result<(), ProjectToFS> {
        todo!()
    }

    ///
    ///
    ///
    fn get_projects(&self, dir: Path) -> Result<Vec<Project>, ProjectToFS> {
        let dir = PDirectory {
            name: "".to_string(),
            path: dir,
            files: vec![],
            directories: vec![],
        };
        let directory = FS_READ_SERVICE.read_dir(&dir);
        if directory.is_err() {
            return Err(ProjectToFS::FS(directory.err().unwrap()));
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
                return Err(
                    ProjectToFS::Project(
                        ProjectError::error(ProjectErrorType::ProjectJsonParseError)));
            }
            let proj = proj.unwrap();
            projects.push(proj);
        }
        Ok(projects)
    }

    ///
    ///
    ///
    fn get_recent_projects(&self) -> Result<Vec<RecentProject>, ProjectToFS> {
        let dir = CONFIG_SERVICE.get_data_dir();
        if dir.is_err() {
            return Err(ProjectToFS::Project(ProjectError::empty()));
        }
        let dir = dir.unwrap();
        let dir = PDirectory::from_path(&dir);
        let ext = FS_READ_SERVICE.exist_dir(&dir);
        if !ext {
            let res =
                FS_WRITE_SERVICE.create_dir(&Path(dir.path.get()));
            if res.is_err() {
                return Err(ProjectToFS::FS(res.err().unwrap()));
            }
            return Ok(vec![]);
        }
        let path_to = make_path(vec![dir.path.get().as_str(), "recent-projects.json"]);
        let file = PFile::from_path_reg(Path(path_to.clone()));
        let ext = FS_READ_SERVICE.exist_file(&file);
        if !ext {
            let res = FS_WRITE_SERVICE.create_file(&Path(path_to));
            if res.is_err() {
                return Err(ProjectToFS::FS(res.err().unwrap()));
            }
            return Ok(vec![]);
        }
        let file = PFile {
            name: "recent-projects.json".to_string(),
            path: Path(path_to),
            typ: FileType::REGULAR,
        };
        let text = FS_READ_SERVICE.read_file(&file);
        if text.is_err() {
            return Err(ProjectToFS::FS(text.err().unwrap()));
        }
        let text = text.unwrap();
        let res = serde_json::from_str::<Vec<RecentProject>>(&text);
        if res.is_err() {
            return
                Err(ProjectToFS::Project(
                        ProjectError::error(ProjectErrorType::ProjectJsonParseError)));
        }
        let projects = res.unwrap();

        Ok(projects)
    }
    ///
    ///
    ///
    fn save_project(&self, project: Project) -> Result<(), ProjectToFS> {
        todo!()
    }

    fn remove_from_recents(&self, proj: &Project) -> Result<(), ProjectToFS> {



        Ok(())
    }

    fn add_to_recents(&self, project: &Project) -> Result<(), ProjectToFS> {
        todo!()
    }
}
