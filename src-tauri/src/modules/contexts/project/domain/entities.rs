use super::default::action::*;
use super::default::section::*;
use super::default::template::*;
use super::default::workspace::*;
use crate::modules::app::CONFIG_SERVICE;
use crate::modules::contexts::filesystem::app::utils::{path_from, PathPart};
use crate::modules::contexts::launch::domain::entities::{
    LaunchObject, LaunchTemplate, LaunchTemplateReference,
};
use crate::modules::contexts::project::domain::values::{
    ActionCommand, ActionOnError, ButtonPos, ParameterLabel, ProjectMeta, TemplateMeta,
};
use crate::modules::services::traits::TConfigService;
use crate::modules::shared::kernel::errors::ProjectError;
use crate::modules::shared::kernel::values::{
    Dependency, DependencyLevel, IfStatementOperation, IfStatementPart, ParameterTyp, Path,
    Platform, PlatformType, Schema, Val,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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
    pub env: Option<HashMap<String, String>>,
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
    pub actions: Vec<PackageAction>,
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
    pub params: Vec<PackageOption>,
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
    #[serde(default)]
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    meta: Option<PackageMeta>,
    #[serde(default)]
    scheme: Schema,
    #[serde(default)]
    dependencies: Vec<PackageDependency>,
    #[serde(default)]
    pub(crate) startup: PackageStartup,
    #[serde(default)]
    pub(crate) var: Option<Vec<Var>>,
    #[serde(default)]
    pub files: PackageFiles,
    #[serde(default)]
    pub highlight: Vec<PackageParser>,
    #[serde(default)]
    components: Option<Vec<PackageComponent>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageParser {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub lang: String,
    #[serde(default)]
    pub extentions: Vec<String>,
    #[serde(default)]
    pub ignore_files: Option<Vec<String>>,
    #[serde(default)]
    pub files: Option<Vec<String>>,
    #[serde(default)]
    pub nodes: HashMap<String, String>,
    #[serde(default)]
    pub syntax: Option<Syntax>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Syntax {
    #[serde(default)]
    base_color: Option<String>,
    #[serde(default)]
    colors: Option<HashMap<String, String>>,
    #[serde(default)]
    tokens: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageFiles {
    #[serde(default)]
    pub extentions: Vec<String>,
    #[serde(default)]
    pub ignore_files: Option<Vec<String>>,
    #[serde(default)]
    pub files: Option<Vec<String>>,
}
impl Default for PackageFiles {
    fn default() -> Self {
        Self {
            extentions: vec![],
            files: None,
            ignore_files: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageComponent {
    pub id: String,
    pub typ: PackageComponentTyp,
    pub program: String,
    pub platform: Platform,
    pub languages: Vec<String>,
    pub priority: Option<u8>,
    pub arguments: Option<Vec<String>>,
    pub builtin_params: PackageComponentBuiltin,
}

impl PackageComponent {
    pub fn get_command(&self, package: String) -> String {
        if let Some(true) = self.builtin_params.in_path {
            if self.builtin_params.is_builtin {
                return self.program.clone();
            }
        } else {
            if let Some(p) = self.builtin_params.path.clone() {
                let dir = CONFIG_SERVICE.get_data_dir().unwrap_or(Path::new(""));
                let path = path_from![dir, "packages", package, p, self.program];
                return path.get();
            }
        }
        String::new()
    }
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
    pub is_builtin: bool,
    pub url: Option<String>,
    pub path: Option<String>,
    pub in_path: Option<bool>,
    pub min_version: Option<String>,
    pub version_check_command: Option<VersionCheckCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VersionCheckCommand {
    SINGLE(String),
    OBJ { platform: Platform, command: String },
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PackageStartup {
    options: Option<Vec<PackageOption>>,
    pub(crate) actions: Option<Vec<PackageAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct PackageAction {
    pub id: i8,
    pub next: Option<Vec<i8>>,
    pub(crate) if_: Option<Vec<Vec<IfStatementPart>>>,
    pub on_error: ActionOnError,
    pub platform: Option<Platform>,
    pub command: Option<Vec<PackageActionCommand>>,
}

impl Default for PackageAction {
    fn default() -> Self {
        Self {
            id: 0,
            next: None,
            if_: None,
            on_error: Default::default(),
            platform: None,
            command: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct PackageActionCommand {
    pub(crate) platform: Option<Platform>,
    pub(crate) cwd: Option<String>,
    pub(crate) shell: Option<String>,
    pub(crate) env: Option<HashMap<String, String>>,
    pub(crate) needed_exit_code: Option<Vec<i64>>,
    pub(crate) command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct PackageOption {
    id: String,
    title: String,
    typ: PackageOptionTyp,
    def: Option<Val>,
    while_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "lowercase")]
pub enum PackageOptionFsType {
    FILE,
    DIR,
    FS,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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
            highlight: vec![PackageParser {
                id: "python".to_string(),
                lang: "python".to_string(),
                extentions: vec![".py".__get(), ".cpy".__get()],
                ignore_files: None,
                files: None,
                nodes: HashMap::from([]),
                syntax: None,
            }],
            files: PackageFiles {
                extentions: vec![".py".__get(), ".cpy".__get()],
                ignore_files: None,
                files: None,
            },
            var: None,
            name: "Python 3.12".to_string(),
            meta: None,
            scheme: Default::default(),
            dependencies: vec![PackageDependency {
                typ: PackageDependencyTyp::PROGRAM,
                name: "python".to_string(),
                version: "3.12".to_string(),
                platform: None,
                version_check_command: Some("python -v".into()),
                level: PackageDependencyLevel::REQUIRED,
            }],
            startup: PackageStartup {
                options: Some(vec![PackageOption {
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
                }]),
                actions: Some(vec![PackageAction {
                    id: 0,
                    next: None,
                    if_: Some(vec![vec![IfStatementPart {
                        from: Some("#main-py".to_string()),
                        oper: IfStatementOperation::EQ,
                        value: Some(Val::BOOL(true)),
                    }]]),
                    on_error: Default::default(),
                    platform: None,
                    command: Some(vec![
                        PackageActionCommand {
                            platform: Some(Platform::windows()),
                            cwd: None,
                            shell: None,
                            env: None,
                            needed_exit_code: None,
                            command: "echo > main.py".to_string(),
                        },
                        PackageActionCommand {
                            platform: Some(Platform::unix_like()),
                            cwd: None,
                            shell: None,
                            env: None,
                            needed_exit_code: None,
                            command: "touch main.py".to_string(),
                        },
                    ]),
                }]),
            },
            components: Some(vec![PackageComponent {
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
            }]),
        }
    }

    pub fn rust() -> Self {
        Self {
            id: "opie.rust".to_string(),
            name: "Rust 1.94".to_string(),
            highlight: vec![],
            files: PackageFiles {
                extentions: vec![".rs".__get()],
                ignore_files: None,
                files: None,
            },
            meta: Some(PackageMeta {
                version: Some("1.0".__get()),
                authors: Some(vec!["OPIE".to_string()]),
                description: None,
                tags: None,
                source: None,
                typ: Some(PackageTyp::LANGUAGE),
                icon: None,
                license: None,
            }),
            scheme: Default::default(),
            dependencies: vec![
                PackageDependency {
                    typ: PackageDependencyTyp::PROGRAM,
                    name: "rustc".to_string(),
                    version: "1.94".to_string(),
                    platform: None,
                    version_check_command: None,
                    level: PackageDependencyLevel::REQUIRED,
                },
                PackageDependency {
                    typ: PackageDependencyTyp::PROGRAM,
                    name: "cargo".to_string(),
                    version: "".to_string(),
                    platform: None,
                    version_check_command: None,
                    level: PackageDependencyLevel::REQUIRED,
                },
            ],
            startup: PackageStartup {
                options: None,
                actions: None,
            },
            var: None,
            components: Some(vec![PackageComponent {
                id: "rust.lsp".to_string(),
                typ: PackageComponentTyp::COMPILER,
                program: "rust-analuzer".to_string(),
                platform: Platform::all(),
                languages: vec![".rs".__get()],
                priority: None,
                arguments: None,
                builtin_params: PackageComponentBuiltin {
                    is_builtin: true,
                    url: None,
                    path: None,
                    in_path: None,
                    min_version: None,
                    version_check_command: None,
                },
            }]),
        }
    }
}

pub type SharedPackages = Arc<Mutex<Vec<Package>>>;

impl ProjectTemplate {
    pub fn rust() -> Self {
        Self {
            id: "opie.rust".to_string(),
            name: "Cargo project".to_string(),
            schema: Default::default(),
            meta: Some(TemplateMeta {
                authors: vec!["OPIE".__get()],
                description: "".to_string(),
                icon: "rust.svg".to_string(),
            }),
            startup: TemplateStartup {
                sections: vec![Section {
                    id: 0,
                    label: "Cargo options".to_string(),
                    list: (true, true),
                    params: vec![PackageOption {
                        id: "type".to_string(),
                        title: "Crate type".to_string(),
                        typ: PackageOptionTyp {
                            typ: PackageOptionTypEnum::LIST,
                            fs_type: None,
                            list_type: Some(vec!["lib".__get(), "bin".__get()]),
                            placeholder: None,
                            required: None,
                            readonly: None,
                            validate: None,
                            fs_filter: None,
                            gen_min: None,
                            gen_max: None,
                        },
                        def: Some(Val::STRING("bin".to_string())),
                        while_: None,
                    }],
                }],
                actions: vec![
                    PackageAction {
                        id: 0,
                        next: None,
                        if_: Some(vec![vec![IfStatementPart {
                            from: Some("#0.type".__get()),
                            oper: IfStatementOperation::EQ,
                            value: Some(Val::STRING("lib".__get())),
                        }]]),
                        on_error: Default::default(),
                        platform: None,
                        command: Some(vec![PackageActionCommand {
                            platform: None,
                            cwd: None,
                            shell: None,
                            env: None,
                            needed_exit_code: None,
                            command: "cargo init --lib".to_string(),
                        }]),
                    },
                    PackageAction {
                        id: 0,
                        next: None,
                        if_: Some(vec![vec![IfStatementPart {
                            from: Some("#0.type".__get()),
                            oper: IfStatementOperation::EQ,
                            value: Some(Val::STRING("bin".__get())),
                        }]]),
                        on_error: Default::default(),
                        platform: None,
                        command: Some(vec![PackageActionCommand {
                            platform: None,
                            cwd: None,
                            shell: None,
                            env: None,
                            needed_exit_code: None,
                            command: "cargo init".to_string(),
                        }]),
                    },
                ],
                var: vec![],
            },
            packages_id: vec!["opie.rust".__get()],
            dependencies: vec![
                Dependency {
                    program: "rustc".to_string(),
                    platform: None,
                    level: DependencyLevel::CRITICAL,
                },
                Dependency {
                    program: "cargo".to_string(),
                    platform: None,
                    level: DependencyLevel::CRITICAL,
                },
            ],
            launches: vec![],
        }
    }
}
