use notify::RecommendedWatcher;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::thread::JoinHandle;

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileType {
    BINARY,
    REGULAR,
}

#[derive(Clone)]

pub enum FileWriteAccess {
    WRITE,
    APPEND,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FsPath {
    SINGLE(String),
    COUPLE(String, String),
}

impl Display for FsPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FsPath::SINGLE(v) => write!(f, "{}", v),
            FsPath::COUPLE(v, _) => write!(f, "{}", v),
        }
    }
}

impl FsPath {
    pub fn to_string_(&self) -> String {
        match self {
            FsPath::SINGLE(s) => s.to_string(),
            FsPath::COUPLE(v, _) => v.to_string(),
        }
    }
}

pub struct WatchInstance {
    pub watcher: RecommendedWatcher,
    pub thread: JoinHandle<()>,
}
