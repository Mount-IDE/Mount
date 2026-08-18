use crate::modules::contexts::filesystem::app::utils::{make_path, PathPart};
use crate::modules::contexts::filesystem::domain::values::FsPath;
use regex::Regex;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename = "lowercase")]
pub enum PlatformType {
    WINDOWS,
    MACOS,
    LINUX,
    ALL,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(untagged)]
pub enum Platform {
    ARRAY(Vec<PlatformType>),
    SINGLE(PlatformType),
}

impl Platform {
    pub fn unix_like() -> Self {
        Self::ARRAY(vec![PlatformType::LINUX, PlatformType::MACOS])
    }
    pub fn all() -> Self {
        Self::SINGLE(PlatformType::ALL)
    }

    pub fn arr(arr: Vec<PlatformType>) -> Self {
        Self::ARRAY(arr)
    }

    pub fn windows() -> Self {
        Self::SINGLE(PlatformType::WINDOWS)
    }

    pub fn macos() -> Self {
        Self::SINGLE(PlatformType::MACOS)
    }

    pub fn linux() -> Self {
        Self::SINGLE(PlatformType::LINUX)
    }

    pub fn is_correct(&self) -> bool {
        match self {
            Platform::ARRAY(v) => {
                for i in v {
                    if i.is_correct() {
                        return true;
                    }
                }
                false
            }
            Platform::SINGLE(v) => v.is_correct(),
        }
    }
}

impl PlatformType {
    pub fn is_correct(&self) -> bool {
        if *self == Self::LINUX && cfg!(target_os = "linux") {
            return true;
        }
        if *self == Self::MACOS && cfg!(target_os = "macos") {
            return true;
        }
        if *self == Self::WINDOWS && cfg!(target_os = "windows") {
            return true;
        }
        if *self == Self::ALL {
            return true;
        }
        false
    }
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
    NONE,
}

impl Val {
    pub fn to_str(&self) -> String {
        match &self {
            Val::STRING(val) => val.clone(),
            Val::NUMBER(val) => val.to_string(),
            Val::BOOL(val) => val.to_string(),
            Val::ARRAY(val) => val.join("::"),
            Val::NONE => "none".__get(),
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
    pub from: Option<String>,
    pub oper: IfStatementOperation,
    pub value: Option<Val>,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[serde(rename_all = "lowercase")]
pub enum IfStatementOperation {
    #[serde(rename = "==")]
    EQ,
    #[serde(rename = "!=")]
    NE,
    #[serde(rename = ">")]
    GT,
    #[serde(rename = "<")]
    LT,
    #[serde(rename = "<=")]
    LE,
    #[serde(rename = ">=")]
    GE,
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "!empty")]
    NonEmpty,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "!in")]
    NonIn,
    #[serde(rename = "regex")]
    Regex,
    #[serde(rename = "!regex")]
    NonRegex,
    #[serde(rename = "!already")]
    NonAlready,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "!stopped")]
    NonStopped,
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "!installed")]
    NonInstalled,
}

impl IfStatementOperation {
    pub fn get_fn(&self) -> fn(a: Val, b: Val) -> bool {
        match self {
            IfStatementOperation::EQ => |a, b| a == b,
            IfStatementOperation::NE => |a, b| a != b,
            IfStatementOperation::GT => |a, b| a > b,
            IfStatementOperation::LT => |a, b| a < b,
            IfStatementOperation::LE => |a, b| a >= b,
            IfStatementOperation::GE => |a, b| a <= b,
            IfStatementOperation::Empty => |a, b| {
                if let Val::STRING(v) = a {
                    return v.is_empty();
                }
                false
            },
            IfStatementOperation::NonEmpty => |a, b| {
                if let Val::STRING(v) = a {
                    return !v.is_empty();
                }
                false
            },
            IfStatementOperation::In => |a, b| {
                if let Val::ARRAY(v) = b {
                    if let Val::STRING(a) = a {
                        return v.contains(&a);
                    }
                }
                false
            },
            IfStatementOperation::NonIn => |a, b| {
                if let Val::ARRAY(v) = b {
                    if let Val::STRING(a) = a {
                        return !v.contains(&a);
                    }
                }
                false
            },
            IfStatementOperation::Regex => |a, b| {
                if let Val::STRING(a) = a {
                    if let Val::STRING(b) = b {
                        let Ok(res) = Regex::new(b.as_str()) else {
                            return false;
                        };
                        return res.is_match(&a);
                    }
                }
                false
            },
            IfStatementOperation::NonRegex => |a, b| {
                if let Val::STRING(a) = a {
                    if let Val::STRING(b) = b {
                        let Ok(res) = Regex::new(b.as_str()) else {
                            return false;
                        };
                        return res.is_match(&a);
                    }
                }
                false
            },
            IfStatementOperation::NonAlready => |a, b| true,
            IfStatementOperation::Stopped => |a, b| true,
            IfStatementOperation::NonStopped => |a, b| true,
            IfStatementOperation::Installed => |a, b| true,
            IfStatementOperation::NonInstalled => |a, b| true,
        }
    }
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
