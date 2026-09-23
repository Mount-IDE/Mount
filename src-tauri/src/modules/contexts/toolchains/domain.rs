#![allow(unused)]
use crate::modules::shared::kernel::values::{Platform, Schema};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Toolchain {
    pub id: String,
    pub version: String,
    pub copy_name: Option<String>,
    pub scheme: Schema,
    pub platform: Option<Platform>,
    pub typ: ToolchainTyp,
    pub components: HashMap<String, ToolchainComponent>,
}
impl Default for Toolchain {
    fn default() -> Self {
        Self {
            id: String::new(),
            version: String::new(),
            copy_name: None,
            scheme: Schema::default(),
            platform: None,
            typ: ToolchainTyp::LANG { languages: vec![] },
            components: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "typ")]
pub enum ToolchainTyp {
    #[serde(rename = "language")]
    LANG { languages: Vec<LanguageUnit> },
    #[serde(rename = "framework")]
    FRAMEWORK { framework: String, version: String },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageUnit {
    language: String,
    version: Option<String>,
    version_check_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum ComponentTyp {
    LSP,
    COMPILER,
    INTERPRETER,
    DEBUGGER,
    FORMATTER,
    #[allow(non_camel_case_types)]
    PACKAGE_MANAGER,
    BUILDER,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainComponent {
    pub typ: ComponentTyp,
    pub platforms: HashMap<String, ToolchainComponentIn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainComponentIn {
    pub program: String,
    pub args: Vec<String>,
    pub version: Option<String>,
    pub min_version: Option<String>,
    pub langs: ToolchainTyp,
    pub requires: Option<Vec<String>>,
    pub repair: Option<ToolchainRepair>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainRepair {
    pub discovery: Option<Vec<ToolchainDiscovery>>,
    pub install: Option<Vec<ComponentRepairRaw>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainDiscovery {
    pub platform: Option<Platform>,
    pub is_builtin: Option<bool>,
    pub is_path_var: Option<bool>,
    pub path_to_program: Option<String>,
    pub version_check_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum ComponentRepairRaw {
    #[serde(rename = "curl")]
    CURL {
        platform: Option<Platform>,
        version_check_command: Option<String>,
        url: String,
        shell: Option<String>,
    },
    #[serde(rename = "pack")]
    PACK {
        platform: Option<Platform>,
        version_check_command: Option<String>,
        pm: String,
        packages: Vec<String>,
    },
    #[serde(rename = "internal")]
    INT { url: String },
}
