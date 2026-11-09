use declarative::domain::{
    PackageAction, PackageActionCommand, PackageActionCommandType, PackageOptionTyp,
    PackageOptionTypEnum, Parameter,
};
use mount_core::domain::{Val, Var};
use mount_core::traits::PathPart;
use project::LAUNCH_COMPILE_SERVICE;
use project::launch::domain::{LaunchTemplate, LaunchTemplateReference};
use project::launch::traits::TLaunchCompileService;
use project::project::domain::Section;
use std::collections::HashMap;

#[test]
pub fn compile_object() {
    let references = LaunchTemplateReference {
        id: 0,
        template: ("template".to_string(), 0),
        icon: None,
        scheme: Default::default(),
        name: "template name".to_string(),
        results: HashMap::from([
            (
                -1,
                HashMap::from([("name".__get(), Val::STRING("run".__get()))]),
            ),
            (
                0,
                HashMap::from([("command".__get(), Val::STRING("cargo run".__get()))]),
            ),
        ]),
        before: None,
    };

    let vars = vec![
        Var::new("project_name", Val::STRING("project".__get())),
        Var::new("project_path", Val::STRING("path\\to\\project".__get())),
    ];
    let template = LaunchTemplate {
        id: 0,
        title: "Cargo template".to_string(),
        scheme: Default::default(),
        icon: None,
        sections: vec![Section {
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
                    placeholder: Some(String::from("type shell command here")),
                    required: None,
                    readonly: None,
                    validate: None,
                    fs_filter: None,
                    gen_min: None,
                    gen_max: None,
                },
                def: Some(Val::STRING("cargo run".__get())),
                while_: None,
            }],
        }],
        actions: vec![PackageAction {
            id: 0,
            next: None,
            if_: None,
            on_error: Default::default(),
            platform: None,
            command: Some(vec![PackageActionCommand {
                platform: None,
                cwd: None,
                shell: None,
                env: None,
                needed_exit_code: None,
                command: PackageActionCommandType::WithArgs {
                    command: "cargo run".__get(),
                    args: vec!["@{project_path}\\@{project_name}\\@{current_path}".__get()],
                },
            }]),
        }],
        functions: vec![],
    };

    let res =
        LAUNCH_COMPILE_SERVICE.compile_object(&references, &vars, &template, &references.results);

    assert!(res.is_some());
}
