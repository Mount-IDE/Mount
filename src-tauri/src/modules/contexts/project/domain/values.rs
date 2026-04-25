use std::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectMeta{
    authors: Vec<String>,
    description: String,
    license: Option<String>,
    group: String,
    tags: Vec<String>,
    
}

impl ProjectMeta{
    pub fn new()->Self{
        Self {
            authors: Vec::new(),
            description: String::new(),
            license: None,
            group: String::new(),
            tags: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PackageMeta {
    authors: Vec<String>,
    description: String,
}

impl PackageMeta{
    pub fn new()->Self{
        Self {
            authors: Vec::new(),
            description: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TemplateMeta {
    authors: Vec<String>,
    description: String,
    icon: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ParameterLabel{
    STR(String),
    COUPLE((String, String)),
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionCommand{
    shell: String,
    env: Option<Vec<(String, String)>>,
    command: ActionCommandIn
}

impl ActionCommand{
    pub fn new()->ActionCommand{
        Self{
            shell: String::new(),
            env: None,
            command: ActionCommandIn::Single(String::new())
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ActionCommandIn{
    Single(String),
    WithArgs(String, Vec<String>)
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ButtonPos{
    LeftTop,
    LeftBottom,
    RightTop,
}


