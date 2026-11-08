pub mod decl;
pub mod platform;

use crate::domain::platform::{Platform, PlatformType};
use crate::traits::PathPart;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Var {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: Val,
    #[serde(default)]
    pub readonly: Option<bool>,
}

impl Var {
    pub fn new(name: String, value: Val) -> Var {
        Self {
            name,
            value,
            readonly: None,
        }
    }

    pub fn readonly(name: String, value: Val) -> Var {
        Self {
            name,
            value,
            readonly: Some(true),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Path(pub String);

impl Path {
    pub fn get(&self) -> String {
        self.0.clone()
    }

    pub fn get_ref(&self) -> &String {
        &self.0
    }

    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn empty() -> Self {
        Self(String::new())
    }

    pub fn split(&self) -> Vec<String> {
        #[cfg(windows)]
        return self
            .0
            .clone()
            .split("\\")
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
        #[cfg(not(windows))]
        return self
            .0
            .clone()
            .split("/")
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Schema(pub u8);

impl Default for Schema {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IfStatementPart {
    pub from: Option<String>,
    pub oper: IfStatementOperation,
    pub value: Option<Val>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

    #[serde(rename = "starts")]
    Starts,
    #[serde(rename = "!starts")]
    NonStarts,
    #[serde(rename = "ends")]
    Ends,
    #[serde(rename = "!ends")]
    NonEnds,
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
            IfStatementOperation::Empty => |a, _| {
                if let Val::STRING(v) = a {
                    return v.is_empty();
                }
                false
            },
            IfStatementOperation::NonEmpty => |a, _| {
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
            IfStatementOperation::NonAlready => |_, _| true,
            IfStatementOperation::Stopped => |_, _| true,
            IfStatementOperation::NonStopped => |_, _| true,
            IfStatementOperation::Installed => |_, _| true,
            IfStatementOperation::NonInstalled => |_, _| true,
            IfStatementOperation::Starts => |a, b| a.to_str().starts_with(b.to_str().as_str()),
            IfStatementOperation::NonStarts => |a, b| !a.to_str().starts_with(b.to_str().as_str()),
            IfStatementOperation::Ends => |a, b| a.to_str().ends_with(b.to_str().as_str()),
            IfStatementOperation::NonEnds => |a, b| !a.to_str().ends_with(b.to_str().as_str()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Dependency {
    pub program: String,
    pub platform: Option<Platform>,
    pub level: DependencyLevel,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum DependencyLevel {
    CRITICAL,
    CONFLICTS,
    OPTIONAL,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shell(pub String);

impl Default for Shell {
    fn default() -> Self {
        let plat = PlatformType::current();
        match plat {
            PlatformType::WINDOWS => Self("cmd".__get()),
            PlatformType::MACOS => Self("zsh".__get()),
            _ => Self("sh".__get()),
        }
    }
}
impl Shell {
    pub fn get_key(&self) -> Option<String> {
        match self.0.as_str() {
            "bash" => Some("-c".to_string()),
            "sh" => Some("-c".to_string()),
            "dash" => Some("-c".to_string()),
            "ksh" => Some("-c".to_string()),
            "zsh" => Some("-c".to_string()),
            "fish" => Some("-c".to_string()),
            "cmd" => Some("/C".to_string()),
            "powershell" => Some("-Command".to_string()),
            _ => None,
        }
    }
}
