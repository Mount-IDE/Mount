use super::default::action::*;
use super::default::template::t_meta_icon;
use crate::modules::shared::kernel::values::Val;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Debug;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct ProjectMeta {
    pub(crate) authors: Vec<String>,
    pub(crate) description: String,
    pub(crate) license: Option<String>,
    pub(crate) group: String,
    pub(crate) tags: Vec<String>,
}

impl Default for ProjectMeta {
    fn default() -> Self {
        Self {
            authors: Vec::new(),
            description: String::new(),
            license: None,
            group: String::from("general"),
            tags: Vec::new(),
        }
    }
}

impl ProjectMeta {
    pub fn new() -> Self {
        Self {
            authors: Vec::new(),
            description: String::new(),
            license: None,
            group: String::new(),
            tags: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct PackageMeta {
    #[serde(default)]
    authors: Vec<String>,
    #[serde(default)]
    description: String,
}

impl Default for PackageMeta {
    fn default() -> Self {
        Self {
            authors: Vec::new(),
            description: String::new(),
        }
    }
}

impl PackageMeta {
    pub fn new() -> Self {
        Self {
            authors: Vec::new(),
            description: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct TemplateMeta {
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default = "t_meta_icon")]
    pub icon: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[serde(untagged)]
pub enum ParameterLabel {
    STR(String),
    COUPLE((String, String)),
}

impl Default for ParameterLabel {
    fn default() -> Self {
        ParameterLabel::STR("".to_string())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct ActionCommand {
    #[serde(default = "t_platform_def")]
    platform: String,
    #[serde(default = "t_shell_def")]
    shell: String,
    env: Option<Vec<(String, String)>>,
    #[serde(default)]
    command: ActionCommandIn,
}

impl ActionCommand {
    pub fn new() -> ActionCommand {
        Self {
            platform: String::new(),
            shell: String::new(),
            env: None,
            command: ActionCommandIn::Single(String::new()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[serde(untagged)]
pub enum ActionCommandIn {
    Single(String),
    WithArgs(String, Vec<String>),
}

impl Default for ActionCommandIn {
    fn default() -> Self {
        Self::Single(String::new())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub enum ButtonPos {
    LeftTop,
    LeftBottom,
    RightTop,
}

pub(crate) type CreateProjectResult = HashMap<String, HashMap<i8, HashMap<String, Val>>>;
