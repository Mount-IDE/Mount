use crate::modules::contexts::project::domain::entities::{Button, ButtonIcon, ProjectTemplate};
use crate::modules::contexts::project::domain::values::{ButtonPos, CreateProjectResult};

pub mod functions {}

pub fn make_tasks(template: ProjectTemplate, values: CreateProjectResult) {}

pub fn make_buttons() -> Vec<Button> {
    let mut res = Vec::<Button>::new();
    res.push(Button {
        alt: "Project".to_string(),
        pos: ButtonPos::LeftTop,
        widget: String::from("FsAside"),
        order: 0,
        keys: String::new(),
        icon: ButtonIcon::Couple("main".to_string(), "dir.svg".to_string()),
    });
    res.push(Button {
        alt: "Commit".to_string(),
        pos: ButtonPos::LeftTop,
        widget: String::from("CommitAside"),
        order: 1,
        keys: String::new(),
        icon: ButtonIcon::Couple("main".to_string(), "commit.svg".to_string()),
    });

    res.push(Button {
        alt: "Debug".to_string(),
        pos: ButtonPos::LeftBottom,
        widget: String::from("DebugAside"),
        order: 0,
        keys: String::new(),
        icon: ButtonIcon::Couple("main".to_string(), "debug.svg".to_string()),
    });
    res.push(Button {
        alt: "Launch".to_string(),
        pos: ButtonPos::LeftBottom,
        widget: String::from("LaunchAside"),
        order: 1,
        keys: String::new(),
        icon: ButtonIcon::Couple("main".to_string(), "play.svg".to_string()),
    });
    res.push(Button {
        alt: "Terminal".to_string(),
        pos: ButtonPos::LeftBottom,
        widget: String::from("TerminalAside"),
        order: 3,
        keys: String::new(),
        icon: ButtonIcon::Couple("main".to_string(), "terminal.svg".to_string()),
    });
    res.push(Button {
        alt: "Problems".to_string(),
        pos: ButtonPos::LeftBottom,
        widget: String::from("ProblemsAside"),
        order: 4,
        keys: String::new(),
        icon: ButtonIcon::Couple("main".to_string(), "problems.svg".to_string()),
    });
    res.push(Button {
        alt: "Git".to_string(),
        pos: ButtonPos::LeftBottom,
        widget: String::from("GitAside"),
        order: 5,
        keys: String::new(),
        icon: ButtonIcon::Couple("main".to_string(), "git.svg".to_string()),


    });res.push(Button {
        alt: "Project Settings".to_string(),
        pos: ButtonPos::RightTop,
        widget: String::from("SettingsAside"),
        order: 0,
        keys: String::new(),
        icon: ButtonIcon::Couple("main".to_string(), "settings.svg".to_string()),
    });

    vec![]
}
