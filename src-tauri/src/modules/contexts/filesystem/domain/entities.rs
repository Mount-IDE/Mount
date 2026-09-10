use crate::modules::contexts::filesystem::app::utils::split_path;
use crate::modules::contexts::filesystem::domain::values::FileType;
use crate::modules::shared::kernel::values::Path;
use serde::{Deserialize, Serialize};
use std::path;
use std::time::Duration;

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
        Self {
            modified: path
                .metadata()
                .unwrap()
                .modified()
                .unwrap_or(std::time::SystemTime::now())
                .elapsed()
                .unwrap_or(Duration::new(0, 0))
                .as_secs(),
            readonly: path.metadata().unwrap().permissions().readonly(),
            memory: path.metadata().unwrap().len(),
        }
        /* Self {
            modified: 0,
            readonly: true,
            memory: 0
        }*/
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
        let path_ = split_path(&path);
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
        let path_ = split_path(&path);
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
