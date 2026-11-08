use mount_core::domain::Path;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct FSFile {
    pub name: String,
    pub path: Path,
    pub content: Option<String>,
}

#[derive(Clone, Debug)]
pub struct FSDir {
    pub name: String,
    pub path: Path,
    pub entities: Option<Vec<FsEntity_>>,
}

impl FSDir {
    pub fn get_path(&self) -> Path {
        if self.path.get().len() == 0 {
            return Path(self.name.clone());
        }
        self.path.clone()
    }
}
#[derive(Clone, Debug)]
pub enum FsEntity_ {
    FILE(FSFile),
    DIR(FSDir),
}

pub trait TFsEntity<T> {
    type F;
    #[allow(unused)]
    fn file(name: T, path: Path) -> Self;

    #[allow(unused)]
    fn file_content(name: T, path: Path, content: T) -> Self;

    fn dir(name: T) -> Self;
    #[allow(unused)]
    fn dir_in(name: T, path: Path) -> Self;
    fn dir_entities(name: T, entities: Vec<Self::F>) -> Self;
    #[allow(unused)]
    fn file_s(name: T) -> Self;
    fn file_s_content(name: T, content: T) -> Self;
}

impl TFsEntity<&str> for FsEntity_ {
    type F = FsEntity_;

    fn file(name: &str, path: Path) -> Self {
        Self::FILE(FSFile {
            name: String::from(name),
            path,
            content: None,
        })
    }

    fn file_content(name: &str, path: Path, content: &str) -> Self {
        Self::FILE(FSFile {
            name: String::from(name),
            path,
            content: Some(content.to_string()),
        })
    }

    fn dir(name: &str) -> Self {
        Self::DIR(FSDir {
            name: name.to_string(),
            path: Path::empty(),
            entities: None,
        })
    }

    fn dir_in(name: &str, path: Path) -> Self {
        Self::DIR(FSDir {
            name: name.to_string(),
            path,
            entities: None,
        })
    }

    fn dir_entities(name: &str, entities: Vec<Self::F>) -> Self {
        Self::DIR(FSDir {
            name: name.to_string(),
            entities: Some(entities),
            path: Path::empty(),
        })
    }

    fn file_s(name: &str) -> Self {
        Self::FILE(FSFile {
            name: name.to_string(),
            path: Path::empty(),
            content: None,
        })
    }

    fn file_s_content(name: &str, content: &str) -> Self {
        Self::FILE(FSFile {
            name: name.to_string(),
            path: Path::empty(),
            content: Some(content.to_string()),
        })
    }
}

impl FsEntity_ {
    #[allow(unused)]
    pub fn get_path(&self) -> Path {
        match self {
            Self::DIR(dir) => {
                if dir.path.get().len() == 0 {
                    Path(dir.name.clone())
                } else {
                    dir.path.clone()
                }
            }
            Self::FILE(file) => file.path.clone(),
        }
    }
}

impl Display for FsEntity_ {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FILE(file) => {
                write!(f, "FILE {} :: {}", file.name, file.path)
            }
            Self::DIR(dir) => {
                write!(
                    f,
                    "DIR {} :: {} :: {}",
                    dir.name,
                    dir.path,
                    dir.entities.clone().unwrap_or(vec![]).len()
                )
            }
        }
    }
}
