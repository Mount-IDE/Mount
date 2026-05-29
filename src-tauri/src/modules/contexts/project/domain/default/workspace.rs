use crate::modules::contexts::project::domain::entities::{Button, ButtonComponentType};
use crate::modules::contexts::project::domain::values::ButtonPos;

pub fn buttons() -> Vec<Button> {
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
