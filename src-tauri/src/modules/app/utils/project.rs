use crate::modules::contexts::project::domain::entities::{
    Button, ButtonComponentType, ProjectTemplate,
};
use crate::modules::contexts::project::domain::values::{ButtonPos, CreateProjectResult};

pub mod functions {}

pub fn make_tasks(template: ProjectTemplate, values: CreateProjectResult) {}

pub fn make_buttons() -> Vec<Button> {
    let mut res = Vec::<Button>::new();
    res.push(Button {
        alt: "Project".to_string(),
        pos: ButtonPos::LeftTop,
        widget: String::from("FsAside"),
        component_type: ButtonComponentType::Light,
        order: 0,
        keys: String::new(),
        icon: "dir.svg".to_string(),
    });
    res.push(Button {
        alt: "Commit".to_string(),
        pos: ButtonPos::LeftTop,
        widget: String::from("CommitAside"),
        component_type: ButtonComponentType::Light,
        order: 1,
        keys: String::new(),
        icon: "commit.svg".to_string(),
    });

    res.push(Button {
        alt: "Debug".to_string(),
        pos: ButtonPos::LeftBottom,
        widget: String::from("DebugAside"),
        component_type: ButtonComponentType::Light,
        order: 0,
        keys: String::new(),
        icon: "debug.svg".to_string(),
    });
    res.push(Button {
        alt: "Launch".to_string(),
        pos: ButtonPos::LeftBottom,
        widget: String::from("LaunchAside"),
        component_type: ButtonComponentType::Light,
        order: 1,
        keys: String::new(),
        icon: "play.svg".to_string(),
    });
    res.push(Button {
        alt: "Terminal".to_string(),
        pos: ButtonPos::LeftBottom,
        widget: String::from("TerminalAside"),
        component_type: ButtonComponentType::Heavy,
        order: 2,
        keys: String::new(),
        icon: "terminal.svg".to_string(),
    });
    res.push(Button {
        alt: "Problems".to_string(),
        pos: ButtonPos::LeftBottom,
        widget: String::from("ProblemsAside"),
        component_type: ButtonComponentType::Light,
        order: 3,
        keys: String::new(),
        icon: "problems.svg".to_string(),
    });
    res.push(Button {
        alt: "Git".to_string(),
        pos: ButtonPos::LeftBottom,
        widget: String::from("GitAside"),
        component_type: ButtonComponentType::Light,
        order: 4,
        keys: String::new(),
        icon: "git.svg".to_string(),
    });
    res.push(Button {
        alt: "Project Settings".to_string(),
        pos: ButtonPos::RightTop,
        widget: String::from("SettingsAside"),
        component_type: ButtonComponentType::Light,
        order: 0,
        keys: String::new(),
        icon: "settings.svg".to_string(),
    });

    res
}
