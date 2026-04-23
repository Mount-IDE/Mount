use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use tauri::ipc::InvokeError;
use crate::modules::shared::kernel::entities::{Error, FileSystemError, ProjectError};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectMeta{
    authors: Vec<String>,
    description: String,
    license: Option<String>,
    group: String,
    tags: Vec<String>,
    
}

impl ProjectMeta{
    pub fn new()->Self{
        Self {
            authors: Vec::new(),
            description: String::new(),
            license: None,
            group: String::new(),
            tags: Vec::new(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct TemplateMeta {
    authors: Vec<String>,
    description: String,
    icon: String
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ParameterLabel{
    STR(String),
    COUPLE((String, String)),
}


#[derive(Serialize, Deserialize, Clone)]
pub struct ActionCommand{
    shell: String,
    env: Option<Vec<(String, String)>>,
    command: ActionCommandIn
}

impl ActionCommand{
    pub fn new()->ActionCommand{
        Self{
            shell: String::new(),
            env: None,
            command: ActionCommandIn::Single(String::new())
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ActionCommandIn{
    Single(String),
    WithArgs(String, Vec<String>)
}



#[derive(Serialize, Deserialize, Clone)]
pub enum ButtonPos{
    LeftTop,
    LeftBottom,
    RightTop,
}

#[derive(Clone, Debug)]
pub enum ProjectToFS{
    Project(ProjectError),
    FS(FileSystemError)
}

impl From<ProjectToFS> for InvokeError {
    fn from(value: ProjectToFS) -> Self {
        match value{
            ProjectToFS::Project(error) => error.into(),
            ProjectToFS::FS(error) => error.into()
        }
    }
}
