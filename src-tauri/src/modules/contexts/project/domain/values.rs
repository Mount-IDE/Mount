use std::process::Command;

pub struct ProjectMeta{
    authors: Vec<String>,
    description: String,
    license: Option<String>,
    group: String,
    tags: Vec<String>,
}

pub struct TemplateMeta {
    authors: Vec<String>,
    description: String,
    icon: String
}

pub enum ParameterLabel{
    STR(String),
    COUPLE((String, String)),
}

pub struct ActionCommand{
    shell: String,
    env: Option<Vec<(String, String)>>,
    command: ActionCommandIn
}

pub enum ActionCommandIn{
    Single(String),
    WithArgs(String, Vec<String>)
}




pub enum ButtonPos{
    LeftTop,
    LeftBottom,
    RightTop,
}