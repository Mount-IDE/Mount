use crate::modules::contexts::project::domain::entities::{
    ActionTask, PackageAction, PackageActionCommand, PackageOption, PackageOptionTyp,
    PackageOptionTypEnum, Section,
};
use crate::modules::contexts::project::domain::values::ActionOnError;
use crate::modules::shared::kernel::values::{IfStatementPart, Schema, Val};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
                    params: vec![PackageOption {
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
                /*LaunchAction {
                    id: 0,
                    next: None,
                    if_: vec![],
                    command: Some(LaunchActionCommand {
                        command: "".to_string(),
                        args: Some(vec!["#0.command".to_string()]),
                        cwd: None,
                        env: None,
                    }),
                }*/
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
                        command: "#command".to_string(),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchSection {
    #[serde(default)]
    pub id: i8,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub options: Vec<LaunchOption>,
}
// -1 - meta (name etc)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchOption {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub typ: LaunchOptionType,
    #[serde(default)]
    pub def: LaunchOptionDefault,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LaunchOptionDefault {
    SINGLE(String),
    Function(i8),
}

impl Default for LaunchOptionDefault {
    fn default() -> Self {
        Self::SINGLE(Default::default())
    }
}

pub type LaunchTemplateResult = HashMap<i8, HashMap<String, Val>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchOptionType {
    #[serde(default)]
    pub typ: LaunchOptionTyp,
    pub list_types: Option<Vec<String>>,
    pub path_type: Option<String>,
    #[serde(default)]
    pub restriction: Option<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaunchOptionTyp {
    #[allow(non_camel_case_types)]
    input,
    #[allow(non_camel_case_types)]
    check,
    #[allow(non_camel_case_types)]
    path,
    #[allow(non_camel_case_types)]
    list,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchAction {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub next: Option<i32>,
    #[serde(default)]
    pub if_: Vec<Vec<IfStatementPart>>,
    #[serde(default)]
    pub command: Option<LaunchActionCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchActionCommand {
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub args: Option<Vec<String>>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub env: Option<Vec<(String, String)>>,
}

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

impl Default for LaunchSection {
    fn default() -> Self {
        Self {
            id: 0,
            title: None,
            options: vec![],
        }
    }
}

impl Default for LaunchOptionTyp {
    fn default() -> Self {
        Self::input
    }
}

impl Default for LaunchOption {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            title: "".to_string(),
            typ: Default::default(),
            def: Default::default(),
        }
    }
}

impl Default for LaunchOptionType {
    fn default() -> Self {
        Self {
            typ: Default::default(),
            restriction: None,
            list_types: None,
            path_type: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchObject {
    pub id: i32,
    pub launch_reference: i8,
    pub scheme: Schema,
    pub tasks: Vec<ActionTask>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaunchTask {
    SINGLE {
        command: String,
        env: Option<Vec<(String, String)>>,
        cwd: Option<String>,
    },
    GRAPH {
        command: String,
        next: Box<LaunchTask>,
        env: Option<Vec<(String, String)>>,
        cwd: Option<String>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaunchFlatTask {
    pub command: String,
    pub env: Option<Vec<(String, String)>>,
    pub cwd: Option<String>,
}
