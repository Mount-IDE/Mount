use serde::{Deserialize, Serialize};

use mount_core::domain::Schema;

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FsIcon {
    //#[serde(rename="type")]
    #[serde(default)]
    pub typ: String,
    #[serde(default)]
    pub ext: Option<Vec<String>>,
    #[serde(default)]
    pub name: Option<Vec<String>>,
    #[serde(default)]
    pub icon: String,
}

///
///
///
#[allow(dead_code, unused)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FsType {
    #[allow(non_camel_case_types)]
    file,
    #[allow(non_camel_case_types)]
    dir,
}

///
///
///
impl Default for FsType {
    fn default() -> Self {
        FsType::file
    }
}

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
