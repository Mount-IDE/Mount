use super::default::action::*;
use super::default::template::t_meta_icon;
use crate::modules::shared::kernel::values::Val;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Debug;
use ts_rs::TS;

#[derive(Clone, TS, Debug, Serialize, Deserialize)]
pub enum ActionOnError {
    #[serde(rename = "continue")]
    CONTINUE,
    #[serde(rename = "stop_all")]
    StopAll,
    #[serde(rename = "stop_graph")]
    StopGraph,
}

impl Default for ActionOnError {
    fn default() -> Self {
        Self::StopAll
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct ProjectMeta {
    pub(crate) authors: Vec<String>,
    pub(crate) description: String,
    pub(crate) license: Option<String>,
    pub(crate) group: String,
    pub(crate) tags: Vec<String>,
    pub(crate) icon: Option<String>,
}

impl Default for ProjectMeta {
    fn default() -> Self {
        Self {
            authors: Vec::new(),
            description: String::new(),
            license: None,
            group: String::from("general"),
            tags: Vec::new(),
            icon: None,
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
            icon: None,
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

impl Default for TemplateMeta {
    fn default() -> Self {
        Self {
            authors: vec![],
            description: String::new(),
            icon: String::from("empty.svg"),
        }
    }
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
    pub platform: String,
    #[serde(default = "t_shell_def")]
    pub shell: String,
    #[serde(default)]
    pub env: Option<Vec<(String, String)>>,
    #[serde(default)]
    pub command: ActionCommandIn,
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
pub struct ActionCommandArgs(pub String, pub Vec<String>);
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[serde(untagged)]
pub enum ActionCommandIn {
    Single(String),
    WithArgs(ActionCommandArgs),
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

pub(crate) type CreateProjectResult = HashMap<String, CreateProjectTemplate>; // template -> sections
pub type CreateProjectTemplate = HashMap<i8, HashMap<String, Val>>; // section id -> paramerer id -> value

pub type CreateProjectPackageResults = HashMap<String, HashMap<String, Val>>; // package_id -> option id -> value

pub type ResultsRecord = HashMap<String, Val>;

pub trait TRes {
    fn get_value(&self, addr: String) -> Option<Val>;
}

impl TRes for CreateProjectTemplate {
    fn get_value(&self, addr: String) -> Option<Val> {
        let point = addr.find(".")?;
        let left = &addr[..point]
            .chars()
            .filter(|e| e.is_ascii_digit())
            .collect::<String>();
        let right = addr[point + 1..].to_string();

        let left = left.parse::<i8>().ok()?;
        let first = self.get(&left)?;
        let sec = first.get(right.clone().as_str())?;
        Some(sec.clone())
    }
}
impl TRes for ResultsRecord {
    fn get_value(&self, addr: String) -> Option<Val> {
        let res = self.get(&addr)?;
        Some(res.clone())
    }
}
