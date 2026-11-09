use crate::launch::domain::{LaunchObject, LaunchTemplate, LaunchTemplateReference};
use crate::project::values::{ButtonPos, ProjectMeta, TemplateMeta};

use config::CONFIG_SERVICE;
use config::traits::TConfigService;
use declarative::domain::{
    ActionOnError, PackageAction, PackageActionCommand, PackageActionCommandType, PackageOptionTyp,
    PackageOptionTypEnum, Parameter, TaskCommand,
};
use mount_core::domain::platform::{Platform, PlatformType};
use mount_core::domain::{
    Dependency, DependencyLevel, IfStatementOperation, IfStatementPart, Path, Schema, Val, Var,
};
use mount_core::make_path;
use mount_core::traits::PathPart;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecentProject {
    pub name: String,
    pub path: Path,
    pub last_opened: u64,
    pub meta: ProjectMeta,
    pub(crate) packages: Vec<String>,
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkSpace {
    pub widgets: Vec<Widget>,
    pub buttons: Vec<Button>,
    pub opened_files: Vec<OpenedFile>,
    pub launch_references: Vec<LaunchTemplateReference>,
    pub launch_objects: Vec<LaunchObject>,
    pub launch_templates: Vec<LaunchTemplate>,
    pub current_launch: Option<i32>,
    #[serde(default)]
    pub aside_results: HashMap<String, HashMap<String, Val>>,
}

impl Default for WorkSpace {
    ///
    ///
    ///
    fn default() -> Self {
        Self {
            widgets: Vec::new(),
            buttons: Button::buttons(),
            opened_files: Vec::new(),
            launch_references: Vec::new(),
            launch_objects: Vec::new(),
            launch_templates: Vec::new(),
            current_launch: None,
            aside_results: HashMap::new(),
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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Widget {}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenedFile {
    #[serde(default)]
    name: String,
    path: Path,
    cursor: (u32, u32),
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
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

impl Button {
    pub fn buttons() -> Vec<Self> {
        vec![
            Button {
                // fs-aside
                pos: ButtonPos::LeftTop,
                widget: "FsAside".to_string(),
                component_type: ButtonComponentType::Light,
                order: 0,
                alt: "Project".to_string(),
                keys: "".to_string(),
                icon: "dir.svg".to_string(),
            },
            Button {
                // commit
                pos: ButtonPos::LeftTop,
                widget: "CommitAside".to_string(),
                component_type: ButtonComponentType::Light,
                order: 1,
                alt: "Commit".to_string(),
                keys: "".to_string(),
                icon: "commit.svg".to_string(),
            },
            Button {
                // Debug
                pos: ButtonPos::LeftBottom,
                widget: "DebugAside".to_string(),
                component_type: ButtonComponentType::Heavy,
                order: 0,
                alt: "Debug".to_string(),
                keys: "".to_string(),
                icon: "debug.svg".to_string(),
            },
            Button {
                // Launch
                pos: ButtonPos::LeftBottom,
                widget: "LaunchAside".to_string(),
                component_type: ButtonComponentType::Heavy,
                order: 1,
                alt: "Launch".to_string(),
                keys: "".to_string(),
                icon: "launch.svg".to_string(),
            },
            Button {
                // Terminal
                pos: ButtonPos::LeftBottom,
                widget: "Terminal".to_string(),
                component_type: ButtonComponentType::Heavy,
                order: 2,
                alt: "Terminal".to_string(),
                keys: "".to_string(),
                icon: "terminal.svg".to_string(),
            },
            Button {
                // Problems
                pos: ButtonPos::LeftBottom,
                widget: "ProblemsAside".to_string(),
                component_type: ButtonComponentType::Heavy,
                order: 3,
                alt: "Problems".to_string(),
                keys: "".to_string(),
                icon: "problems.svg".to_string(),
            },
            Button {
                // Git
                pos: ButtonPos::LeftBottom,
                widget: "GitAside".to_string(),
                component_type: ButtonComponentType::Heavy,
                order: 4,
                alt: "Git".to_string(),
                keys: "".to_string(),
                icon: "git.svg".to_string(),
            },
            Button {
                // Git
                pos: ButtonPos::RightTop,
                widget: "SettingsAside".to_string(),
                component_type: ButtonComponentType::Heavy,
                order: 0,
                alt: "Settings".to_string(),
                keys: "".to_string(),
                icon: "settings.svg".to_string(),
            },
            Button {
                // Git
                pos: ButtonPos::RightTop,
                widget: "DatabaseAside".to_string(),
                component_type: ButtonComponentType::Heavy,
                order: 0,
                alt: "Database".to_string(),
                keys: "".to_string(),
                icon: "db.svg".to_string(),
            },
        ]
    }
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum ButtonComponentType {
    #[default]
    Light,
    Heavy,
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    id: i32,
    // action: TaskAction
}

#[derive(Clone, Debug)]
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

impl _Task {
    #[allow(unused)]
    pub fn empty() -> Self {
        Self::SINGLE {
            commands: vec![],
            on_error: ActionOnError::CONTINUE,
        }
    }
}

///
///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectTemplate {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub schema: Schema,
    pub meta: Option<TemplateMeta>,
    #[serde(default)]
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

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Section {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub list: (bool, bool),
    #[serde(default)]
    pub params: Vec<Parameter>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub meta: Option<PackageMeta>,
    #[serde(default)]
    pub scheme: Schema,
    #[serde(default)]
    pub dependencies: Vec<PackageDependency>,
    #[serde(default)]
    pub startup: PackageStartup,
    #[serde(default)]
    pub var: Option<Vec<Var>>,
    #[serde(default)]
    pub files: PackageFiles,
    #[serde(default)]
    pub highlight: Vec<PackageParser>,
    #[serde(default)]
    pub components: Option<Vec<PackageComponent>>,
    #[serde(default)]
    pub toolchains: HashMap<String, PackageToolchain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageToolchain {
    id: String,
    version: String,
    repair: PackageRepair,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageRepairType {
    REPLACE,
    ADD,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageRepair {
    addition_type: PackageRepairType,
    components: HashMap<String, Repair>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repair {
    discovery: Option<DiscoveryRepair>,
    install: Option<InstallRepair>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryRepair {
    pub platform: Option<Platform>,
    pub is_builtin: Option<bool>,
    pub is_path_var: Option<bool>,
    pub path_to_program: Option<String>,
    pub version_check_command: Option<String>,
    pub needed_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallRepair {
    pub platform: Option<Platform>,
    pub version_check_command: Option<String>,
    pub needed_version: Option<String>,
    pub program: Option<String>,

    pub method: RepairMethod,

    pub url: Option<String>,
    pub shell: Option<String>,

    pub pm: Option<String>,
    pub packages: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RepairMethod {
    CURL,
    PACK,
    INTERNAL,
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
    pub fn get_command(&self, package: String) -> Option<String> {
        if let Some(true) = self.builtin_params.in_path {
            if self.builtin_params.is_builtin {
                return Some(self.program.clone());
            }
        } else {
            if let Some(p) = self.builtin_params.path.clone() {
                let Ok(dir) = CONFIG_SERVICE.get_data_dir() else {
                    return None;
                };
                let path = make_path![dir, "packages", package, p, self.program];
                return Some(path.get());
            }
        }
        Some(String::new())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
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
    options: Option<Vec<Parameter>>,
    pub(crate) actions: Option<Vec<PackageAction>>,
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
    #[allow(non_camel_case_types)]
    BUILD_SYSTEM,
}

impl Package {
    pub fn python() -> Self {
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
                options: Some(vec![Parameter {
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
                            command: PackageActionCommandType::Single("echo > main.py".to_string()),
                        },
                        PackageActionCommand {
                            platform: Some(Platform::unix_like()),
                            cwd: None,
                            shell: None,
                            env: None,
                            needed_exit_code: None,
                            command: PackageActionCommandType::Single("touch main.py".to_string()),
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
            toolchains: Default::default(),
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
            toolchains: Default::default(),
        }
    }

    pub fn general() -> Self {
        Self {
            id: "__opie.general__".to_string(),
            name: "General".to_string(),
            meta: None,
            scheme: Default::default(),
            dependencies: vec![],
            startup: Default::default(),
            var: None,
            files: PackageFiles {
                extentions: vec![
                    ".rs".__get(),
                    ".toml".__get(),
                    ".py".__get(),
                    ".h".__get(),
                    ".hpp".__get(),
                    ".c".__get(),
                    ".cpp".__get(),
                    ".ixx".__get(),
                    ".html".__get(),
                    ".css".__get(),
                    ".js".__get(),
                    ".jsx".__get(),
                    ".tsx".__get(),
                    ".ts".__get(),
                    ".mjs".__get(),
                    ".mts".__get(),
                    ".java".__get(),
                    ".csharp".__get(),
                    ".sh".__get(),
                    ".bash".__get(),
                    ".zsh".__get(),
                    ".cmd".__get(),
                    ".bat".__get(),
                    ".csv".__get(),
                    ".dart".__get(),
                    ".d".__get(),
                    ".gitignore".__get(),
                    ".gitattributes".__get(),
                    ".go".__get(),
                    ".kt".__get(),
                    ".md".__get(),
                    ".mdx".__get(),
                    ".php".__get(),
                    ".scala".__get(),
                    ".sln".__get(),
                    ".sql".__get(),
                    ".json".__get(),
                    ".vue".__get(),
                    ".yml".__get(),
                    ".yaml".__get(),
                    ".zig".__get(),
                    ".vim".__get(),
                    ".lua".__get(),
                    ".make".__get(),
                    ".ini".__get(),
                ],
                ignore_files: None,
                files: Some(vec!["CMakeLists.txt".__get(), "cargo.toml".__get()]),
            },
            highlight: vec![PackageParser {
                id: "rust".to_string(),
                lang: "rust".to_string(),
                extentions: vec![".rs".__get()],
                ignore_files: None,
                files: None,
                nodes: HashMap::from([
                    ("keyword".__get(), "other_kw_decl".__get()),
                    ("comment".__get(), "comment".__get()),
                    ("constructor".__get(), "spec_fn".__get()),
                    ("constant.builtin".__get(), "number".__get()),
                    ("funciton".__get(), "fn_decl".__get()),
                    ("type.builtin".__get(), "type_builtin".__get()),
                    ("comment.documentation".__get(), "doc_comment".__get()),
                    ("attribute".__get(), "attribute".__get()),
                    ("type".__get(), "type".__get()),
                    ("label".__get(), "label".__get()),
                    ("property".__get(), "property".__get()),
                    ("string".__get(), "string".__get()),
                    ("variable.builtin".__get(), "identificator".__get()),
                    ("funciton.macro".__get(), "macro".__get()),
                ]),
                syntax: Some(Syntax {
                    base_color: Some("#bcbec4".__get()),
                    colors: None,
                    tokens: Some(HashMap::from([
                        ("other_kw_decl".__get(), "#cf8e6d".__get()),
                        ("comment".__get(), "#7a7e85".__get()),
                        ("doc_comment".__get(), "#5f826b".__get()),
                        ("spec_fn".__get(), "#8d91dc".__get()),
                        ("number".__get(), "#2aacb8".__get()),
                        ("fn_decl".__get(), "#6aa2d7".__get()),
                        ("type_builtin".__get(), "#cf8e6d".__get()),
                        ("attribute".__get(), "#b3ae60".__get()),
                        ("type".__get(), "#a6bb77".__get()),
                        ("label".__get(), "#20999d".__get()),
                        ("property".__get(), "#c77dbb".__get()),
                        ("string".__get(), "#6aab73".__get()),
                        ("identifier".__get(), "#bcbec4".__get()),
                        ("macro".__get(), "#d5a563".__get()),
                    ])),
                }),
            }],
            components: None,
            toolchains: Default::default(),
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
                    params: vec![Parameter {
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
                            command: PackageActionCommandType::Single(
                                "cargo new --lib".to_string(),
                            ),
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
                            command: PackageActionCommandType::Single("cargo new".to_string()),
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
