use std::collections::HashMap;
use super::default::action::*;
use super::default::section::*;
use super::default::template::*;
use super::default::workspace::*;
use crate::modules::contexts::launch::domain::entities::{
    LaunchObject, LaunchTemplate, LaunchTemplateReference,
};
use crate::modules::contexts::project::domain::values::{
    ActionCommand, ActionOnError, ButtonPos, ParameterLabel, ProjectMeta, TemplateMeta,
};
use crate::modules::shared::kernel::errors::ProjectError;
use crate::modules::shared::kernel::values::{Dependency, IfStatementPart, ParameterTyp, Path, Platform, PlatformType, Schema, Val};
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
    pub current_launch: Option<i32>,
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
            current_launch: None,
        }
    }
}
impl WorkSpace {
    ///
    ///
    ///
    pub fn new() -> Self {
        Self::default()
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

impl Section {
    #[allow(unused)]
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
    #[allow(unused)]
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
    #[allow(unused)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    id: String,
    name: String,
    meta: Option<PackageMeta>,
    scheme: Schema,
    dependencies: Vec<PackageDependency>,
    startup: PackageStartup,
    components: Option<Vec<PackageComponent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageComponent {
    id: String,
    typ: PackageComponentTyp,
    program: String,
    platform: Platform,
    languages: Vec<String>,
    priority: Option<u8>,
    arguments: Option<Vec<String>>,
    builtin_params: PackageComponentBuiltin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageComponentTyp {
    COMPILER,
    TRANSPILER,
    INTERPRETER,
    LSP,
    FORMATTER,
    DEBUG,
    BUILD_SYSTEM,
    PACK_MANAGER,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageComponentBuiltin {
    is_builtin: bool,
    url: Option<String>,
    path: Option<String>,
    in_path: Option<bool>,
    min_version: Option<String>,
    version_check_command: Option<VersionCheckCommand>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VersionCheckCommand {
    SINGLE(String),
    OBJ {
        platform: Platform,
        command: String,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageStartup {
    options: Option<Vec<PackageOption>>,
    actions: Option<Vec<PackageAction>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageAction {
    id: i8,
    next: Option<Vec<i8>>,
    if_: Option<Vec<Vec<IfStatementPart>>>,
    on_error: String,
    platform: Option<Platform>,
    command: Option<PackageActionCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageActionCommand {
    platform: Option<Platform>,
    cwd: Option<String>,
    env: Option<HashMap<String, String>>,
    needed_exit_code: Option<Vec<i64>>,
    command: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageOption {
    id: String,
    title: String,
    typ: PackageOptionTyp,
    def: Option<Val>,
    while_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageOptionTyp {
    typ: PackageOptionTypEnum,
    fs_type: Option<PackageOptionFsType>,
    list_type: Option<Vec<String>>,
    placeholder: Option<String>,
    required: Option<bool>,
    readonly: Option<bool>,
    validate: Option<String>,
    fs_filter: Option<Vec<String>>,
    gen_min: Option<u8>,
    gen_max: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageOptionFsType {
    FILE,
    DIR,
    FS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageOptionTypEnum {
    INPUT,
    AREA,
    LIST,
    GEN,
    FILE,
    CHECK,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDependency {
    typ: PackageDependencyTyp,
    name: String,
    version: String,
    platform: Option<Platform>,
    version_check_command: Option<String>,
    level: PackageDependencyLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageDependencyTyp {
    PACKAGE,
    PROGRAM,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageDependencyLevel {
    REQUIRED,
    CONFLICTS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMeta {
    version: Option<String>,
    authors: Option<Vec<String>>,
    description: Option<String>,
    tags: Option<String>,
    source: Option<String>,
    typ: Option<PackageTyp>,
    icon: Option<String>,
    license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageTyp {
    LANGUAGE,
    FRAMEWORK,
    TOOL,
    BUILD_SYSTEM,
}


impl Package {
    pub(crate) fn python() -> Self {
        Self {
            id: "opie.python".to_string(),

            name: "Python 3.12".to_string(),
            meta: None,
            scheme: Default::default(),
            dependencies: vec![
                PackageDependency {
                    typ: PackageDependencyTyp::PROGRAM,
                    name: "python".to_string(),
                    version: "3.12".to_string(),
                    platform: None,
                    version_check_command: Some("python -v".into()),
                    level: PackageDependencyLevel::REQUIRED,
                }
            ],
            startup: PackageStartup {
                options: Some(vec![
                    PackageOption {
                        id: "main-py".to_string(),
                        title: "Add main.py".to_string(),
                        typ: PackageOptionTyp {
                            typ: PackageOptionTypEnum::CHECK,
                            fs_type: None,
                            list_type: None,
                            placeholder: None,
                            required: None,
                            readonly: None,
                            validate: None,
                            fs_filter: None,
                            gen_min: None,
                            gen_max: None,
                        },
                        def: Some(Val::BOOL(false)),
                        while_: None,
                    }
                ]),
                actions: None,
            },
            components: Some(
                vec![
                    PackageComponent {
                        id: "python".to_string(),
                        typ: PackageComponentTyp::INTERPRETER,
                        program: "python".to_string(),
                        platform: Platform::SINGLE(PlatformType::ALL),
                        languages: vec![".py".to_string()],
                        priority: None,
                        arguments: None,
                        builtin_params: PackageComponentBuiltin {
                            is_builtin: true,
                            url: None,
                            path: None,
                            in_path: Some(true),
                            min_version: None,
                            version_check_command: None,
                        },
                    }
                ]
            ),
        }
    }
}