use crate::modules::contexts::filesystem::app::utils::make_path;
use crate::modules::contexts::filesystem::domain::values::FsPath;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum PlatformType {
    WINDOWS,
    MACOS,
    LINUX,
    ALL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Platform {
    ARRAY(Vec<PlatformType>),
    SINGLE(PlatformType),
}



#[derive(Clone, Serialize, Debug, Deserialize, TS)]
pub struct Path(pub String);
impl Path {
    pub fn new(name: &str) -> Self {
        Self {
            0: name.to_string(),
        }
    }
    pub fn get(&self) -> String {
        self.0.clone()
    }

    #[allow(unused)]
    pub fn from_fs_path(path_: Vec<FsPath>) -> Self {
        let str_ = path_
            .iter()
            .map(|e| e.to_string_())
            .collect::<Vec<String>>();
        let str_ = str_.iter().map(|e| e.as_str()).collect::<Vec<&str>>();

        let path = make_path(str_);
        path
    }

    pub fn empty() -> Self {
        Self("".to_string())
    }
}

impl Default for Path {
    fn default() -> Self {
        Self("".to_string())
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
pub struct Schema(pub u8);

impl Default for Schema {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Val {
    NUMBER(f64),
    STRING(String),
    BOOL(bool),
    ARRAY(Vec<String>),
}

impl Val {
    pub fn to_str(&self) -> String {
        match &self {
            Val::STRING(val) => val.clone(),
            Val::NUMBER(val) => val.to_string(),
            Val::BOOL(val) => val.to_string(),
            Val::ARRAY(val) => val.join("::"),
        }
    }
}

impl Default for Val {
    fn default() -> Self {
        Val::NUMBER(0.0)
    }
}

#[derive(Clone, Debug, TS)]
pub enum ParameterTyp {
    INPUT(String),
    CHECK,
    LIST(Vec<String>),
    FILE(Vec<String>),
}

impl Default for ParameterTyp {
    fn default() -> Self {
        Self::CHECK
    }
}

impl<'de> Deserialize<'de> for ParameterTyp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Vec<&str> = Vec::deserialize(deserializer)?;

        match v.as_slice() {
            ["check"] => Ok(ParameterTyp::CHECK),
            ["input", val] => Ok(ParameterTyp::INPUT(val.to_string())),
            ["list", rest @ ..] => Ok(ParameterTyp::LIST(
                rest.iter().map(|s| s.to_string()).collect(),
            )),
            ["file", rest @ ..] => Ok(ParameterTyp::FILE(
                rest.iter().map(|s| s.to_string()).collect(),
            )),
            _ => Err(de::Error::custom("invalid ParameterTyp")),
        }
    }
}

impl Serialize for ParameterTyp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ParameterTyp::INPUT(val) => vec!["input", val].serialize(serializer),

            ParameterTyp::CHECK => vec!["check"].serialize(serializer),

            ParameterTyp::LIST(val) => vec!["list"]
                .into_iter()
                .chain(val.iter().map(|s| s.as_str()))
                .collect::<Vec<_>>()
                .serialize(serializer),

            ParameterTyp::FILE(val) => vec!["file"]
                .into_iter()
                .chain(val.iter().map(|s| s.as_str()))
                .collect::<Vec<_>>()
                .serialize(serializer),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, TS)]
pub struct IfStatementPart {
    pub from: String,
    pub oper: String,
    pub value: Val,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS)]
pub struct Dependency {
    pub program: String,
    pub platform: Option<String>,
    pub level: DependencyLevel,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
pub enum DependencyLevel {
    CRITICAL,
    CONFLICTS,
    OPTIONAL,
}
