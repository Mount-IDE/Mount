use crate::errors::ProjectError;
use crate::project::domain::{Package, Project, ProjectTemplate, RecentProject};
use crate::project::traits::TProjectService;
use config::CONFIG_SERVICE;
use config::errors::ConfigError;
use config::traits::TConfigService;
use filesystem::domain::{PDirectory, PFile};
use filesystem::traits::{TFSReadService, TFSWriteService};
use filesystem::values::FileWriteAccess;
use filesystem::{FS_READ_SERVICE, FS_WRITE_SERVICE};
use mount_core::domain::Path;
use mount_core::make_path;
use parsing::PARSING_SERVICE;
use parsing::traits::TParsingService;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ProjectService;

impl TProjectService for ProjectService {
    ///
    ///
    ///
    fn open_project(&self, project_path: &Path) -> Result<Project, ProjectError> {
        let path = make_path![*project_path, ".mount", "project.json",];
        let config = PFile::from_path_reg(path);

        let json = FS_READ_SERVICE.read_file(&config)?;
        let proj = PARSING_SERVICE._from_string::<Project>(&json)?;
        Ok(proj)
    }

    ///
    ///
    ///
    fn delete_project(&self, project_path: &Path) -> Result<Project, ProjectError> {
        let path_ = make_path![*project_path, ".mount", "project.json",];
        let file = PFile::from_path_reg(path_.clone());
        let text = FS_READ_SERVICE.read_file(&file)?;
        let json = PARSING_SERVICE._from_string::<Project>(&text)?;

        let proj_dir = PDirectory::from_path(&project_path.clone());
        FS_WRITE_SERVICE.remove_dir(&proj_dir)?;
        Ok(json)
    }

    ///
    ///
    ///
    fn get_projects(&self, dir: &Path) -> Result<Vec<Project>, ProjectError> {
        let dir = PDirectory::from_path(dir);

        let directory = FS_READ_SERVICE.read_dir(&dir)?;
        let mut projects: Vec<Project> = vec![];
        for dir in directory.directories {
            let mount = dir.directories.iter().find(|e| e.name == ".mount");
            if mount.is_none() {
                continue;
            }
            let mount = mount.unwrap().clone();
            let path_to = make_path![mount.path, "project.json"];

            let file = PFile::from_path_reg(path_to);
            let Ok(file) = FS_READ_SERVICE.read_file(&file) else {
                continue;
            };
            let proj = PARSING_SERVICE._from_string::<Project>(&file)?;
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
        let path_to = make_path![dir.path, "recent-projects.json"];

        let file = PFile::from_path_reg(path_to.clone());
        let ext = FS_READ_SERVICE.exist_file(&file);
        if !ext {
            FS_WRITE_SERVICE.create_file(&path_to)?;
            return Ok(vec![]);
        }
        let file = PFile::from_path_reg(path_to);

        let text = FS_READ_SERVICE.read_file(&file)?;
        let mut projects = PARSING_SERVICE._from_string::<Vec<RecentProject>>(&text)?;

        projects.sort_by(|a, b| b.last_opened.cmp(&a.last_opened));
        Ok(projects)
    }
    ///
    ///
    ///
    fn save_project(&self, _project: &Project) -> Result<(), ProjectError> {
        let path = _project.path.clone();
        let path_ = make_path![path, _project.name, ".mount", "project.json",];
        let json = PARSING_SERVICE.to_string(&_project)?;
        let file = PFile::from_path_reg(path_);
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
        Ok(())
    }

    fn save_packages(&self, packs: Vec<Package>, path: Path) -> Result<(), ProjectError> {
        let path_ = make_path![path, ".mount", "packages.json"];
        let json = PARSING_SERVICE.to_string(&packs)?;
        let file = PFile::from_path_reg(path_);
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;

        Ok(())
    }

    fn remove_from_recents(&self, _proj: &Project) -> Result<(), ProjectError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path_ = make_path![dir, "recent-projects.json"];
        let file = PFile::from_path_reg(path_.clone());
        let text = FS_READ_SERVICE.read_file(&file)?;
        let json = PARSING_SERVICE._from_string::<Vec<RecentProject>>(&text)?;

        let path1 = make_path![_proj.path, _proj.name,];
        let new_json = json
            .iter()
            .filter_map(|el| {
                let path2 = make_path![el.path, el.name,];
                if path1.get() == path2.get() {
                    return None;
                }
                Some(el.clone())
            })
            .collect::<Vec<RecentProject>>();

        let json = PARSING_SERVICE.to_string(&new_json)?;
        FS_WRITE_SERVICE.write_file(&file, json, FileWriteAccess::WRITE)?;
        Ok(())
    }

    fn add_to_recents(&self, _project: &Project) -> Result<(), ProjectError> {
        let data = SystemTime::now();
        let now = data.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let path = _project.path.clone();
        let name = _project.name.clone();
        let meta = _project.meta.clone();
        let packages = _project.packages.clone();
        let recent = RecentProject {
            name,
            path,
            last_opened: now,
            meta,
            packages,
        };
        let dir = CONFIG_SERVICE.get_data_dir()?;
        // if dir.is_err() {
        //    /* //CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
        //     dir = CONFIG_SERVICE.get_data_dir();
        //     if dir.is_err() {
        //         return Err(dir.err().unwrap().into());
        //     }*/
        //     return Err(ProjectError::ConfigError())
        // }
        // let dir = dir.unwrap();
        let path_ = make_path![dir, "recent-projects.json"];
        let file = PFile::from_path_reg(path_.clone());
        let text = FS_READ_SERVICE.read_file(&file)?;
        /*if text.is_err() {
            CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
            text = FS_READ_SERVICE.read_file(&file);
            if text.is_err() {
                return Err(text.err().unwrap().into());
            }
        }*/
        // let text = text.unwrap();
        let mut data = PARSING_SERVICE._from_string::<Vec<RecentProject>>(&text)?;
        data.push(recent);
        data.sort_by(|a, b| a.last_opened.cmp(&b.last_opened));

        let text = PARSING_SERVICE.to_string(&data)?;
        FS_WRITE_SERVICE.write_file(&file, text, FileWriteAccess::WRITE)?;
        Ok(())
    }

    fn read_templates(&self) -> Result<Vec<ProjectTemplate>, ConfigError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let path_ = make_path![dir, "templates.json"];
        let file = PFile::from_path_reg(path_.clone());

        let content = FS_READ_SERVICE.read_file(&file)?;

        let json = PARSING_SERVICE._from_string::<Vec<ProjectTemplate>>(&content)?;
        Ok(json)
    }
}
