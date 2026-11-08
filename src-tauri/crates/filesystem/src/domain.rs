use crate::FS_WRITE_SERVICE;
use crate::traits::TFSWriteService;
use crate::values::{FileType, FileWriteAccess};
use declarative::domain::{
    PackageAction, PackageActionCommand, PackageActionCommandType, PackageOptionTyp,
    PackageOptionTypEnum, Parameter,
};
use mount_core::domain::platform::Platform;
use mount_core::domain::{IfStatementOperation, IfStatementPart, Path, Val};
use mount_core::make_path;
use mount_core::traits::PathPart;
use serde::{Deserialize, Serialize};
use std::{
    path,
    time::{Duration, SystemTime},
};
use thiserror::Error;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FsMeta {
    pub modified: u64,
    pub readonly: bool,
    pub memory: u64,
}

impl FsMeta {
    pub fn by_path(path: Path) -> Self {
        let path = path.clone().get();
        let path = path::Path::new(&path);
        let mut modified = 0u64;
        let mut readonly = false;
        let mut memory = 0u64;
        if let Ok(meta) = path.metadata() {
            modified = meta
                .modified()
                .unwrap_or(SystemTime::now())
                .elapsed()
                .unwrap_or(Duration::new(0, 0))
                .as_secs();
            readonly = meta.permissions().readonly();
            memory = meta.len();
        }
        Self {
            modified,
            readonly,
            memory,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PFile {
    pub name: String,
    pub path: Path,
    pub typ: FileType,

    pub meta: Option<FsMeta>,
}

impl PFile {
    #[allow(unused)]
    pub fn regular(name: String, path: Path) -> PFile {
        Self {
            name,
            path,
            typ: FileType::REGULAR,
            meta: None,
        }
    }
    #[allow(unused)]
    pub fn binary(name: String, path: Path) -> PFile {
        Self {
            name,
            path,
            typ: FileType::BINARY,
            meta: None,
        }
    }

    pub fn ext(&self) -> Option<String> {
        let found = self.name.rfind(".");
        if found.is_some() {
            let found = found.unwrap();
            let slice = &self.name.clone()[found + 1..];
            return Some(slice.to_string());
        }
        None
    }

    pub fn from_path_reg(path: Path) -> PFile {
        let path_ = path.split();
        if path_.len() == 0 {
            return Self {
                name: "".to_string(),
                path: Path(String::new()),
                typ: FileType::REGULAR,
                meta: None,
            };
        }
        let name = path_.get(path_.len() - 1);
        if name.is_none() {
            return Self {
                name: "".to_string(),
                path: Path(String::new()),
                typ: FileType::REGULAR,
                meta: None,
            };
        }
        let name = name.unwrap();
        Self {
            name: name.to_string(),
            path,
            typ: FileType::REGULAR,
            meta: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PDirectory {
    pub name: String,
    pub path: Path,
    pub files: Vec<PFile>,
    pub directories: Vec<PDirectory>,
    pub meta: Option<FsMeta>,
}
impl PDirectory {
    pub fn new() -> PDirectory {
        Self {
            name: String::new(),
            path: Path(String::new()),
            files: Vec::new(),
            directories: Vec::new(),
            meta: None,
        }
    }

    pub fn from_path(path: &Path) -> PDirectory {
        let path_ = path.split();
        if path_.len() == 0 {
            return PDirectory::new();
        }
        let name = path_.get(path_.len() - 1);
        if name.is_none() {
            return PDirectory::new();
        }
        let name = name.unwrap();
        Self {
            name: name.to_string(),
            path: path.clone(),
            files: Vec::new(),
            directories: Vec::new(),
            meta: None,
        }
    }
}

#[derive(Error, Debug)]
#[allow(unused)]
pub enum FileSystemError {
    #[error("failed to create directory: {path}")]
    DirCreation {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to create file: {path}")]
    FileCreation {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to open file: {path}")]
    FileOpen {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to read file: {path}")]
    FileRead {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to read directory: {path}")]
    DirRead {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to write file: {path}")]
    FileWrite {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to remove directory: {path}")]
    DirRemove {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to remove file: {path}")]
    FileRemove {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("file already exists: {path}")]
    FileAlreadyExists {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("directory already exists: {path}")]
    DirAlreadyExists {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to parse path: {path}")]
    PathParsing { path: Path },
    #[error("failed to watch directory {path}")]
    Watch { path: Path },
    #[error("failed to rename from \"{from}\" to \"{to}\"")]
    Rename {
        from: Path,
        to: Path,
        e: std::io::Error,
    },

    #[error("results ddon't have any record with id {0}")]
    FsEntityResults(String),
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
    pub options: Option<Vec<Parameter>>,
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
                let path = make_path![path, *name];
                let _ = FS_WRITE_SERVICE.create_dir(&path.clone())?;
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
                let path = make_path![path, *name];
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
            options: Some(vec![Parameter {
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

    #[cfg(any(test, debug_assertions))]
    pub fn cargo() -> Self {
        Self {
            id: "cargo".to_string(),
            title: "Cargo Crate".to_string(),
            icon: Some("cargo.svg".__get()),
            parameters: None,
            options: Some(vec![Parameter {
                id: "typ".to_string(),
                title: "".to_string(),
                typ: PackageOptionTyp {
                    typ: PackageOptionTypEnum::LIST,
                    fs_type: None,
                    list_type: Some(vec![
                        "binary".__get(),
                        "library".__get(),
                        "proc_macro".__get(),
                        "workspace".__get(),
                    ]),
                    placeholder: None,
                    required: None,
                    readonly: None,
                    validate: None,
                    fs_filter: None,
                    gen_min: None,
                    gen_max: None,
                },
                def: Some(Val::STRING("binary".__get())),
                while_: None,
            }]),
            actions: Some(vec![PackageAction {
                id: 0,
                next: None,
                if_: Some(vec![vec![
                    IfStatementPart {
                        from: Some("#typ".__get()),
                        oper: IfStatementOperation::EQ,
                        value: Some(Val::STRING("binary".__get())),
                    },
                    IfStatementPart {
                        from: Some("#meta.name".__get()),
                        oper: IfStatementOperation::NonEmpty,
                        value: None,
                    },
                    IfStatementPart {
                        from: Some("#meta.path".__get()),
                        oper: IfStatementOperation::NonEmpty,
                        value: None,
                    },
                ]]),
                on_error: Default::default(),
                platform: None,
                command: Some(vec![
                    PackageActionCommand {
                        platform: Some(Platform::unix_like()),
                        cwd: None,
                        shell: None,
                        env: None,
                        needed_exit_code: None,
                        command: PackageActionCommandType::WithArgs {
                            command: "cargo new".to_string(),
                            args: vec!["#{meta.path}/#{meta.name}".__get()],
                        },
                    },
                    PackageActionCommand {
                        platform: Some(Platform::windows()),
                        cwd: None,
                        shell: None,
                        env: None,
                        needed_exit_code: None,
                        command: PackageActionCommandType::WithArgs {
                            command: "cargo new".to_string(),
                            args: vec!["#{meta.path}\\#{meta.name}".__get()],
                        },
                    },
                ]),
            }]),
            entities: None,
        }
    }
}
