use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::modules::contexts::project::domain::values::{ActionCommand, ButtonPos, PackageMeta, ParameterLabel, ProjectMeta, TemplateMeta};
use crate::modules::shared::kernel::values::{IfStatement, ParameterTyp, Path, Schema, Val};
use super::default::template::*;
use super::default::section::*;
use super::default::action::*;

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct Project {
    pub name: String,
    pub(crate) path: Path,
    pub template: ProjectTemplate,
    pub meta: ProjectMeta,
    pub schema: Schema,
    pub workspace: WorkSpace,
    pub vars: Vec<Var>,
    pub tasks: Vec<Task>,
    pub packages: Vec<String>,
}


impl Project {
    pub fn new()->Project {
        Self {
            name: String::new(),
            path: Path(String::new()),
            template: ProjectTemplate::new(),
            meta: ProjectMeta::new(),
            schema: Schema(1),
            workspace: WorkSpace::new(),
            vars: Vec::new(),
            tasks: Vec::new(),
            packages: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct WorkSpace {
    widgets: Vec<Widget>,
    buttons: Vec<Button>,
    opened_files: Vec<OpenedFile>
}

impl WorkSpace{
    pub fn new() -> Self {
        Self{
            widgets: Vec::new(),
            buttons: Vec::new(),
            opened_files: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct Widget {}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct OpenedFile {
    path: Path,
    cursor: (u32, u32),
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct Button{
    pos: ButtonPos,
    widget: String,
    order: u8
}


#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct Task {}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct Var {
    #[serde(default)]
    name: String,
    #[serde(default)]
    value: Val
}
impl Var {
    pub fn new(name: String, value: Val) -> Var {
        Self {
            name,
            value,
        }
    }
    pub fn from(_json: String) {
        todo!()
    }
    pub fn to_json(&self) -> String{
        todo!()
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct ProjectTemplate {
    #[serde(default="t_id")]
    pub id: String,
    #[serde(default="t_name")]
    pub name: String,
    #[serde(default="t_schema")]
    pub schema: Schema,
    pub meta: Option<TemplateMeta>,
    #[serde(default="t_startup")]
    pub startup: TemplateStartup,
    #[serde(default)]
    pub packages_id: Vec<String>,

}

impl ProjectTemplate {
    pub fn new()->Self{
        Self{
            id: String::new(),
            name: String::new(),
            schema: Schema(1),
            meta: None,
            startup: TemplateStartup::new(),
            packages_id: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct TemplateStartup {
    #[serde(default)]
    pub sections: Vec<Section>,
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default)]
    pub var: Vec<Var>,
}

impl TemplateStartup {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            actions: Vec::new(),
            var: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct Section {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub label: String,
    #[serde(default="t_list")]
    pub list: (bool, bool),
    #[serde(default)]
    pub params: Vec<Parameter>
}

#[allow(unused_variables)]
impl Section{
    pub fn new(id: i32, label: String, list: Option<(bool, bool)>) -> Section {
        Self {
            id: 0,
            label: String::new(),
            list: (false, false),
            params: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct Action {
    #[serde(default)]
    pub id: u32,
    pub for_: Option<String>,
    pub callable: Option<bool>,
    #[serde(default)]
    pub if_: Vec<IfStatement>,
    #[serde(default="t_error")]
    pub on_error: String,
    pub next: Option<u32>,
    #[serde(default)]
    pub command: Vec<ActionCommand>

}
impl Action {
    pub fn new()->Action {
        Self{
            id: 0,
            for_: None,
            callable: None,
            if_: Vec::new(),
            on_error: String::new(),
            next: None,
            command: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct Parameter {
    #[serde(default)]
    pub out: String,
    #[serde(default)]
    pub label: ParameterLabel,
    #[serde(default)]
    pub typ: ParameterTyp,
    #[serde(default)]
    pub def: Val,
    pub while_: Option<String>
}

impl Parameter {
    pub fn new() -> Parameter {
        Self {
            out: String::new(),
            label: ParameterLabel::STR(String::new()),
            def: Val::NUMBER(0.0),
            while_: None,
            typ: ParameterTyp::CHECK
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct ProjectPackage{
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    meta: PackageMeta,
    #[serde(default)]
    startup: PackageStartup
}


impl ProjectPackage{
    pub fn new()->ProjectPackage {
        Self {
            id: String::new(),
            name: String::new(),
            meta: PackageMeta::new(),
            startup: PackageStartup::new(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct PackageStartup {
    #[serde(default)]
    var: Vec<Var>,
    #[serde(default)]
    actions: Vec<Action>,
    #[serde(default)]
    parameters: Vec<Parameter>
}

impl Default for PackageStartup{
    fn default() -> Self {
        Self {
            var: Vec::new(),
            actions: Vec::new(),
            parameters: Vec::new()
        }
    }
}

impl PackageStartup {
    pub fn new()-> PackageStartup {
        Self {
            var: Vec::new(),
            parameters: Vec::new(),
            actions: Vec::new(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct ProjectTag {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub name: String,
}

impl Default for ProjectTag{
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
        }
    }
}
