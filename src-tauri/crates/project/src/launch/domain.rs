use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use declarative::domain::{ActionOnError, ActionTask, PackageAction, PackageActionCommand, PackageActionCommandType, PackageOptionTyp, PackageOptionTypEnum, Parameter};
use mount_core::domain::{Schema, Val};
use crate::project::domain::Section;
//use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchTemplate {
    #[serde(default)]
    pub id: i8,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub scheme: Schema,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub sections: Vec<Section>,
    #[serde(default)]
    pub actions: Vec<PackageAction>,
    #[serde(default)]
    pub functions: Vec<LaunchFunction>,
}

impl Default for LaunchTemplate {
    fn default() -> Self {
        Self {
            id: -1,
            title: String::from("Basic Configuration"),
            scheme: Default::default(),
            icon: None,
            sections: vec![
                /*LaunchSection {
                    id: 0,
                    title: None,
                    options: vec![LaunchOption {
                        id: "command".to_string(),
                        title: "Command".to_string(),
                        typ: Default::default(),
                        def: Default::default(),
                    }],
                }*/
                Section {
                    id: 0,
                    label: "".to_string(),
                    list: (false, false),
                    params: vec![Parameter {
                        id: "command".to_string(),
                        title: "Command".to_string(),
                        typ: PackageOptionTyp {
                            typ: PackageOptionTypEnum::INPUT,
                            fs_type: None,
                            list_type: None,
                            placeholder: None,
                            required: Some(true),
                            readonly: None,
                            validate: None,
                            fs_filter: None,
                            gen_min: None,
                            gen_max: None,
                        },
                        def: None,
                        while_: None,
                    }],
                },
            ],
            actions: vec![
                PackageAction {
                    id: 0,
                    next: None,
                    if_: None,
                    on_error: ActionOnError::StopAll,
                    platform: None,
                    command: Some(vec![PackageActionCommand {
                        platform: None,
                        cwd: None,
                        shell: None,
                        env: None,
                        needed_exit_code: None,
                        command: PackageActionCommandType::Single("#0.command".to_string()),
                    }]),
                },
            ],
            functions: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchTemplateReference {
    #[serde(default)]
    pub id: i8,
    #[serde(default)]
    pub template: (String, i8),
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub scheme: Schema,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub results: LaunchTemplateResult,
    #[serde(default)]
    pub before: Option<Vec<LaunchReferenceBefore>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "typ", rename_all = "lowercase")]
pub enum LaunchReferenceBefore {
    CUSTOM {
        #[serde(default)]
        scheme: Schema,
        #[serde(default)]
        template: (String, i32),
        #[serde(default)]
        icon: Option<String>,
        #[serde(default)]
        name: String,
        #[serde(default)]
        results: LaunchTemplateResult,
    },
    OTHER {
        #[serde(default)]
        id: i8,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchFunction {
    #[serde(default)]
    pub id: i8,
    #[serde(default)]
    pub actions: Vec<LaunchFunctionAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchFunctionAction {
    pub function: String,
    pub args: Option<Vec<LaunchFunctionArgument>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LaunchFunctionArgument {
    STRING(String),
    VEC(Vec<LaunchVecType>),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LaunchVecType {
    STRING(String),
    NUMBER(i32),
}


pub type LaunchTemplateResult = HashMap<i8, HashMap<String, Val>>;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(unused)]
pub enum LaunchFunctionOutType {
    STRING(String),
    VEC(Vec<String>),
}

impl Default for LaunchFunctionOutType {
    fn default() -> Self {
        Self::STRING("".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchObject {
    pub id: i32,
    pub launch_reference: i8,
    pub scheme: Schema,
    pub tasks: Vec<ActionTask>,
    pub before: Option<Vec<i32>>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaunchFlatTask {
    pub command: Vec<String>,
    pub env: Option<HashMap<String, String>>,
    pub cwd: Option<String>,
}
