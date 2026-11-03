use super::values::FsIcon;
use crate::modules::app::FS_WRITE_SERVICE;
use crate::modules::contexts::filesystem::app::traits::TFSWriteService;
use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::contexts::filesystem::domain::values::FileWriteAccess;
use crate::modules::contexts::project::domain::entities::{
    PackageAction, PackageOption, PackageOptionTyp, PackageOptionTypEnum,
};
use crate::modules::shared::kernel::errors::FileSystemError;
use crate::modules::shared::kernel::values::{Path, Schema};
use crate::path_from;
use serde::{Deserialize, Serialize};

///
///
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsConfigIcons {
    theme: String,
    icons: Vec<FsIcon>,
    scheme: Schema,
}

impl Default for FsConfigIcons {
    fn default() -> Self {
        Self {
            theme: "_".to_string(),
            scheme: Schema(1),
            icons: vec![],
        }
    }
}

///
///
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFsTemplate {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    parameters: Option<ConfigFsParameters>,
    #[serde(default)]
    pub options: Option<Vec<PackageOption>>,
    #[serde(default)]
    pub actions: Option<Vec<PackageAction>>,
    #[serde(default)]
    pub entities: Option<Vec<ConfigFsEntity>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFsParameters {
    #[serde(default)]
    pub default_name: Option<String>,
    #[serde(default)]
    pub const_path: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "typ", rename_all = "lowercase")]
pub enum ConfigFsEntity {
    Dir {
        name: String,
        entities: Option<Vec<ConfigFsEntity>>,
    },
    File {
        name: String,
        content: Option<String>,
    },
}

impl ConfigFsEntity {
    pub fn create(&self, path: Path) -> Result<(), FileSystemError> {
        println!("| start");
        match self {
            ConfigFsEntity::Dir { name, entities } => {
                println!("|\tDIR");
                let path = path_from![path, name];
                let dir = FS_WRITE_SERVICE.create_dir(&path.clone())?;
                println!("\tDIR created");
                if let Some(entities) = entities {
                    for i in entities.iter() {
                        println!("\t | \t entity {i:?}");
                        i.create(path.clone())?
                    }
                }
            }
            ConfigFsEntity::File { name, content } => {
                println!("|\tFILE");
                let path = path_from![path, name];
                println!("\t{path:?}");
                let file = FS_WRITE_SERVICE.create_file(&path.clone())?;
                println!("\tfile created");
                if let Some(con) = content {
                    FS_WRITE_SERVICE.write_file(&file, con.clone(), FileWriteAccess::WRITE)?;
                    println!("\twrited");
                }
            }
        };
        Ok(())
    }
}

impl ConfigFsTemplate {
    pub fn file() -> Self {
        Self {
            id: "file".to_string(),
            title: "File".to_string(),
            icon: Some("any.svg".to_string()),
            parameters: None,
            options: Some(vec![PackageOption {
                id: "content".to_string(),
                title: "Content".to_string(),
                typ: PackageOptionTyp {
                    typ: PackageOptionTypEnum::INPUT,
                    fs_type: None,
                    list_type: None,
                    placeholder: None,
                    required: None,
                    readonly: None,
                    validate: None,
                    fs_filter: None,
                    gen_min: None,
                    gen_max: None,
                },
                def: None,
                while_: None,
            }]),
            actions: None,
            entities: Some(vec![ConfigFsEntity::File {
                name: "#meta.name".to_string(),
                content: Some("#content".to_string()),
            }]),
        }
    }

    pub fn dir() -> Self {
        Self {
            id: "dir".to_string(),
            title: "Directory".to_string(),
            icon: Some("dir.svg".to_string()),
            parameters: None,
            options: None,
            actions: None,
            entities: Some(vec![ConfigFsEntity::Dir {
                name: "#meta.name".to_string(),
                entities: None,
            }]),
        }
    }
}
