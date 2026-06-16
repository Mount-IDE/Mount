use super::values::{FsIcon, FsType};
use crate::modules::shared::kernel::values::Schema;
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
    id: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    typ: FsType,
    #[serde(default)]
    icon: Option<String>,
    #[serde(default)]
    ext: Option<String>,
    #[serde(default)]
    default_content: Option<String>,
    #[serde(default)]
    inner: Option<Vec<ConfigFsTemplate>>,
    #[serde(default)]
    base_name: Option<String>,
}

impl ConfigFsTemplate {
    pub fn file() -> Self {
        Self {
            id: "empty".to_string(),
            title: "Empty File".to_string(),
            typ: FsType::file,
            icon: Some("any.svg".to_string()),
            ext: None,
            default_content: None,
            inner: None,
            base_name: None,
        }
    }

    pub fn dir() -> Self {
        Self {
            id: "dir".to_string(),
            title: "Directory".to_string(),
            typ: FsType::dir,
            icon: Some("dir.svg".to_string()),
            ext: None,
            default_content: None,
            inner: None,
            base_name: None,
        }
    }
}
