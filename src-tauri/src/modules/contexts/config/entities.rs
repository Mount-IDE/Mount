use super::values::{FsIcon, FsType};
use crate::modules::shared::kernel::values::Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsConfigIcons {
    theme: String,
    icons: Vec<FsIcon>,
    scheme: Schema,
}


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
