use serde::{Deserialize, Serialize};
use std::{
    path,
    time::{Duration, SystemTime},
};
use thiserror::Error;
use mount_core::domain::Path;
use crate::values::FileType;

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




