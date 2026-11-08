use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use mount_core::domain::platform::Platform;
use mount_core::domain::{Shell, Val};
//use ts_rs::TS;


#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TemplateMeta {
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
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

#[allow(dead_code, unused)]
#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[allow(dead_code, unused)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionCommand {
    #[serde(default)]
    pub platform: Platform,
    #[serde(default)]
    pub shell: Shell,
    #[serde(default)]
    pub env: Option<Vec<(String, String)>>,
    #[serde(default)]
    pub command: ActionCommandIn,
}

impl Default for ActionCommand {
    fn default() -> ActionCommand {
        Self {
            platform: Platform::current(),
            shell: Shell::default(),
            env: None,
            command: ActionCommandIn::Single(String::new()),
        }
    }
}

#[allow(dead_code, unused)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionCommandArgs(pub String, pub Vec<String>);

#[allow(dead_code, unused)]
#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ButtonPos {
    LeftTop,
    LeftBottom,
    RightTop,
}

pub(crate) type CreateProjectResult = HashMap<String, CreateProjectTemplate>; // template -> sections
pub type CreateProjectTemplate = HashMap<i8, HashMap<String, Val>>; // section id -> paramerer id -> value

pub type CreateProjectPackageResults = HashMap<String, HashMap<String, Val>>; // package_id -> option id -> value

pub type ResultsRecord = HashMap<String, Val>;
#[allow(unused)]
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
