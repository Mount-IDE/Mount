use super::default::action::*;
use super::default::section::*;
use super::default::template::*;
use super::default::workspace::*;
use crate::modules::contexts::launch::domain::entities::{
    LaunchObject, LaunchTemplate, LaunchTemplateReference,
};
use crate::modules::contexts::project::domain::values::{
    ActionCommand, ActionOnError, ButtonPos, PackageMeta, ParameterLabel, ProjectMeta, TemplateMeta,
};
use crate::modules::shared::kernel::errors::ProjectError;
use crate::modules::shared::kernel::values::{
    Dependency, IfStatementPart, ParameterTyp, Path, Schema, Val,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct Project {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: Path,
    #[serde(default)]
    pub template: ProjectTemplate,
    #[serde(default)]
    pub meta: ProjectMeta,
    #[serde(default)]
    pub schema: Schema,
    #[serde(default)]
    pub workspace: WorkSpace,
    #[serde(default)]
    pub vars: Vec<Var>,
    #[serde(default)]
    pub tasks: Vec<Task>,
    #[serde(default)]
    pub packages: Vec<String>,
}

impl Project {
    ///
    ///
    ///
    pub fn new() -> Project {
        Self {
            name: String::new(),
            path: Path(String::new()),
            template: ProjectTemplate::new(),
            meta: ProjectMeta::new(),
            schema: Schema(1),
            workspace: WorkSpace::new(),
            vars: Vec::new(),
            tasks: Vec::new(),
            packages: Vec::new(),
        }
    }
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct WorkSpace {
    pub widgets: Vec<Widget>,
    pub buttons: Vec<Button>,
    pub opened_files: Vec<OpenedFile>,
    pub launch_references: Vec<LaunchTemplateReference>,
    pub launch_objects: Vec<LaunchObject>,
    pub launch_templates: Vec<LaunchTemplate>,
}

impl Default for WorkSpace {
    ///
    ///
    ///
    fn default() -> Self {
        Self {
            widgets: Vec::new(),
            buttons: buttons(),
            opened_files: Vec::new(),
            launch_references: Vec::new(),
            launch_objects: Vec::new(),
            launch_templates: Vec::new(),
        }
    }
}
impl WorkSpace {
    ///
    ///
    ///
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            buttons: Vec::new(),
            opened_files: Vec::new(),
            launch_objects: Vec::new(),
            launch_references: Vec::new(),
            launch_templates: Vec::new(),
        }
    }
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct Widget {}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct OpenedFile {
    #[serde(default)]
    name: String,
    path: Path,
    cursor: (u32, u32),
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct Button {
    pub pos: ButtonPos,
    pub widget: String,
    #[serde(default)]
    pub component_type: ButtonComponentType,
    pub order: u8,
    pub alt: String,
    pub keys: String,
    pub icon: String,
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, Default, TS)]
pub enum ButtonComponentType {
    #[default]
    Light,
    Heavy,
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct Task {
    id: i32,
    // action: TaskAction
}

#[derive(Clone, Debug, TS)]
pub enum _Task {
    GRAPH {
        next: Box<_Task>,
        commands: Vec<TaskCommand>,
        on_error: ActionOnError,
    },
    SINGLE {
        commands: Vec<TaskCommand>,
        on_error: ActionOnError,
    },
}

#[derive(Debug, Clone, TS)]
pub struct TaskCommand {
    pub command: String,
    pub shell: String,
    pub env: Option<Vec<(String, String)>>,
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct Var {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: Val,
}
impl Var {
    pub fn new(name: String, value: Val) -> Var {
        Self { name, value }
    }
    pub fn from(_json: String) {
        todo!()
    }
    pub fn to_json(&self) -> String {
        todo!()
    }
}

///
///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct ProjectTemplate {
    #[serde(default = "t_id")]
    pub id: String,
    #[serde(default = "t_name")]
    pub name: String,
    #[serde(default = "t_schema")]
    pub schema: Schema,
    pub meta: Option<TemplateMeta>,
    #[serde(default = "t_startup")]
    pub startup: TemplateStartup,
    #[serde(default)]
    pub packages_id: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    #[serde(default)]
    pub launches: Vec<LaunchTemplate>,
}

impl Default for ProjectTemplate {
    fn default() -> Self {
        Self {
            id: "opie.empty".to_string(),
            name: "Empty Project".to_string(),
            schema: Schema(1),
            meta: Some(TemplateMeta::default()),
            startup: TemplateStartup::new(),
            packages_id: vec![],
            dependencies: Vec::new(),
            launches: vec![LaunchTemplate::default()],
        }
    }
}
impl ProjectTemplate {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            schema: Schema(1),
            meta: None,
            startup: TemplateStartup::new(),
            packages_id: Vec::new(),
            dependencies: Vec::new(),
            launches: Vec::new(),
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
    #[serde(default = "t_list")]
    pub list: (bool, bool),
    #[serde(default)]
    pub params: Vec<Parameter>,
}

#[allow(unused_variables)]
impl Section {
    pub fn new(id: i32, label: String, list: Option<(bool, bool)>) -> Section {
        Self {
            id: 0,
            label: String::new(),
            list: (false, false),
            params: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct Action {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub if_: Option<Vec<Vec<IfStatementPart>>>,
    #[serde(default = "t_error")]
    pub on_error: String,
    #[serde(default)]
    pub next: Option<i32>,
    #[serde(default)]
    pub command: Vec<ActionCommand>,
}
impl Action {
    pub fn new() -> Action {
        Self {
            id: 0,
            // for_: None,
            // callable: None,
            if_: None,
            on_error: String::new(),
            next: None,
            command: Vec::new(),
        }
    }

    pub fn getaddr(addr: String) -> Result<(i8, String), ProjectError> {
        let point = addr.find(".");
        if let None = point {
            return Err(ProjectError::IncorrectAddress { address: addr });
        }
        let point = point.unwrap();
        if point == addr.len() {
            return Err(ProjectError::IncorrectAddress { address: addr });
        }
        let section = addr[..point].to_string().clone();
        let parameter = addr[point..].to_string().clone();
        let section = section
            .parse::<i8>()
            .map_err(|_| ProjectError::IncorrectAddress { address: addr })?;
        Ok((section, parameter))
    }
    // pub fn get_address(&self) -> Result<(i8, String), ProjectError> {
    //     if let None = self.for_ {
    //         return Err(ProjectError::IncorrectAddress {
    //             address: "".to_string(),
    //         });
    //     }
    //     Self::getaddr(self.for_.clone().unwrap())
    // }
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
    #[serde(default)]
    pub while_: Option<String>,
}

impl Parameter {
    pub fn new() -> Parameter {
        Self {
            out: String::new(),
            label: ParameterLabel::STR(String::new()),
            def: Val::NUMBER(0.0),
            while_: None,
            typ: ParameterTyp::CHECK,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct ProjectPackage {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    meta: PackageMeta,
    #[serde(default)]
    startup: PackageStartup,
}

impl ProjectPackage {
    pub fn new() -> ProjectPackage {
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
    parameters: Vec<Parameter>,
}

impl Default for PackageStartup {
    fn default() -> Self {
        Self {
            var: Vec::new(),
            actions: Vec::new(),
            parameters: Vec::new(),
        }
    }
}

impl PackageStartup {
    pub fn new() -> PackageStartup {
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

impl Default for ProjectTag {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
        }
    }
}
