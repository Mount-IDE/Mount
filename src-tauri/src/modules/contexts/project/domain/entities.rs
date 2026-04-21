use crate::modules::contexts::project::domain::values::{ActionCommand, ButtonPos, ParameterLabel, ProjectMeta, TemplateMeta};
use crate::modules::shared::kernel::entities::Package;
use crate::modules::shared::kernel::values::{IfStatement, Path, Schema, Val};

pub struct Project {
    name: String,
    path: Path,
    template: ProjectTemplate,
    meta: ProjectMeta,
    schema: Schema,
    workspace: WorkSpace,
    vars: Vec<Var>,
    tasks: Vec<Task>,
    packages: Vec<Package>,
}



pub struct WorkSpace {
    widgets: Vec<Widget>,
    buttons: Vec<Button>,
    opened_files: Vec<OpenedFile>
}
pub struct Widget {}
pub struct OpenedFile {
    path: Path,
    cursor: (u32, u32),
}
pub struct Button{
    pos: ButtonPos,
    widget: String,
    order: u8
}



pub struct Task {}


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
    pub fn from(json: String) {
        todo!()
    }
    pub fn to_json(&self) -> String{
        todo!()
    }
}



pub struct ProjectTemplate {
    id: String,
    name: String,
    schema: Schema,
    meta: Option<TemplateMeta>,
    startup: TemplateStartup,
    packages_id: Vec<String>,

}

pub struct TemplateStartup {
    sections: Vec<Section>,
    actions: Vec<Action>,
    var: Vec<Var>,
}

pub struct Section {
    id: i32,
    label: String,
    list: Option<(bool, bool)>

}


pub struct Action {
    id: u32,
    callable: bool,
    if_: Vec<IfStatement>,
    on_error: String,
    next: u32,
    commands: ActionCommand

}



pub struct Parameter {
    out: String,
    label: ParameterLabel,
    val: Val,
    def: Val,
    while_: String
}
