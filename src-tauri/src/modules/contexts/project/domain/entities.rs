use serde::{Deserialize, Serialize};
use crate::modules::contexts::project::domain::values::{ActionCommand, ButtonPos, ParameterLabel, ProjectMeta, TemplateMeta};
use crate::modules::shared::kernel::values::{IfStatement, Path, Schema, Val};

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Widget {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenedFile {
    path: Path,
    cursor: (u32, u32),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Button{
    pos: ButtonPos,
    widget: String,
    order: u8
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Var {
    name: String,
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


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectTemplate {
    id: String,
    name: String,
    schema: Schema,
    meta: Option<TemplateMeta>,
    startup: TemplateStartup,
    packages_id: Vec<String>,

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TemplateStartup {
    sections: Vec<Section>,
    actions: Vec<Action>,
    var: Vec<Var>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Section {
    id: i32,
    label: String,
    list: Option<(bool, bool)>,
    params: Vec<Parameter>
}

#[allow(unused_variables)]
impl Section{
    pub fn new(id: i32, label: String, list: Option<(bool, bool)>) -> Section {
        Self {
            id: 0,
            label: String::new(),
            list: None,
            params: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Action {
    id: u32,
    callable: bool,
    if_: Vec<IfStatement>,
    on_error: String,
    next: Option<u32>,
    commands: ActionCommand

}
impl Action {
    pub fn new()->Action {
        Self{
            id: 0,
            callable: true,
            if_: Vec::new(),
            on_error: String::new(),
            next: None,
            commands: ActionCommand::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parameter {
    out: String,
    label: ParameterLabel,
    val: Val,
    def: Val,
    while_: String
}

impl Parameter {
    pub fn new() -> Parameter {
        Self {
            out: String::new(),
            label: ParameterLabel::STR(String::new()),
            val: Val::NUMBER(0.0),
            def: Val::NUMBER(0.0),
            while_: String::new()
        }
    }
}
